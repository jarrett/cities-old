import bpy

class CitiesPanel(bpy.types.Panel):
  bl_idname = "OBJECT_PT_cities"
  bl_label = "Cities"
  bl_space_type = "PROPERTIES"
  bl_region_type = "WINDOW"
  bl_context = "render"
  
  def draw(self, context):
    layout = self.layout
    row = layout.row()
    row.operator("render.cities_export")