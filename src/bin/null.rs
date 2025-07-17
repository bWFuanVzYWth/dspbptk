use dspbptk::{
    blueprint::{content::ContentData, header::HeaderData},
    error::DspbptkError,
    io::{BlueprintKind, LegalBlueprintFileType},
};

fn main() -> Result<(), DspbptkError<'static>> {
    let zopfli_options = zopfli::Options::default();
    let header_data = HeaderData::default();
    let content_data = ContentData::default();

    if let BlueprintKind::Txt(blueprint) = dspbptk::io::process_back_end(
        &header_data,
        &content_data,
        &zopfli_options,
        &LegalBlueprintFileType::Txt,
    )? {
        print!("{blueprint}");
    }

    Ok(())
}
