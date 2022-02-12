extern crate image;

use std::path::Path;
use bimap::BiMap;

use image::{GenericImageView, ImageBuffer, RgbImage, DynamicImage, imageops::BiLevel};


#[derive(PartialEq, Clone, Copy)]
struct PixelCoords {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Copy, Clone, Hash, Eq, PartialOrd, Ord)]
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
    explored: bool,
    next_nodetype: NodeType,
    magic_number_a: u32, // total inputs for input nodes
    magic_number_b: u32, // active inputs for input nodes
}


// nodetypes and whether they are active or not
#[derive(PartialEq, Copy, Clone, Hash, Eq)]
enum NodeType {
    Input(bool),
    Output(bool),
    OrangeWire(bool),
    SaphireWire(bool),
    LimeWire(bool),
    Xor(bool),
    And(bool),
    None(bool),
}

// mapping node types to colors

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    fn to_nodetype(&self, mappings: &BiMap<NodeType, Color>) -> NodeType {
        let node_type = mappings.get_by_right(&self);
        if node_type == None {
            NodeType::None(true)
        } else {
            node_type.unwrap().clone()

    }
}
}

impl Node {
    fn new(id: u32, color: Color, mappings: &BiMap<NodeType, Color>) -> Node {
        Node {
            nodetype: color.to_nodetype(mappings),
            color: color,
            id: id,
            pixels: Vec::new(),
            connections: Vec::new(),
            explored: false,
            next_nodetype: NodeType::None(true),
            magic_number_a: 0,
            magic_number_b: 0,
        }
    }

    fn Copy(&self) -> Node {
        Node {
            nodetype: self.nodetype,
            color: self.color,
            id: self.id,
            pixels: self.pixels.clone(),
            connections: self.connections.clone(),
            explored: self.explored,
            next_nodetype: self.next_nodetype,
            magic_number_a: self.magic_number_a,
            magic_number_b: self.magic_number_b,
        }
    }

    fn add_pixel(&mut self, x: u32, y: u32) {
        self.pixels.push(PixelCoords { x: x, y: y });
    }

    fn make_explored(&mut self) {
        self.explored = true;
    }

