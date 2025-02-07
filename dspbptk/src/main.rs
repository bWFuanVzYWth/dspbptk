mod blueprint;
mod edit;
mod error;
mod md5;

use std::path::{Path, PathBuf};

use clap::Parser;
use rayon::prelude::*;
use walkdir::WalkDir;

use error::{DspbptkError, DspbptkError::*, DspbptkWarn, DspbptkWarn::*};
use log::{error, warn};

use blueprint::content::ContentData;
use blueprint::header::HeaderData;

// TODO 把文件io单独拆一个mod

fn read_content_file(path: &std::path::PathBuf) -> Result<Vec<u8>, DspbptkError> {
    std::fs::read(path).map_err(|e| CanNotReadFile {
        path: path,
        source: e,
    })
}

fn read_blueprint_file(path: &std::path::PathBuf) -> Result<String, DspbptkError> {
    std::fs::read_to_string(path).map_err(|e| CanNotReadFile {
        path: path,
        source: e,
    })
}

<<<<<<< HEAD
fn is_valid_blueprint<'a>(blueprint_content: &str) -> Result<(), DspbptkError<'a>> {
=======
fn is_valid_blueprint(blueprint_content: &str, file_in: &std::path::PathBuf) -> Option<()> {
>>>>>>> 2a97416ef636a5266e30c86f0f0ebf139b7f9ef2
    if blueprint_content.chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        Err(NotBlueprint)
    } else {
        Ok(())
    }
}

fn read_file(path: &PathBuf) -> Result<BlueprintKind, DspbptkError> {
    use crate::BlueprintKind::*;
    match classify_file_type(path) {
        FileType::Txt => {
            let blueprint_string = read_blueprint_file(path)?;
            is_valid_blueprint(&blueprint_string)?;
            Ok(Blueprint(blueprint_string))
        }
        FileType::Content => {
            let content_bin = read_content_file(path)?;
            Ok(Content(content_bin))
        }
        _ => Err(UnknownFileType),
    }
}

fn create_father_dir(path: &PathBuf) -> Result<(), DspbptkError> {
    let parent = match path.parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    std::fs::create_dir_all(&parent).map_err(|e| CanNotWriteFile {
        path: path,
        source: e,
    })
}

fn write_blueprint_file(path: &PathBuf, blueprint: String) -> Result<(), DspbptkError> {
    create_father_dir(path)?;
    std::fs::write(path, blueprint).map_err(|e| CanNotWriteFile {
        path: path,
        source: e,
    })
}

fn write_content_file(path: &PathBuf, content: Vec<u8>) -> Result<(), DspbptkError> {
    create_father_dir(path)?;
    std::fs::write(path, content).map_err(|e| CanNotWriteFile {
        path: path,
        source: e,
    })
}

fn write_file(path: &PathBuf, blueprint_kind: BlueprintKind) -> Result<(), DspbptkError> {
    match blueprint_kind {
        BlueprintKind::Blueprint(blueprint) => write_blueprint_file(path, blueprint),
        BlueprintKind::Content(content) => write_content_file(path, content),
    }
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

    let relative_path = relative_path.strip_prefix(root_path_in).expect("Fatal error: can not process file path");

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

<<<<<<< HEAD
fn process_front_end<'a>(
    blueprint: &'a BlueprintKind,
    blueprint_content_bin: &'a mut Vec<u8>,
) -> Result<(HeaderData, ContentData, Vec<DspbptkWarn>), DspbptkError<'a>> {
    match blueprint {
        BlueprintKind::Blueprint(blueprint_string) => {
            let (blueprint_data, warns_blueprint) = blueprint::parse(&blueprint_string)?;
            blueprint::content::bin_from_string(blueprint_content_bin, &blueprint_data.content)?;
            let (content_data, warns_content) =
                blueprint::content::data_from_bin(blueprint_content_bin.as_slice())?;
            let (header_data, warns_header) = blueprint::header::parse(&blueprint_data.header)?;
            Ok((
                header_data,
                content_data,
                [
                    warns_blueprint.as_slice(),
                    warns_content.as_slice(),
                    warns_header.as_slice(),
                ]
                .concat(),
            ))
        }
        BlueprintKind::Content(content_bin) => {
            let (content_data, warns_content) = blueprint::content::data_from_bin(&content_bin)?;
            const HEADER: &str = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,";
            let (header_data, warns_header) = blueprint::header::parse(HEADER)?;
            Ok((
                header_data,
                content_data,
                [warns_content.as_slice(), warns_header.as_slice()].concat(),
            ))
        }
    }
}

