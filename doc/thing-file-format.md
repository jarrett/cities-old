# Format of a "Thing" File

What is a Thing? It could be a building, a tree, a retaining wall, or almost any other
stationary object in the world. We prefer to process all those types of objects with the
same code, so we treat them all as Things and write generic Thing-handling code.

Not everything is a Thing. Most notably, overlay textures are not Things in themselves.
However, a Thing *can* (and often does) include overlay textures.

If each instance of a particular kind of building, tree, etc, is a Thing, then what do we
call a single class of building, tree, etc? In other words, what do we call the Platonic
form of, say, Birch Tree Variant 6? We call it a MetaThing. We could have called it a
ThingClass or ThingType, but that would misleadingly conflate our concept of a Platonic
Thing with programming languages' type systems. To be clear, they *are* different. We
*don't* define a Rust struct or what have you for each MetaThing. MetaThings are just
packets of configuration data.

## Vertical Positioning

Each MetaThing has rules for its vertical positioning. Different types of Things behave
very differently in that respect. For example, a tree might need to sit right on the
surface of the ground. But a building might need a foundation to provide a visually
pleasing transition between the building's model and the sloped terrain. Even more
complicated, a bridge segment might need a footer model, a pylon model that repeats
vertically to span the depth of the water, and an above-water model. MetaThings define
all this logic in their configuration data. Many MetaThings won't use nearly all of the
available options, of course.

A word about slope: In three dimensions, we define the slope of a terrain triangle as
rise over run on of the side with the greatest Z difference. That's an approximation,
because technically a triangle does not have slope--only a particular line segment on its
surface has slope, and there are infinite such line segments you could pick.

### Base Alignment

When drawing a Thing, its base is vertically aligned with the terrain as follows. The
Thing's bottom quad is Z-positioned such that all of the quad is at or below the surface
of the terrain. In other words, the Thing is sunken into the ground as much as necessary
to ensure that no part of it is floating above the ground.

However, for MetaThings that would look bad sunken into the ground, we can configure a
foundation. The foundation can be either built or dug. A built foundation creates a
level platform above the terrain. A dug foundation creates a level pit in the terrain. We
can enable both built and dug foundations for the same MetaThing, in which case the game
will be free to select either on a case-by-case basis.

Some MetaThings need to be vertically aligned with some of their neighbors. For example, a
retaining wall must be aligned with its left and right neighbors, but doesn't care about
its front and back neighbors. We can configure these constraints.

## Configs

`builds_foundation`: Boolean. If true, then this thing can build a foundation underneath
itself if the terrain is too sloped. The top of the foundation will be aligned with the
highest point of the land this thing occupies.
Default: false.

`digs_foundation`: Boolean. If true, then this thing can dig a foundation underneath
itself if the terrain is too sloped. The bottom of the foundation will be aligned with the
lowest point of the land this thing occupies.

`foundation_min_slope`: Float. If foundations are enabled and the slope is greater than or
equal to this threshold, this thing either draws or digs a foundation.

`built_foundation_wall_texture`: String. The name of the texture to be used on the walls
(vertical polygons) of the foundation.

`built_foundation_cap_texture`: String. The name of the texture to be used on the cap
(horizontal polygons) of the foundation.

`dug_foundation_wall_texture`: String. The name of the texture to be used on the walls
(vertical polygons) of the foundation.

`dug_foundation_floor_texture`: String. The name of the texture to be used on the floor
(horizontal polygons) of the foundation.

`has_footer`: Boolean. If true, the model has a footer--a model that will be drawn
exactly once at the base of the model. Designed to be used in combination with the pylon
configs, e.g. for a bridge segment. Default: false.

`footer_model`: String. The name of the model to be used for the footer.

`has_pylon`: Boolean. If true, the model has a footer--a model that will be drawn a
variable number of times, stacked vertically. Useful for, e.g. the pylons of a bridge
segment that need to fill the vertical space between the seafloor and the surface.
Default: false.

`pylon_model`: The name of the model to be used for the pylon.

`pylon_align`: How to determine the Z position of the pylon(s). Options:

* `PYLON_ALIGN_WATER_TABLE`: The top of the highest pylon is at the level of the water
table. This is the default.

`pylon_repeat`: How the number of pylon repetitions is computed. Options:

* `PYLON_REPEAT_ZERO`: The pylon model is repeated such that it fills in at least to
absolute zero Z. The lowest pylon may (and likely will) extend below zero Z. This is
the default.

### Examples

**Tree**: A tree would simply need to sit on the surface. But its trunk is much narrower
than the total bounding box. So it can have multiple billboards.

## The Binary Format

* Header

  * 2 byte unsigned int: Size of header in bytes.
  
  * 2 byte unsigned int: File format version.
  
  * 2 byte unsigned int: Size of author name in bytes.
  
  * String (no null terminator): Author name. Can be any length up to 65535. ASCII encoded.
  
  * 2 byte unsigned int: Size of thing name in bytes.
  
  * String (no null terminator): Thing name. Can be any length up to 65535. ASCII encoded.
  
  * 1 byte unsigned int: Size of each configuration key in bytes. Currently, this must be
    1. Meaning we allow for up to 256 possible keys.
    
* Models
  
  * 4 byte unsigned int: Size of models section in bytes.
  
  * 2 byte unsigned int: Number of models.
  
  * Models, where each Model consists of:
    * 2 byte unsigned int: Size of author name in bytes.
    
    * String (no null terminator): Author name. Can be any length up to 65535. ASCII encoded.
    
    * 2 byte unsigned int: Size of model name in bytes.
    
    * String (no null terminator): Model name. Can be any length up to 65535. ASCII encoded.
    
    * 1 byte unsigned int: Direction model is facing.
    
    * 3 x 4 byte floats: X, Y, Z coords of Model's origin relative to Thing's origin.

* Configs
  
  * 4 byte unsigned int: Total size of all config key/value pairs in bytes.
  
  * 2 byte unsigned int: Number of models.

  * Config key/value pairs, where each consists of:

    * 1 byte unsigned int: Key. Maps to a config option, e.g. `footer_model`. See
      `meta_thing.hpp` for the mappings.
  
    * 2 byte unsigned int: Size of value in bytes.
  
    * Value. Can be any length up to 65535, and any data type.