# Example usage:
# python py/build_thing.py assets/things jarrett test

import sys
import json
import os.path
from struct import *
import thing_configs

class ThingBuilder:
  def __init__(self, folder, author_name, thing_name):
    self.folder = folder
    self.author_name = author_name
    self.thing_name = thing_name
    self.json_path  = os.path.join(folder, author_name + '-' + thing_name + '.thing.json')
    self.thing_path = os.path.join(folder, author_name + '-' + thing_name + '.thing')
  
  def build(self):
    json_file = open(self.json_path, 'r')
    self.j = json.loads(json_file.read())
    json_file.close()
    
    thing_file = open(self.thing_path, 'wb')
    
    # Header size. Sum of: its own size, version, author name size, author name string,
    # thing name size, thing name string, key size.
    header_size = 2 + 2 + 2 + len(self.author_name) + 2 + len(self.thing_name) + 1
    thing_file.write(pack('!H', header_size))
    
    # File format version.
    thing_file.write(pack('!H', 0))
    
    # Author name size.
    thing_file.write(pack('!H', len(self.author_name)))
    
    # Author name string.
    thing_file.write(self.author_name)
    
    # Thing name size.
    thing_file.write(pack('!H', len(self.thing_name)))
    
    # Thing name string.
    thing_file.write(self.thing_name)
    
    # Key size.
    thing_file.write(pack('B', 1))
    
    # Models section size. Sum of: its own size, model count, size of all models.
    total_size = 4 + 2
    for model in self.j['models']:
      total_size += 2 + len(model['authorName']) + 2 + len(model['modelName']) + (4 * 3)
    thing_file.write(pack('!L', total_size))
    
    # Number of models.
    thing_file.write(pack('!H', len(self.j['models'])))
    
    # Models.
    for model in self.j['models']:
      thing_file.write(pack('!H', len(model['authorName'])))
      thing_file.write(model['authorName'])
      thing_file.write(pack('!H', len(model['modelName'])))
      thing_file.write(model['modelName'])
      thing_file.write(pack('Bfff', model['direction'], model['x'], model['y'], model['z']))
    
    # Key-value pairs size. Sum of: its own size, pair count, size of all pairs.
    total_size = 4 + 2
    for key, value in self.j['configs']:
      size, typeCode = valueBinaryType(value)
      total_size += size
    thing_file.write(pack('!L', total_size))
    
    # Number of key-value pairs.
    thing_file.write(pack('!H', len(self.j['configs'])))
    
    # Key-value pairs.
    for key, value in self.j['configs']:
      size, typeCode = valueBinaryType(value)
      thing_file.write(pack('B', self.binaryKey(key)))
      thing_file.write(pack('!H', size))
      thing_file.write(pack(typeCode, value))
    
    thing_file.close()
  
  def valueBinaryType(self, value):
    if (type(value) is int) or (type(value) is long):
      if (value >= -127) and (value <= 127):
        return (1, 'b') # Signed char.
      elif (value >= -32767) and (value <= 32767):
        return (2, '!h') # Signed short.
      elif (value >= -2147483647) and (value <= 2147483647):
        return (4, '!l') # Signed long.
      else:
        return (8, '!q') # Signed long long.
    elif type(value) is float:
      return (4, 'f')
    elif type(value) is str:
      return (len(value), 's')
  
  def binaryKey(self, key):
    return thing_configs.KEYS.index(key)

if __name__ == "__main__":
  ThingBuilder(sys.argv[1], sys.argv[2], sys.argv[3]).build()