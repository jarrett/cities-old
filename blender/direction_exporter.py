PIXELS_PER_WORLD_UNIT = 100

import bpy
from math import radians
from mathutils import Euler
from cities.bounding_box import model_bounding_box
from cities.camera import model_view_matrix, projection_matrix

# Create a new instance of this for each rendering direction.
class DirectionExporter:
  def __init__(self, direction, image_path):
    self.direction = direction
    self.image_path = image_path
  
  def export(self):
    self.bounds = model_bounding_box()
    self.bounds_verts = self.bounds.verts(self.direction)
    # Placing the camera depends on knowing the bounds.
    self.place_rig()
    # The MVM must be calculated after the camera is placed.
    self.mvm = model_view_matrix()
    # The camera's projection can only be determined once we know the MVM.
    self.zoom_camera()
    
    render = bpy.data.scenes["Scene"].render
    
    render.filepath = '%s-%d.png' % (self.image_path, self.direction)
    bpy.ops.render.render(write_still = True)
    
    dir_uv_data = { 'tb': (self.projection * self.mvm * self.bounds_verts.tb).to_2d().to_tuple(),
                    'tr': (self.projection * self.mvm * self.bounds_verts.tr).to_2d().to_tuple(),
                    'tf': (self.projection * self.mvm * self.bounds_verts.tf).to_2d().to_tuple(),
                    'tl': (self.projection * self.mvm * self.bounds_verts.tl).to_2d().to_tuple(),
                    'bl': (self.projection * self.mvm * self.bounds_verts.bl).to_2d().to_tuple(),
                    'bf': (self.projection * self.mvm * self.bounds_verts.bf).to_2d().to_tuple(),
                    'br': (self.projection * self.mvm * self.bounds_verts.br).to_2d().to_tuple() }
    return dir_uv_data
    
    
  
  def place_rig(self):
    camera_parent = bpy.data.objects['Camera Parent']
    lighting_parent = bpy.data.objects['Lighting Parent']
    
    # Rotate the lighting and camera rig.
    z_rotation = radians(28) + (self.direction * radians(45))
    camera_parent.rotation_euler   = Euler((radians(48), 0, z_rotation), 'XYZ')
    lighting_parent.rotation_euler = Euler((0,           0, z_rotation), 'XYZ')
    
    # Move the camera target to the model's vertical center. The camera target remains
    # at x = 0 and y = 0, though.
    #model_x_size = self.bounds.max_x - self.bounds.min_x
    #model_y_size = self.bounds.max_y - self.bounds.min_y
    model_z_size = self.bounds.max_z - self.bounds.min_z
    #rig_parent.location.x = (model_x_size / 2) + self.bounds.min_x
    #rig_parent.location.y = (model_y_size / 2) + self.bounds.min_y
    camera_parent.location.z = (model_z_size / 2) + self.bounds.min_z
    bpy.data.scenes['Scene'].update() # Or else the matrix will be stale when we retrieve it.
  
  def zoom_camera(self):
    # We need to know the ideal width and height of the orthographic camera's viewport,
    # measured in world units. Those values are proportional to the width and height of
    # the model transformed with the modelview matrix.
    cam = bpy.data.objects['Camera'].data
    
    tb_trans = self.mvm * self.bounds_verts.tb         # Top-back vertex.
    bf_trans = self.mvm * self.bounds_verts.bf         # Bottom-front vertex.
    bl_trans = self.mvm * self.bounds_verts.bl         # Bottom-left vertex.
    br_trans = self.mvm * self.bounds_verts.br         # Bottom-right vertex.
    
    height = (tb_trans.y - bf_trans.y)    * 1.02  # Height of the camera's viewport, in world units.
    width  = abs(bl_trans.x - br_trans.x) * 1.02  # Width of the camera's viewport, in world units.
    
    render = bpy.data.scenes['Scene'].render
    render.image_settings.color_mode = 'RGBA'
    if width >= height or render.engine == 'YAFA_RENDER':
      # In the Blender UI and Blender Render, the camera's ortho scale defines the size
      # (in world units) of the viewport's largest dimension. But in Yafaray, the ortho
      # scale always maps to the width.
      cam.ortho_scale = width
    else:
      cam.ortho_scale = height
    
    render.resolution_x = width  * PIXELS_PER_WORLD_UNIT
    render.resolution_y = height * PIXELS_PER_WORLD_UNIT
    
    self.projection = projection_matrix(width, height)