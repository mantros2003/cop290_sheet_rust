use super::range::DependencyObject;
use rstar::{RTree, AABB};

pub struct DepStore {
    store: RTree<DependencyObject>,
}

impl DepStore {
    pub fn new() -> Self {
        DepStore {
            store: RTree::new(),
        }
    }

    pub fn insert(&mut self, dep: DependencyObject) {
        self.store.insert(dep);
    }

    /// To remove a dependency
    /// Used when a cell with a formula is modified and the previous formmula has to be discarded
    pub fn remove(&mut self, dep: DependencyObject) {
        self.store.remove(&dep);
    }

    /// To get all ranges a cell is part of
    pub fn get_from_point(&self, pt: u32) -> Vec<&DependencyObject> {
        let v: Vec<&DependencyObject> = self
            .store
            .locate_in_envelope_intersecting(&AABB::from_point([
                (pt % 1000) as i64,
                (pt / 1000) as i64,
            ]))
            .collect();
        return v;
    }
}

#[cfg(test)]
mod tests {
    use super::super::range::DependencyNums;
    use super::*;

    #[test]
    fn test_dep_store() {
        let mut store = DepStore::new();

        store.insert(DependencyObject::new(
            1001,
            0,
            DependencyNums::U32(2002),
            DependencyNums::U32(4004),
        ));
        store.insert(DependencyObject::new(
            10010,
            0,
            DependencyNums::U32(5005),
            DependencyNums::U32(9009),
        ));
        let v = store.get_from_point(3005);
        // let temp = &DependencyObject::new(1001, 0, DependencyNums::U32(2002), DependencyNums::U32(4004));
        let v1: Vec<&DependencyObject> = vec![];
        assert!(v == v1);
    }
}
