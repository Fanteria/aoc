use super::Point;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub x: i64,
    pub y: i64,
}

impl Path {
    pub fn from_points(from: &Point, to: &Point) -> Self {
        let (to_x, to_y) = to.coordinaes();
        let (from_x, from_y) = from.coordinaes();
        Self {
            x: to_x as i64 - from_x as i64,
            y: to_y as i64 - from_y as i64,
        }
    }
}
