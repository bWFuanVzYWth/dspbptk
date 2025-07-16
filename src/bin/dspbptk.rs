#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::path::{Path, PathBuf};

use clap::Parser;
use dspbptk::{
    dspbptk_building::fix_dspbptk_buildings_index,
    toolkit::blueprint::sort::{fix_buildings_index, sort_buildings},
};
use log::{error, warn};
use nalgebra::Vector3;
use rayon::prelude::*;
use walkdir::WalkDir;

use dspbptk::blueprint::content::ContentData;
use dspbptk::blueprint::header::HeaderData;
use dspbptk::io::{self, FileType};
fn collect_files(path_in: &Path) -> Vec<PathBuf> {
    WalkDir::new(path_in)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| {
            let entry_path = entry.path();
            matches!(
                dspbptk::io::classify_file_type(entry_path),
                FileType::Txt | FileType::Content
            )
        })
        .map(walkdir::DirEntry::into_path)
        .collect()
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

    let stripped_path = relative_path
        .strip_prefix(root_path_in)
        .expect("unreachable: can not process file path");

    if stripped_path == Path::new("") {
        root_path_out.to_path_buf().with_extension(extension)
    } else {
        root_path_out.join(stripped_path).with_extension(extension)
    }
}

pub trait DspbptkMap {
    fn apply(&self, header: &HeaderData, content: &ContentData) -> (HeaderData, ContentData);
}

impl DspbptkMap for LinearPatternArgs {
    fn apply(&self, header: &HeaderData, content_in: &ContentData) -> (HeaderData, ContentData) {
        use dspbptk::toolkit::dspbptk::offset::linear_pattern;

        let dspbptk_buildings_in = content_in
            .buildings
            .iter()
            .map(|building| building.to_dspbptk_building_data().unwrap())
            .collect::<Vec<_>>();
        let basis_vector = Vector3::<f64>::new(self.x, self.y, self.z);
        let dspbptk_buildings_out = fix_dspbptk_buildings_index(linear_pattern(
            &dspbptk_buildings_in,
            &basis_vector,
            self.n,
        ));
        let buildings_out = dspbptk_buildings_out
            .iter()
            .map(|building| building.to_building_data().unwrap())
            .collect::<Vec<_>>();
        let new_content = ContentData {
            buildings_length: u32::try_from(buildings_out.len()).unwrap(),
            buildings: buildings_out,
            ..content_in.clone()
        };

        (header.clone(), new_content)
    }
}

fn process_middle_layer(
    header_data_in: HeaderData,
    content_data_in: ContentData,
    sorting_buildings: bool,
    rounding_local_offset: bool,
    sub_command: &Option<SubCommand>,
) -> (HeaderData, ContentData) {
    let (header_data_out, mut content_data_out) = match sub_command {
        Some(SubCommand::LinearPattern(linear_pattern_args)) => {
            linear_pattern_args.apply(&header_data_in, &content_data_in)
        }
        None => (header_data_in, content_data_in),
    };

    if rounding_local_offset {
        content_data_out.buildings.iter_mut().for_each(|building| {
            building.round_float();
        });
    }

    if sorting_buildings {
        content_data_out.buildings = sort_buildings(&content_data_out.buildings, true);
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
    sub_command: &Option<SubCommand>,
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
        sub_command,
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

    let _result: Vec<Option<()>> = files
        .par_iter()
        .map(|file_path_in| {
            process_one_file(
                file_path_in,
                path_in,
                path_out,
                &zopfli_options,
                &output_type,
                sorting_buildings,
                rounding_local_offset,
                &args.subcommand,
            )
        })
        .collect();

    // TODO 数据统计
}

const fn configure_zopfli_options(args: &Args) -> zopfli::Options {
    // 参数的正确性必须由用户保证，如果参数无效则拒绝处理，然后立即退出程序
    let iteration_count = args
        .iteration_count
        .expect("arg error: unknown iteration_count");
    let iterations_without_improvement = args
        .iterations_without_improvement
        .expect("arg error: unknown iterations_without_improvement");
    let maximum_block_splits = args
        .maximum_block_splits
        .expect("arg error: unknown maximum_block_splits");

    zopfli::Options {
        iteration_count: std::num::NonZero::new(iteration_count)
            .expect("arg error: iteration_count must > 0"),
        iterations_without_improvement: std::num::NonZero::new(iterations_without_improvement)
            .expect("arg error: iterations_without_improvement must > 0"),
        maximum_block_splits,
    }
}

#[derive(Parser, Debug)]
struct ProcessArgs {
    #[clap(flatten)]
    global: Args,
}

#[derive(Parser, Debug, Clone)]
struct LinearPatternArgs {
    #[clap(index = 1)]
    x: f64,

    #[clap(index = 2)]
    y: f64,

    #[clap(index = 3)]
    z: f64,

    #[clap(index = 4)]
    n: u32,
}

#[derive(Parser, Debug)]
enum SubCommand {
    LinearPattern(LinearPatternArgs),
}

#[derive(Parser, Debug)]
#[command(
    version = "dspbptk0.2.0-dsp0.10.31.24632",
    author = "bWFuanVzYWth",
    about = "Dyson Sphere Program Blueprint Toolkit"
)]
struct Args {
    /// Input from file/dir. (*.txt *.content dir/)
    #[clap(value_name = "INPUT")]
    input: PathBuf,

    /// Output to file/dir. (*.* dir/)
    #[clap(long, short, value_name = "OUTPUT", global = true)]
    output: Option<PathBuf>,

    /// Output type: txt, content.
    #[clap(long, short, default_value = "txt", value_name = "TYPE", global = true)]
    type_output: Option<String>,

    /// Round `local_offset` to 1/300 may make blueprint smaller. Lossy.
    #[clap(long, short, global = true)]
    rounding_local_offset: bool,

    /// Sorting buildings may make blueprint smaller. Lossless.
    #[clap(long, global = true)]
    no_sorting_buildings: bool,

    /// Compress arguments: zopfli `iteration_count`.
    #[clap(long, default_value = "16", value_name = "COUNT", global = true)]
    iteration_count: Option<u64>,

    /// Compress arguments: zopfli `iterations_without_improvement`.
    #[clap(
        long,
        default_value = "18446744073709551615",
        value_name = "COUNT",
        global = true
    )]
    iterations_without_improvement: Option<u64>,

    /// Compress arguments: zopfli `maximum_block_splits`.
    #[clap(long, default_value = "0", value_name = "COUNT", global = true)]
    maximum_block_splits: Option<u16>,

    #[command(subcommand)]
    subcommand: Option<SubCommand>,
}

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    eprintln!("https://github.com/bWFuanVzYWth/dspbptk");
    let args = Args::parse();

    process_workflow(&args);
}
