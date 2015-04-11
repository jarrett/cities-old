# Save File Format

## Thing Table

A save file records the various Things that are placed in the world. Each Meta Thing may
have many Things in a given save file. For example, there may be a Meta Thing called "Oak
Tree 1," and that tree may be placed in 100 different positions in the world.

In most cases, Meta Things are uniquely identified by their names. But that would be
highly inefficient in a save file. So in a save file, we represent a Meta Thing's identity
as an integer. For each Meta Thing in use in the world, we assign a unique integer. That
mapping is unique to each save file. So "Oak Tree 1" may be assigned to 42 in one save
file and 36 in another.

Those mappings are recorded in the Meta Things Table. The Meta Things Table is just a
sequence of strings. Each string represents the Meta Thing's name. The string's index in
the sequence is the Meta Thing's integer identifier.

## Binary Format

* Header

  * 2 byte unsigned int: Size of header in bytes.
  
  * 2 byte unsigned int: File format version.
  
  * 2 byte unsigned int: Size of name in bytes.
  
  * String (no null terminator): Name. Can be any length up to 65535. UTF-8 encoded.

* Terrain
  
  * 4 byte unsigned int: Size of terrain section in bytes.
  
  * 1 byte unsigned int: 1 if the terrain data is stored in this file. 0 if the terrain
    data is stored in an external image.
  
  * If terrain data is stored in this file:
  
    * 4 byte unsigned int: Number of vertices along the X axis.
  
    * 4 byte unsigned int: Number of vertices along the Y axis.
  
    * Array of 4-byte floats. Each float represents the Z coord. The index in the
      array represents the X and Y coords according to the formula:
      index = (Y * number of vertices along the X axis) + X.
  
  * If terrain data is stored in an external image:
    
    * 2 byte unsigned int: Size of image path in bytes.
    
    * String (no null terminator): Image path. Can be any length up to 65535.
      ASCII encoded.

* Meta Things Table

  * 4 byte unsigned int: Size of Meta Things table in bytes.
  
  * 4 byte unsigned int: Number of Meta Things in table.
  
  * Table entries, where each entry consists of:
  
    * 2 byte unsigned int: Size of name in bytes.
  
    * String (no null terminator): Name. Can be any length up to 65535. ASCII encoded.

* Things
  
  * 4 byte unsigned int: Size of Things section in bytes.
  
  * 4 byte unsigned int: Number of Things in the world.
  
  * Things, where each Thing consists of:
  
    * 4 byte unsigned int: Meta Thing ID. (Maps to index in Meta Things table.)
    
    * 1 byte unsigned int: Direction the Thing is facing, in the open interval [0, 7].
    
    * 2 x 4 byte floats: X, Y coords of Things's origin relative to world origin.
    
    * 4 byte float: Z coord of Thing's origin relative to the ground.
    
    * 4 byte unsigned int: Size of reserved section in bytes.
    
    * Reserved data. (Currently empty.)