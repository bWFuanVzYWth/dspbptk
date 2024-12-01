mod blueprint;
mod dybp;
mod md5;

use blueprint::error::DspbptkError;
use clap::Parser;
use log::{error, info, warn, trace};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(name = "DSPBPTK")]
#[command(version = "DSPBPTK: 0.1.0, DSP: 0.10.31.24632")]
#[command(about = "Dyson Sphere Program Blueprint Toolkit", long_about = None)]
struct Args {
    /// Input from file
    input: std::path::PathBuf,
}

fn recompress_blueprint(base64_string_in: &str) -> Result<String, DspbptkError> {
    // 蓝图字符串 -> 蓝图数据
    let bp_data_in = blueprint::parse(base64_string_in)?;


    // content子串 -> 二进制流
    let memory_stream_in = blueprint::decode_content(bp_data_in.content)?;

    // 二进制流 -> content数据
    let mut content = blueprint::content::parse(memory_stream_in.as_slice())?;

    // 蓝图处理
    blueprint::content::sort_buildings(&mut content.buildings);

    // content数据 -> 二进制流
    let memory_stream_out = blueprint::content::serialization(content);

    // 二进制流 -> content子串
    let content_out = blueprint::encode_content(memory_stream_out);

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

    // println!("{}",base64_string_in);

    let slice = &base64_string_in[0..12];
    if slice != "BLUEPRINT:0," {
        trace!("Not blueprint: {:?}", path_in);
        return;
    }

    let base64_string_out = match recompress_blueprint(&base64_string_in) {
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
                "Unsuccess: {:.3}%, {} -x-> {}, path_in:{:?}",
                percent, string_in_length, string_out_length, path_in
            )
        }
        std::cmp::Ordering::Equal => {
            warn!(
                "Unsuccess: {:.3}%, {} -x-> {}",
                percent, string_in_length, string_out_length
            )
        }
        std::cmp::Ordering::Greater => match std::fs::write(path_out, base64_string_out) {
            Ok(_) => {
                info!(
                    "Success: {:.3}%, {} ---> {}",
                    percent, string_in_length, string_out_length
                );
            }
            Err(why) => {
                error!("{:#?}: path_in: {:?}", why, path_in);
                return;
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

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    eprintln!("https://github.com/bWFuanVzYWth/dspbptk");
    let args = Args::parse();

    muti_threaded_work(&args.input);
    // single_threaded_work(&args.input, &args.input);
}
