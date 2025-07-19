pub mod content;
pub mod header;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlueprintData<'a> {
    pub header: &'a str,
    pub content: &'a str,
    pub md5f: &'a str,
    pub unknown: &'a str,
}
