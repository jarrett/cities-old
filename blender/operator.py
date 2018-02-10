import bpy
import json
from cities.direction_exporter import DirectionExporter
from cities.bounding_box import model_bounding_box

class CitiesExportOperator(bpy.types.Operator):
  bl_idname = "render.cities_export"
  bl_label = "Render & Export"
  
  def execute(self, context):
    bounds = model_bounding_box()
    output = {'xSize': bounds.x_size(),
              'ySize': bounds.y_size(),
              'zSize': bounds.z_size(),
              'directions': {}}
    
    for direction in range(0,8):
      # FIXME
      exporter = DirectionExporter(direction, '/users/jarrett/apps/cities/assets/sprites/jarrett-test')
      dir_uv_data = exporter.export()
      output['directions'][direction] = dir_uv_data
    
    # FIXME
    file = open('/users/jarrett/apps/cities/assets/models/jarrett-test.model.blendout.json', 'w')
    file.write(json.dumps(output))
    file.close()
      
    return {'FINISHED'}
  
  @classmethod
  def poll(cls, context):
    return True  