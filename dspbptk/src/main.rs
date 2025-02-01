mod blueprint;
mod md5;

use std::path::{Path, PathBuf};

use clap::Parser;
use log::{debug, error, info, warn};
use rayon::prelude::*;
use walkdir::WalkDir;

use blueprint::error::BlueprintError;

// TODO 补充注释，优化可读性
fn recompress_blueprint(
    blueprint_in: &str,
    zopfli_options: &zopfli::Options,
) -> Result<(String, Vec<String>), BlueprintError<String>> {
    use blueprint::content::{data_from_string, string_from_data};
    use blueprint::edit::{fix_buildings_index, sort_buildings};

    let mut warnings = Vec::new();

    // 1. 解析blueprint
    let blueprint_data = blueprint::parse(blueprint_in)?;
    if blueprint_data.unknown.len() > 9 {
        warnings.push(format!(
            "{} unknown after blueprint: (QUITE A LOT)",
            blueprint_data.unknown.len()
        ));
    } else if blueprint_data.unknown.len() > 0 {
        warnings.push(format!(
            "{} unknown after blueprint: {:?}",
            blueprint_data.unknown.len(),
            blueprint_data.unknown
        ));
    }

    // 2. 解析content
    let mut content_data = data_from_string(blueprint_data.content)?;
    if content_data.unknown.len() > 9 {
        warnings.push(format!(
            "{} unknown after content: (QUITE A LOT)",
            content_data.unknown.len()
        ));
    } else if content_data.unknown.len() > 0 {
        warnings.push(format!(
            "{} unknown after content: {:?}",
            content_data.unknown.len(),
            content_data.unknown
        ));
    }

    // 3. 重新排序建筑
    sort_buildings(&mut content_data.buildings);
    fix_buildings_index(&mut content_data.buildings);

    // 4. 序列化content
    let content_string = string_from_data(content_data, zopfli_options)?;

    // 5. 序列化blueprint
    let blueprint_string = blueprint::serialization(blueprint_data.header, &content_string);

    Ok((blueprint_string, warnings))
}

fn read_content_file(file_in: &std::path::PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let content_bin = std::fs::read(file_in)?;
    debug!("Ok: read from {}", file_in.display());
    Ok(content_bin)
}

fn encode_content_file(
    content_bin: Vec<u8>,
    file_in: &std::path::PathBuf,
    zopfli_options: &zopfli::Options,
) -> Result<String, BlueprintError<String>> {
    let content_string = blueprint::content::string_from_bin(content_bin, zopfli_options)?;
    debug!("Ok: encode from {}", file_in.display());
    Ok(content_string)
}

fn write_blueprint_file(
    file_out: &std::path::PathBuf,
    blueprint_string: String,
) -> Result<(), std::io::Error> {
    std::fs::write(file_out, blueprint_string)?;
    info!("Ok: encode to {}", file_out.display());
    Ok(())
}

fn blueprint_from_content_file(
    file_in: &std::path::PathBuf,
    file_out: &std::path::PathBuf,
    zopfli_options: &zopfli::Options,
) {
    // 1. 读取content文件
    let content_bin = match read_content_file(file_in) {
        Ok(result) => result,
        Err(why) => {
            error!("{:#?}: read from {}", why, file_in.display());
            return;
        }
    };

    // 2. 编码content文件
    let content_string = match encode_content_file(content_bin, file_in, zopfli_options) {
        Ok(content) => content,
        Err(why) => {
            error!("{:#?}: encode from {}", why, file_in.display());
            return;
        }
    };

    // 3. 生成blueprint字符串
    const HEADER: &str = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,";
    let blueprint_string = blueprint::serialization(HEADER, &content_string);

    // 4. 写入blueprint文件
    if let Err(why) = write_blueprint_file(file_out, blueprint_string) {
        error!("{:#?}: encode from {}", why, file_out.display());
    }
}

fn recompress_blueprint_file(
    file_in: &std::path::PathBuf,
    file_out: &std::path::PathBuf,
    zopfli_options: &zopfli::Options,
) {
    // 1. 读取blueprint文件
    let blueprint_in = read_blueprint_file(file_in);

    // 2. 检查是否为blueprint
    if !is_valid_blueprint(&blueprint_in, file_in) {
        return;
    }

    // 3. 重新压缩blueprint文件并处理警告信息
    let (blueprint_out, _) = process_recompression(&blueprint_in, file_in, zopfli_options);

    // 4. 分析压缩率
    let _ = calculate_compression_rate(&blueprint_in, &blueprint_out);

    // 5. 创建输出目录和写入文件
    write_recompressed_file(file_out, &blueprint_out);
}

