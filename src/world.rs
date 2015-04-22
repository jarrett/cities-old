use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use num::integer::Integer;
use cgmath::Point3;

use chunk::Chunk;
use terrain;
use water;
use camera::Camera;
use thing::{Thing, MetaThing, MetaThingsMap};
use futil::{read_string_16, write_string_16, read_point_3, write_point_3, IoErrorLine};

pub struct World {
    pub name: String,
    pub x_verts: u32,       // X dimension of world.
    pub y_verts: u32,       // Y dimension of world.
    pub x_size: u32,        // X dimension of world. Equal to xVerts - 1.
    pub y_size: u32,        // X dimension of world. Equal to xVerts - 1.
    pub x_chunks: u32,      // Number of chunks along the X axis.
    pub y_chunks: u32,      // Number of chunks along the Y axis.
    pub chunk_x_verts: u32, // X vertices of each chunk.
    pub chunk_y_verts: u32, // Y vertices of each chunk.
    pub chunk_x_size: u32,  // X dimension of each chunk. Equal to chunkXVerts - 1;
    pub chunk_y_size: u32,  // Y dimension of each chunk. Equal to chunkYVerts - 1;
    pub chunks: Vec<Vec<RefCell<Chunk>>>,
    pub things: Vec<Rc<Thing>>
}

impl World {
    pub fn new<T: terrain::Source>(
        name: String, terrain_source: T,
        terrain_program: &terrain::Program, water_program: &water::Program,
        chunk_x_verts: u32, chunk_y_verts: u32
      ) -> World {
        let mut world = World {
          name:          name,
          x_verts:       terrain_source.x_verts(),
          y_verts:       terrain_source.y_verts(),
          x_size:        terrain_source.x_verts() - 1,
          y_size:        terrain_source.y_verts() - 1,
          chunk_x_verts: chunk_x_verts,
          chunk_y_verts: chunk_y_verts,
          chunk_x_size:  chunk_x_verts - 1,
          chunk_y_size:  chunk_y_verts - 1,
          x_chunks:      0,
          y_chunks:      0,
          chunks:        Vec::with_capacity(((terrain_source.y_verts() - 1) / chunk_y_verts) as usize),
          things:        Vec::new()
        };
        
        if world.x_verts % chunk_x_verts != 0 {
          panic!("x_verts ({}) is not a multiple of chunk_x_verts ({})", world.x_verts, chunk_x_verts);
        }
        if world.y_verts % chunk_y_verts != 0 {
          panic!("y_verts ({}) is not a multiple of chunk_y_verts ({})", world.y_verts, chunk_y_verts);
        }
  
        world.x_chunks = world.x_size / chunk_x_verts;
        world.y_chunks = world.y_size / chunk_y_verts;
        
        for _ in 0u32..world.y_chunks {
          let inner_vec = Vec::with_capacity(world.x_chunks as usize);
          world.chunks.push(inner_vec);
        }
  
        // Iterate over all the chunks. The chunks aren't actually allocated yet; we're
        // about to create them in this loop. We'll set their vertex positions, but we
        // can't set the normals or buffer to the GPU yet. To calculate the normals, all
        // the positions must already exist. So we need a second pass for that.
        for chunk_y in 0u32..world.y_chunks {
            for chunk_x in 0u32..world.x_chunks {
                // This is where we finally initialize the chunks themselves.
                let min_x: u32 = chunk_x * world.chunk_x_size;
                let min_y: u32 = chunk_y * world.chunk_y_size;
                let mut chunk: Chunk = Chunk::new(terrain_program, water_program, min_x, min_y, chunk_x_verts, chunk_y_verts);
                
                // Drill down deeper: Go through the full X/Y range of each chunk and set the height
                // for each vertex.
                for rel_y in 0u32..chunk_y_verts {
                    for rel_x in 0u32..chunk_x_verts {
                        let x: u32 = min_x + rel_x;
                        let y: u32 = min_y + rel_y;
                        
                        let height: f32 = terrain_source.vert_z_at(x, y);
                        chunk.set_height(rel_x, rel_y, height);
                    }
                }
                
                // Finally, move the chunk. The vector owns it now, and it's wrapped
                // in a RefCell.
                world.chunks[chunk_y as usize].push(RefCell::new(chunk));
            }
        }
        
        /* Now it's time for the second pass, wherein we calculate the normals and buffer
        to the GPU. */
        for inner_vec in world.chunks.iter() {
            for cell in inner_vec.iter() {
                // Should never panic. If we've
                let chunk = &mut cell.borrow_mut();
                chunk.calc_normals(&world);
                chunk.buffer_positions();
                chunk.buffer_depths();
                chunk.buffer_normals();
                chunk.buffer_indices();
            }
        }
        
        world
    }
    
