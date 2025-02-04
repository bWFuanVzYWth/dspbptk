// TODO 整理测试用例
// FIXME 现在的warn之类的很乱，逐个检查log等级和输出内容

mod blueprint;
mod edit;
mod error;
mod md5;

use std::path::{Path, PathBuf};

use clap::Parser;
use log::{error, info, warn};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::error::DspbptkError;
use crate::error::DspbptkError::*;

fn read_content_file(path: &std::path::PathBuf) -> std::io::Result<Vec<u8>> {
    match std::fs::read(path) {
        Ok(content) => {
            info!("Read content from {}", path.display());
            Ok(content)
        }
        Err(why) => {
            error!("{:#?}Read content from {}", why, path.display());
            Err(why)
        }
    }
}

fn read_blueprint_file(path: &std::path::PathBuf) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

// fn write_blueprint_file(path: &std::path::PathBuf, blueprint: String) -> Result<(), DspbptkError> {
//     // create dir

//     std::fs::write(path, blueprint).map_err(|e| CanNotWriteFile {
//         path: std::ffi::OsString::from(path),
//         source: e,
//     })
// }

fn write_blueprint_file(path: &PathBuf, blueprint: String) -> Result<(), DspbptkError> {
    // 获取父目录
    let parent = match path.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };

    // 创建所有必要的父目录
    std::fs::create_dir_all(&parent).map_err(|e| DspbptkError::CanNotWriteFile {
        path: std::ffi::OsString::from(path),
        source: e,
    })?;

    // 写入文件内容到指定路径
    std::fs::write(path, blueprint).map_err(|e| DspbptkError::CanNotWriteFile {
        path: std::ffi::OsString::from(path),
        source: e,
    })
}

// fn write_content_file(path: &std::path::PathBuf, content: Vec<u8>) -> Result<(), DspbptkError> {
//     std::fs::write(path, content).map_err(|e| CanNotWriteFile {
//         path: std::ffi::OsString::from(path),
//         source: e,
//     })
// }

fn write_content_file(path: &PathBuf, content: Vec<u8>) -> Result<(), DspbptkError> {
    // 获取父目录
    let parent = match path.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };

    // 创建所有必要的父目录
    std::fs::create_dir_all(&parent).map_err(|e| DspbptkError::CanNotWriteFile {
        path: std::ffi::OsString::from(path),
        source: e,
    })?;

    // 写入文件内容到指定路径
    std::fs::write(path, content).map_err(|e| DspbptkError::CanNotWriteFile {
        path: std::ffi::OsString::from(path),
        source: e,
    })
}

// 检查是否为有效的blueprint文件
fn is_valid_blueprint(blueprint_content: &str, file_in: &std::path::PathBuf) -> bool {
    if blueprint_content.chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        info!("Not blueprint: {}", file_in.display());
        return false;
    } else {
        return true;
    }
}

// TODO 接入，现在的输出难以对应到正确的文件路径，用户反馈不足。
// 计算压缩率并返回统计信息
fn calculate_compression_rate(blueprint_in: &str, blueprint_out: &str) -> (usize, usize, f64) {
    let string_in_length = blueprint_in.len();
    let string_out_length = blueprint_out.len();
    let percent = (string_out_length as f64 / string_in_length as f64) * 100.0;
    info!(
        "{:3.3}%, {} -> {}",
        percent, string_in_length, string_out_length
    );
    (string_in_length, string_out_length, percent)
}

pub enum FileType {
    Unknown,
    Other,
    Txt,
    Content,
}

fn classify_file_type(entry: &std::path::PathBuf) -> FileType {
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

// 收集文件路径
fn collect_files(path_in: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path_in).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.into_path();
        match classify_file_type(&entry_path) {
            FileType::Txt => files.push(entry_path),
            FileType::Content => files.push(entry_path),
            _ => {}
        }
    }

    files
}

