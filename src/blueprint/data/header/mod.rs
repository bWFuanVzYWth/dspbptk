#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HeaderData {
    pub layout: String,
    pub icons_0: String,
    pub icons_1: String,
    pub icons_2: String,
    pub icons_3: String,
    pub icons_4: String,
    pub time: String,
    pub game_version: String,
    pub short_desc: String,
    pub desc: String,
    pub unknown: String,
}

impl Default for HeaderData {
    fn default() -> Self {
        Self {
            layout: "0".to_string(),
            icons_0: "0".to_string(),
            icons_1: "0".to_string(),
            icons_2: "0".to_string(),
            icons_3: "0".to_string(),
            icons_4: "0".to_string(),
            time: "0".to_string(),
            game_version: String::new(),
            short_desc: String::new(),
            desc: String::new(),
            unknown: String::new(),
        }
    }
}
