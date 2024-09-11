use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct ShapePacker {
    width: u32,
    height: u32,
    shapes: Vec<Shape>,
}

struct Shape {
    x: f64,
    y: f64,
    radius: f64,
}

#[wasm_bindgen]
impl ShapePacker {
    pub fn new(width: u32, height: u32) -> ShapePacker {
        ShapePacker {
            width,
            height,
            shapes: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, x: f64, y: f64, radius: f64) {
        self.shapes.push(Shape { x, y, radius });
    }

    pub fn pack_shapes(&mut self) {
        // Implement shape packing algorithm here
        // This is a placeholder for the actual packing logic
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        for shape in &self.shapes {
            context.begin_path();
            context.arc(shape.x, shape.y, shape.radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            context.stroke();
        }
    }
}

#[wasm_bindgen]
pub fn init_shape_packer(width: u32, height: u32) -> ShapePacker {
    ShapePacker::new(width, height)
}