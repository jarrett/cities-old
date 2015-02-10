# Plugins

You can write plugins in C, C++, or Haskell. You might even be able to write them in
other languages. So long as you can get a pointer to your public API functions, you're
good.

## Function Registry

Often, configuration files (such as a MetaThing file) refer to functions by name. For example,
there is a Thing config called `pylon_function`, which is a string naming a function that
defines how pylons behave in a particular MetaThing.

But how can that be? In a C/C++ program, without relying on OS-specific functionality, you
can't dynamically call a function by name. E.g. you can't do something like
`call("MyFunction")`. So, to provide such functionality, we maintain a lookup table. The
keys are function names, and the values are function pointers.

You need to register your plugin's public API with that lookup table. (The public API is
whatever functions the game calls directly. You don't have to register every function
your plugin calls internally.) We have a C and a Haskell API for registering plugin
functions, but both APIs operate on the same internal lookup table.