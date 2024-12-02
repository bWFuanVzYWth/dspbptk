mod blueprint;
mod dybp;
mod md5;

use clap::Parser;
use log::{debug, error, info, trace, warn};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use blueprint::error::DspbptkError;

// FIXME recompress content
fn recompress_blueprint(
    base64_string_in: &str,
    path_in: &std::path::PathBuf, // for warning
) -> Result<String, DspbptkError<&str>> {
    // 蓝图字符串 -> 蓝图数据
    let bp_data_in = blueprint::parse(base64_string_in)?;
    if bp_data_in.unknown.len() > 0 {
        if bp_data_in.unknown.len() > 256 {
            warn!(
                "{} Unknown after blueprint(QUITE A LOT), path_in: {:?}",
                bp_data_in.unknown.len(),
                path_in
            )
        } else {
            warn!(
                "{} Unknown after blueprint: {:?}, path_in: {:?}",
                bp_data_in.unknown.len(),
                bp_data_in.unknown,
                path_in
            )
        }
    };

    // content子串 -> 二进制流
    let memory_stream_in = blueprint::decode_content(bp_data_in.content)?;

    // 二进制流 -> content数据
    let mut content = blueprint::content::parse(memory_stream_in.as_slice())?;
    if content.unknown.len() > 0 {
        if content.unknown.len() > 256 {
            warn!(
                "{} Unknown after content(QUITE A LOT), path_in: {:?}",
                content.unknown.len(),
                path_in
            );
        } else {
            warn!(
                "{} Unknown after content: {:?}, path_in: {:?}",
                content.unknown.len(),
                content.unknown,
                path_in
            );
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

    Ok(base64_string_out)
}

fn single_threaded_work(path_in: &std::path::PathBuf, path_out: &std::path::PathBuf) {
    let base64_string_in = match std::fs::read_to_string(path_in) {
        Ok(result) => result,
        Err(why) => {
            error!("{:#?}: path_in: {:?}", why, path_in);
            return;
        }
    };

    if (&base64_string_in).chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        debug!("Not blueprint: {:?}", path_in);
        return;
    }

    let base64_string_out = match recompress_blueprint(&base64_string_in, path_in) {
        Ok(result) => result,
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
                "Fail: {:3.3}%, {} -x-> {}",
                percent, string_in_length, string_out_length
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

fn muti_threaded_work(path_dir: &std::path::PathBuf) {
    let mut path_txts = Vec::new();
    for entry in WalkDir::new(path_dir)
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
        Some(args_output) => args_output,
        None => path_in,
    };

    muti_threaded_work(&args.input);
}
