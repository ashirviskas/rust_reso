extern crate image;

use std::f32;
use std::fs::File;
use std::path::Path;


use image::Pixels;
use image::Pixel;
use image::Rgba;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};


#[derive(PartialEq, Clone)]
struct PixelCoords {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct Node {
    color: Color,
    id: u32,
    pixels: Vec<PixelCoords>,
    connections: Vec<u32>,
    nodetype: NodeType,
}

#[derive(PartialEq, Copy, Clone)]
enum NodeType {
    Input,
    Output,
    Wire,
    Xor,
    And,
    None,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    fn to_nodetype(&self) -> NodeType {
        if self.r == 255 && self.g == 128 && self.b == 0 { // orange
            NodeType::Wire
        } else if self.r == 128 && self.g == 64 && self.b == 0 {
            NodeType::Wire
        } else if self.r == 0 && self.g == 128 && self.b == 255 { // saphire
            NodeType::Wire
        } else if self.r == 0 && self.g == 64 && self.b == 128 {
            NodeType::Wire
        } else if self.r == 128 && self.g == 255 && self.b == 0 { //lime
            NodeType::Wire
        } else if self.r == 64 && self.g == 128 && self.b == 0 {
            NodeType::Wire
        } else if self.r == 128 && self.g == 0 && self.b == 255 { // purple
            NodeType::Output
        } else if self.r == 64 && self.g == 0 && self.b == 128 {
            NodeType::Input
        } else if self.r == 0 && self.g == 255 && self.b == 128 { // Teal
            NodeType::Xor
        } else if self.r == 0 && self.g == 128 && self.b == 64 {
            NodeType::And
        } else {
            NodeType::None
        }
    }
}

impl Node {
    fn new(id: u32, color: Color) -> Node {
        Node {
            nodetype: color.to_nodetype(),
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
        self.connections.push(node.id);
    }

    fn merge(&self, other: &Node) -> Node {
        Self {
            nodetype: self.nodetype,
            color: self.color,
            id: self.id,
            pixels: {
                let mut temp = self.pixels.to_vec();
                temp.extend(other.pixels.iter().cloned());
                temp
            },
            connections: {
                let mut temp = self.connections.to_vec();
                temp.extend(other.connections.iter().cloned());
                temp
            },
        }
    }
}

fn init(input_path: &str) {
    let img = image::open(&Path::new(input_path)).unwrap();

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;

    let mut nodes: Vec<Node> = Vec::new();
     // A redundant loop to demonstrate reading image data
     for x in 0..img_width {
         println!("{}", x);
        for y in 0..img_height {
            let mut pixel = img.get_pixel(x, y);
            // println!("pixel: {} {} {}", pixel.0[0], pixel.0[1], pixel.0[2]);
            let mut node = Node::new(x * img_height + y, Color::new(pixel.0[0], pixel.0[1], pixel.0[2]));
            if node.nodetype == NodeType::None {
                continue;
            }
            node.add_pixel(x, y);
            // Checking for nodes in pixels before and if they match, merge them
            let mut found = false;
            let mut startval_x = x;
            let mut startval_y = y;
            if x != 0{
                startval_x = x - 1;
            }
            if y != 0 {
                startval_y = y - 1;
            }
            'outer: for xx in startval_x..x + 1 {
                for yy in startval_y..y + 1  {
                    if xx < img_width && yy < img_height {
                        if xx == x && yy == y {
                            continue;
                        }
                        for node_other_idx in (0..nodes.len()).rev() {
                            let mut node_other = &nodes[node_other_idx];
                            if node_other.pixels.contains(&PixelCoords { x: xx, y: yy }) {
                                if node_other.color == node.color {
                                    // println!("Merging nodes: {} {}", node.id, node_other.id);
                                    let node = node_other.merge(&node);
                                    found = true;
                                    // nodes[node_other_idx] = new_node;
                                    nodes.remove(node_other_idx);
                                }
                            }
                        }
                    }
                }
            }
            nodes.push(node);
        }
    }
    for n in &nodes {
        println!("{} {}", n.id, n.color.r);
    }
    println!("{}", nodes.len());
}

fn main() {
    init("/home/mati/personal_projects/reso_rust/reso.png");
}
