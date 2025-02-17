use crate::item::Item;

#[derive(Debug)]
pub struct Row {
    pub t: Item,
    pub y: f64, // 这一行建筑坐标的中心
    pub n: u64, // 这一行建筑的数量
}
