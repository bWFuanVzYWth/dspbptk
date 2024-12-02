mod blueprint;
mod md5;

use clap::Parser;
use log::{debug, error, info, trace, warn};
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

fn single_threaded_work(path_in: &std::path::PathBuf, path_out: &std::path::PathBuf) {
    let base64_string_in = match std::fs::read_to_string(path_in) {
        Ok(result) => {
            debug!("std::fs::read_to_string match Ok: path_in: {:?}", path_in);
            result
        }
        Err(why) => {
            error!("{:#?}: path_in: {:?}", why, path_in);
            return;
        }
    };

    // TODO 补点debug和trace
    if (&base64_string_in).chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        debug!("Not blueprint: {:?}", path_in);
        return;
    }

    let base64_string_out = match recompress_blueprint(&base64_string_in) {
        Ok((base64_string, warnings)) => {
            warnings
                .iter()
                .for_each(|warning| warn!("{}: path_in: {:?}", warning, path_in));
            debug!("recompress_blueprint match Ok: path_in: {:?}", path_in);
            base64_string
        }
        Err(why) => {
            error!("{:#?}: path_in: {:?}", why, path_in);
            return;
        }
    };

    let string_in_length = base64_string_in.len();
    let string_out_length = base64_string_out.len();
    let percent = (string_out_length as f64 / string_in_length as f64) * 100.0;

    // let order = std::cmp::Ordering::Greater;
    let order = string_in_length.cmp(&string_out_length);

    match order {
        std::cmp::Ordering::Less => {
            warn!(
                "Fail: {:3.3}%, {} -x-> {}, path_in:{:?}",
                percent, string_in_length, string_out_length, path_in
            );
        }
        std::cmp::Ordering::Equal => {
            warn!(
                "Fail: {:3.3}%, {} -x-> {}, path_in:{:?}",
                percent, string_in_length, string_out_length, path_in
            );
        }
        std::cmp::Ordering::Greater => match std::fs::write(path_out, base64_string_out) {
            Ok(_) => {
                info!(
                    "Success: {:3.3}%, {} ---> {}",
                    percent, string_in_length, string_out_length
                );
            }
            Err(why) => {
                error!("{:#?}: path_in: {:?}", why, path_in);
            }
        },
    }
}

fn is_txt(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".txt"))
        .unwrap_or(false)
}

fn is_json(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".json"))
        .unwrap_or(false)
}

fn is_content(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".content"))
        .unwrap_or(false)
}

fn muti_threaded_work(path_in: &std::path::PathBuf, path_out: &std::path::PathBuf) {
    let mut path_txts = Vec::new();
    for entry in WalkDir::new(path_in)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if is_txt(&entry) {
            path_txts.push(entry.into_path());
        }
    }

    path_txts.par_iter().for_each(|path_txt| {
        single_threaded_work(path_txt, path_txt);
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

    let path_in = &args.input;
    let path_out = match &args.output {
        Some(args_output) => {
            if let Ok(meta_data_in) = std::fs::metadata(path_in) {
                if let Ok(meta_data_out) = std::fs::metadata(args_output) {
                    if meta_data_in.is_dir() && meta_data_out.is_file() {
                        panic!("Fatal error: Cannot input directory when output to file!");
                    }
                }
            }
            args_output
        }
        None => path_in,
    };

    // TODO 文件输出
    muti_threaded_work(path_in, path_in);
}
