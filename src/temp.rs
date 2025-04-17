use std::collections::BTreeMap;

#[derive(Ordering)]
struct Range(u32, u32);

fn main() {
    let mut parent_list = BTreeMap::new();
    let _ = parent_list.insert("three", 3);
    println!("{:?}", parent_list.get("three"));
}