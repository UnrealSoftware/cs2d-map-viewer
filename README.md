# CS2D Map Viewer
CS2D (the game, www.cs2d.com) map viewer written in Rust. Using Macroquad for rendering.
Runs on Win and web. Should also work on Linux and Mac (both unstested).
I chose this tech stack to ensure that the web build is tiny and loads quickly (currently the wasm file is less than 1 mb).
Another option would have been pure C with RayLib but Rust sounded safer.

Comes with JetBrains RustRover project setup but you can use whatever IDE you want.

## Current State
Currently this is work in progress and misses most features. Current features
- loadng map files (currenltly always de_dust2)
- scrolling with W/A/S/D, arrow keys and mouse
- render tiles, respecting some tile modifiers such as rotation and color/brightness
- render basic shadows
- loading content from zip files (if a zip with gfx/sfx/maps folders of cs2d is provided, no other files need to be loaded anymore; less overhead)

Missing:
- entities (they are loaded but not displayed yet)
- light engine
- tile blending
- tile fx
- resource mangement
- particles and other effects
- user interface (load other maps, show debug info, minimap)
- 3D rendering
- proper error handling / logging
- probably more...?

## Why?
The main reasons why I started this project are:
- Previewing CS2D maps on the web is nice. Plan is to embed this into the CS2D file archive at www.unrealsoftware.de
- Providing an open source example for loading and rendering CS2D maps
- Evaluating a new tech stack

## License
The source code of this project is licensed under the [MIT License](LICENSE).

**Crucial Exception:** All media assets (including images, logos, sounds, and music) are **All Rights Reserved** and are explicitly excluded from the MIT License. You may not reuse, redistribute, or modify these assets without written permission.