// 计算输出路径
fn generate_output_path(
    root_path_in: &Path,
    root_path_out: &Path,
    relative_path: &Path,
    file_type: &FileType,
) -> PathBuf {
    let extension = match file_type {
        FileType::Txt => "txt",
        FileType::Content => "content",
        _ => panic!("Unsupported file type"),
    };

    let relative_path = relative_path.strip_prefix(root_path_in).unwrap();
    // 把relative_path的文件后缀名替换为file_type的后缀名

    let mut output_path = if relative_path == Path::new("") {
        root_path_out.to_path_buf()
    } else {
        root_path_out.join(relative_path)
    };

    output_path.set_extension(extension);
    output_path
}

fn process_front_end(file_path_in: &PathBuf) -> Option<(String, Vec<u8>)> {
    match classify_file_type(file_path_in) {
        FileType::Txt => {
            // 1.1 读取blueprint文件
            let blueprint_str = match read_blueprint_file(file_path_in) {
                Ok(result) => {
                    info!("read from {}", file_path_in.display());
                    result
                }
                Err(why) => {
                    error!("{:#?}", why);
                    return None;
                }
            };

            if is_valid_blueprint(&blueprint_str, file_path_in) == false {
                error!(
                    "{:#?}",
                    NotBlueprint(std::ffi::OsString::from(file_path_in))
                );
                return None;
            }

            // 1.2 解析blueprint
            let blueprint_data = match blueprint::parse(&blueprint_str) {
                Ok(result) => result,
                Err(why) => {
                    error!("{:#?}", why);
                    return None;
                }
            };
            if blueprint_data.unknown.len() > 9 {
                warn!(
                    "{} unknown after blueprint: (QUITE A LOT)",
                    blueprint_data.unknown.len()
                );
            } else if blueprint_data.unknown.len() > 0 {
                warn!(
                    "{} unknown after blueprint: {:?}",
                    blueprint_data.unknown.len(),
                    blueprint_data.unknown
                );
            }

            // 1.3. 解码content
            let content_bin = match blueprint::content::bin_from_string(&blueprint_data.content) {
                Ok(result) => result,
                Err(why) => {
                    error!("{:#?}: decode from content", why);
                    return None;
                }
            };

            Some((blueprint_data.header.to_string(), content_bin))
        }
        FileType::Content => {
            let content_bin = match read_content_file(file_path_in) {
                Ok(result) => {
                    info!("read from {}", file_path_in.display());
                    result
                }
                Err(why) => {
                    error!(
                        "{:#?}",
                        CanNotReadFile {
                            path: std::ffi::OsString::from(file_path_in),
                            source: why
                        }
                    );
                    return None;
                }
            };

            const HEADER: &str = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,";
            Some((HEADER.to_string(), content_bin))
        }
        _ => {
            panic!("Fatal error: unknown file type");
        }
    }
}

fn process_middle_layer(
    file_path_out: &PathBuf,
    header_str: &str,
    content_bin: Vec<u8>,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
    should_sort_buildings: bool,
) {
    use blueprint::content::data_from_bin;
    use edit::{fix_buildings_index, sort_buildings};

    // 可以用来分析蓝图数据，备用
    // use blueprint::header::parse;
    // let header_data = parse(header_str)?;

    let mut content_data = match data_from_bin(&content_bin) {
        Ok(content) => content,
        Err(why) => {
            error!("{:#?}: decode from content", why);
            return;
        }
    };
    if content_data.unknown.len() > 9 {
        warn!(
            "{} unknown after content: (QUITE A LOT)",
            content_data.unknown.len()
        );
    } else if content_data.unknown.len() > 0 {
        warn!(
            "{} unknown after content: {:?}",
            content_data.unknown.len(),
            content_data.unknown
        );
    }

    // 3. 重新排序建筑
    if should_sort_buildings {
        sort_buildings(&mut content_data.buildings);
        fix_buildings_index(&mut content_data.buildings);
    }

    process_back_end(
        file_path_out,
        header_str,
        content_data,
        zopfli_options,
        output_type,
    );
}

