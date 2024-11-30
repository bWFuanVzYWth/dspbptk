mod blueprint;
mod dybp;
mod md5;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "DSPBPTK_v0.0.0")]
#[command(author = "bWFuanVzYWth")]
#[command(version = "v0.10.31.24632")]
#[command(about = "Dyson Shpere Program Blueprint Toolkit", long_about = None)]
struct Args {
    /// Input from file
    input: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    // TODO 警告：多余的字符
    // TODO 去掉unwrap()
    let args = Args::parse();

    let b64str_in = std::fs::read_to_string(&args.input)?;

    // 蓝图字符串 -> 蓝图数据
    let (_unknown_after_bp_data, bp_data_in) = blueprint::parse(&b64str_in).unwrap();

    // content子串 -> 二进制流
    let memory_stream_in = crate::blueprint::decode_content(bp_data_in.content);

    // 二进制流 -> content数据
    let (_unknown_after_content, mut content) =
        blueprint::content::parse(memory_stream_in.as_slice()).unwrap();

    // 蓝图处理
    blueprint::content::sort_buildings(&mut content.buildings);

    // content数据 -> 二进制流
    let memory_stream_out = blueprint::content::serialization(content);

    // 二进制流 -> content子串
    let content_out = crate::blueprint::encode_content(memory_stream_out);

    // 合并新老蓝图数据
    let bp_data_out = crate::blueprint::BlueprintData {
        header: bp_data_in.header,
        content: &content_out,
        md5f: &crate::blueprint::compute_md5f_string(bp_data_in.header, &content_out),
    };

    // 蓝图数据 -> 蓝图字符串
    let b64str_out = crate::blueprint::serialization(bp_data_out);
    println!("{}", b64str_out);

    Ok(())
}
