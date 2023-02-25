## B3D

The [Blitz3d game engine](https://github.com/blitz-research/blitz3d) uses the `.b3d` extension, which is provided by this crate.

### Examples

Parsing and retrieving the positions and normals.

```rust
let bytes = unimplemented!();

let b3d = b3d::B3D::read(bytes)?;

let vertices = b3d.node.mesh.vertices.vertices;
let positions: Vec<_> = vertices.iter().map(|v| v.position).collect();
let normals: Vec<_> = vertices.iter().map(|v| v.normal).collect();

println!("Postions: {:#?}", positions);
println!("Normals: {:#?}", normals);
```

### Chunk Tables

**NOT ALL of these labels connects to the struct directly. (For instance, `TEXS` is an array of `Texture`).**

| Label | Struct |
|---------|---------|
| BB3D     | BB3D     |
| TEXS     | Texture     |
| BRUS     | Brush     |
| NODE     | Node     |
| MESH     | Mesh     |
| BONE     | Bone     |
| KEYS     | Key     |
| ANIM     | Animation     |
| SEQS     | Sequence     |

### Task list

- [ ] Write documentation
- [ ] Create a writer
- [ ] Optimize and cleanup the code

#### References
- [Blender b3d import/export plugin](https://github.com/joric/io_scene_b3d)
