use glam::Vec2;

use crate::universe::star::Star;

static DIM:usize = 4;

#[derive(Debug)]
pub struct Tree {
    size:f32,
    root:Option<Box<Node>>
}

impl Tree {
    pub fn new(size:f32) -> Self{
        Tree {size, root:None}
    }

    pub fn insert(&mut self, stars:&Vec<Star>, i:usize) {       
        match &mut self.root {
            None => {
                let mut node = Node::new(Vec2::new(0.,0.), self.size);
                Self::insert_recursive(&mut node, stars, i);
                self.root = node.into();
            }
            Some(node) => {Self::insert_recursive(node, stars, i)}
        }
    }

    fn insert_recursive(node: &mut Node, stars:&Vec<Star>, i:usize) {
        if node.childs.len() == DIM {
            Self::childs_insert(node, stars, i)
        }
        else {
            if node.stars.len() < DIM {
                node.stars.push(i)
            }
            else {
                let half = node.size/2.;
                let x0 = node.pos.x;
                let x1 = x0 + half;
                let y0 = node.pos.y;
                let y1 = y0 + half;
                node.childs.push(Node::new(node.pos, half));
                node.childs.push(Node::new(Vec2 { x: x1, y: y0 }, half));
                node.childs.push(Node::new(Vec2 { x: x0, y: y1 }, half));
                node.childs.push(Node::new(Vec2 { x: x1, y: y1 }, half));
                for index in 0..node.stars.len() {
                    Self::childs_insert(node, stars, index)
                }
                node.stars.clear();
                Self::childs_insert(node, stars, i)
            }
        }
    }

    fn childs_insert(node: &mut Node, stars:&Vec<Star>, i:usize) {
        for child in &mut node.childs {
            if child.is_in(stars[i].get_pos()) {
                Self::insert_recursive(child, stars, i);
                return
        }
        }
    }

    // pub fn get_nb_stars(&self) -> usize {
    //     match &self.root {
    //         None => {0}
    //         Some(node) => {Self::get_nb_stars_r(node)}
    //     }
    // }

    // fn get_nb_stars_r(node: &Node) -> usize {
    //     let mut a = node.stars.len();
    //     for child in &node.childs{
    //         a += Self::get_nb_stars_r(child);
    //     }
    //     a
    // }

    pub fn update_tree(&mut self, stars:&Vec<Star>) {
        match &mut self.root {
            None => {}
            Some(node) => {Self::update_tree_r(node, stars);}
        }
    }

    fn update_tree_r(node:&mut Node, stars:&Vec<Star>) -> (f32, Vec2) {
        for i in &node.stars{
            node.cog += stars[*i].get_pos();
            node.mass += stars[*i].get_mass();
        }
        for child in &mut node.childs{
            Self::update_tree_r(child, stars);
            node.cog += child.cog * child.mass;
            node.mass += child.mass;
        }
        if node.mass > 0. {node.cog = node.cog / node.mass};
        (node.mass, node.cog)
    }

    pub fn compute_interactions(&mut self, stars:&mut Vec<Star>, time_step:f32) {
        let data = vec![];
        match &mut self.root {
            None => {}
            Some(node) => {Self::compute_interaction_r(node, data, stars, time_step);}
        }
    }

    fn compute_interaction_r(node:&mut Node, data:Vec<(Vec2, f32)>, stars:&mut Vec<Star>, time_step:f32){
        // println!("{:?}", data);
        if node.childs.len() > 0{
            for i in 0..node.childs.len() {
                let mut tmp_data = vec![];
                for j in 0..node.childs.len() {
                    if i != j && node.childs[j].mass > 0.{
                        tmp_data.push((node.childs[j].cog.clone(), node.childs[j].mass))
                    }
                }
                tmp_data.append(&mut data.clone());
                Self::compute_interaction_r(&mut node.childs[i], tmp_data, stars, time_step)
            }
        }
        else {
            
            for i in 0..node.stars.len() {
                for t in &data {
                    stars[node.stars[i]].update_attraction_vec(*t, time_step);
                }
                for j in 0..node.stars.len() {
                    if i != j {
                        let tmp = node.stars[j];
                        let tmpstar = stars[tmp].clone();
                        stars[node.stars[i]].update_attraction(tmpstar, time_step)
                    }
                }

            }
        }
    }

}

#[derive(Debug)]
struct Node {
    mass:f32,
    cog:Vec2,
    pos:Vec2,
    size:f32,
    childs:Vec<Node>,
    stars:Vec<usize>,
}

impl Node {
    fn new(pos:Vec2, size:f32) -> Self {
        Node{mass:0., cog:Vec2::new(0.,0.), pos, size, childs:vec![], stars:vec![]}
    }

    fn is_in(&self, coord:Vec2) -> bool {
        coord.cmpge(self.pos).all() && coord.cmplt(self.pos + self.size).all()
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
    
}