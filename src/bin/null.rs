use dspbptk::{
    blueprint::data::{content::Content, header::Header},
    error::DspbptkError,
    workflow::{BlueprintKind, LegalBlueprintFileType, process},
};

fn main() -> Result<(), DspbptkError> {
    let zopfli_options = zopfli::Options::default();
    let header_data = Header::default();
    let content_data = Content::default();

    if let BlueprintKind::Txt(blueprint) = process::process_back_end(
        &header_data,
        &content_data,
        &zopfli_options,
        &LegalBlueprintFileType::Txt,
    )? {
        print!("{blueprint}");
    }

    Ok(())
}
