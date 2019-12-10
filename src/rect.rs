use std::ops;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Rect {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}

impl Rect {
    pub fn new(x:i32, y: i32, w:i32, h:i32) -> Rect {
        Rect{x1:x, y1:y, x2:x+w, y2:y+h}
    }

    pub fn zero() -> Rect {
        Rect{x1:0, y1:0, x2: 0, y2: 0}
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other:&Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }

    pub fn point_in_rect(&self, point : crate::Point) -> bool  {
        point.x >= self.x1 && point.x <= self.x2 && point.y >= self.y1 && point.y <= self.y2
    }

    pub fn xy_in_rect(&self, point : (i32,i32)) -> bool  {
        point.0 >= self.x1 && point.0 <= self.x2 && point.1 >= self.y1 && point.1 <= self.y2
    }

    pub fn for_each<F>(&self, mut f: F) where F: FnMut((i32, i32)) {
        for y in self.y1 .. self.y2 {
            for x in self.x1 .. self.x2 {
                f((x, y));
            }
        }
    }

    pub fn width(&self) -> i32 {
        i32::abs(self.x2 - self.x1)
    }

    pub fn height(&self) -> i32 {
        i32::abs(self.y2 - self.y1)
    }
}

impl ops::Add<Rect> for Rect {
    type Output = Rect;
    fn add(mut self, rhs: Rect) -> Rect {
        let w = self.width();
        let h = self.height();
        self.x1 += rhs.x1;
        self.x2 = self.x1 + w;
        self.y1 += rhs.y1;
        self.y2 = self.y1 + h;
        self
    }
}
