#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::path::{Path, PathBuf};

use clap::Parser;
use log::{error, warn};
use rayon::prelude::*;
use walkdir::WalkDir;

use dspbptk::blueprint::content::ContentData;
use dspbptk::blueprint::header::HeaderData;
use dspbptk::io::{self, FileType};

fn collect_files(path_in: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path_in)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let entry_path = entry.into_path();
        match dspbptk::io::classify_file_type(&entry_path) {
            FileType::Txt | FileType::Content => files.push(entry_path),
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

    let relative_path = relative_path
        .strip_prefix(root_path_in)
        .expect("Fatal error: can not process file path");

    let mut output_path = if relative_path == Path::new("") {
        root_path_out.to_path_buf()
    } else {
        root_path_out.join(relative_path)
    };

    output_path.set_extension(extension);
    output_path
}

fn process_middle_layer(
    header_data_in: HeaderData,
    content_data_in: ContentData,
    sorting_buildings: bool,
    rounding_local_offset: bool,
) -> (HeaderData, ContentData) {
    use dspbptk::toolkit::{fix_buildings_index, sort_buildings};

    let header_data_out = header_data_in;
    let mut content_data_out = content_data_in;

    if rounding_local_offset {
        content_data_out.buildings.iter_mut().for_each(|building| {
            const ROUND_SCALE: f32 = 300.0;
            building.local_offset_x = (building.local_offset_x * ROUND_SCALE).round() / ROUND_SCALE;
            building.local_offset_y = (building.local_offset_y * ROUND_SCALE).round() / ROUND_SCALE;
            building.local_offset_z = (building.local_offset_z * ROUND_SCALE).round() / ROUND_SCALE;
        });
    }

    if sorting_buildings {
        sort_buildings(&mut content_data_out.buildings);
        content_data_out.buildings = fix_buildings_index(content_data_out.buildings);
    }

    (header_data_out, content_data_out)
}

fn process_one_file(
    file_path_in: &Path,
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
    sorting_buildings: bool,
    rounding_local_offset: bool,
) -> Option<()> {
    let blueprint_kind_in = match dspbptk::io::read_file(file_path_in) {
        Ok(result) => result,
        Err(e) => {
            error!("\"{}\": {:?}", file_path_in.display(), e);
            return None;
        }
    };

    let mut content_bin_in = Vec::new();

    let (header_data_in, content_data_in) =
        match io::process_front_end(&blueprint_kind_in, &mut content_bin_in) {
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

    let (header_data_out, content_data_out) = process_middle_layer(
        header_data_in,
        content_data_in,
        sorting_buildings,
        rounding_local_offset,
    );
    let blueprint_kind_out = match io::process_back_end(
        &header_data_out,
        &content_data_out,
        zopfli_options,
        output_type,
    ) {
        Ok(result) => result,
        Err(e) => {
            error!("\"{}\": {:?}", file_path_in.display(), e);
            return None;
        }
    };

    let file_path_out = generate_output_path(path_in, path_out, file_path_in, output_type);
    match dspbptk::io::write_file(&file_path_out, blueprint_kind_out) {
        Ok(()) => Some(()),
        Err(e) => {
            error!("\"{}\": {:?}", file_path_in.display(), e);
            None
        }
    }

    // TODO 数据统计
}

fn process_all_files(
    files: &[PathBuf],
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
    output_type: &FileType,
    sorting_buildings: bool,
    rounding_local_offset: bool,
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
                sorting_buildings,
                rounding_local_offset,
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

    let output_type = match args.type_output.as_deref() {
        Some("txt") => FileType::Txt,
        Some("content") => FileType::Content,
        _ => panic!("Unsupported file type"),
    };

    let sorting_buildings = !args.no_sorting_buildings;
    let rounding_local_offset = args.rounding_local_offset;

    process_all_files(
        &files,
        path_in,
        path_out,
        &zopfli_options,
        &output_type,
        sorting_buildings,
        rounding_local_offset,
    );
}

const fn configure_zopfli_options(args: &Args) -> zopfli::Options {
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
        maximum_block_splits,
    }
}

#[derive(Parser, Debug)]
#[command(
    version = "dspbptk0.2.0-dsp0.10.31.24632",
    author = "bWFuanVzYWth",
    about = "Dyson Sphere Program Blueprint Toolkit"
)]
struct Args {
    // TODO 蓝图分析命令：分析蓝图文件，输出一个结构化信息展示蓝图的一些信息
    // TODO 批量删除（对文件操作）蓝图中沙盒模式下的物流塔物品锁
    // TODO 根据传送带标记和自动判断的传送带方向，添加额外的传送带建筑，生成垂直带线头，然后删除已处理的标记
    // TODO 经度锁：解锁/上锁

    // TODO 多文件同时输入
    /// Input from file/dir. (*.txt *.content dir/)
    input: std::path::PathBuf,

    /// Output to file/dir. (*.* dir/)
    #[clap(long, short)]
    output: Option<std::path::PathBuf>,

    /// Output type: txt, content.
    #[clap(long, short, default_value = "txt")]
    type_output: Option<String>,

    /// Round `local_offset` to 1/300 may make blueprint smaller. Lossy.
    #[clap(long, short)]
    rounding_local_offset: bool,

    /// Sorting buildings may make blueprint smaller. Lossless.
    #[clap(long)]
    no_sorting_buildings: bool,

    /// Compress arguments: zopfli `iteration_count`.
    #[clap(long, default_value = "256")]
    iteration_count: Option<u64>,

    /// Compress arguments: zopfli `iterations_without_improvement`.
    #[clap(long, default_value = "18446744073709551615")]
    iterations_without_improvement: Option<u64>,

    /// Compress arguments: zopfli `maximum_block_splits`.
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
