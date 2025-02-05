// TODO 整理测试用例

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
use crate::error::DspbptkInfo::*;
use crate::error::DspbptkWarn::*;

use crate::blueprint::header::HeaderData;
use blueprint::content::ContentData;

fn read_content_file(path: &std::path::PathBuf) -> Option<Vec<u8>> {
    match std::fs::read(path) {
        Ok(content) => {
            info!("{:?}", ReadFile(std::ffi::OsString::from(path)));
            Some(content)
        }
        Err(why) => {
            error!(
                "{:?}",
                CanNotReadFile {
                    path: std::ffi::OsString::from(path),
                    source: why,
                }
            );
            None
        }
    }
}

fn read_blueprint_file(path: &std::path::PathBuf) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            info!("{:?}", ReadFile(std::ffi::OsString::from(path)));
            Some(content)
        }
        Err(why) => {
            error!(
                "{:?}",
                CanNotReadFile {
                    path: std::ffi::OsString::from(path),
                    source: why,
                }
            );
            None
        }
    }
}

fn is_valid_blueprint<'a>(blueprint_content: &str, file_in: &std::path::PathBuf) -> Option<()> {
    if blueprint_content.chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        warn!("{:?}", NotBlueprint(std::ffi::OsString::from(file_in)));
        None
    } else {
        Some(())
    }
}

fn read_file(path: &PathBuf) -> Option<BlueprintKind> {
    use crate::BlueprintKind::*;
    match classify_file_type(path) {
        FileType::Txt => {
            let blueprint_string = read_blueprint_file(path)?;
            is_valid_blueprint(&blueprint_string, path)?;
            Some(Blueprint(blueprint_string))
        }
        FileType::Content => {
            let content_bin = read_content_file(path)?;
            Some(Content(content_bin))
        }
        _ => None,
    }
}

fn creat_father_dir(path: &PathBuf) -> Option<()> {
    let parent = match path.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    match std::fs::create_dir_all(&parent) {
        Err(why) => {
            error!(
                "{:?}",
                CanNotWriteFile {
                    path: std::ffi::OsString::from(path),
                    source: why,
                }
            );
            None
        }
        Ok(_) => Some(()),
    }
}

fn write_blueprint_file(path: &PathBuf, blueprint: String) -> Option<()> {
    creat_father_dir(path)?;
    match std::fs::write(path, blueprint) {
        Ok(_) => Some(()),
        Err(why) => {
            error!(
                "{:?}",
                CanNotWriteFile {
                    path: std::ffi::OsString::from(path),
                    source: why,
                }
            );
            None
        }
    }
}

fn write_content_file(path: &PathBuf, content: Vec<u8>) -> Option<()> {
    creat_father_dir(path)?;
    match std::fs::write(path, content) {
        Ok(_) => Some(()),
        Err(why) => {
            error!(
                "{:?}",
                CanNotWriteFile {
                    path: std::ffi::OsString::from(path),
                    source: why,
                }
            );
            None
        }
    }
}

fn write_file(path: &PathBuf, blueprint_kind: BlueprintKind) -> Option<()> {
    match blueprint_kind {
        BlueprintKind::Blueprint(blueprint) => write_blueprint_file(path, blueprint),
        BlueprintKind::Content(content) => write_content_file(path, content),
    }
}

// TODO 接入，现在的输出难以对应到正确的文件路径，用户反馈不足。
// 计算压缩率并返回统计信息
fn calculate_compression_rate(blueprint_in: &str, blueprint_out: &str) -> (usize, usize, f64) {
    let string_in_length = blueprint_in.len();
    let string_out_length = blueprint_out.len();
    let percent = (string_out_length as f64 / string_in_length as f64) * 100.0;
    // FIXME 改结构化
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

    let mut output_path = if relative_path == Path::new("") {
        root_path_out.to_path_buf()
    } else {
        root_path_out.join(relative_path)
    };

    output_path.set_extension(extension);
    output_path
}

pub enum BlueprintKind {
    Blueprint(String),
    Content(Vec<u8>),
}

// FIXME 把文件读写提出去
fn process_front_end<'a>(blueprint: BlueprintKind) -> Option<(HeaderData, ContentData)> {
    match blueprint {
        BlueprintKind::Blueprint(blueprint_string) => {
            let blueprint_data = blueprint::parse(&blueprint_string)?;
            let content_bin = blueprint::content::bin_from_string(&blueprint_data.content)?;
            let content_data = blueprint::content::data_from_bin(&content_bin)?;
            let header_data = blueprint::header::parse(&blueprint_data.header)?;
            Some((header_data, content_data))
        }
        BlueprintKind::Content(content_bin) => {
            let content_data = blueprint::content::data_from_bin(&content_bin)?;
            const HEADER: &str = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,";
            let header_data = blueprint::header::parse(HEADER)?;
            Some((header_data, content_data))
        }
        _ => None,
    }
}

fn process_middle_layer(
    header_data_in: HeaderData,
    content_data_in: ContentData,
    should_sort_buildings: bool,
) -> Option<(HeaderData, ContentData)> {
    use edit::{fix_buildings_index, sort_buildings};

    // 这里应该是唯一一处深拷贝，这是符合直觉的，可以极大优化用户的使用体验
    // FIXME 传入解析过的头，而不是字符串
    let header_data_out = header_data_in.clone();
    let mut content_data_out = content_data_in.clone();

    // edit
    if should_sort_buildings {
        sort_buildings(&mut content_data_out.buildings);
        fix_buildings_index(&mut content_data_out.buildings);
    }

    Some((header_data_out, content_data_out))
}

fn process_back_end(
    header_data: &HeaderData,
    content_data: &ContentData,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
) -> Option<BlueprintKind> {
    use crate::blueprint::content::bin_from_data;
    use crate::blueprint::content::string_from_data;
    match output_type {
        FileType::Txt => {
            let header_string = blueprint::header::serialization(header_data);
            let content_string = string_from_data(content_data, zopfli_options)?;
            Some(BlueprintKind::Blueprint(blueprint::serialization(
                &header_string,
                &content_string,
            )))
        }
        FileType::Content => Some(BlueprintKind::Content(bin_from_data(content_data))),
        _ => None,
    }
}

fn process_one_file(
    file_path_in: &PathBuf,
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
    sort_buildings: bool,
) -> Option<()> {
    let blueprint_kind_in = read_file(file_path_in)?;

    let (header_data_in, content_data_in) = process_front_end(blueprint_kind_in)?;
    let (header_data_out, content_data_out) =
        process_middle_layer(header_data_in, content_data_in, sort_buildings)?;
    let blueprint_kind_out = process_back_end(
        &header_data_out,
        &content_data_out,
        &zopfli_options,
        &output_type,
    )?;

    let file_path_out = generate_output_path(path_in, path_out, file_path_in, output_type);
    write_file(&file_path_out, blueprint_kind_out)
}

fn process_all_files(
    files: Vec<PathBuf>,
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
    sort_buildings: bool,
) {
    // TODO 改成map(|path| result)，收集处理结果
    let squares: Vec<Option<()>> = files
        .par_iter()
        .map(|file_path_in| {
            process_one_file(
                file_path_in,
                path_in,
                path_out,
                zopfli_options,
                output_type,
                sort_buildings,
            )
        })
        .collect();
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

    process_all_files(
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