    pub fn from_file(
        terrain_program: &terrain::Program, water_program: &water::Program,
        chunk_x_verts: u32, chunk_y_verts: u32,
        meta_things_map: &MetaThingsMap, path: &Path
    ) -> Result<World, IoErrorLine> {
        let mut file = tryln!(File::open(path));
        
        // Read header.
        tryln!(file.read_u16::<BigEndian>());        // Header size.
        tryln!(file.read_u16::<BigEndian>());        // Version.
        let name: String = tryln!(read_string_16(&mut file)); // World name.
        
        // Read terrain.
        tryln!(file.read_u32::<BigEndian>()); // Terrain size.
        tryln!(file.read_u8());   // Terrain storage method.
        let terrain_path = tryln!(read_string_16(&mut file));
        let terrain_path = Path::new(terrain_path.as_str());
        let z_scale: f32 = tryln!(file.read_f32::<BigEndian>());
        let terrain_source = terrain::source::ImageSource::new(&terrain_path, z_scale);
        let mut world = World::new(name, terrain_source, terrain_program, water_program, chunk_x_verts, chunk_y_verts);
        
        // Read meta things table.
        tryln!(file.read_u32::<BigEndian>()); // Table size.
        let meta_thing_count = tryln!(file.read_u32::<BigEndian>());
        let mut indexed_meta_things: Vec<Rc<MetaThing>> = Vec::with_capacity(meta_thing_count as usize);        
        for _ in 0u32..meta_thing_count {
            let meta_thing_name = tryln!(read_string_16(&mut file));
            let meta_thing: Rc<MetaThing> = meta_things_map.get(&meta_thing_name).unwrap().clone();
            indexed_meta_things.push(meta_thing);
            
        }
        
        // Read things.
        tryln!(file.read_u32::<BigEndian>()); // Things section size.
        let thing_count = tryln!(file.read_u32::<BigEndian>());
        for _ in 0u32..thing_count {
            let meta_thing_index = tryln!(file.read_u32::<BigEndian>());
            let meta_thing = &indexed_meta_things[meta_thing_index as usize];
            let direction = tryln!(file.read_u8());
            let position = tryln!(read_point_3(&mut file));
            let thing = Thing::new(meta_thing, &position, direction);
            tryln!(file.read_u32::<BigEndian>()); // Size of reserved section.
            world.things.push(Rc::new(thing));
        }
        Ok(world)
    }
    
