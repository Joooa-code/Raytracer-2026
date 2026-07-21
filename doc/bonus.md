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

## Example
loading a Heart model, only focus on position and shape, and ignoring normal and uv

see in `output/geometry/image1.png`

![Heart obj](../output/geometry/image1.png)

# Texture Mapping

## Overview

After implementing OBJ model loading, I extended the ray tracer with **texture mapping support** using the UV coordinates provided by OBJ files.

The OBJ format stores not only vertex positions, but also texture coordinates (`vt`) that describe how a 2D image should be mapped onto the 3D surface. By interpolating these UV coordinates on each triangle, the renderer can retrieve the corresponding pixel from a texture image and use it as the surface color.

This allows imported models to preserve their original appearance instead of being rendered with a single solid material color.

## Implementation

### 1. OBJ UV Coordinate Loading

The OBJ loader was extended to read texture coordinates:

```rust
let texcoords = &mesh.texcoords;
let uv0 =
(
    texcoords[i0 * 2] as f64,
    texcoords[i0 * 2 + 1] as f64
);

let uv1 =
(
    texcoords[i1 * 2] as f64,
    texcoords[i1 * 2 + 1] as f64
);

let uv2 =
(
    texcoords[i2 * 2] as f64,
    texcoords[i2 * 2 + 1] as f64
);
```

### 2. Triangle UV Interpolation

A triangle surface point can be represented using barycentric coordinates:

P=αP0+βP1+γP2
	​
where:

α+β+γ=1

The same interpolation method is applied to UV coordinates.

### 3. Image Texture Implementation

The image data is shared using Arc because multiple triangles can reference the same texture.
```
pub struct ImageTexture {

    image: Arc<RTWImage>

}
```

### 4. Material Loading from MTL Files

OBJ models usually reference a material file (.mtl).

Example:
```
newmtl Cat

Kd 1.0 1.0 1.0

map_Kd Cat_diffuse.jpg
```
map_Kd represents the diffuse texture.

During loading:
```
if let Some(texture_path)
    = &mat.diffuse_texture
{

    let tex =
        Arc::new(
            ImageTexture::new(texture_path)
        );

    return Arc::new(
        Lambertian::new(tex)
    );
}
```
If no texture exists, the diffuse color is used:

```
Lambertian::new_color(
    diffuse_color
)
```

### 5. Rendering Pipeline

The final pipeline becomes:
```
OBJ file
   |
   |
   v
Load vertices
Load UV coordinates
Load material IDs
   |
   |
   v
Triangle
(position + uv)
   |
   |
   v
Ray intersection
   |
   |
   v
Compute barycentric coordinates
   |
   |
   v
Interpolate UV
   |
   |
   v
Sample texture image
   |
   |
   v
Return surface color
```

## Example
loading a cat with origin texture

see in output/geometry/image2.png
![Heart obj](../output/geometry/image2.png)

# PDF Method

finish chapter1~chapter6

# Optimization
## Multi-threading
To improve the rendering performance of the ray tracer, I implemented a multi-threaded rendering pipeline using the Rayon parallel computing library in Rust.

### Motivation
The original renderer processes pixels sequentially. For each scanline, it iterates through every pixel and performs Monte Carlo sampling. Since each pixel is independent from others, the rendering process is highly parallelizable.

### Original Implementation

```
 let mut img: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for sj in 0..self.sqrt_spp {
                    for si in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, si, sj);
                        pixel_color += self.ray_color(&r, self.max_depth, world.as_ref());
                    }
                }
                pixel_color *= self.pixel_samples_scale;
                let pixel = img.get_pixel_mut(i as u32, j as u32);
                *pixel = color::write_color(&pixel_color);
            }
        }
        eprintln!("\rDone.");
        img.save(path).expect("Cannot save the image to the file");
```        

### Parallel Implementation
I replaced the outer scanline loop with Rayon’s parallel iterator:
Instead of manually creating and managing threads, Rayon automatically distributes scanlines among available CPU threads.

```
use rayon::prelude::*;
...
let mut pixels: Vec<(usize, usize, Color)> = (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                let mut row = Vec::with_capacity(self.image_width);

                for i in 0..self.image_width {
                    let mut pixel_color = Color::zero();
                    for sj in 0..self.sqrt_spp {
                        for si in 0..self.sqrt_spp {
                            let r = self.get_ray(i, j, si, sj);

                            pixel_color += self.ray_color(&r, self.max_depth, world.as_ref());
                        }
                    }
                    pixel_color *= self.pixel_samples_scale;
                    row.push((i, j, pixel_color));
                }

                row
            })
            .collect();

        eprintln!("\rDone rendering.");
        let mut img: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);
        for (i, j, color) in pixels {
            let pixel = img.get_pixel_mut(i as u32, j as u32);
            *pixel = color::write_color(&color);
        }
        img.save(path).expect("Cannot save image");
```        

### Advantages
1. Better CPU utilization

    The original version only uses one CPU core.
    
    The Rayon implementation automatically uses multiple cores:

```
Before:

CPU Core 1  ███████████
CPU Core 2  ░░░░░░░░░░░
CPU Core 3  ░░░░░░░░░░░
CPU Core 4  ░░░░░░░░░░░


After:

CPU Core 1  ███████████
CPU Core 2  ███████████
CPU Core 3  ███████████
CPU Core 4  ███████████
```

2. No manual thread management

3. Maintains Rust safety guarantees

    The world is shared through:

```
Arc<dyn Hittable + Send + Sync>
```

This guarantees that multiple rendering threads can safely  access the scene.