# Ray Tracer Bonus
# Geometry

## Overview

This document records the implementation of the Geometry module in the
Rust Ray Tracer project.

The geometry system is responsible for representing objects in the
scene, calculating ray-object intersections, and providing intersection
information for shading and material calculations.

## Implementation
### Triangle

File:

    src/triangle.rs

Triangle is the fundamental primitive for representing complex 3D
models.

A triangle consists of three vertices:

            v2
            *
           / \
          /   \
         /     \
        *-------*
       v0       v1

The triangle can be represented as:

    P = A + u(B-A) + v(C-A)

where `u` and `v` are barycentric coordinates.

The intersection algorithm uses the Möller--Trumbore algorithm.

The algorithm solves:

    Ray:
    P = O + tD

    Triangle:
    P = A + u(B-A) + v(C-A)

The resulting parameters:

-   `t`: distance from ray origin to intersection
-   `u,v`: position inside triangle

A valid intersection requires:

    u >= 0
    v >= 0
    u + v <= 1

### Model

`model.rs` is responsible for loading external 3D models into the ray tracer.

The main purpose of this module is to convert an OBJ model file into the internal geometry representation used by the renderer.

```
OBJ File

    |
    v

obj loader

    |
    v

Vertex positions

Triangle indices

    |
    v

Triangle objects

    |
    v

HittableList

    |
    v

BVHNode
```

the specific implementation
```
pub fn load(filename: &str, mat: Arc<dyn Material + Send + Sync>) -> HittableList {
    let options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };
    let (models, _materials) = tobj::load_obj(filename, &options).expect("cannot load obj");
    let mut world = HittableList::default();
    for model in models {
        let mesh = model.mesh;
        let positions = &mesh.positions;
        let indices = &mesh.indices;

        for face in indices.chunks(3) {
            ...
        }
    }
    world
}
```

### Example
loading a Heart model, only focus on position and shape, and ignoring normal and uv

see in 'output/geometry/image1.png'

![Heart obj](../output/geometry/image1.png)