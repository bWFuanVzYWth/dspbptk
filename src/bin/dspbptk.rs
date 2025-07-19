#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::{
    num::NonZero,
    path::{Path, PathBuf},
};

use clap::Parser;
use dspbptk::{
    blueprint::editor::sort::{fix_buildings_index, sort_buildings},
    dspbptk_blueprint::convert::fix_dspbptk_buildings_index,
    io::LegalBlueprintFileType,
};
use log::{error, warn};
use nalgebra::Vector3;
use rayon::prelude::*;
use walkdir::WalkDir;

use dspbptk::blueprint::data::content::Content;
use dspbptk::blueprint::data::header::Header;
use dspbptk::io::{self, FileType};
fn collect_files(path_in: &Path) -> Vec<PathBuf> {
    WalkDir::new(path_in)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| {
            let entry_path = entry.path();
            matches!(
                dspbptk::io::classify_file_type(entry_path),
                FileType::Blueprint(_)
            )
        })
        .map(walkdir::DirEntry::into_path)
        .collect()
}

fn generate_output_path(
    root_path_in: &Path,
    root_path_out: &Path,
    relative_path: &Path,
    file_type: &LegalBlueprintFileType,
) -> PathBuf {
    let extension = match file_type {
        LegalBlueprintFileType::Txt => "txt",
        LegalBlueprintFileType::Content => "content",
    };

    let stripped_path = relative_path
        .strip_prefix(root_path_in)
        .unwrap_or_else(|_| panic!("invalid path: {}", relative_path.display()));

    if stripped_path == Path::new("") {
        root_path_out.to_path_buf().with_extension(extension)
    } else {
        root_path_out.join(stripped_path).with_extension(extension)
    }
}

impl LinearPatternArgs {
    fn apply(&self, content_in: Content) -> Content {
        let dspbptk_buildings_in = content_in
            .buildings
            .into_iter()
            .map(|building| building.as_dspbptk_building_data().unwrap())
            .collect::<Vec<_>>();
        let basis_vector = Vector3::<f64>::new(self.x, self.y, self.z);
        let dspbptk_buildings_out =
            fix_dspbptk_buildings_index(dspbptk::editor::dspbptk::offset::linear_pattern(
                &dspbptk_buildings_in,
                &basis_vector,
                self.n,
            ));
        let buildings_out = dspbptk_buildings_out
            .into_iter()
            .map(|building| building.as_building_data().unwrap())
            .collect::<Vec<_>>();

        Content {
            buildings_length: u32::try_from(buildings_out.len()).unwrap(),
            buildings: buildings_out,
            ..content_in
        }
    }
}

impl OffsetArgs {
    fn apply(&self, content_in: Content) -> Content {
        let dspbptk_buildings_in = content_in
            .buildings
            .into_iter()
            .map(|building| building.as_dspbptk_building_data().unwrap())
            .collect::<Vec<_>>();
        let basis_vector = Vector3::<f64>::new(self.x, self.y, self.z);
        let dspbptk_buildings_out = fix_dspbptk_buildings_index(
            dspbptk::editor::dspbptk::offset::offset(dspbptk_buildings_in, &basis_vector),
        );
        let buildings_out = dspbptk_buildings_out
            .into_iter()
            .map(|building| building.as_building_data().unwrap())
            .collect::<Vec<_>>();

        Content {
            buildings_length: u32::try_from(buildings_out.len()).unwrap(),
            buildings: buildings_out,
            ..content_in
        }
    }
}

