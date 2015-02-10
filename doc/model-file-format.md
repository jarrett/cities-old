# Format of a "Model" File

A Model represents the geometry and texturing of a stationary object in the world, such
as a building or a tree. The game is built around an idea similar to "billboard" sprites.
Objects are drawn as 3d boxes with sprite images projected onto them. Because the sprites
are rendered from the same perspective as the in-game camera, this projection gives the
illusion that the original, detailed 3d models are actually present in the game world.

## Sprites

Each model can have one or more sprites. A sprite is either 2d or 3d. A 2d sprite is just
a quad, always Z-rotated to face the camera, with its local Z axis aligned to the global
Z axis. A 3d sprite has three sides: right, left, and top, each of which is an axis-
aligned quad.

## Names

By "names," we mean string identifiers that are used internally only. (As opposed to
textual data that might be presented to the user. The latter might include things you
would think of as a "name," e.g. the user-readable name of a building. But we're not
talking about such strings right now.) Names are limited to lowercase English letters,
numbers, and `-`. The encoding is always ASCII.

## The Binary Format

* Header

  * 2 byte unsigned int: Size of header in bytes.
  
  * 2 byte unsigned int: File format version.
  
  * 1 byte: Shape. 0x00 means 3d. 0x01 means 2d.  
  
  * 1 byte bool: 0x01 if images are embedded; 0x00 if external files. If external, images
    are named, e.g. `test-0.png`, where `test` is the name of the model, and `0` is the
    the direction (orientation) of the sprite. Must be 0x00 for now, because embedded
    images are not currently implemented.
  
  * 2 byte unsigned int: Size of name in bytes.
  
  * String (no null terminator): Name. Can be any length up to 65535. ASCII encoded.

* If 3d:
  
  * 2 byte unsigned int: Geometry section size in bytes.
  
  * 4 byte float: X size.
  
  * 4 byte float: Y size.
  
  * 4 byte float: Z size.
  
  * For each direction (0-7):
  
    * 2 x 4 byte floats: UV coord of top-back vert.

    * 2 x 4 byte floats: UV coord of top-right vert.

    * 2 x 4 byte floats: UV coord of top-front vert.

    * 2 x 4 byte floats: UV coord of top-left vert.

    * 2 x 4 byte floats: UV coord of bottom-left vert.

    * 2 x 4 byte floats: UV coord of bottom-front vert.

    * 2 x 4 byte floats: UV coord of bottom-right vert.
  
* If 2d:
  
  * 2 byte unsigned int: Geometry section size in bytes.
  
  * 4 byte float: Width (world units).
  
  * 4 byte float: Height (world units).
  
  * 2 x 4 byte float: UV coord of top-left vertex.
  
  * 2 x 4 byte float: UV coord of bottom-right vertex.