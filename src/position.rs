
use std::cmp::{PartialOrd, PartialEq, Ordering};

#[derive(Debug)]
pub struct Position(pub usize, pub usize, pub usize);

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
        let k = self.0 == other.0;
        let l = self.1 == other.1;
        let m = self.2 == other.2;

        m & k & l
    }
}


impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        if self.2 > other.2 {
            return Some(Ordering::Greater)
        } else if self.2 < self.2 {
            return Some(Ordering::Less)
        } else if self.1 > other.1 {
            return Some(Ordering::Greater)
        } else if self.1 < other.1 {
            return Some(Ordering::Less)
        } else if self.0 > other.0 {
            return Some(Ordering::Greater)
        } else if self.0 < other.0 {
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
        let p = Position(1,0,1);
        let t = Position(1,2,8);
        let q = Position(0,1,1);
        let y = Position(1,0,1);

        assert!(p < t);
        assert!(p < q);
        assert!(p == y);
        assert!(y < q);
        assert!(y <= q);
    }

    #[test]
    fn position_list(){
        let v = [Position(1,0,1),Position(1,2,8),Position(0,1,1)];
        let max = max_position(&v);
        assert_eq!(max, &Position(1,2,8));
    }
}
