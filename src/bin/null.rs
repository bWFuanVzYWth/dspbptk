use dspbptk::{
    blueprint::{content::ContentData, header::HeaderData},
    error::DspbptkError,
    io::{BlueprintKind, FileType},
};

fn main() -> Result<(), DspbptkError<'static>> {
    let header_data = HeaderData::default();
    let zopfli_options = zopfli::Options::default();

    // edit blueprint here
    let content_data = ContentData::default();

    if let BlueprintKind::Txt(blueprint) =
        dspbptk::io::process_back_end(&header_data, &content_data, &zopfli_options, &FileType::Txt)?
    {
        print!("{}", blueprint);
    }

    Ok(())
}
