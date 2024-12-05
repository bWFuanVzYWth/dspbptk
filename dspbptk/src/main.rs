mod blueprint;
mod md5;

use clap::Parser;
use log::{debug, error, info, warn};

use blueprint::error::BlueprintError;

fn recompress_blueprint(
    blueprint_in: &str,
) -> Result<(String, Vec<String>), BlueprintError<String>> {
    use blueprint::content;
    use blueprint::content::{data_from_string, string_from_data};

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

    content::fix_buildings_index(&mut content_data.buildings);

    let content_string = string_from_data(content_data)?;

    let blueprint_string = blueprint::serialization(blueprint_data.header, &content_string);

    Ok((blueprint_string, warnings))
}

fn blueprint_from_content_rw(file_in: &std::path::PathBuf, file_out: &std::path::PathBuf) {
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

    let content_string = match blueprint::content::string_from_bin(content_bin) {
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

fn recompress_blueprint_rw(file_in: &std::path::PathBuf, file_out: &std::path::PathBuf) {
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

    let blueprint_out = match recompress_blueprint(&blueprint_in) {
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

fn cook(path_in: &std::path::PathBuf, path_out: &std::path::PathBuf) {
    use rayon::prelude::*;
    use walkdir::WalkDir;

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
        recompress_blueprint_rw(file_in, &file_out);
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
        blueprint_from_content_rw(file_in, &file_out);
    });
}

#[derive(Parser, Debug)]
#[command(name = "DSPBPTK")]
#[command(version = "dspbptk0.2.0-dsp0.10.31.24632")]
#[command(about = "Dyson Sphere Program Blueprint Toolkit", long_about = None)]
struct Args {
    /// Input from file/dir. (*.txt *.content dir/)
    input: std::path::PathBuf,

    /// Output to file/dir.
    #[arg(long, short)]
    output: Option<std::path::PathBuf>,
}

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    eprintln!("https://github.com/bWFuanVzYWth/dspbptk");
    let args = Args::parse();

    let path_in = &args.input;
    let path_out = match &args.output {
        None => path_in,
        Some(path) => path,
    };

    cook(path_in, path_out);
}
