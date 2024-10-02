use crate::universe::star::Star;
use glam::Vec3A;

const DIM: usize = 8;

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    nb_nodes: usize,
}

impl Tree {
    pub fn new(size: f32) -> Self {
        Tree {
            nodes: vec![Node::new(Vec3A::ZERO - (size / 2.), size)],
            nb_nodes: 1,
        }
    }

    fn is_inside_root(&self, star: &Star) -> bool {
        let root = &self.nodes[0];
        star.pos.x >= root.pos.x
            && star.pos.x < root.pos.x + root.size
            && star.pos.y >= root.pos.y
            && star.pos.y < root.pos.y + root.size
            && star.pos.z >= root.pos.z
            && star.pos.z < root.pos.z + root.size
    }

    pub fn insert(&mut self, stars: &Vec<Star>, star: usize) {
        if !self.is_inside_root(&stars[star]) {
            println!("star not in root");
            return;
        }
        self.insert_recursive(0, stars, star)
    }

    fn insert_recursive(&mut self, node: usize, stars: &Vec<Star>, star: usize) {
        if self.nodes[node].first_child != 0 {
            // println!("inserting in child");
            let node = &self.nodes[node];
            self.insert_recursive(
                node.first_child
                    + Self::get_child_id(
                        stars,
                        star,
                        node.pos.x + node.size / 2.,
                        node.pos.y + node.size / 2.,
                        node.pos.z + node.size / 2.,
                    ),
                stars,
                star,
            );
        } else if self.nodes[node].nb_stars < DIM {
            // println!("inserting in node");
            let nb_stars = self.nodes[node].nb_stars;
            self.nodes[node].stars[nb_stars] = star;
            self.nodes[node].nb_stars += 1;
        } else {
            // println!("splitting node");
            self.nb_nodes += 8;
            // println!("nb_nodes: {}", self.nb_nodes);
            let half = self.nodes[node].size / 2.;
            let x0 = self.nodes[node].pos.x;
            let x1 = x0 + half;
            let y0 = self.nodes[node].pos.y;
            let y1 = y0 + half;
            let z0 = self.nodes[node].pos.z;
            let z1 = z0 + half;
            self.nodes[node].first_child = self.nodes.len();
            self.nodes.push(Node::new(Vec3A::new(x0, y0, z0), half));
            self.nodes.push(Node::new(Vec3A::new(x1, y0, z0), half));
            self.nodes.push(Node::new(Vec3A::new(x0, y1, z0), half));
            self.nodes.push(Node::new(Vec3A::new(x1, y1, z0), half));
            self.nodes.push(Node::new(Vec3A::new(x0, y0, z1), half));
            self.nodes.push(Node::new(Vec3A::new(x1, y0, z1), half));
            self.nodes.push(Node::new(Vec3A::new(x0, y1, z1), half));
            self.nodes.push(Node::new(Vec3A::new(x1, y1, z1), half));

            for index in 0..self.nodes[node].stars.len() {
                self.insert_recursive(
                    self.nodes[node].first_child
                        + Self::get_child_id(stars, self.nodes[node].stars[index], x1, y1, z1),
                    stars,
                    self.nodes[node].stars[index],
                );
            }
            self.insert_recursive(
                self.nodes[node].first_child + Self::get_child_id(stars, star, x1, y1, z1),
                stars,
                star,
            );
        }
    }

    #[inline]
    fn get_child_id(stars: &[Star], star: usize, x1: f32, y1: f32, z1: f32) -> usize {
        let star_pos = stars[star].get_pos();
        let mut value = 0;
        value |= (star_pos.x > x1) as usize;
        value |= ((star_pos.y > y1) as usize) << 1;
        value |= ((star_pos.z > z1) as usize) << 2;
        value
    }

    pub fn update_tree(&mut self, stars: &Vec<Star>) {
        self.update_tree_r(0, stars);
    }

    fn update_tree_r(&mut self, node: usize, stars: &Vec<Star>) -> (f32, Vec3A) {
        let first_child = self.nodes[node].first_child;
        if first_child == 0 {
            let node = &mut self.nodes[node];

            for i in node.stars {
                node.cog += stars[i].get_pos();
                node.mass += stars[i].get_mass();
            }
        } else {
            for child in first_child..first_child + 8 {
                self.update_tree_r(child, stars);
                let cog = self.nodes[child].cog;
                let mass = self.nodes[child].mass;
                self.nodes[node].cog += cog * mass;
                self.nodes[node].mass += mass;
            }
        }

        let node = &mut self.nodes[node];
        if node.mass > 0. {
            node.cog /= node.mass
        };
        (node.mass, node.cog)
    }

