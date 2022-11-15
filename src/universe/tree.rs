use crate::universe::vec::Vec2;
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

    pub fn get_nb_stars(&self) -> usize {
        match &self.root {
            None => {0}
            Some(node) => {Self::get_nb_stars_r(node)}
        }
    }

    fn get_nb_stars_r(node: &Node) -> usize {
        let mut a = node.stars.len();
        for child in &node.childs{
            a += Self::get_nb_stars_r(child);
        }
        a
    }

    pub fn update_tree(&mut self) {
        match &mut self.root {
            None => {}
            Some(node) => {Self::update_tree_r(node);}
        }
    }

    fn update_tree_r(node:&mut Node) -> (f32, Vec2) {
        node.mass += node.stars.len() as f32;
        for star in &node.stars{
            node.cog += star.get_pos();
            node.mass += 1.;
        }
        for child in &mut node.childs{
            Self::update_tree_r(child);
            node.cog += child.cog * child.mass;
            node.mass += child.mass;
        }
        node.cog = node.cog / node.mass;
        (node.mass, node.cog)
    }

    pub fn compute_interactions(&mut self) {
        match &mut self.root {
            None => {}
            Some(node) => {Self::compute_interaction_1_r(node);}
        }
    }

    fn compute_interaction_1_r(node:&mut Node) {
        if node.childs.len() > 0 {
            for i in 0..node.childs.len(){
                Self::compute_interaction_1_r(&mut node.childs[i])
            }
        }
        else {
            for i in 0..node.stars.len() {
                for j in 0..node.stars.len() {
                    if i != j {
                        let tmp = node.stars[j];
                        node.stars[i].update_attraction(tmp)
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