    pub fn to_file(&self, path: &Path) -> Result<(), IoErrorLine> {
        let mut file = tryln!(File::create(path));
        
        // Write header;
        tryln!(file.write_u16::<BigEndian>(48 + self.name.len() as u16)); // Header size.
        tryln!(file.write_u16::<BigEndian>(0)); // Version.
        tryln!(write_string_16(&mut file, &self.name));
        
        // Write terrain.
        let terrain_path: String = format!("assets/height/{}.png", &self.name);
        tryln!(file.write_u32::<BigEndian>(56 + terrain_path.len() as u32)); // Terrain section size.
        tryln!(file.write_u8(0)); // Terrain storage method.
        tryln!(write_string_16(&mut file, &terrain_path));
        tryln!(file.write_f32::<BigEndian>(0.1)); // FIXME. Make this value dynamic?
        
        // Build a hash set representing the list of unique meta things in this world.
        let mut hash_set: HashSet<String> = HashSet::new();
        for thing in self.things.iter() {
            hash_set.insert(thing.meta_thing.full_name().clone());
        }
        
        // Write the header data for the meta things table.
        // (Section size, number of meta things.)
        let mut section_size: u32 = 8;
        for meta_thing_name in hash_set.iter() {
            section_size = section_size + 2 + meta_thing_name.len() as u32;
        }
        tryln!(file.write_u32::<BigEndian>(section_size)); // Section size.
        tryln!(file.write_u32::<BigEndian>(hash_set.len() as u32)); // Number of meta things.
        
        // Write each of the unique meta things to the table. Also build a map from
        // the meta thing to its index. We'll use that later when we write the things.
        let mut hash_map: HashMap<String, u32> = HashMap::new();
        for (i, meta_thing_name) in hash_set.drain().enumerate() {
            tryln!(write_string_16(&mut file, &meta_thing_name));
            hash_map.insert(meta_thing_name, i as u32);          
        }
        
        // Write the header data for the things list. (Section size, number of things.)
        tryln!(file.write_u32::<BigEndian>(8 + 21 * self.things.len() as u32)); // Section size.
        tryln!(file.write_u32::<BigEndian>(self.things.len() as u32)); // Number of things.
        for thing in self.things.iter() {
            let meta_thing: &MetaThing = &thing.meta_thing;
            let meta_thing_index: u32 = *hash_map.get(&meta_thing.full_name()).unwrap();
            tryln!(file.write_u32::<BigEndian>(meta_thing_index));
            tryln!(file.write_u8(thing.direction));
            tryln!(write_point_3(&mut file, &thing.position));
            tryln!(file.write_u32::<BigEndian>(0)); // Size of reserved section.
        }
        
        Ok(())
    }
    
    // Up to four chunks may contain a point, because some points are on the edges or
    // corners of a chunk. This method returns the one with the highest x, y. Or, if no
    // chunk contains the point, it returns None. We require integer input because with
    // floating-point values, making correct decisions at the edges of chunks could become
    // problematic. If you need to find the chunk containing a floating-point coord, round
    // it yourself in whatever way is logical.
    pub fn chunk_containing(&self, abs_x: u32, abs_y: u32) -> Option<&RefCell<Chunk>> {
        // x_idx and y_idx are indices int the chunks array.
        let x_idx: usize = abs_x.div_floor(&self.chunk_x_size) as usize;
        let y_idx: usize = abs_y.div_floor(&self.chunk_y_size) as usize;
        
        if (x_idx as u32) < self.x_chunks && (y_idx as u32) < self.y_chunks {
            Some(&self.chunks[y_idx][x_idx])
        } else {
            None
        }
    }

    pub fn vert_position_at(&self, abs_x: i32, abs_y: i32) -> Option<Point3<f32>> {
        if abs_x >= 0 && abs_y >= 0 {
            match self.chunk_containing(abs_x as u32, abs_y as u32) {
                Some(cell) => {
                    let chunk = cell.borrow();
                    chunk.vert_position_at(abs_x as u32, abs_y as u32)
                },
                None => None
            }
        } else {
            None
        }
    }
    
    pub fn draw(&self, camera: &Camera, terrain_program: &terrain::Program, water_program: &water::Program) {
        // Draw the terrain first.
        for inner_vec in self.chunks.iter() {
            for cell in inner_vec.iter() {
                let chunk = cell.borrow();
                chunk.draw_terrain(camera, terrain_program);
            }
        }
        
        // Draw the water second, because it's partially transparent.
        for inner_vec in self.chunks.iter() {
            for cell in inner_vec.iter() {
                let chunk = cell.borrow();
                chunk.draw_water(camera, water_program);
            }
        }
    }
}