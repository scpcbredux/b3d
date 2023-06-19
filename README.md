## B3D

[![crates.io](https://img.shields.io/crates/v/b3d.svg)](https://crates.io/crates/b3d) [![docs.rs](https://docs.rs/b3d/badge.svg)](https://docs.rs/b3d)

The [Blitz3d game engine](https://github.com/blitz-research/blitz3d) uses the `.b3d` extension, which is provided by this crate.

### Usage

```rust
let bytes = unimplemented!();

let b3d = b3d::B3D::read(bytes).unwrap();

let vertices = b3d.node.mesh.vertices.vertices;
let positions: Vec<_> = vertices.iter().map(|v| v.position).collect();
let normals: Vec<_> = vertices.iter().map(|v| v.normal).collect();

println!("Postions: {:#?}", positions);
println!("Normals: {:#?}", normals);
```

### Task list

- [ ] Write documentation
- [ ] Switch to binrw
- [ ] Implement bones and weights
- [ ] Add examples

#### Similar Projects
- [Blender b3d import/export plugin](https://github.com/joric/io_scene_b3d)
