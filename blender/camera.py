import bpy
from mathutils import Matrix

def model_view_matrix():
  return bpy.data.objects['Camera Parent'].matrix_world.inverted()

# An orthographic matrix. This assumes the projection is centered on the origin. (Because
# we'll be multiplying by the modelview matrix first, the origin for this purpose isn't
# Blender's world origin, but rather the origin in modelview space.)
def projection_matrix(width, height):
  m = Matrix.Identity(4)
  m[0][0] = 1 / width
  m[1][1] = -1 / height
  m[0][3] = 0.5
  m[1][3] = 0.5
  return m