use std::io::{self, Write};
use std::collections::HashMap;
use crate::database::Database;

enum VisitState {
    NotVisited,
    Visiting,
    Visited,
}

pub fn get_ip(sz: usize) -> String {
    let mut input = String::new();
    
    // Flush stdout to ensure any prompt is shown before user input
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut input).unwrap();
    
    // Trim newline and clip to maximum allowed length (sz - 1)
    input.trim_end().chars().take(sz - 1).collect()
}

fn dfs(db: &Database, node: u32, visited: &mut HashMap<u32, VisitState>, result: &mut Vec<u32>) -> bool {
    match visited.get(&node) {
        Some(VisitState::Visiting) => return true, // cycle
        Some(VisitState::Visited) => return false, // already done
        _ => {}
    }

    visited.insert(node, VisitState::Visiting);

    for neighbor in db.get_cell_children(node).iter().map(|dep| dep.get_target()) {
        if dfs(db, neighbor, visited, result) {
            return true; // propagate cycle
        }
    }

    visited.insert(node, VisitState::Visited);
    result.push(node);
    false
}

pub fn topological_sort(db: &Database, start: u32) -> Result<Vec<u32>, ()> {
    let mut visited = HashMap::new();
    let mut result = Vec::new();
    // let mut has_cycle = false;

    if dfs(db, start, &mut visited, &mut result) {
        return Err(());
    }

    result.reverse(); // reverse to get correct topological order
    Ok(result)
}
