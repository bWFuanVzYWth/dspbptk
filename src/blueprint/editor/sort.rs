use crate::blueprint::Building;

// TODO 重写拓扑排序，需要考虑更多建筑，不只是传送带

/// 排序建筑，通常有利于蓝图尺寸
#[must_use]
pub fn sort_buildings(buildings: Vec<Building>, reserved: bool) -> Vec<Building> {
    let mut sorted = stable_sort_by_building_key(buildings);

    // 游戏内的建造顺序与蓝图顺序相反
    if reserved {
        sorted.reverse();
    }

    sorted
}

// fn split_belt_and_non_belt(buildings: Vec<Building>) -> (Vec<Building>, Vec<Building>) {
//     buildings
//         .into_iter()
//         .partition(|building| (2001..=2009).contains(&building.item_id))
// }

fn stable_sort_by_building_key(mut buildings: Vec<Building>) -> Vec<Building> {
    buildings.sort_by_cached_key(|building| {
        // 预计算排序键，实现Schwartzian transform优化
        (
            building.item_id,
            building.model_index,
            building.recipe_id,
            building.area_index,
            calculate_offset_score(building).to_bits(),
        )
    });

    buildings
}

fn calculate_offset_score(b: &Building) -> f64 {
    let (x, y, z) = (
        f64::from(b.local_offset_x),
        f64::from(b.local_offset_y),
        f64::from(b.local_offset_z),
    );
    y.mul_add(256.0, x).mul_add(1024.0, z)
}