fn process_back_end(
    file_path_out: &PathBuf,
    header_str: &str,
    content_data: blueprint::content::ContentData,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
) {
    use crate::blueprint::content::bin_from_data;
    use crate::blueprint::content::string_from_data;
    // 3. 后端：输出
    match output_type {
        FileType::Txt => {
            let content_string = match string_from_data(content_data, zopfli_options) {
                Ok(content) => content,
                Err(why) => {
                    error!("{:#?}: encode from {}", why, file_path_out.display());
                    return;
                }
            };
            let blueprint_string = blueprint::serialization(header_str, &content_string);

            match write_blueprint_file(&file_path_out, blueprint_string) {
                Ok(_) => {
                    info!("encode to {}", file_path_out.display());
                }
                Err(why) => {
                    error!("can not write file: {:?}", why)
                }
            }
        }
        FileType::Content => {
            let content_bin = match bin_from_data(content_data) {
                Ok(content) => content,
                Err(why) => {
                    error!("{:#?}: encode from {}", why, file_path_out.display());
                    return;
                }
            };
            match write_content_file(&file_path_out, content_bin) {
                Ok(_) => {
                    info!("encode to {}", file_path_out.display());
                }
                Err(why) => {
                    error!("can not write file: {}", why)
                }
            }
        }
        _ => {
            panic!("Fatal error: unknown file type"); // Should not reach here
        }
    }
}

fn process_files(
    files: Vec<PathBuf>,
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
    sort_buildings: bool,
) {
    // TODO 改成map(|path| result)，收集处理结果
    files.par_iter().for_each(|file_path_in| {
        let file_path_out = generate_output_path(path_in, path_out, file_path_in, output_type);

        match process_front_end(file_path_in) {
            Some((header_str, content_bin)) => {
                process_middle_layer(
                    &file_path_out,
                    &header_str,
                    content_bin,
                    zopfli_options,
                    output_type,
                    sort_buildings,
                );
            }
            None => {
                error!("can not process {}", file_path_in.display());
            }
        }
    });
}

// FIXME 直接传递args的引用
// 蓝图处理工作流
fn process_workflow(args: &Args) {
    let zopfli_options = configure_zopfli_options(args);
    let path_in = &args.input;
    let path_out = args.output.as_deref().unwrap_or(path_in);

    let files = collect_files(path_in);
    // TODO
    let output_type = match args.filetype.as_deref() {
        Some("txt") => FileType::Txt,
        Some("content") => FileType::Content,
        _ => panic!("Unsupported file type"),
    };

    let sort_buildings = args.sort_buildings;

    process_files(
        files,
        path_in,
        path_out,
        &zopfli_options,
        &output_type,
        sort_buildings,
    );
}

// 创建zopfli选项
fn configure_zopfli_options(args: &Args) -> zopfli::Options {
    let iteration_count = args
        .iteration_count
        .expect("Fatal error: unknown iteration_count");
    let iterations_without_improvement = args
        .iterations_without_improvement
        .expect("Fatal error: unknown iterations_without_improvement");
    let maximum_block_splits = args
        .maximum_block_splits
        .expect("Fatal error: unknown maximum_block_splits");

    zopfli::Options {
        iteration_count: std::num::NonZero::new(iteration_count)
            .expect("Fatal error: iteration_count must > 0"),
        iterations_without_improvement: std::num::NonZero::new(iterations_without_improvement)
            .expect("Fatal error: iterations_without_improvement must > 0"),
        maximum_block_splits: maximum_block_splits,
    }
}

// TODO 蓝图分析命令：分析蓝图文件，输出统计信息

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

    /// Output type: txt, content.
    #[clap(long, short, default_value = "txt")]
    filetype: Option<String>,

    /// Actions of edit blueprint.
    #[clap(short, long, num_args = 0..)]
    actions: Option<Vec<String>>,

    /// Sort buildings for smaller blueprint.
    #[clap(short, long, default_value = "true")]
    sort_buildings: bool,

    /// Compress arguments: zopfli iteration_count.
    #[clap(long, default_value = "256")]
    iteration_count: Option<u64>,

    /// Compress arguments: zopfli iterations_without_improvement.
    #[clap(long, default_value = "18446744073709551615")]
    iterations_without_improvement: Option<u64>,

    /// Compress arguments: zopfli maximum_block_splits.
    #[clap(long, default_value = "0")]
    maximum_block_splits: Option<u16>,
}

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    eprintln!("https://github.com/bWFuanVzYWth/dspbptk");
    let args = Args::parse();

    process_workflow(&args);
}
