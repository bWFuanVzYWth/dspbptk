use std::collections::{HashMap, VecDeque};

pub fn sort_belt_buildings(buildings: &mut [building::BuildingData]) -> usize {
    let n = buildings.len();
    if n == 0 {
        return 0;
    }

    // 构建索引映射：原始 index -> 连续索引
    let index_map: HashMap<i32, usize> = buildings
        .iter()
        .enumerate()
        .map(|(i, b)| (b.index, i))
        .collect();

    // 构建邻接表和入度表
    let mut adj = vec![vec![]; n];
    let mut in_degree = vec![0; n];

    for (i, building) in buildings.iter().enumerate() {
        if building.temp_output_obj_idx != building::INDEX_NULL {
            if let Some(&j) = index_map.get(&building.temp_output_obj_idx) {
                adj[i].push(j);
                in_degree[j] += 1;
            }
        }
    }

    let mut visited = vec![false; n];
    let mut result = Vec::new();
    let mut success_count = 0;

    for i in 0..n {
        if !visited[i] {
            // Kahn 算法进行拓扑排序
            let mut queue: VecDeque<usize> = VecDeque::new();
            let mut current_in_degree = in_degree.clone();

            for j in 0..n {
                if visited[j] {
                    continue;
                }
                if current_in_degree[j] == 0 {
                    queue.push_back(j);
                }
            }

            let mut sorted_nodes = Vec::new();
            while let Some(node) = queue.pop_front() {
                if visited[node] {
                    continue;
                }
                visited[node] = true;
                sorted_nodes.push(node);

                for &next in &adj[node] {
                    current_in_degree[next] -= 1;
                    if current_in_degree[next] == 0 {
                        queue.push_back(next);
                    }
                }
            }

            // 如果排序后的节点数等于当前连通图的节点数，说明排序成功
            if sorted_nodes.len() > 0 {
                result.extend(sorted_nodes);
                success_count += 1;
            }
        }
    }

    if result.is_empty() {
        return 0;
    }

    // 将排序后的索引映射回原始数组
    let mut temp = Vec::with_capacity(n);
    for &idx in &result {
        temp.push(buildings[idx].clone());
    }

    for i in 0..n {
        buildings[i] = temp[i].clone();
    }

    success_count
}