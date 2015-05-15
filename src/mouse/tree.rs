use cgmath::{Aabb, Aabb3, Point3, Ray3, Triangle};

use camera::Camera;
use world::World;
use chunk::Chunk;
use math::{
    split_aabb3_for_quadtree, aabb3_contains_aabb3_xy, aabb3_from_tris,
    ray3_intersects_aabb3, quad_to_tris
};
use gldebug::DebugLines;
use super::target::{Target, Hit};

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
            let (tri1, tri2): (Triangle<Point3<f32>>, Triangle<Point3<f32>>) = quad_to_tris(quad);
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
    
    #[allow(dead_code)]
    pub fn add_to_debug_lines(&self, lines: &mut DebugLines, depth: usize, colors: &Vec<(f32, f32, f32)>) {
        let &(r, g, b) = colors.get(depth).unwrap_or(&(1.0, 1.0, 1.0));
        let (nx, ny, nz) = (self.bb.min.x, self.bb.min.y, self.bb.min.z);
        let (px, py, pz) = (self.bb.max.x, self.bb.max.y, self.bb.max.z);
        
        // Top square.
        lines.add_segment(Point3::new(px, ny, pz), Point3::new(px, py, pz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(px, py, pz), Point3::new(nx, py, pz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(nx, py, pz), Point3::new(nx, ny, pz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(nx, ny, pz), Point3::new(px, ny, pz), r, g, b, r, g, b);
        
        // Bottom square.
        lines.add_segment(Point3::new(px, ny, nz), Point3::new(px, py, nz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(px, py, nz), Point3::new(nx, py, nz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(nx, py, nz), Point3::new(nx, ny, nz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(nx, ny, nz), Point3::new(px, ny, nz), r, g, b, r, g, b);
        
        // Vertical lines.
        lines.add_segment(Point3::new(px, ny, pz), Point3::new(px, ny, nz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(px, py, pz), Point3::new(px, py, nz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(nx, py, pz), Point3::new(nx, py, nz), r, g, b, r, g, b);
        lines.add_segment(Point3::new(nx, ny, pz), Point3::new(nx, ny, nz), r, g, b, r, g, b);
        
        match self.children {
            Some(ref c) => {
                c.q1.add_to_debug_lines(lines, depth + 1, colors);
                c.q2.add_to_debug_lines(lines, depth + 1, colors);
                c.q3.add_to_debug_lines(lines, depth + 1, colors);
                c.q4.add_to_debug_lines(lines, depth + 1, colors);
            }
            None => ()
        }
    }
    
    pub fn build(&mut self) {
        if self.size > 4 {
            let size = self.size / 2;
            let (bb1, bb2, bb3, bb4) = split_aabb3_for_quadtree(&self.bb);
            let mut children = Children {
                q1: Box::new(Tree::new(size, bb1)),
                q2: Box::new(Tree::new(size, bb2)),
                q3: Box::new(Tree::new(size, bb3)),
                q4: Box::new(Tree::new(size, bb4))
            };
            children.q1.build();
            children.q2.build();
            children.q3.build();
            children.q4.build();
            self.children = Some(children);
        }
    }
    
    pub fn insert(&mut self, target: Target) {
        self.expand_by(target.bb());
        match self.children {
            Some(ref mut c) => {
                if        aabb3_contains_aabb3_xy(&c.q1.bb, target.bb()) {
                    c.q1.insert(target);
                } else if aabb3_contains_aabb3_xy(&c.q2.bb, target.bb()) {
                    c.q2.insert(target);
                } else if aabb3_contains_aabb3_xy(&c.q3.bb, target.bb()) {
                    c.q3.insert(target);
                } else if aabb3_contains_aabb3_xy(&c.q4.bb, target.bb()) {
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
    
    // The main public interface for the tree. Looks for an intersection between a ray
    // and any object in the tree.
    pub fn intersects_ray3(&self, ray: &Ray3<f32>, camera: &Camera) -> Option<Hit> {
        // Look for possible mouse targets in this node and its children. If search
        // returns Some, it'll be bound to the targets variable. Else, intersects_ray3
        // returns None.
        self.search(ray, 0).and_then(|mut targets| {
            // targets is a list of mouse targets whose bounding boxes intersect the ray.
            // But that doesn't mean each target was really hit; they're just candidates.
            
            // Sort by distance from camera so we can find the first hit.
            targets.sort_by(|a, b| {
                camera.distance_to(&a.bb().center()).partial_cmp(
                &camera.distance_to(&b.bb().center())).unwrap()
            });
            
            // Starting from the camera, work through the possible targets. Return as
            // soon as we find a hit. If none of the targets was hit, return None.
            targets.iter().filter_map(|target| {
                target.intersects_ray(ray)
            }).next()
        })
    }
    
    fn expand_by(&mut self, bb: &Aabb3<f32>) {
        self.bb.max.z = self.bb.max.z.max(bb.max.z);
        self.bb.min.z = self.bb.min.z.min(bb.min.z);
    }
    
    // This method is private because it's just a helper for intersects_ray3.
    //    
    // Looks for possible mouse targets in this node and its children. If the ray
    // intersects this node's bounding box, returns a list of targets (which may be
    // empty). Else, returns None.
    fn search<'a>(&'a self, ray: &Ray3<f32>, depth: u8) -> Option<Vec<&'a Target>> {
        if ray3_intersects_aabb3(ray, &self.bb) {
            let mut found: Vec<&Target> = Vec::new();
            
            for target in self.targets.iter() {
                if ray3_intersects_aabb3(ray, target.bb()) {
                    found.push(target);
                }
            }
            
            match self.children {
                Some(ref c) => {
                    self.search_quadrant(&mut found, &c.q1, ray, depth);
                    self.search_quadrant(&mut found, &c.q2, ray, depth);
                    self.search_quadrant(&mut found, &c.q3, ray, depth);
                    self.search_quadrant(&mut found, &c.q4, ray, depth);
                },
                None => ()
            }
            Some(found)
        } else {
            None
        }
    }
    
    fn search_quadrant<'a>(&self, found: &mut Vec<&'a Target>, tree: &'a Tree, ray: &Ray3<f32>, depth: u8) {
        match tree.search(ray, depth + 1) {
            Some(mut targets) => { found.append(&mut targets); },
            None => ()
        }
    }
}