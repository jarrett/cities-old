import math
import bpy
from mathutils import Vector, Matrix, Euler

class BoundingBox:
  # For a given viewing direction, returns the coordinates of the seven
  # visible vertices. Each step, the camera orbits counter-clockwise. Equivalently, the
  # model rotates clockwise.
  # 
  # We define only four possible return values. That's because each diagonal is, for
  # this purpose at least, equivalent to the preceding orthogonal direction. E.g. the
  # verts for viewing direction 3 (a diagonal) are the same as for 2.
  def verts(s, direction):
    return  [ BoundingBoxVerts( Vector([s.min_x, s.max_y, s.max_z]),  # Top back.
                                Vector([s.max_x, s.max_y, s.max_z]),  # Top right.
                                Vector([s.max_x, s.min_y, s.max_z]),  # Top front.
                                Vector([s.min_x, s.min_y, s.max_z]),  # Top left.
                                Vector([s.min_x, s.min_y, s.min_z]),  # Bottom left.
                                Vector([s.max_x, s.min_y, s.min_z]),  # Bottom front.
                                Vector([s.max_x, s.max_y, s.min_z])), # Bottom right.
              
              BoundingBoxVerts( Vector([s.min_x, s.min_y, s.max_z]),  # Top back.
                                Vector([s.min_x, s.max_y, s.max_z]),  # Top right.
                                Vector([s.max_x, s.max_y, s.max_z]),  # Top front.
                                Vector([s.max_x, s.min_y, s.max_z]),  # Top left.
                                Vector([s.max_x, s.min_y, s.min_z]),  # Bottom left.
                                Vector([s.max_x, s.max_y, s.min_z]),  # Bottom front.
                                Vector([s.min_x, s.max_y, s.min_z])), # Bottom right.
              
              BoundingBoxVerts( Vector([s.max_x, s.min_y, s.max_z]),  # Top back.
                                Vector([s.min_x, s.min_y, s.max_z]),  # Top right.
                                Vector([s.min_x, s.max_y, s.max_z]),  # Top front.
                                Vector([s.max_x, s.max_y, s.max_z]),  # Top left.
                                Vector([s.max_x, s.max_y, s.min_z]),  # Bottom left.
                                Vector([s.min_x, s.max_y, s.min_z]),  # Bottom front.
                                Vector([s.min_x, s.min_y, s.min_z])), # Bottom right.
              
              BoundingBoxVerts( Vector([s.max_x, s.max_y, s.max_z]),  # Top back.
                                Vector([s.max_x, s.min_y, s.max_z]),  # Top right.
                                Vector([s.min_x, s.min_y, s.max_z]),  # Top front.
                                Vector([s.min_x, s.max_y, s.max_z]),  # Top left.
                                Vector([s.min_x, s.max_y, s.min_z]),  # Bottom left.
                                Vector([s.min_x, s.min_y, s.min_z]),  # Bottom front.
                                Vector([s.max_x, s.min_y, s.min_z])), # Bottom right.
             ][math.floor(direction / 2)]
  
  def x_size(self):
    return self.max_x - self.min_x
    
  def y_size(self):
    return self.max_y - self.min_y
  
  def z_size(self):
    return self.max_z - self.min_z

class BoundingBoxVerts:
  def __init__(self, tb, tr, tf, tl, bl, bf, br):
    self.tb = tb; self.tr = tr; self.tf = tf; self.tl = tl
    self.bl = bl; self.bf = bf; self.br = br

# Computes the bounding box of the entire model, i.e. everything on layer 0.
def model_bounding_box():
  r = BoundingBox()
  r.min_x = 0
  r.max_x = 0
  r.min_y = 0
  r.max_y = 0
  r.min_z = 0
  r.max_z = 0
  for obj in model_objects():
    b = object_bounding_box(obj)
    if b.min_x < r.min_x:
      r.min_x = b.min_x
    if b.max_x > r.max_x:
      r.max_x = b.max_x
    if b.min_y < r.min_y:
      r.min_y = b.min_y
    if b.max_y > r.max_y:
      r.max_y = b.max_y
    if b.min_z < r.min_z:
      r.min_z = b.min_z
    if b.max_z > r.max_z:
      r.max_z = b.max_z
  return r

# Map Blender's definition of the bounding box into a more sane representation.  
def object_bounding_box(o):
  b = o.bound_box
  r = BoundingBox()
  r.min_x = (b[0][0] * o.scale.x) + o.location.x
  r.max_x = (b[4][0] * o.scale.x) + o.location.x
  r.min_y = (b[0][1] * o.scale.y) + o.location.y
  r.max_y = (b[3][1] * o.scale.y) + o.location.y
  r.min_z = (b[0][2] * o.scale.z) + o.location.z
  r.max_z = (b[1][2] * o.scale.z) + o.location.z
  return r

# The model is defined as the set of all objects on layer 0.
def model_objects():
  return [o for o in bpy.data.objects if o.layers[0]]