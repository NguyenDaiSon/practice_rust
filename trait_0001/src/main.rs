/// This program is to learn basic about trait.
fn main() {
    draw_shape(Circle {});
    draw_shape(Square {});
}

/// Provides common interfaces on how to deal with a shape
trait Shape {
    //!
    //! `fn draw` draws a shape
    fn draw(&self);
}

/// Represents a Circle.
struct Circle;
impl Shape for Circle {
    fn draw(&self) {
        //! Draws a circle.
        println!("()");
    }
}

/// Represents a Square.
struct Square;
impl Shape for Square {
    fn draw(&self) {
        //! Draws a square.
        println!("[]");
    }
}

/// Draw a shape.
fn draw_shape(shape: impl Shape) {
    shape.draw();
}