// 读取blueprint文件内容
fn read_blueprint_file(file_in: &std::path::PathBuf) -> String {
    match std::fs::read_to_string(file_in) {
        Ok(result) => {
            debug!("Ok: read from {}", file_in.display());
            result
        }
        Err(why) => {
            error!("{:#?}: read from {}", why, file_in.display());
            return "".to_string(); // 返回空字符串以避免后续处理失败
        }
    }
}

// 检查是否为有效的blueprint文件
fn is_valid_blueprint(blueprint_content: &str, file_in: &std::path::PathBuf) -> bool {
    if blueprint_content.chars().take(12).collect::<String>() != "BLUEPRINT:0," {
        debug!("Not blueprint: {}", file_in.display());
        return false;
    } else {
        debug!("Is blueprint: {}", file_in.display());
        return true;
    }
}

// TODO 更优雅的警告处理
// 处理重新压缩并收集警告信息
fn process_recompression(
    blueprint_in: &str,
    file_in: &std::path::PathBuf,
    zopfli_options: &zopfli::Options,
) -> (String, Vec<String>) {
    match recompress_blueprint(blueprint_in, zopfli_options) {
        Ok((blueprint, warnings)) => {
            warnings.iter().for_each(|warning| {
                warn!("{} from {}", warning, file_in.display());
            });
            debug!("Ok: recompress from {}", file_in.display());
            (blueprint, warnings)
        }
        Err(why) => {
            error!("{:#?}: recompress from {}", why, file_in.display());
            // 返回空字符串和空警告列表以避免后续处理失败
            return ("".to_string(), Vec::new());
        }
    }
}

// TODO 现在的输出难以对应到正确的文件路径，用户反馈不足。思考有没有更优雅的解决方法。
// 计算压缩率并返回统计信息
fn calculate_compression_rate(blueprint_in: &str, blueprint_out: &str) -> (usize, usize, f64) {
    let string_in_length = blueprint_in.len();
    let string_out_length = blueprint_out.len();
    let percent = (string_out_length as f64 / string_in_length as f64) * 100.0;
    info!(
        "Ok: {:3.3}%, {} -> {}",
        percent, string_in_length, string_out_length
    );
    (string_in_length, string_out_length, percent)
}

// 创建输出目录并写入重新压缩后的文件
fn write_recompressed_file(file_out: &std::path::PathBuf, blueprint_out: &str) {
    // 创建输出目录
    if let Err(why) = std::fs::create_dir_all(file_out.parent().unwrap()) {
        error!("{:#?}: create dir {}", why, file_out.display());
        return;
    }

    match std::fs::write(file_out, blueprint_out) {
        Ok(_) => {
            info!("Ok: write to {}", file_out.display());
        }
        Err(why) => {
            error!("{:#?}: write to {}", why, file_out.display());
        }
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

// 收集文件路径
fn collect_files(path_in: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut blueprints = Vec::new();
    let mut contents = Vec::new();

    for entry in WalkDir::new(path_in).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.into_path();
        match classify_file_type(&entry_path) {
            FileType::Txt => blueprints.push(entry_path),
            FileType::Content => contents.push(entry_path),
            _ => {}
        }
    }

    (blueprints, contents)
}

// 计算输出路径
fn generate_output_path(path_in: &Path, path_out: &Path, entry_path: &Path) -> PathBuf {
    let relative_path = entry_path.strip_prefix(path_in).unwrap();
    if relative_path == Path::new("") {
        path_out.to_path_buf()
    } else {
        path_out.join(relative_path)
    }
}

// 处理蓝图文件
fn process_blueprint_files(
    blueprints: Vec<PathBuf>,
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
) {
    blueprints.par_iter().for_each(|file_in| {
        let file_out = generate_output_path(path_in, path_out, file_in);
        debug!("Processing blueprint: {}", file_out.display());
        recompress_blueprint_file(file_in, &file_out, zopfli_options);
    });
}

// 处理内容文件
fn process_content_files(
    contents: Vec<PathBuf>,
    path_in: &Path,
    path_out: &Path,
    zopfli_options: &zopfli::Options,
) {
    contents.par_iter().for_each(|file_in| {
        let file_out = generate_output_path(path_in, path_out, file_in).with_extension("txt");
        debug!("Processing content: {}", file_out.display());
        blueprint_from_content_file(file_in, &file_out, zopfli_options);
    });
}

// 主函数
fn cook(args: &Args) {
    let zopfli_options = configure_zopfli_options(args);
    let path_in = &args.input;
    let path_out = args.output.as_deref().unwrap_or(path_in);

    let (blueprints, contents) = collect_files(path_in);

    process_blueprint_files(blueprints, path_in, path_out, &zopfli_options);
    process_content_files(contents, path_in, path_out, &zopfli_options);
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

    /// Actions of edit blueprint.
    #[clap(short, long, num_args = 0..)]
    actions: Option<Vec<String>>,

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

    cook(&args);
}
