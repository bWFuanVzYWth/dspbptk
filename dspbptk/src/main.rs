mod blueprint;
mod md5;

use clap::Parser;
use log::{debug, error, info, warn};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use blueprint::error::DspbptkError;

// FIXME recompress content
fn recompress_blueprint(
    base64_string_in: &str,
) -> Result<(String, Vec<String>), DspbptkError<String>> {
    let mut warnings = Vec::new();

    // 蓝图字符串 -> 蓝图数据
    let bp_data_in = blueprint::parse(base64_string_in)?;
    if bp_data_in.unknown.len() > 0 {
        if bp_data_in.unknown.len() > 64 {
            warnings.push(format!(
                "{} unknown after blueprint: (QUITE A LOT)",
                bp_data_in.unknown.len()
            ));
        } else {
            warnings.push(format!(
                "{} unknown after blueprint: {:?}",
                bp_data_in.unknown.len(),
                bp_data_in.unknown
            ));
        }
    };

    // content子串 -> 二进制流
    let memory_stream_in = blueprint::decode_content(bp_data_in.content)?;

    // 二进制流 -> content数据
    let mut content = blueprint::content::parse(memory_stream_in.as_slice())?;
    if content.unknown.len() > 0 {
        if content.unknown.len() > 64 {
            warnings.push(format!(
                "{} unknown after content: (QUITE A LOT)",
                content.unknown.len()
            ));
        } else {
            warnings.push(format!(
                "{} unknown after content: {:?}",
                content.unknown.len(),
                content.unknown
            ));
        }
    };

    // 蓝图处理
    blueprint::content::fix_buildings_index(&mut content.buildings);

    // content数据 -> 二进制流
    let memory_stream_out = blueprint::content::serialization(content);

    // 二进制流 -> content子串
    let content_out = blueprint::encode_content(memory_stream_out)?;

    // 蓝图数据 -> 蓝图字符串
    let base64_string_out = blueprint::serialization(bp_data_in.header, &content_out);

    Ok((base64_string_out, warnings))
}

fn recompress_blueprint_with_fs_io(file_in: &std::path::PathBuf, file_out: &std::path::PathBuf) {
    let base64_string_in = match std::fs::read_to_string(file_in) {
        Ok(result) => {
            debug!(
                "std::fs::read_to_string match Ok: file_in: \"{}\"",
                file_in.display()
            );
            result
        }
        Err(why) => {
            error!("{:#?}: file_in: \"{}\"", why, file_in.display());
            return;
        }
    };

    // 快速排除非蓝图txt，尽早返回
    if (&base64_string_in).chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        debug!("Not blueprint: \"{}\"", file_in.display());
        return;
    }

    let base64_string_out = match recompress_blueprint(&base64_string_in) {
        Ok((base64_string, warnings)) => {
            warnings
                .iter()
                .for_each(|warning| warn!("{}: file_in: \"{}\"", warning, file_in.display()));
            debug!(
                "recompress_blueprint match Ok: file_in: \"{}\"",
                file_in.display()
            );
            base64_string
        }
        Err(why) => {
            error!("{:#?}: file_in: \"{}\"", why, file_in.display());
            return;
        }
    };

    let string_in_length = base64_string_in.len();
    let string_out_length = base64_string_out.len();
    let percent = (string_out_length as f64 / string_in_length as f64) * 100.0;

    let order = string_in_length.cmp(&string_out_length);

    match order {
        std::cmp::Ordering::Less => {
            warn!(
                "Fail: {:3.3}%, {} -x-> {}, file_in:\"{}\"",
                percent,
                string_in_length,
                string_out_length,
                file_in.display()
            );
        }
        std::cmp::Ordering::Equal => {
            warn!(
                "Fail: {:3.3}%, {} -x-> {}, file_in:\"{}\"",
                percent,
                string_in_length,
                string_out_length,
                file_in.display()
            );
        }
        std::cmp::Ordering::Greater => {
            match std::fs::create_dir_all(file_out.parent().unwrap(/*impossible*/)) {
                Ok(_) => {
                    debug!(
                        "std::fs::create_dir_all match Ok: file_out:{}",
                        file_out.display()
                    );
                }
                Err(why) => {
                    error!("{:#?}: file_out: \"{}\"", why, file_out.display());
                }
            };
            match std::fs::write(file_out, base64_string_out) {
                Ok(_) => {
                    info!(
                        "Success: {:3.3}%, {} ---> {}",
                        percent, string_in_length, string_out_length
                    );
                }
                Err(why) => {
                    error!("{:#?}: file_out: \"{}\"", why, file_out.display());
                }
            }
        }
    }
}

fn is_txt(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".txt"))
        .unwrap_or(false)
}

fn _is_json(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".json"))
        .unwrap_or(false)
}

fn _is_bpraw(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".bpraw"))
        .unwrap_or(false)
}

fn cook_blueprint_directory_with_fs_io(
    path_in: &std::path::PathBuf,
    path_out: &std::path::PathBuf,
) {
    let mut path_maybe_blueprints = Vec::new();
    for entry in WalkDir::new(path_in).into_iter().filter_map(|e| e.ok()) {
        // FIXME 还要检查是不是文件
        if is_txt(&entry) {
            path_maybe_blueprints.push(entry.into_path());
        }
    }

    path_maybe_blueprints.par_iter().for_each(|file_in| {
        let relative_path_in = file_in.strip_prefix(path_in).unwrap(/*impossible*/);
        let file_out = path_out.join(relative_path_in);
        recompress_blueprint_with_fs_io(file_in, &file_out);
    });
}

#[derive(Parser, Debug)]
#[command(name = "DSPBPTK")]
#[command(version = "DSPBPTK: 0.1.0, DSP: 0.10.31.24632")]
#[command(about = "Dyson Sphere Program Blueprint Toolkit", long_about = None)]
struct Args {
    /// Input from file. Support *.txt *.json *.content
    input: std::path::PathBuf,

    /// Input to file. Support *.txt *.json *.content
    #[arg(long, short)]
    output: Option<std::path::PathBuf>,
}

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    eprintln!("https://github.com/bWFuanVzYWth/dspbptk");
    let args = Args::parse();

    // 快速排除不正常参数，尽早主动崩溃并报错
    let path_in = &args.input;
    let path_out = match &args.output {
        None => path_in,
        Some(path) => path,
    };

    cook_blueprint_directory_with_fs_io(path_in, path_out);
}
