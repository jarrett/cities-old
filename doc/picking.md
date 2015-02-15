# Picking

This is the algorithm for determining where the mouse is pointing.

The terrain and all Things are referenced in a quad-tree. The leaf size is four. In
other words, each leaf has four or fewer triangles and Things.