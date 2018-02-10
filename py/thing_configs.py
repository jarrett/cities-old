# Usage:
# python py/thing_configs.py

KEYS = [
  "thing_main_model",
  "thing_base_x_size",
  "thing_base_y_size",
  "thing_base_x_offset",
  "thing_base_y_offset",
  "thing_builds_foundation",
  "thing_digs_foundation",
  "thing_foundation_min_slope",
  "thing_built_foundation_wall_texture",
  "thing_built_foundation_cap_texture",
  "thing_dug_foundation_wall_texture",
  "thing_dug_foundation_floor_texture",
  "thing_has_footer",
  "thing_footer_model",
  "thing_has_pylon",
  "thing_pylon_model",
  "thing_pylon_align",
  "thing_pylon_repeat"
]

if __name__ == "__main__":
  rust_file = open("src/thing_configs.rs", "w")
  for idx, key in enumerate(KEYS):
    rust_file.write("static %s: u8 = %d;\n" % (key.upper(), idx))
  rust_file.close()