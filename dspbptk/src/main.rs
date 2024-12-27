mod blueprint;
mod md5;

use clap::Parser;
use log::{debug, error, info, warn};

use blueprint::error::BlueprintError;

fn recompress_blueprint(
    blueprint_in: &str,
    zopfli_options: &zopfli::Options,
) -> Result<(String, Vec<String>), BlueprintError<String>> {
    use blueprint::content::{data_from_string, string_from_data};
    use blueprint::edit::{fix_buildings_index, sort_buildings};

    let mut warnings = Vec::new();

    let blueprint_data = blueprint::parse(blueprint_in)?;
    if blueprint_data.unknown.len() > 9 {
        warnings.push(format!(
            "{} unknown after blueprint: (QUITE A LOT)",
            blueprint_data.unknown.len()
        ));
    } else if blueprint_data.unknown.len() > 0 {
        warnings.push(format!(
            "{} unknown after blueprint: {:?}",
            blueprint_data.unknown.len(),
            blueprint_data.unknown
        ));
    }

    let mut content_data = data_from_string(blueprint_data.content)?;

    if content_data.unknown.len() > 9 {
        warnings.push(format!(
            "{} unknown after content: (QUITE A LOT)",
            content_data.unknown.len()
        ));
    } else if content_data.unknown.len() > 0 {
        warnings.push(format!(
            "{} unknown after content: {:?}",
            content_data.unknown.len(),
            content_data.unknown
        ));
    }

    sort_buildings(&mut content_data.buildings);
    fix_buildings_index(&mut content_data.buildings);

    let content_string = string_from_data(content_data, zopfli_options)?;

    let blueprint_string = blueprint::serialization(blueprint_data.header, &content_string);

    Ok((blueprint_string, warnings))
}

fn blueprint_from_content_rw(
    file_in: &std::path::PathBuf,
    file_out: &std::path::PathBuf,
    zopfli_options: &zopfli::Options,
) {
    let content_bin = match std::fs::read(file_in) {
        Ok(result) => {
            debug!("Ok: read from {}", file_in.display());
            result
        }
        Err(why) => {
            error!("{:#?}: read from {}", why, file_in.display());
            return;
        }
    };

    let content_string = match blueprint::content::string_from_bin(content_bin, zopfli_options) {
        Ok(content) => {
            debug!("Ok: encode from {}", file_in.display());
            content
        }
        Err(why) => {
            error!("{:#?}: encode from {}", why, file_in.display());
            return;
        }
    };

    const HEADER: &str = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,";
    let blueprint_string = blueprint::serialization(HEADER, &content_string);

    match std::fs::write(file_out, blueprint_string) {
        Ok(_) => {
            info!(
                "Ok: encode from {} to {}",
                file_in.display(),
                file_out.display()
            );
        }
        Err(why) => {
            error!("{:#?}: encode from {}", why, file_out.display());
        }
    }
}

fn recompress_blueprint_rw(
    file_in: &std::path::PathBuf,
    file_out: &std::path::PathBuf,
    zopfli_options: &zopfli::Options,
) {
    let blueprint_in = match std::fs::read_to_string(file_in) {
        Ok(result) => {
            debug!("Ok: read from {}", file_in.display());
            result
        }
        Err(why) => {
            error!("{:#?}: read from {}", why, file_in.display());
            return;
        }
    };

    // 快速排除非蓝图txt，尽早返回
    if (&blueprint_in).chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        debug!("Not blueprint: {}", file_in.display());
        return;
    }

    let blueprint_out = match recompress_blueprint(&blueprint_in, zopfli_options) {
        Ok((blueprint, warnings)) => {
            warnings
                .iter()
                .for_each(|warning| warn!("{} from {}", warning, file_in.display()));
            debug!("Ok: recompress from {}", file_in.display());
            blueprint
        }
        Err(why) => {
            error!("{:#?}: recompress from {}", why, file_in.display());
            return;
        }
    };

    let string_in_length = blueprint_in.len();
    let string_out_length = blueprint_out.len();
    let percent = (string_out_length as f64 / string_in_length as f64) * 100.0;

    match std::fs::create_dir_all(file_out.parent().unwrap(/*impossible*/)) {
        Ok(_) => {
            debug!("Ok: create dir {}", file_out.display());
        }
        Err(why) => {
            error!("{:#?}: create dir {}", why, file_out.display());
        }
    };

    match std::fs::write(file_out, blueprint_out) {
        Ok(_) => {
            info!(
                "Ok: {:3.3}%, {} -> {}, from {} to {}",
                percent,
                string_in_length,
                string_out_length,
                file_in.display(),
                file_out.display()
            );
        }
        Err(why) => {
            error!("{:#?}: write to {}", why, file_out.display());
        }
    }
}

