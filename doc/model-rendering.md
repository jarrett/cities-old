# Model Rendering

All meta models share a single VAO. Thus, they share a single attribute buffer and a single
index buffer.

Each meta model has a block of memory in the attribute buffer and the index buffer. The
size of the blocks depends on whether the meta model is 2d or 3d. Each meta model knows
its offset into the index buffer.

Each meta model buffers a separate set of vertex attributes for each viewing direction.

All meta models are buffered in one go during the game's initialization.

When a model renders, it delegates to its meta model, passing the model's absolute
position.

For now, each model and each direction has its own texture. That's inefficient, so in the
future, we'll stitch the textures together.

A model's vertices are labelled as follows:

                      TB (-x, -y, +z)
                     /   \
                   /       \
     (-x, +y, +z) TL        TR (+x, -y, +z)
                  |\       /|
                  |  \   /  |
                  |   TF    |
     (-x, +y, -z) BL   |    BR (+x, -y, -z)
                   \   |   /
                     \ | /
                      BF (+x, +y, -z)
    

           +Z axis 
              |
              |
              |
             / \
           /     \
    +Y axis      +X axis