extern crate image;

use std::path::Path;

use image::{GenericImageView, ImageBuffer, RgbImage, DynamicImage};


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

fn explore_node(node: &Node, image: &DynamicImage, explored_pixels: &mut Vec<Vec<bool>>) -> Node {
    let mut prev_node = Node::new(node.id, node.color);
    prev_node = prev_node.merge(node);
    let mut cur_node = Node::new(node.id, node.color);
    cur_node = cur_node.merge(node);
    loop {
        let mut next_node = Node::new(node.id, node.color);
        let prev_size = prev_node.pixels.len();
        for pixel_idx in (0..cur_node.pixels.len()) {
            let pixel = &cur_node.pixels[pixel_idx];
            if explored_pixels[pixel.x as usize][pixel.y as usize] {
                continue;
            }
            explored_pixels[pixel.x as usize][pixel.y as usize] = true;   
            for x in -1..2 {
                for y in -1..2 {
                    let xx = pixel.x as i32 + x;
                    let yy = pixel.y as i32 + y;
                    if xx >= 0 && yy >= 0 && xx < image.width() as i32 && yy < image.height() as i32 {
                        if explored_pixels[xx as usize][yy as usize] {
                            continue;
                        }
                        let pixel_color = image.get_pixel(xx as u32, yy as u32);
                        if pixel_color.0[0] == cur_node.color.r && pixel_color.0[1] == cur_node.color.g && pixel_color.0[2] == cur_node.color.b {
                            // let sum: i32 = explored_pixels.iter().map(|xx| -> i32 { xx.iter().map(|&x| -> i32 { if x { 1 } else { 0 } }).sum() }).sum();
                            // println!("{} {} {}", x, y, sum);
                            println!("{} {}", xx, yy);
                            next_node.add_pixel(xx as u32, yy as u32);
                        }
                    }
                }
            }
            // if !explored_pixels[pixel.y as usize][pixel.x as usize] {
            //     explored_pixels[pixel.y as usize][pixel.x as usize] = true;
            //     let pixel_color = image.get_pixel(pixel.x, pixel.y);
            //     if pixel_color[0] == node.color.r && pixel_color[1] == node.color.g && pixel_color[2] == node.color.b {
            //         node.add_pixel(pixel.x, pixel.y);
            //         explore_node(node, image, explored_pixels);
            //     }
            // }
           
        }
        prev_node = prev_node.merge(&next_node);
        cur_node = next_node;
        if prev_node.pixels.len() == prev_size {
            break;
        }
    }
    println!("Returning {:?}", prev_node.pixels.len());
    prev_node
}

fn init(input_path: &str) {
    let img = image::open(&Path::new(input_path)).unwrap();

    let img_width:u32 = img.dimensions().0;
    let img_height:u32 = img.dimensions().1;

    let mut nodes: Vec<Node> = Vec::new();
     // TODO: map nodes by exploring neighbours instead of looking through all the pixels
    let mut explored_pixels:Vec<Vec<bool>> = vec![vec![false; img_height as usize]; img_width as usize];
    // find first node
    'outer: for x in 0..img_width {
        for y in 0..img_height {
            let pixel = img.get_pixel(x, y);
            let color = Color::new(pixel[0], pixel[1], pixel[2]);
            if color.to_nodetype() != NodeType::None {
                let mut node = Node::new(nodes.len() as u32, color);
                node.add_pixel(x, y);
                nodes.push(node);
                // explored_pixels[x as usize][y as usize] = true;
                break 'outer;
            }
        }
    }
    nodes[0] = explore_node(&nodes[0], &img, &mut explored_pixels);

    // takes 2m 5s to finish
    //  for x in 0..img_width {
    //      println!("{}", x);
    //     for y in 0..img_height {
    //         let pixel = img.get_pixel(x, y);
    //         // println!("pixel: {} {} {}", pixel.0[0], pixel.0[1], pixel.0[2]);
    //         let mut node = Node::new(x * img_height + y, Color::new(pixel.0[0], pixel.0[1], pixel.0[2]));
    //         if node.nodetype == NodeType::None {
    //             continue;
    //         }
    //         node.add_pixel(x, y);
    //         // Checking for nodes in pixels before and if they match, merge them
    //         let mut startval_x = x;
    //         let mut startval_y = y;
    //         if x != 0{
    //             startval_x = x - 1;
    //         }
    //         if y != 0 {
    //             startval_y = y - 1;
    //         }
    //         for xx in startval_x..x + 2 {
    //             for yy in startval_y..y + 2 {
    //                 if xx < img_width && yy < img_height {
    //                     if xx == x && yy == y {
    //                         continue;
    //                     }
    //                     for node_other_idx in (0..nodes.len()).rev() {
    //                         let node_other = &nodes[node_other_idx];
    //                         if node_other.pixels.contains(&PixelCoords { x: xx, y: yy }) {
    //                             if node_other.color == node.color {
    //                                 // println!("Merging nodes: {} {}", node.id, node_other.id);
    //                                 node = node_other.merge(&node);
    //                                 nodes.remove(node_other_idx);
    //                             } 
    //                             // else {
    //                             //     // println!("Adding connection: {} {}", node.id, node_other.id);
    //                             //     node.add_connection(node_other.clone());
    //                             // }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         nodes.push(node);
    //     }
    // }
    // for n in &nodes {
    //     println!("{} {}", n.id, n.color.r);
    // }
    println!("{}", nodes.len());

    let mut new_img: RgbImage = ImageBuffer::new(img_width, img_height);

    for n in &nodes {
        for p in &n.pixels {
            let mut pixel = new_img.get_pixel_mut(p.x, p.y);
            // pixel.0[0] = (n.id % 255 as u32) as u8;
            // pixel.0[1] = (n.id / 2 % 255 as u32) as u8;
            // pixel.0[2] = (n.id / 3 % 255 as u32) as u8;
            pixel.0[0] = n.color.r;
            pixel.0[1] = n.color.g;
            pixel.0[2] = n.color.b;
        }
    }
    new_img.save("output.png").unwrap();
}

fn main() {
    init("/home/mati/personal_projects/reso_rust/reso.png");
}
