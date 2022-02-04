extern crate image;

use std::f32;
use std::fs::File;
use std::path::Path;


use image::Pixels;
use image::Pixel;
use image::Rgba;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

struct PixelCoords {
    x: u32,
    y: u32,
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct Node {
    color: Color,
    id: u32,
    pixels: Vec<PixelCoords>,
    connections: Vec<Node>,
}

enum NodeType {
    Input,
    Output,
    Wire,
    Xor,
    And,
}

impl Node {
    fn new(id: u32, color: Color) -> Node {
        Node {
            color: color,
            id: id,
            pixels: Vec::new(),
            connections: Vec::new(),
        }
    }

    fn add_pixel(&mut self, x: u32, y: u32) {
        self.pixels.push(PixelCoords { x: x, y: y });
    }

    fn add_connection(&mut self, node: Node) {
        self.connections.push(node);
    }
}

fn init(input_path: &str) {
    let img = image::open(&Path::new(input_path)).unwrap();

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;

    for p in img.pixels() { 
        println!("pixel: {} {} {}", p.2[0], p.2[1], p.2[2]);
    }
}

fn main() {
    init("/home/mati/personal_projects/reso_rust/reso.png");
}
