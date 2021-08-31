---
# These are meta-variables defined with YAML syntax, change them as you wish.
# see https://pandoc.org/MANUAL.html#variables
title: TDT4195 Assignment 1
author:
- Andreas Klavenes Berg
- Steffen André Haug
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: "List of Contents"
toc-depth: 2
numbersections: true
colorlinks: true
links-as-notes: true
# The document is following the break written using Markdown syntax
---

## 1c

![5 distinct triangles](./images/file)

## (2a) Clipping

1. *Name of the phenomenon when clipping happens* 
2. *short*: This happens when a shape goes outside the ±1.0 bounding box that defines the viewport.
3. *short, purpose*: The purpose of clipping is to avoid spending resources on calculating fragments that will not eventually appear on screen.

## (2b) Backface culling

(1.2.3.) Changing the order of the indices to clockwise order makes the triangles disappear. This happens because backface culling is enabled, meaning that faces that are expected to be backsides, i.e. only visible inside a closed model, are ignored to save resources.

## 2c

1. *Why does the depth buffer need to be reset each frame?*
    - The depth buffer is used in the depth test by starting far out and updating with the closest fragment to the camera at any point. This means the buffer must be reset to a high value for each frame in order to work.
2. *In which situation can the fragment shader be executed multiple times for the same pixel?*
    - Multisampling
    - Overlapping triangles; depth test after FS (https://www.khronos.org/opengl/wiki/Rendering_Pipeline_Overview)
3. *What are the two most commonly used types of shaders? What are the responsibilities of each of them?*
    - The two most commonly used types of shaders are vertex and fragment shaders.
    - The vertex shader transforms the model from the model coordinate system into positions for drawing on the screen.
    - The fragment shader runs once per fragment, i.e. approximately once per pixel, in order to set its colour.
4. *Why is it common to use an index buffer to specify which vertices should be connected into triangles, as opposed to relying on the order in which vertices are specified into this function?*
    - Using an index buffer to select vertices from the vertex buffer can save a lot of memory, as most vertices in a model appear in multiple triangles. Even in a simple cube, one vertex can be a part of up to six triangles.
5. *While the last input of `gl::VertexAttribPointer()` is a pointer, usually a null pointer is passed in. Describe a situation in which you would pass a non-zero value into this function.*
    - One could have an attribute stored in a separate array. If so, a pointer to it would need to be passed. More usually, all attributes after the first one also needs to pass the offset of the attribute. If the vertex attributes consist of position and texture mapping `[x, y, z, u, v]`, all being 4 byte floats, then the texture map would be at an offset `3 * 4 = 12`.

!!! note
    Hei

## 2d

![Mirrored scene](./images/mirrored_scene)

Mirroring the scene is accomplished by mirroring the coordinates in the shader before passing them to `gl_Position`

![Changed colour](./images/changes_colour)

Colouring the triangle was accomplished by forwarding the position vector from the vertex shader to the fragment shader, and passing the coordinates as colour values.
