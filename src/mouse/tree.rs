use cgmath::{Aabb, Aabb3};

use world::World;
use chunk::Chunk;
use math::{Triangle, split_aabb3_for_quadtree, aabb3_contains_aabb3, aabb3_from_tris, quad_to_tris};
use super::target::Target;

pub struct Tree {
    pub size: u32,
    pub bb: Aabb3<f32>,
    pub targets: Vec<Target>,
    pub children: Option<Children>
}

pub struct Children {
    pub q1: Box<Tree>,
    pub q2: Box<Tree>,
    pub q3: Box<Tree>,
    pub q4: Box<Tree>
}

impl Tree {
    pub fn new(size: u32, bb: Aabb3<f32>) -> Tree {
        Tree { size: size, bb: bb, targets: Vec::with_capacity(2), children: None }
    }
    
    pub fn add_chunk(&mut self, chunk: &Chunk) {
        for quad in chunk.quads() {
            let (tri1, tri2): (Triangle, Triangle) = quad_to_tris(quad);
            let bb: Aabb3<f32> = aabb3_from_tris(&tri1, &tri2);
            self.insert(
                Target::Ground(bb, tri1, tri2)
            );
        }
    }
    
    pub fn add_chunks_from_world(&mut self, world: &World) {
        for row in world.chunks.iter() {
            for chunk in row.iter() {
                self.add_chunk(&chunk.borrow());
            }
        }
    }
    
    pub fn build(&mut self) {
        if self.size > 1 {
            let size = self.size / 2;
            let (bb1, bb2, bb3, bb4) = split_aabb3_for_quadtree(&self.bb);
            self.children = Some(Children {
                q1: Box::new(Tree::new(size, bb1)),
                q2: Box::new(Tree::new(size, bb2)),
                q3: Box::new(Tree::new(size, bb3)),
                q4: Box::new(Tree::new(size, bb4))
            });
        }
    }
    
    pub fn insert(&mut self, target: Target) {
        self.expand_by(target.bb());
        match self.children {
            Some(ref mut c) => {
                if        aabb3_contains_aabb3(&c.q1.bb, target.bb()) {
                    c.q1.insert(target);
                } else if aabb3_contains_aabb3(&c.q2.bb, target.bb()) {
                    c.q2.insert(target);
                } else if aabb3_contains_aabb3(&c.q3.bb, target.bb()) {
                    c.q3.insert(target);
                } else if aabb3_contains_aabb3(&c.q4.bb, target.bb()) {
                    c.q4.insert(target);
                } else {
                    self.targets.push(target);
                }
            },
            None => {
                self.targets.push(target);
            }
        }
    }
    
    fn expand_by(&mut self, bb: &Aabb3<f32>) {
        self.bb.max.z = self.bb.max.z.max(bb.max.z);
        self.bb.min.z = self.bb.min.z.min(bb.min.z);
    }
}