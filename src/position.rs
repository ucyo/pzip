
use std::cmp::{PartialOrd, PartialEq, Ordering};

#[derive(Debug)]
pub struct Position {
    pub x :usize,
    pub y :usize,
    pub z :usize,
}

pub fn max_position(positions: &[Position]) -> &Position {
    let mut result = &positions[0];
    for pos in positions.iter() {
        if result < pos {
            result = pos;
        }
    }
    result
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        let k = self.x == other.x;
        let l = self.y == other.y;
        let m = self.z == other.z;

        m & k & l
    }
}


impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        if self.z > other.z {
            return Some(Ordering::Greater)
        } else if self.z < self.z {
            return Some(Ordering::Less)
        } else if self.y > other.y {
            return Some(Ordering::Greater)
        } else if self.y < other.y {
            return Some(Ordering::Less)
        } else if self.x > other.x {
            return Some(Ordering::Greater)
        } else if self.x < other.x {
            return Some(Ordering::Less)
        } else {
            return Some(Ordering::Equal)
        }
    }
}


#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn position_order() {
        let t = Position{x:1,y:2,z:8};
        let p = Position{x:1,y:0,z:1};
        let q = Position{x:0,y:1,z:1};
        let y = Position{x:1,y:0,z:1};

        assert!(p < t);
        assert!(p < q);
        assert!(p == y);
        assert!(y < q);
        assert!(y <= q);
    }

    #[test]
    fn position_list(){
        let v = [Position{x:1,y:0,z:1},
                 Position{x:1,y:2,z:8},
                 Position{x:0,y:1,z:1}];
        let max = max_position(&v);
        assert_eq!(max, &Position{x:1,y:2,z:8});
    }
}
