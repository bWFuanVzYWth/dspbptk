use crate::item::Item;

#[derive(Debug)]
pub struct Row {
    pub t: Item,
    pub y: f64, // 这一行建筑坐标的中心
    pub n: u64, // 这一行建筑的数量
}

pub fn next_row(row: Row, building_type:Item, offset_y: f64, n: u64) -> Row{
    Row{
        t: building_type,
        y: row.y + offset_y,
        n: n,
    }
}