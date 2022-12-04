use crate::universe::vec::Vec2;
use crate::universe::star::Star;

use super::star;


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

    pub fn insert(&mut self, star:Star) {       
        match &mut self.root {
            None => { self.root = Node::new(Vec2::new(0.,0.), self.size).into()}
            Some(_node) => {}
        }
        match &mut self.root {
            None => {}
            Some(node) => {Self::insert_recursive(node, star)}
        }
    }

    fn insert_recursive(node: &mut Node, star:Star) {
        if node.childs.len() == DIM {
            Self::childs_insert(node, star)
        }
        else {
            if node.stars.len() < DIM {
                node.stars.push(star)
            }
            else {
                let half = node.size/2.;
                node.childs.push(Node::new(node.pos, half));
                node.childs.push(Node::new(node.pos.add_x(half), half));
                node.childs.push(Node::new(node.pos.add_y(half), half));
                node.childs.push(Node::new(node.pos.add_x(half).add_y(half), half));
                while node.stars.len() > 0 {
                    match node.stars.pop() {
                        None => {}
                        Some(old_star) => {Self::childs_insert(node, old_star)}
                    }
                }
                Self::childs_insert(node, star)
            }
        }
    }

    fn childs_insert(node: &mut Node, star:Star) {
        for child in &mut node.childs {
            if child.is_in(star.get_pos()) {
                Self::insert_recursive(child, star);
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

    pub fn update_tree(&mut self) {
        match &mut self.root {
            None => {}
            Some(node) => {Self::update_tree_r(node);}
        }
    }

    fn update_tree_r(node:&mut Node) -> (f32, Vec2) {
        for star in &node.stars{
            node.cog += star.get_pos();
            node.mass += star.get_mass();
        }
        for child in &mut node.childs{
            Self::update_tree_r(child);
            node.cog += child.cog * child.mass;
            node.mass += child.mass;
        }
        if node.mass > 0. {node.cog = node.cog / node.mass};
        (node.mass, node.cog)
    }

    pub fn compute_interactions(&mut self, time_step:f32) {
        let data = vec![];
        match &mut self.root {
            None => {}
            Some(node) => {Self::compute_interaction_r(node, data, time_step);}
        }
    }

    fn compute_interaction_r(node:&mut Node, data:Vec<(Vec2, f32)>, time_step:f32){
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
                Self::compute_interaction_r(&mut node.childs[i], tmp_data, time_step)
            }
        }
        else {
            
            for i in 0..node.stars.len() {
                for t in &data {
                    node.stars[i].update_attraction_vec(*t, time_step);
                }
                for j in 0..node.stars.len() {
                    if i != j {
                        let tmp = node.stars[j];
                        node.stars[i].update_attraction(tmp, time_step)
                    }
                }

            }
        }
    }

    pub fn get_updated_stars(&mut self) -> Vec<Star>{
        let mut stars = vec![];
        match &mut self.root {
            None => {}
            Some(node) => {Self::get_updated_stars_r(node, &mut stars)}
        }
        stars
    }

    fn get_updated_stars_r(node:&mut Node, stars:&mut Vec<Star>) {
        for i in 0..node.childs.len(){
            Self::get_updated_stars_r(&mut node.childs[i], stars)
        }
        stars.append(&mut node.stars)
    }
}

#[derive(Debug)]
struct Node {
    mass:f32,
    cog:Vec2,
    pos:Vec2,
    size:f32,
    childs:Vec<Node>,
    stars:Vec<Star>,
}

impl Node {
    fn new(pos:Vec2, size:f32) -> Self {
        Node{mass:0., cog:Vec2::new(0.,0.), pos, size, childs:vec![], stars:vec![]}
    }

    fn is_in(&self, coord:Vec2) -> bool {
        coord.sup_eq(self.pos) && coord.inf(self.pos + self.size)
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
    
}