use std::option::Option;
use std::boxed::Box;

#[derive(Debug, PartialEq)]
pub struct Packed<T: WidthHeight> {
    pub inner: T,     // The object to pack. Typically an image.
    pub min_x: usize, // The X coord within the packed image.
    pub min_y: usize  // The Y coord within the packed image.
}

pub trait WidthHeight {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

struct Node {
    rc: Rectangle,
    is_split: bool,
    down: Option<Box<Node>>,
    right: Option<Box<Node>>
}

struct Rectangle {
    min_x:  usize,
    min_y:  usize,
    width:  usize,
    height: usize
}

// Accepts a vector of items with the WidthHeight trait, typically images. Tries to pack
// as many of the items as will fit. Returns a new vector with the items wrapped in
// Packed structs. Those that didn't fit stay in the passed-in vector.
// 
// Call sort_for_packing on the vector before you pass it in for the first time.
pub fn pack_some<T: WidthHeight>(width: usize, height: usize, items_to_pack: &mut Vec<T>) -> Vec<Packed<T>> {
    let mut tree = Node::new(Rectangle {min_x: 0, min_y: 0, width: width, height: height});
    let mut items_packed: Vec<Packed<T>> = Vec::new();
    for item in items_to_pack.drain(..) {
        let result: Option<Rectangle> = tree.add(item.width(), item.height());
        match result {
            Some(packed_rect) => {
                items_packed.push(
                    Packed { inner: item, min_x: packed_rect.min_x, min_y: packed_rect.min_y }
                );
            },
            None => { break; }
        }
        
    }
    
    items_packed
}

pub fn sort_for_packing<T: WidthHeight>(items_to_pack: &mut Vec<T>) {
    // Sort by area, largest first.
    items_to_pack.as_mut_slice().sort_by(|a, b|
        ( a.width() * a.height()).cmp(
        &(b.width() * b.height())).reverse()
    );
}

impl <T: WidthHeight> Packed<T> {
    pub fn into_inner(self) -> T { self.inner }
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
    pub fn add(&mut self, width: usize, height: usize) -> Option<Rectangle> {
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

impl Rectangle {
    fn min_x(&self)  -> usize { self.min_x }
    fn min_y(&self)  -> usize { self.min_y }
    fn max_x(&self)  -> usize { self.min_x + self.width - 1 }
    fn max_y(&self)  -> usize { self.min_y + self.height - 1 }
    fn width(&self)  -> usize { self.width }
    fn height(&self) -> usize { self.height }
}

#[cfg(test)]
mod tests {
    use super::{Packed, WidthHeight, pack_some, sort_for_packing};
    
    #[derive(Debug, PartialEq)]
    struct MockImage { w: usize, h: usize }
    impl WidthHeight for MockImage {
        fn width(&self)  -> usize { self.w }
        fn height(&self) -> usize { self.h }
    }
    
    #[test]
    fn test_pack_some() {
        let mut images = vec!(
            MockImage { w: 20, h: 20 },
            MockImage { w: 50, h: 75 },
            MockImage { w: 40, h: 30 },
            MockImage { w: 50, h: 60 }
        );
        
        sort_for_packing(&mut images);
        
        let packed = pack_some(130, 100, &mut images);
        
        assert_eq!(       
            vec!(
              Packed { inner: MockImage { w: 50, h: 75 }, min_x:   0, min_y:  0 },
              Packed { inner: MockImage { w: 50, h: 60 }, min_x:  50, min_y:  0 },
              Packed { inner: MockImage { w: 40, h: 30 }, min_x:  50, min_y: 60 },
              Packed { inner: MockImage { w: 20, h: 20 }, min_x: 100, min_y:  0 }
            ),
            packed
        );
        
        assert_eq!(0, images.len());
    }
}