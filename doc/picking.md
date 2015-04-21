# Picking

This is the algorithm for determining where the mouse is pointing.

The terrain and all Things are referenced in a quad-tree. A "leaf" is a node without
children. A "branch" is a node with children.

Each node of the quad-tree, whether leaf or branch, has zero or more click targets. A
click target could be a terrain triangle or a Thing.

The leaf size is one world unit. Every branch of the tree is fully subdivided into leaves.
In other words, the tree is balanced.

Accordingly, each leaf fully contains exactly two terrain triangles.

Sometimes, a Thing doesn't fit inside a single leaf. In this case, it belongs to the
smallest branch that fully contains it.

## Building the tree

Start with the root node. Recursively:

    If size > 1, then for each quadrant:
      Replace the quadrant, which is currently empty, with a new node,
      half the size of this one.
      Recur.      

## Insertion

Start with the root node. Recursively:

    If not leaf and q1 fully contains this target:
      Recur in q1.
    Else if not leaf and q2 fully contains this target:
      Recur in q2.
    Else if not leaf and q3 fully contains this target:
      Recur in q3.
    Else if not leaf and q4 fully contains this target:
      Recur in q4.
    Else:
      Append target to node.
      Node max Z = max(node max Z, target max Z).
      Node min Z = min(node min Z, target min Z).

## Removal
    
    Node max Z = 0.
    Node min Z = 0.
    For each child:
      Node max Z = max(node max Z, child max Z)
      Node min Z = min(node min Z, child min Z)
    For each target:
      Node max Z = max(node max Z, target max Z)
      Node min Z = min(node min Z, target min Z)