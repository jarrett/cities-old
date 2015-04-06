# Example usage:
# python py/build_model.py assets/models jarrett test

import sys
import json
import os.path
from struct import *

class ModelBuilder:
  def __init__(self, folder, author_name, model_name):
    self.folder = folder
    self.author_name = author_name
    self.model_name = model_name
    self.blender_json_path = os.path.join(folder, author_name + '-' + model_name + '.model.blendout.json')
    self.config_json_path  = os.path.join(folder, author_name + '-' + model_name + '.model.config.json')
    self.model_path        = os.path.join(folder, author_name + '-' + model_name + '.model')
  
  def build(self):
    blender_json_file = open(self.blender_json_path, 'r')
    self.bj = json.loads(blender_json_file.read())
    blender_json_file.close()
    
    config_json_file = open(self.config_json_path, 'r')
    self.cj = json.loads(config_json_file.read())
    config_json_file.close()
    
    model_file = open(self.model_path, 'wb')
    
    # Header size. Sum of: its own size, version, images embedded, author name size,
    # author name string model name size, model name string.
    header_size = 2 + 2 + 1 + 2 + len(self.author_name) + 2 + len(self.model_name)
    model_file.write(pack('!H', header_size))
    
    # File format version.
    model_file.write(pack('!H', 0))
    
    # Sprite shape.
    model_file.write(pack('!B', self.cj['shape']))
    
    # Images embedded flag.
    model_file.write(pack('!B', 0))
    
    # Author name size.
    model_file.write(pack('!H', len(self.author_name)))
    
    # Author name string.
    model_file.write(self.author_name)
    
    # Model name size.
    model_file.write(pack('!H', len(self.model_name)))
    
    # Model name string.
    model_file.write(self.model_name)
          
    if self.cj['shape'] == 0: # 3d sprite.
      # Geometry section size. Sum of: its own size, dimensions xyz (3 floats),
      # UV coords (8 directions x 7 float pairs).
      geometry_size = 2 + (3 * 4) + (8 * 7 * 2 * 4)
      model_file.write(pack('!H', geometry_size))
      
      # Dimensions.
      model_file.write(pack('!fff', self.bj['xSize'],
                                    self.bj['ySize'],
                                    self.bj['zSize']))
      
      for direc in range(0, 8):
        if self.bj['directions'].has_key(str(direc)):
          dj = self.bj['directions'][str(direc)]
          model_file.write(pack('!ffffffffffffff',  dj['tb'][0],
                                                    dj['tb'][1],
                                                    dj['tr'][0],
                                                    dj['tr'][1],
                                                    dj['tf'][0],
                                                    dj['tf'][1],
                                                    dj['tl'][0],
                                                    dj['tl'][1],
                                                    dj['bl'][0],
                                                    dj['bl'][1],
                                                    dj['bf'][0],
                                                    dj['bf'][1],
                                                    dj['br'][0],
                                                    dj['br'][1]))
        else:
          raise StandardError('Missing key % in directions hash' % direc)
          # Zero-fill.
          #for i in range(0, 7 * 2 * 4):
          #  model_file.write(pack('!B', 0))
        
    else: # 2d sprite.
      # Geometry section size. Sum of: its own size, dimensions wh,
      # UV coords (4 floats).
      geometry_size = 2 + (2 * 4) + (4 * 4)
      model_file.write(pack('!H', geometry_size))
      
      raise NotImplementedError("TODO")
    
    model_file.close()

if __name__ == "__main__":
  ModelBuilder(sys.argv[1], sys.argv[2], sys.argv[3]).build()