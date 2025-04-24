use rstar::{RTreeObject, AABB};

#[derive(Debug, Clone, Copy)]
pub enum DependencyNums {
    U32(u32),
    I32(i32),
    F32(f32),
}

/// Represents dependencies as oper(pre, post)
/// This representation can represent all kinds of dependencies
/// FN(RANGE_START: RANGE_END)
/// OPER(CELL, VAL), e.g. DIV(2, A1) for =2/A1
#[derive(Debug, Clone, Copy)]
pub struct DependencyData {
    oper: u8,
    pre: DependencyNums,
    post: DependencyNums,
}

/// Wrapper over DependencyData to save in Cell
#[derive(Debug, Clone, Copy)]
pub struct DependencyObject {
    target: u32,
    data: DependencyData,
}

impl std::fmt::Display for DependencyNums {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyNums::I32(i) => write!(f, "{}", i),
            DependencyNums::F32(fl) => write!(f, "{:.2}", fl),
            DependencyNums::U32(u) => write!(f, "{}", u),
        }
    }
}

impl DependencyData {
    pub fn new(oper: u8, pre: DependencyNums, post: DependencyNums) -> Self {
        DependencyData { oper, pre, post }
    }

    pub fn get_oper(&self) -> u8 {
        self.oper
    }

    pub fn get_pre(&self) -> DependencyNums {
        self.pre
    }

    pub fn get_post(&self) -> DependencyNums {
        self.post
    }
}

impl DependencyObject {
    pub fn new(target: u32, oper: u8, pre: DependencyNums, post: DependencyNums) -> Self {
        DependencyObject {
            target,
            data: DependencyData { oper, pre, post },
        }
    }

    pub fn from_dep_data(target: u32, dep_data: DependencyData) -> DependencyObject {
        DependencyObject {
            target,
            data: dep_data,
        }
    }

    pub fn get_target(&self) -> u32 {
        self.target
    }
}

impl RTreeObject for DependencyObject {
    type Envelope = AABB<[i64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        fn to_point(c: u32) -> [i64; 2] {
            [(c % 1000) as i64, (c / 1000) as i64]
        }

        match (&self.data.pre, &self.data.post) {
            (DependencyNums::U32(c1), DependencyNums::U32(c2)) => {
                AABB::from_corners(to_point(*c1), to_point(*c2))
            }
            (DependencyNums::U32(c1), _) => AABB::from_point(to_point(*c1)),
            (_, DependencyNums::U32(c2)) => AABB::from_point(to_point(*c2)),
            _ => panic!("Invalid combination: neither pre nor post is U32"),
        }
    }
}

impl PartialEq for DependencyNums {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DependencyNums::U32(u1), DependencyNums::U32(u2)) => {
                if u1 == u2 {
                    return true;
                }
                false
            }
            (DependencyNums::I32(i1), DependencyNums::I32(i2)) => {
                if i1 == i2 {
                    return true;
                }
                false
            }
            (DependencyNums::F32(f1), DependencyNums::F32(f2)) => {
                if f1 == f2 {
                    return true;
                }
                false
            }
            (_, _) => false,
        }
    }
}

impl PartialEq for DependencyObject {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}