fn process_middle_layer(
    header_data_in: Header,
    content_data_in: Content,
    sorting_buildings: bool,
    rounding_local_offset: bool,
    sub_command: &Option<SubCommand>,
) -> (Header, Content) {
    let (header_data_out, mut content_data_out) = match sub_command {
        Some(SubCommand::LinearPattern(linear_pattern_args)) => {
            (header_data_in, linear_pattern_args.apply(content_data_in))
        }
        Some(SubCommand::Offset(offset_args)) => {
            (header_data_in, offset_args.apply(content_data_in))
        }
        None => (header_data_in, content_data_in),
    };

    if rounding_local_offset {
        content_data_out.buildings.iter_mut().for_each(|building| {
            building.round_float();
        });
    }

    if sorting_buildings {
        content_data_out.buildings = sort_buildings(content_data_out.buildings, true);
        content_data_out.buildings = fix_buildings_index(content_data_out.buildings);
    }

    (header_data_out, content_data_out)
}

// TODO 返回处理是否成功
fn process_one_file(
    file_path_in: &Path,
    file_path_out: &Path,
    zopfli_options: &zopfli::Options,
    output_type: &LegalBlueprintFileType,
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

    let (header_data_in, content_data_in) = match io::process_front_end(&blueprint_kind_in) {
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

    match dspbptk::io::write_file(file_path_out, blueprint_kind_out) {
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

    let sorting_buildings = !args.no_sorting_buildings;
    let rounding_local_offset = args.rounding_local_offset;

    let _result: Vec<Option<()>> = files
        .par_iter()
        .map(|file_path_in| {
            let file_path_out =
                generate_output_path(path_in, path_out, file_path_in, &args.type_output);

            process_one_file(
                file_path_in,
                &file_path_out,
                &zopfli_options,
                &args.type_output,
                sorting_buildings,
                rounding_local_offset,
                &args.subcommand,
            )
        })
        .collect();

    // TODO 数据统计
}

const fn configure_zopfli_options(args: &Args) -> zopfli::Options {
    let iteration_count = args.iteration_count;
    let iterations_without_improvement = args.iterations_without_improvement;
    let maximum_block_splits = args.maximum_block_splits;

    zopfli::Options {
        iteration_count,
        iterations_without_improvement,
        maximum_block_splits,
    }
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

#[derive(Parser, Debug, Clone)]
struct OffsetArgs {
    #[clap(index = 1)]
    x: f64,

    #[clap(index = 2)]
    y: f64,

    #[clap(index = 3)]
    z: f64,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Linear pattern blueprint with vector XYZ and count N
    LinearPattern(LinearPatternArgs),

    /// Offset blueprint with vector XYZ
    Offset(OffsetArgs),
}

#[derive(Parser, Debug)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    /// Input from file/dir. (*.txt *.content dir/)
    #[clap(value_name = "INPUT")]
    input: PathBuf,

    #[command(subcommand)]
    subcommand: Option<SubCommand>,

    /// Output to file/dir (*.* dir/)
    #[clap(long, short, value_name = "OUTPUT", global = true)]
    output: Option<PathBuf>,

    /// Output type: txt, content
    #[clap(long, short, default_value = "txt", value_name = "TYPE", global = true)]
    type_output: LegalBlueprintFileType,

    /// Round `local_offset` to 1/300 may make blueprint smaller. Lossy.
    #[clap(long, global = true)]
    rounding_local_offset: bool,

    /// Sorting buildings may make blueprint smaller. Lossless.
    #[clap(long, global = true)]
    no_sorting_buildings: bool,

    /// Compress arguments: zopfli `iteration_count`
    #[clap(long, default_value = "15", value_name = "COUNT", global = true)]
    iteration_count: NonZero<u64>,

    /// Compress arguments: zopfli `iterations_without_improvement`
    #[clap(
        long,
        default_value = "18446744073709551615",
        value_name = "COUNT",
        global = true
    )]
    iterations_without_improvement: NonZero<u64>,

    /// Compress arguments: zopfli `maximum_block_splits`
    #[clap(long, default_value = "15", value_name = "COUNT", global = true)]
    maximum_block_splits: u16,
}

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    eprintln!("https://github.com/bWFuanVzYWth/dspbptk");
    let args = Args::parse();

    process_workflow(&args);
}
