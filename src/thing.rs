use raylib::prelude::*;

pub struct Thing {
    pub position: Vector2,
    pub velocity: Vector2,
    pub bounding_box: Rectangle,
    pub color: Color,
}

impl Thing {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.bounding_box.x = self.position.x;
        self.bounding_box.y = self.position.y;
        d.draw_rectangle_rec(self.bounding_box, self.color);
    }
}