fn process_middle_layer(
    header_data_in: HeaderData,
    content_data_in: ContentData,
    should_sort_buildings: bool,
) -> (HeaderData, ContentData) {
    use edit::{fix_buildings_index, sort_buildings};

    // 这里应该是唯一一处非必要的深拷贝，但这是符合直觉的，可以极大优化用户的使用体验
    let header_data_out = header_data_in.clone();
    let mut content_data_out = content_data_in.clone();

    if should_sort_buildings {
        sort_buildings(&mut content_data_out.buildings);
        fix_buildings_index(&mut content_data_out.buildings);
    }

    (header_data_out, content_data_out)
}

fn process_back_end(
    header_data: &HeaderData,
    content_data: &ContentData,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
) -> Result<BlueprintKind, DspbptkError<'static>> {
    use crate::blueprint::content::bin_from_data;
    use crate::blueprint::content::string_from_data;
    match output_type {
        FileType::Txt => {
            let header_string = blueprint::header::serialization(header_data);
            let content_string = string_from_data(content_data, zopfli_options)?;
            Ok(BlueprintKind::Blueprint(blueprint::serialization(
                &header_string,
                &content_string,
            )))
        }
        FileType::Content => Ok(BlueprintKind::Content(bin_from_data(content_data))),
        _ => Err(UnknownFileType),
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
    let blueprint_kind_in = match read_file(file_path_in) {
        Ok(result) => result,
        Err(e) => {
            error!("\"{}\": {:?}", file_path_in.display(), e);
            return None;
        }
    };

    let mut content_bin_in = Vec::new();

    let (header_data_in, content_data_in) =
        match process_front_end(&blueprint_kind_in, &mut content_bin_in) {
            Ok((header_data_in, content_data_in, warns_front_end)) => {
                for warn in warns_front_end {
                    warn!("\"{}\": {:?}", file_path_in.display(), warn);
                }
                (header_data_in, content_data_in)
            }
            Err(e) => {
                error!("\"{}\": {:?}", file_path_in.display(), e);
                return None;
            }
        };

    let (header_data_out, content_data_out) =
        process_middle_layer(header_data_in, content_data_in, sort_buildings);
    let blueprint_kind_out = match process_back_end(
        &header_data_out,
        &content_data_out,
        &zopfli_options,
        &output_type,
    ) {
        Ok(result) => result,
        Err(e) => {
            error!("\"{}\": {:?}", file_path_in.display(), e);
            return None;
        }
    };

    let file_path_out = generate_output_path(path_in, path_out, file_path_in, output_type);
    match write_file(&file_path_out, blueprint_kind_out) {
        Ok(_) => Some(()),
        Err(e) => {
            error!("\"{}\": {:?}", file_path_in.display(), e);
            return None;
        }
    }

    // TODO 数据统计
}

fn process_all_files(
    files: Vec<PathBuf>,
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
    sort_buildings: bool,
) {
    let _result: Vec<Option<()>> = files
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

    // TODO 数据统计
}

fn process_workflow(args: &Args) {
    let zopfli_options = configure_zopfli_options(args);
    let path_in = &args.input;
    let path_out = args.output.as_deref().unwrap_or(path_in);

    let files = collect_files(path_in);

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

fn configure_zopfli_options(args: &Args) -> zopfli::Options {
    // 参数的正确性必须由用户保证，如果参数无效则拒绝处理，然后立即退出程序
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

#[derive(Parser, Debug)]
#[command(
    version = "dspbptk0.2.0-dsp0.10.31.24632",
    author = "bWFuanVzYWth",
    about = "Dyson Sphere Program Blueprint Toolkit"
)]
struct Args {
    // TODO 蓝图分析命令：分析蓝图文件，输出统计信息

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