    pub fn compute_interactions(&mut self, stars: &mut Vec<Star>, time_step: f32) {
        let data = vec![];
        self.compute_interaction_r(0, data, stars, time_step);
    }

    fn compute_interaction_r(
        &mut self,
        node: usize,
        data: Vec<(Vec3A, f32)>,
        stars: &mut Vec<Star>,
        time_step: f32,
    ) {
        // println!("{:?}", data);
        let first_child = self.nodes[node].first_child;
        if first_child == 0 {
            for i in 0..8 {
                let mut tmp_data = vec![];
                for j in 0..8 {
                    let child = &self.nodes[first_child + j];
                    if i != j && child.mass > 0. {
                        tmp_data.push((child.cog, child.mass))
                    }
                }
                tmp_data.append(&mut data.clone());
                self.compute_interaction_r(first_child + i, tmp_data, stars, time_step)
            }
        } else {
            let node = &self.nodes[node];
            for i in 0..node.stars.len() {
                for t in &data {
                    stars[node.stars[i]].update_attraction_vec(*t, time_step);
                }
                for j in 0..node.stars.len() {
                    if i != j {
                        let tmp = node.stars[j];
                        let tmpstar = stars[tmp];
                        stars[node.stars[i]].update_attraction(&tmpstar, time_step)
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    pos: Vec3A,
    size: f32,
    first_child: usize,
    nb_stars: usize,
    stars: [usize; DIM],

    mass: f32,
    cog: Vec3A,
}

impl Node {
    fn new(pos: Vec3A, size: f32) -> Self {
        Node {
            pos,
            size,
            first_child: 0,
            nb_stars: 0,
            stars: [0; DIM],
            mass: 0.,
            cog: Vec3A::ZERO,
        }
    }

    // fn is_inside(&self, star: &Star) -> bool {
    //     star.pos.x >= self.pos.x
    //         && star.pos.x < self.pos.x + self.size
    //         && star.pos.y >= self.pos.y
    //         && star.pos.y < self.pos.y + self.size
    //         && star.pos.z >= self.pos.z
    //         && star.pos.z < self.pos.z + self.size
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn cff() {
    //     let x = 10.;
    //     let y = 10.;
    //     let z = 10.;
    //     let stars = vec![
    //         Star::new(Vec3A::new(5., 5., 5.), Vec3A::ZERO, 1.),
    //         Star::new(Vec3A::new(15., 5., 5.), Vec3A::ZERO, 1.),
    //         Star::new(Vec3A::new(5., 15., 5.), Vec3A::ZERO, 1.),
    //         Star::new(Vec3A::new(15., 15., 5.), Vec3A::ZERO, 1.),
    //         Star::new(Vec3A::new(5., 5., 15.), Vec3A::ZERO, 1.),
    //         Star::new(Vec3A::new(15., 5., 15.), Vec3A::ZERO, 1.),
    //         Star::new(Vec3A::new(5., 15., 15.), Vec3A::ZERO, 1.),
    //         Star::new(Vec3A::new(15., 15., 15.), Vec3A::ZERO, 1.),
    //     ];
    //
    //     let expected = [0, 1, 2, 3, 4, 5, 6, 7];
    //     (0..8).for_each(|i| {
    //         assert_eq!(Tree::get_child_id(&stars, i, x, y, z), expected[i]);
    //     });
    // }

    #[test]
    fn insert() {
        let stars = vec![
            Star::new(Vec3A::new(5., 5., 5.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(15., 5., 5.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(5., 15., 5.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(15., 15., 5.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(5., 5., 15.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(15., 5., 15.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(5., 15., 15.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(15., 15., 15.), Vec3A::ZERO, 1.),
            Star::new(Vec3A::new(1005., 15., 15.), Vec3A::ZERO, 1.),
        ];

        let tree = &mut Tree::new(100.);

        for i in 0..stars.len() {
            tree.insert(&stars, i);
        }
    }
}
