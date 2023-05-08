OpenGL in X-Plane using Rust PoC Plugin

This is a bare-bones PoC plugin in Rust to make OpenGL work inside X-Plane on macOS.

- macOS only supports old GL context versions.
- macOS requires static bindings.
- macOS will tell you that dynamic bindings work and then crash or not draw.

There is a custom build.rs and some attribs in the rust files to make things work.

More work needs to be done to make this a cross plaform example.


Thanks to Sam Crow for his work on a rust framework for X-Plane (rust-xplm).


