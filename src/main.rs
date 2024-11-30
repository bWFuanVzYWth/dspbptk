mod blueprint;
mod dybp;
mod md5;

fn main() {
    let b64str_in = "BLUEPRINT:0,10,1303,0,0,0,0,0,637859821645895869,0.9.24.11286,%E6%9E%81%E6%85%A2,\"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\"D9DFF5FE1B596D34946DC60BD42FAA36";

    // TODO 警告：多余的字符
    // TODO 去掉unwrap()

    // 蓝图字符串 -> 蓝图数据
    let (_unknown_after_bp_data, bp_data_in) = blueprint::parse(b64str_in).unwrap();

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
}