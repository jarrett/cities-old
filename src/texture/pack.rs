use std::option::Option;
use std::boxed::Box;
use std::cmp::Ordering::{Less, Equal, Greater};

// Accepts a vector of items with the WidthHeight trait, typically images. Consumes the
// vector. Returns a new vector with the items wrapped in Packed structs.
pub fn pack_all<T: WidthHeight>(width: u32, height: u32, mut items_to_pack: Vec<T>) -> Vec<Packed<T>> {
    // Sort by area, largest first.
    items_to_pack.as_mut_slice().sort_by(|a, b|
        ( a.width() * a.height()).cmp(
        &(b.width() * b.height())).reverse()
    );
    
    let mut tree = Node::new(Rectangle {min_x: 0, min_y: 0, width: width, height: height});
    let items_packed: Vec<Packed<T>> = items_to_pack.drain().map(|item| {
        let packed_rect: Rectangle = tree.add(item.width(), item.height()).unwrap();
        Packed { inner: item, min_x: packed_rect.min_x, min_y: packed_rect.min_y }
    }).collect();
    
    items_packed
}

pub trait WidthHeight {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

#[derive(Debug, PartialEq)]
pub struct Packed<T: WidthHeight> {
    pub inner: T, // The object to pack. Typically an image.
    pub min_x: u32, // The X coord within the packed image.
    pub min_y: u32  // The Y coord within the packed image.
}

impl <T: WidthHeight> Packed<T> {
    pub fn new(inner: T) -> Packed<T> {
        Packed {inner: inner, min_x: 0, min_y: 0}
    }
    
    pub fn into_inner(self) -> T { self.inner }
}

struct Node {
    rc: Rectangle,
    is_split: bool,
    down: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    pub fn new(rectangle: Rectangle) -> Node {
        Node {rc: rectangle, is_split: false, down: None, right: None}
    }
    
    // Given width and height, adds a new rectangle to the tree. Returns a clone of the
    // added rectangle, complete with X and Y coordinates relative to the root node.
    // 
    // This is recursive. You should call it on the root node. The final rectangle will
    // be passed back up the stack.
    pub fn add(&mut self, width: u32, height: u32) -> Option<Rectangle> {
        if self.is_split {
            // This node has already been split up, so we need to go deeper.
            // Arbitrarily, we try to fit the added rectangle into the right side first.
            self.right.as_mut().unwrap().add(width, height)
            .or(self.down.as_mut().unwrap().add(width, height))
        } else {
            // This node is empty thus far. We'll drop the new rectangle in its upper
            // left corner and split the remaining space. Or, if the new rectangle doesn't
            // fit, we return None.
            if self.rc.width >= width && self.rc.height >= height {
                let rect = Rectangle {
                    min_x: self.rc.min_x,
                    min_y: self.rc.min_y,
                    width: width,
                    height: height
                };
                self.split(&rect);
                Some(rect)
            } else {
                None
            }
        }
    }
    
    // Splits the node. Assumes that ins has been inserted in the upper left of the node.
    // Splits one of two ways:
    //
    //           |   dw  |
    //      _____________    ______________
    //     | ins | right |  | ins  | right |
    // __  |_____|_______|  |______|       |
    // dh  |    down     |  | down |       |
    // __  |_____________|  |______|_______|
    //
    //            A                B
    //
    // If dh >= dw, chooses A. Else, chooses B.
    pub fn split(&mut self, ins: &Rectangle) {
        let dw = self.rc.width()  - ins.width();
        let dh = self.rc.height() - ins.height();
        if dh >= dw {
            self.down = Some(Box::new(Node::new(Rectangle {
                min_x:  self.rc.min_x(),
                min_y:  ins.max_y() + 1,
                width:  self.rc.width(),
                height: dh
            })));
        
            self.right = Some(Box::new(Node::new(Rectangle {
                min_x:  ins.max_x() + 1,
                min_y:  self.rc.min_y(),
                width:  dw,
                height: ins.height()
            })));
        } else {
            self.down = Some(Box::new(Node::new(Rectangle {
                min_x:  self.rc.min_x(),
                min_y:  ins.max_y() + 1,
                width:  ins.width(),
                height: dh
            })));
            
            self.right = Some(Box::new(Node::new(Rectangle {
                min_x:  ins.max_x() + 1,
                min_y:  self.rc.min_y(),
                width:  dw,
                height: self.rc.height()
            })));
        }
        self.is_split = true;
    }
}

struct Rectangle {
    min_x:  u32,
    min_y:  u32,
    width:  u32,
    height: u32
}

impl Rectangle {
    fn min_x(&self)  -> u32 { self.min_x }
    fn min_y(&self)  -> u32 { self.min_y }
    fn max_x(&self)  -> u32 { self.min_x + self.width - 1 }
    fn max_y(&self)  -> u32 { self.min_y + self.height - 1 }
    fn width(&self)  -> u32 { self.width }
    fn height(&self) -> u32 { self.height }
    fn area(&self)   -> u32 { self.width * self.height }
}

#[cfg(test)]
mod tests {
    use super::pack_all;
    use super::WidthHeight;
    use super::Packed;
    
    #[derive(Debug, PartialEq)]
    struct MockImage { w: u32, h: u32 }
    impl WidthHeight for MockImage {
        fn width(&self)  -> u32 { self.w }
        fn height(&self) -> u32 { self.h }
    }
    
    #[test]
    fn test_pack_all() {
        let images = vec!(
            MockImage { w: 20, h: 20 },
            MockImage { w: 50, h: 75 },
            MockImage { w: 40, h: 30 },
            MockImage { w: 50, h: 60 }
        );
        
        let packed = pack_all(130, 100, images);
        
        println!("{:?}", packed);
        
        assert_eq!(       
            vec!(
              Packed { inner: MockImage { w: 50, h: 75 }, min_x:   0, min_y:  0 },
              Packed { inner: MockImage { w: 50, h: 60 }, min_x:  50, min_y:  0 },
              Packed { inner: MockImage { w: 40, h: 30 }, min_x:  50, min_y: 60 },
              Packed { inner: MockImage { w: 20, h: 20 }, min_x: 100, min_y:  0 }
            ),
            packed
        )
    }
}