    fn add_connection(&mut self, id: u32) {
        if !self.connections.contains(&id) {
            self.connections.push(id);
        }
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
            explored: self.explored,
            next_nodetype: self.next_nodetype,
            magic_number_a: self.magic_number_a,
            magic_number_b: self.magic_number_b,
        }
    }
    // finds and adds neighbours to the connected nodes
    fn find_neighbours(&mut self, explored_pixels: &mut Vec<Vec<u32>>) {
        for pixel_idx in 0..self.pixels.len() {
            let pixel = self.pixels[pixel_idx];
            for x in pixel.x - 1..pixel.x + 2 {
                for y in pixel.y - 1..pixel.y + 2 {
                    if x >= explored_pixels.len() as u32 || y >= explored_pixels[0].len() as u32 {
                        continue;
                    }
                    let id = explored_pixels[x as usize][y as usize];
                    if id == 0 || id == self.id || id == u32::MAX {
                        continue;
                    }
                    if x != pixel.x && y != pixel.y {
                        continue;
                    }
                    self.add_connection(id);
                }
            }
        }
    }

    fn interact_with_neighbour(&mut self, other: &Node) {
        match self.nodetype {
            NodeType::Input(true) => {
                match other.nodetype {
                    NodeType::OrangeWire(true) => {
                        self.magic_number_a += 1;
                        self.magic_number_b += 1;
                    }
                    NodeType::OrangeWire(false) => {
                        self.magic_number_a += 1;
                    }
                    NodeType::SaphireWire(true) => {
                        self.magic_number_a += 1;
                        self.magic_number_b += 1;
                    }
                    NodeType::SaphireWire(false) => {
                        self.magic_number_a += 1;
                    }
                    NodeType::LimeWire(true) => {
                        self.magic_number_a += 1;
                        self.magic_number_b += 1;
                    }
                    NodeType::LimeWire(false) => {
                        self.magic_number_a += 1;
                    }
                    _ => {}
                }
            }
            NodeType::Output(true) => {
                match other.nodetype {
                    NodeType::And(true) => {
                        if other.magic_number_a == other.magic_number_b && other.magic_number_a > 0 {
                            self.magic_number_a += 1;
                            self.magic_number_b += 1;
                        } else {
                            self.magic_number_a += 1;
                        }
                    }
                    NodeType::Xor(true) => {
                        if other.magic_number_a >= other.magic_number_b && other.magic_number_b == 1 {
                            self.magic_number_a += 1;
                            self.magic_number_b += 1;
                        } else {
                            self.magic_number_a += 1;
                        }
                    }
                    NodeType::Input(true) => {
                        if other.magic_number_b != 0 {
                            self.magic_number_a += 1;
                            self.magic_number_b += 1;
                        } else {
                            self.magic_number_a += 1;
                        }
                    }
                    _ => {}
                }
            }
            NodeType::And(true) => {
                match other.nodetype {
                    NodeType::Input(true) => {
                        self.magic_number_a = other.magic_number_a;
                        self.magic_number_b = other.magic_number_b;
                    }
                    _ => {}
                }
            }
            NodeType::Xor(true) => {
                match other.nodetype {
                    NodeType::Input(true) => {
                        self.magic_number_a = other.magic_number_a;
                        self.magic_number_b = other.magic_number_b;
                    }
                    _ => {}
                }
            }
            NodeType::OrangeWire(false) => {
                match other.nodetype {
                    NodeType::Output(true) => {
                        if other.magic_number_a == other.magic_number_b {
                            self.next_nodetype = NodeType::OrangeWire(true);
                        } else {
                            self.next_nodetype = NodeType::OrangeWire(false);
                        }
                    }
                    _ => {}
                }
            }
            NodeType::OrangeWire(true) => {
                match other.nodetype {
                    NodeType::Output(true) => {
                        if other.magic_number_a == other.magic_number_b {
                            self.next_nodetype = NodeType::OrangeWire(true);
                        } else {
                            self.next_nodetype = NodeType::OrangeWire(false);
                        }
                    }
                    _ => {}
                }
            }
            NodeType::SaphireWire(false) => {
                match other.nodetype {
                    NodeType::Output(true) => {
                        if other.magic_number_a == other.magic_number_b {
                            self.next_nodetype = NodeType::SaphireWire(true);
                        } else {
                            self.next_nodetype = NodeType::SaphireWire(false);
                        }
                    }
                    _ => {}
                }
            }
            NodeType::SaphireWire(true) => {
                match other.nodetype {
                    NodeType::Output(true) => {
                        if other.magic_number_a == other.magic_number_b {
                            self.next_nodetype = NodeType::SaphireWire(true);
                        } else {
                            self.next_nodetype = NodeType::SaphireWire(false);
                        }
                    }
                    _ => {}
                }
            }
            NodeType::LimeWire(false) => {
                match other.nodetype {
                    NodeType::Output(true) => {
                        if other.magic_number_a == other.magic_number_b {
                            self.next_nodetype = NodeType::LimeWire(true);
                        } else {
                            self.next_nodetype = NodeType::LimeWire(false);
                        }
                    }
                    _ => {}
                }
            }
            NodeType::LimeWire(true) => {
                match other.nodetype {
                    NodeType::Output(true) => {
                        if other.magic_number_a == other.magic_number_b {
                            self.next_nodetype = NodeType::LimeWire(true);
                        } else {
                            self.next_nodetype = NodeType::LimeWire(false);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn update_next_step(&mut self) {
        if self.next_nodetype != NodeType::None(true) {
            self.nodetype = self.next_nodetype;
            self.next_nodetype = NodeType::None(true);
        }
        self.magic_number_a = 0;
        self.magic_number_b = 0;
    }
}

fn get_node_color_mappings() -> BiMap<NodeType, Color> {
    let mut mappings = BiMap::new();
   
    mappings.insert(NodeType::OrangeWire(true), Color::new(255, 128, 0));
    mappings.insert(NodeType::OrangeWire(false), Color::new(128, 64, 0));
    mappings.insert(NodeType::SaphireWire(true), Color::new(0, 128, 255));
    mappings.insert(NodeType::SaphireWire(false), Color::new(0, 64, 128));
    mappings.insert(NodeType::LimeWire(true), Color::new(128, 255, 0));
    mappings.insert(NodeType::LimeWire(false), Color::new(64, 128, 0));
    mappings.insert(NodeType::Output(true), Color::new(128, 0, 255));
    mappings.insert(NodeType::Input(true), Color::new(64, 0, 128));
    mappings.insert(NodeType::Xor(true), Color::new(0, 255, 128));
    mappings.insert(NodeType::And(true), Color::new(0, 128, 64));
    mappings.insert(NodeType::None(true), Color::new(0, 0, 0));
    mappings
}

fn explore_node_cluster(node: &Node, image: &DynamicImage, explored_pixels: &mut Vec<Vec<u32>>, mappings: &BiMap<NodeType, Color>) -> Node {
    let mut prev_node = Node::new(node.id, node.color, mappings);
    prev_node = prev_node.merge(node);
    let mut cur_node = Node::new(node.id, node.color, mappings);
    cur_node = cur_node.merge(node);
    loop {
        let mut next_node = Node::new(node.id, node.color, mappings);
        let prev_size = prev_node.pixels.len();
        for pixel_idx in (0..cur_node.pixels.len()) {
            let pixel = &cur_node.pixels[pixel_idx];
            if explored_pixels[pixel.x as usize][pixel.y as usize] != 0 {
                continue;
            }
            explored_pixels[pixel.x as usize][pixel.y as usize] = node.id;   
            for x in -1..2 {
                for y in -1..2 {
                    let xx = pixel.x as i32 + x;
                    let yy = pixel.y as i32 + y;
                    if xx >= 0 && yy >= 0 && xx < image.width() as i32 && yy < image.height() as i32 {
                        if explored_pixels[xx as usize][yy as usize] != 0{
                            continue;
                        }
                        let pixel_color = image.get_pixel(xx as u32, yy as u32);
                        if pixel_color.0[0] == cur_node.color.r && pixel_color.0[1] == cur_node.color.g && pixel_color.0[2] == cur_node.color.b {
                            // let sum: i32 = explored_pixels.iter().map(|xx| -> i32 { xx.iter().map(|&x| -> i32 { if x { 1 } else { 0 } }).sum() }).sum();
                            // println!("{} {} {}", x, y, sum);
                            // println!("{} {}", xx, yy);
                            next_node.add_pixel(xx as u32, yy as u32);
                        } 
                    }
                }
            }
        }
        prev_node = prev_node.merge(&next_node);
        cur_node = next_node;
        if prev_node.pixels.len() == prev_size {
            break;
        }
    }
    // println!("Returning {:?}", prev_node.pixels.len());
    prev_node.make_explored();
    prev_node
}

// colors pixels in image from node
fn color_node(node: &Node, image: &mut RgbImage) {
    for pixel in node.pixels.iter() {
        let mut pixel = image.get_pixel_mut(pixel.x, pixel.y);
        pixel.0[0] = node.color.r;
        pixel.0[1] = node.color.g;
        pixel.0[2] = node.color.b;
    }
}

fn init(input_path: &str) {
    let mappings = get_node_color_mappings();
    let img = image::open(&Path::new(input_path)).unwrap();

    let img_width:u32 = img.dimensions().0;
    let img_height:u32 = img.dimensions().1;

    let mut nodes: Vec<Node> = Vec::new();
     // TODO: map nodes by exploring neighbours instead of looking through all the pixels
    let mut explored_pixels:Vec<Vec<u32>> = vec![vec![0; img_height as usize]; img_width as usize];
    // find and explore all the nodes
    for x in 0..img_width {
        for y in 0..img_height {
            if explored_pixels[x as usize][y as usize] != 0 {
                continue;
            }
            let pixel = img.get_pixel(x, y);
            let color = Color::new(pixel[0], pixel[1], pixel[2]);
            if color.to_nodetype(&mappings) != NodeType::None(true) {
                let mut node = Node::new((nodes.len() + 1) as u32, color, &mappings);
                node.add_pixel(x, y);
                node = explore_node_cluster(&node, &img, &mut explored_pixels, &mappings);
                nodes.push(node);
            } else {
                explored_pixels[x as usize][y as usize] = u32::MAX;
            }
            
        }
    }
    // exploring first node
    // let (nodes[ = explore_node_cluster(&nodes[0], &img, &mut explored_pixels);
    println!("{}", nodes.len());

    for n_idx in 0..nodes.len() {
        let n = nodes.get_mut(n_idx).unwrap();
        n.find_neighbours(&mut explored_pixels);
    }
    let mut new_img: RgbImage = ImageBuffer::new(img_width, img_height);

    for n in &nodes {
        color_node(&n, &mut new_img);
    }
    new_img.save("output.png").unwrap();
    simulation_loop(&mappings, &mut nodes, 15, &new_img);
}

fn simulation_loop(mappings: &BiMap<NodeType, Color>, nodes: &mut Vec<Node>, steps: u32, image: &RgbImage) {
    for i in 0..steps {
        let mut new_img: RgbImage = ImageBuffer::new(image.width(), image.height());
        let nodetype_turns: Vec<NodeType> = vec![NodeType::Input(true), NodeType::Xor(true), NodeType::And(true), NodeType::Output(true), NodeType::OrangeWire(true), NodeType::OrangeWire(false), NodeType::LimeWire(true), NodeType::LimeWire(false), NodeType::SaphireWire(true), NodeType::SaphireWire(false)];
        for nodetype_turn in nodetype_turns.iter() {
           
            for node_idx in 0..nodes.len() {
                let mut node = nodes.get(node_idx).unwrap().Copy();
                if node.nodetype != *nodetype_turn {
                    continue;
                }
                for node_neighbour_idx in 0..node.connections.len() {
                    let neighbour_idx = node.connections[node_neighbour_idx] - 1;
                    let neighbour = nodes.get(neighbour_idx as usize).unwrap();
                    node.interact_with_neighbour(&neighbour);
                }
                nodes[node_idx] = node;
            }
        }

        for node_idx in 0..nodes.len() {
            let mut node = nodes.get(node_idx).unwrap().Copy();
            node.update_next_step();
            let new_node_color = mappings.get_by_left(&node.nodetype).unwrap();
            node.color = *new_node_color;
            nodes[node_idx] = node;
        }
        for node_idx in 0..nodes.len() {
            let node = nodes.get_mut(node_idx).unwrap();
            color_node(&node, &mut new_img);
        }
        let image_path = format!("./output/output_{}.png", i);
        new_img.save(image_path).unwrap();
    }

   
}
fn main() {
    init("./reso.png");
}
