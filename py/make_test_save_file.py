# Usage:
# python py/make_test_save_file.py

from struct import *

f = open('saves/test.city', 'wb')

name = "Test Save File"
terrain_path = 'height/test-height-128x128.png'
meta_things = ['test']
things = [{'name': 'test', 'direction': 0, 'x': 10, 'y': 5, 'z': 0}]

# Header size.
header_size = 2 + 2 + 2 + len(name)
f.write(pack('!H', header_size))

# File format version.
f.write(pack('!H', 0))

# Name length.
f.write(pack('!H', len(name)))

# Name.
f.write(name)

# Terrain section size.
terrain_size = 4 + 1 + 2 + len(terrain_path)
f.write(pack('!L', terrain_size))

# Terrain storage strategy.
f.write(pack('B', 0))

# Terrain path size.
f.write(pack('!H', len(terrain_path)))

# Terrain path.
f.write(terrain_path)

# Meta Things table size.
meta_things_table_size = 4 + 4
for meta_thing in meta_things:
  meta_things_table_size += (2 + len(meta_thing))
f.write(pack('!L', meta_things_table_size))

# Meta Things table count.
f.write(pack('!L', len(meta_things)))

# Meta Things table.
for meta_thing in meta_things:
  f.write(pack('!H', len(meta_thing)))
  f.write(meta_thing)


# Thing section size.
thing_section_size = 4 + 4 + (len(things) * (4 + 1 + 4 + 4 + 4))
f.write(pack('!L', thing_section_size))

# Number of Things.
f.write(pack('!L', len(things)))

# Things.
for thing in things:
  idx = meta_things.index(thing['name'])
  f.write(pack('!L', idx))
  f.write(pack('B', thing['direction']))
  f.write(pack('fff', thing['x'], thing['y'], thing['z']))

f.close()