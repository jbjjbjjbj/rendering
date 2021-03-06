use crate::world::{Hittable, Intersection};
use crate::core::{Bound3f, Ray};
use crate::{Float, INFTY};

use std::sync::Arc;

enum NodeType {
    Empty,
    Leaf(Arc<dyn Hittable>),
    Full(Arc<Node>, Arc<Node>),
}

pub struct Node {
    nt: NodeType,
    area: Bound3f,
}

pub type HittableTree = Node;

impl Node {
    pub fn new() -> Self {
        Self::default()
    }

    fn new_leaf(h: Arc<dyn Hittable>) -> Arc<Self> {
        let b = h.bounding_box();

        Arc::new(Self {
            nt: NodeType::Leaf(h),
            area: b,
        })
    }

    pub fn add(&mut self, op: Box<dyn Hittable>) {
        let new = self.nt.add(op);
        if let Some(t) = new {
            self.nt = t;
        }

        self.area = match &self.nt {
            NodeType::Empty => Bound3f::EMPTY,
            NodeType::Leaf(h) => h.bounding_box(),
            NodeType::Full(n1, n2) => n1.area.combine(&n2.area),
        };
    }

    /// Check how good a bound will fit
    ///
    /// Smaller value is better
    fn score(&self, op: &Bound3f) -> Float {
        self.area.combine(&op).area()
    }

    pub fn print(&self, d: u32) {
        for _ in 0..d {
            print!(" ");
        }
        match &self.nt {
            NodeType::Empty => println!("Empty"),
            NodeType::Leaf(_) => println!("Leaf"),
            NodeType::Full(n1, n2) => {
                println!("Full");
                n1.print(d+1);
                n2.print(d+1);
            }
        }
    }
}

impl Hittable for Node {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.area.intersect(ray, 0.0, INFTY) {
            println!("YAAH");
            return None;
        }
        println!("BOOO");

        match &self.nt {
            NodeType::Empty => None,
            NodeType::Leaf(h) => {
                println!("Leaf");
                h.intersect(ray)
            },

            NodeType::Full(o1, o2) => {
                println!("FULL");
                // Check intersect for sub nodes
                let i1 = o1.intersect(ray);
                let i2 = o2.intersect(ray);

                match (i1, i2) {
                    // Two intersections, take the smallest
                    (Some(i1), Some(i2)) => {
                        if i1.t < i2.t {
                            Some(i1)
                        } else {
                            Some(i2)
                        }
                    },

                    // One of them intersect
                    (None, Some(i)) => Some(i),
                    (Some(i), None) => Some(i),

                    // None of them intersect
                    _ => None,
                }
            }
        }
    }

    fn bounding_box(&self) -> Bound3f {
        self.area.clone()
    }
}

impl NodeType {
    fn add(&mut self, op: Box<dyn Hittable>) -> Option<Self> {
        let opb = op.bounding_box();

        match self {
            // Empty node create a new one
            NodeType::Empty => Some(NodeType::Leaf(Arc::from(op))),

            // Promote a node from a leaf
            NodeType::Leaf(on) => Some(NodeType::Full(
                Node::new_leaf(Arc::from(op)), Node::new_leaf(on.clone()))),

            // Pass the object to the one with the largest overlap
            NodeType::Full(o1, o2) => {
                // Find overlaps
                let op1 = o1.score(&opb);
                let op2 = o2.score(&opb);

                // Compare and pass it along
                let co = if op1 < op2 {
                    o1
                } else {
                    o2
                };

                Arc::get_mut(co).unwrap().add(op);

                None
            }
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            nt: NodeType::Empty,
            area: Bound3f::EMPTY,
        }
    }
}
