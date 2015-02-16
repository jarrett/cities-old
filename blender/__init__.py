# To install this script in such a way that you can conveniently edit and test it,
# navigate to the Blender addons folder, e.g.:
# 
# cd "~/Library/Application Support/Blender/2.67/scripts/addons"
# 
# Symlink from this script's containing folder, e.g.:
#
# ln -s ~/apps/cities/blender cities
# 
# In Blender, choose File -> User Preferences -> Addons. You should see an addon called
# "Import-Export: Cities." Click the checkbox to enable it. The addon's GUI should appear
# at the bottom of the Render panel.

bl_info = { "name": "Cities",
            "description": "Utilities for the open-source cities game.",
            "author": "Jarrett Colby",
            "version": (0, 0),
            "blender": (2, 67, 0),
            "category": "Import-Export" }
  
import bpy
from cities.panel import CitiesPanel
from cities.operator import CitiesExportOperator

def register():
  bpy.utils.register_class(CitiesExportOperator)
  bpy.utils.register_class(CitiesPanel)

def unregister():
  bpy.utils.unregister_class(CitiesExportOperator)
  bpy.utils.unregister_class(CitiesPanel)
  
if __name__ == "__main__":
  register()