pub enum FileType {
    Unknown,
    Other,
    Txt,
    Content,
}

fn file_type(entry: &std::path::PathBuf) -> FileType {
    if let Some(extension) = entry.extension() {
        match extension.to_str() {
            Some("txt") => FileType::Txt,
            Some("content") => FileType::Content,
            _ => FileType::Other,
        }
    } else {
        FileType::Unknown
    }
}

fn cook(args: &Args) {
    use rayon::prelude::*;
    use walkdir::WalkDir;

    // TODO 考虑默认行为，这样是否合适
    let path_in = &args.input;
    let path_out = match &args.output {
        None => path_in,
        Some(path) => path,
    };

    // 防呆不防傻，希望用户自行检查压缩参数是否合理
    let iteration_count = args
        .iteration_count
        .expect("Fatal error: unknown iteration_count");
    let iterations_without_improvement = args
        .iterations_without_improvement
        .expect("Fatal error: unknown iterations_without_improvement");
    let maximum_block_splits = args
        .maximum_block_splits
        .expect("Fatal error: unknown maximum_block_splits");
    let zopfli_options = zopfli::Options {
        iteration_count: std::num::NonZero::new(iteration_count)
            .expect("Fatal error: iteration_count must > 0"),
        iterations_without_improvement: std::num::NonZero::new(iterations_without_improvement)
            .expect("Fatal error: iterations_without_improvement must > 0"),
        maximum_block_splits: maximum_block_splits,
    };

    let mut maybe_blueprint_paths = Vec::new();
    let mut maybe_content_paths = Vec::new();

    for entry in WalkDir::new(path_in).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.into_path();
        match file_type(&entry_path) {
            FileType::Txt => maybe_blueprint_paths.push(entry_path),
            FileType::Content => maybe_content_paths.push(entry_path),
            _ => {}
        }
    }

    maybe_blueprint_paths.par_iter().for_each(|file_in| {
        let relative_path_in = file_in.strip_prefix(path_in).unwrap(/*impossible*/);
        debug!("relative_path_in = {}", relative_path_in.display());
        let mut file_out = path_out.clone();
        if relative_path_in != std::path::Path::new("").as_os_str() {
            file_out = file_out.join(relative_path_in);
        }
        debug!("file_out = {}", file_out.display());
        recompress_blueprint_rw(file_in, &file_out, &zopfli_options);
    });

    maybe_content_paths.par_iter().for_each(|file_in| {
        let relative_path_in = file_in.strip_prefix(path_in).unwrap(/*impossible*/);
        debug!("relative_path_in = {}", relative_path_in.display());
        let mut file_out = path_out.clone();
        if relative_path_in != std::path::Path::new("").as_os_str() {
            file_out = file_out.join(relative_path_in);
        }
        file_out = file_out.with_extension("txt");
        debug!("file_out = {}", file_out.display());
        blueprint_from_content_rw(file_in, &file_out, &zopfli_options);
    });
}

#[derive(Parser, Debug)]
#[command(
    version = "dspbptk0.2.0-dsp0.10.31.24632",
    author = "bWFuanVzYWth",
    about = "Dyson Sphere Program Blueprint Toolkit"
)]
struct Args {
    // TODO 多文件同时输入
    /// Input from file/dir. (*.txt *.content dir/)
    input: std::path::PathBuf,

    /// Output to file/dir. (*.txt dir/)
    #[clap(long, short)]
    output: Option<std::path::PathBuf>,

    /// Actions of edit blueprint.
    #[clap(short, long, num_args = 0..)]
    action: Option<Vec<String>>,

    // TODO 注释
    #[clap(long, default_value = "256")]
    iteration_count: Option<u64>,

    #[clap(long, default_value = "18446744073709551615")]
    iterations_without_improvement: Option<u64>,

    #[clap(long, default_value = "0")]
    maximum_block_splits: Option<u16>,
}

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    eprintln!("https://github.com/bWFuanVzYWth/dspbptk");
    let args = Args::parse();

    cook(&args);
}
