#[derive(Default)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
}

#[derive(Default)]
pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32
}

impl Rect {
    pub fn size(&self) -> Vector2 {
        Vector2::new(self.right - self.left, self.bottom - self.top)
    }
}
