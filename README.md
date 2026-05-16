# CS2D Map Viewer
CS2D (the game, www.cs2d.com) map viewer written in Rust. Using Macroquad for rendering.
Runs on Win and web. Should also work on Linux and Mac (both unstested).
I chose this tech stack to ensure that the web build is tiny and loads quickly (currently the wasm file is less than 1 mb).
Another option would have been pure C with RayLib but Rust sounded safer.

Comes with JetBrains RustRover project setup but you can use whatever IDE you want.

## Current State
Currently this is heavily work in progress and misses most features. Current features
- always simply loads de_dust2
- scrolling with W/A/S/D, arrow keys and mouse
- render tiles, respecting some tile modifiers
- renders basic shadows

Missing:
- loading other maps (from file system or from folders/zips on the web server)
- entities (they are loaded but not displayed yet)
- light engine
- tile blending
- tile fx
- resource mangement
- particles and other effects
- user interface

## Why?
- Previewing CS2D maps on the web is nice. Plan is to embed this into the CS2D file archive at www.unrealsoftware.de
- Providing an open source example for loading and rendering CS2D maps
- Evaluating a new tech stack
