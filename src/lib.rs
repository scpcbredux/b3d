mod error;
mod utils;

use std::io::Cursor;
use std::io::{Read, Seek};
use byteorder::{ReadBytesExt, LittleEndian};
use anyhow::Result;
use crate::error::B3dError;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Texture {
    pub file: String,
    pub flags: u32,
    pub blend: u32,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Texture {
    pub fn read<T>(data: &mut T) -> Result<Self>
    where
        T: Read + Seek
    {
        let file = read_null_term_string(data);
        let flags = data.read_u32::<LittleEndian>()?;
        let blend = data.read_u32::<LittleEndian>()?;
        let mut position = [0.0; 2];
        data.read_f32_into::<LittleEndian>(&mut position)?;
        let mut scale = [0.0; 2];
        data.read_f32_into::<LittleEndian>(&mut scale)?;
        let rotation = data.read_f32::<LittleEndian>()?;

        Ok(Self {
            file,
            flags,
            blend,
            position,
            scale,
            rotation,
        })
    }
}

#[derive(Debug)]
pub struct Brush {
    pub name: String,
	pub color: Vec4,
	pub shininess: f32,
	pub blend: u32,
	pub fx: u32,
	pub texture_id: Vec<u32>,
}

impl Brush {
    pub fn read<T>(data: &mut T, n_texs: usize) -> Result<Self>
    where
        T: Read + Seek
    {
        let name = read_null_term_string(data);
        let mut color = [0.0; 4];
        data.read_f32_into::<LittleEndian>(&mut color)?;
        let shininess = data.read_f32::<LittleEndian>()?;
        let blend = data.read_u32::<LittleEndian>()?;
        let fx = data.read_u32::<LittleEndian>()?;

        let mut texture_id = vec![];

        for _ in 0..n_texs {
            texture_id.push(data.read_u32::<LittleEndian>()?);
        }

        Ok(Self {
            name,
            color,
            shininess,
            blend,
            fx,
            texture_id,
        })
    }
}

#[derive(Debug, Default)]
pub struct Vertice {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec4,
    pub tex_coords: Vec2,
}

#[derive(Debug, Default)]
pub struct Verts {
    pub flags: u32,
    pub tex_coord_sets: u32,
    pub tex_coord_set_size: u32,
    pub vertices: Vec<Vertice>,
}

impl Verts {
    pub fn read<T>(data: &mut T, next: u64) -> Result<Self>
    where
        T: Read + Seek
    {
        let flags = data.read_u32::<LittleEndian>()?;
        let tex_coord_sets = data.read_u32::<LittleEndian>()?;
        let tex_coord_set_size = data.read_u32::<LittleEndian>()?;

        let mut vertices: Vec<Vertice> = Vec::new();

        while eof(data, next)? {
            let mut position = [0.0; 3];
            data.read_f32_into::<LittleEndian>(&mut position)?;
            let mut normal = [0.0; 3];
            if flags & 1 != 0 {
                data.read_f32_into::<LittleEndian>(&mut normal)?;
            }
            let mut color = [0.0; 4];
            if flags & 2 != 0 {
                data.read_f32_into::<LittleEndian>(&mut color)?;
            }
            // This system doesn't work with bevy >:(
            // let mut tex_coords = Vec::new();
            // for _ in 0..(tex_coord_sets * tex_coord_set_size) as usize {
            //     tex_coords.push(data.read_f32::<LittleEndian>()?);
            // }
            let mut tex_coords = [0.0; 2];
            data.read_f32_into::<LittleEndian>(&mut tex_coords)?;

            vertices.push(Vertice {
                position,
                normal,
                color,
                tex_coords,
            });
        }

        Ok(Self {
            flags,
            tex_coord_sets,
            tex_coord_set_size,
            vertices,
        })
    }
}

#[derive(Debug)]
pub struct Tris {
    pub brush_id: u32,
    pub indices: Vec<[u32; 3]>,
}

impl Tris {
    pub fn read<T>(data: &mut T, next: u64) -> Result<Self>
    where
        T: Read + Seek
    {
        let brush_id = data.read_u32::<LittleEndian>()?;
        let mut indices = Vec::new();

        while eof(data, next)? {
            let mut face = [0; 3];
            data.read_u32_into::<LittleEndian>(&mut face)?;
            indices.push(face);
        }

        Ok(Self {
            brush_id,
            indices,
        })
    }
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub brush_id: u32,
    pub vertices: Verts,
    pub triangles: Vec<Tris>,
}

impl Mesh {
    pub fn read<T>(data: &mut T, next: u64) -> Result<Self>
    where
        T: Read + Seek
    {
        let brush_id = data.read_u32::<LittleEndian>()?;
        let vert_chunk = Chunk::read(data)?;
        let vertices = Verts::read(data, vert_chunk.next)?;
        let mut triangles = Vec::new();

        while eof(data, next)? {
            let tri_chunk = Chunk::read(data)?;
            triangles.push(Tris::read(data, tri_chunk.next)?);
        }

        Ok(Self {
            brush_id,
            vertices,
            triangles,
        })
    }
}

#[derive(Debug, Default)]
pub struct Bone {
    pub vertex_id: u32,
    pub weight: f32,
}

impl Bone {
    pub fn read<T>(data: &mut T) -> Result<Self>
    where
        T: Read
    {
        Ok(Self {
            vertex_id: data.read_u32::<LittleEndian>()?,
            weight: data.read_f32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
pub struct Key {
    pub frame: u32,
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec4,
}

impl Key {
    pub fn read<T>(data: &mut T, flags: u32) -> Result<Self>
    where
        T: Read + Seek
    {
        let frame = data.read_u32::<LittleEndian>()?;

        let mut position = [0.0; 3];
        if flags & 1 != 0 {
            data.read_f32_into::<LittleEndian>(&mut position)?;
        }
        let mut scale = [0.0; 3];
        if flags & 2 != 0 {
            data.read_f32_into::<LittleEndian>(&mut scale)?;
        }
        let mut rotation = [0.0; 4];
        if flags & 4 != 0 {
            data.read_f32_into::<LittleEndian>(&mut rotation)?;
        }

        Ok(Self {
            frame,
            position,
            scale,
            rotation,
        })
    }
}

#[derive(Debug, Default)]
pub struct Animation {
    pub flags: u32,
    pub frames: u32,
    pub fps: f32,
}

impl Animation {
    pub fn read<T>(data: &mut T, _next: u64) -> Result<Self>
    where
        T: Read + Seek
    {
        Ok(Self {
            flags: data.read_u32::<LittleEndian>()?,
            frames: data.read_u32::<LittleEndian>()?,
            fps: data.read_f32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
pub struct Sequence {
    pub name: String,
    pub something: u32,
    pub something2: u32,
    pub something3: u32,
}

impl Sequence {
    pub fn read<T>(data: &mut T, _next: u64) -> Result<Self>
    where
        T: Read + Seek
    {
        Ok(Self {
            name: read_null_term_string(data),
            something: data.read_u32::<LittleEndian>()?,
            something2: data.read_u32::<LittleEndian>()?,
            something3: data.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
pub struct Node {
    pub name: String,
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Vec4,
    pub mesh: Mesh,
    pub bones: Vec<Bone>,
    pub key_flags: u32,
    pub keys: Vec<Key>,
    pub children: Vec<Node>,
    pub animation: Animation,
    pub sequences: Vec<Sequence>,
}

impl Node {
    pub fn read<T>(data: &mut T, next: u64) -> Result<Self>
    where
        T: Read + Seek
    {
        let name = read_null_term_string(data);
        let mut position = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut position)?;
        let mut scale = [0.0; 3];
        data.read_f32_into::<LittleEndian>(&mut scale)?;
        let mut rotation = [0.0; 4];
        data.read_f32_into::<LittleEndian>(&mut rotation)?;

        let mut mesh = Mesh::default();
        let mut children = Vec::new();
        let mut bones = Vec::new();
        let mut animation = Animation::default();
        let mut sequences = Vec::new();
        let mut key_flags = 0;
        let mut keys = Vec::new();

        while eof(data, next)? {
            let chunk = Chunk::read(data)?;
            match chunk.tag.as_str() {
                "MESH" => mesh = Mesh::read(data, chunk.next)?,
                "BONE" => bones = Self::read_bones(data, chunk.next)?,
                "KEYS" => {
                    key_flags = data.read_u32::<LittleEndian>()?;
                    keys = Self::read_keys(data, chunk.next, key_flags)?;
                },
                "NODE" => children.push(Node::read(data, chunk.next)?),
                "ANIM" => animation = Animation::read(data, chunk.next)?,
                "SEQS" => sequences.push(Sequence::read(data, chunk.next)?),
                _ => return Err(B3dError::InvalidChunk(chunk).into()),
            }
        }

        Ok(Self {
            name,
            position,
            scale,
            rotation,
            mesh,
            bones,
            key_flags,
            keys,
            children,
            animation,
            sequences,
        })
    }

    pub fn read_bones<T>(data: &mut T, next: u64) -> Result<Vec<Bone>>
    where
        T: Read + Seek
    {
        let mut bones = vec![];
        while eof(data, next)? {
            bones.push(Bone::read(data)?);
        }
        Ok(bones)
    }

    pub fn read_keys<T>(data: &mut T, next: u64, flags: u32) -> Result<Vec<Key>>
    where
        T: Read + Seek
    {
        let mut keys = vec![];
        while eof(data, next)? {
            keys.push(Key::read(data, flags)?);
        }
        Ok(keys)
    }
}

/// Example
///
/// ```rs
/// let bytes = unimplemented!();
///
/// let b3d = b3d::B3D::read(bytes)?;
///
/// let vertices = b3d.node.mesh.vertices.vertices;
/// let positions: Vec<_> = vertices.iter().map(|v| v.position).collect();
/// let normals: Vec<_> = vertices.iter().map(|v| v.normal).collect();
///
/// println!("Postions: {:#?}", positions);
/// println!("Normals: {:#?}", normals);
/// ```
#[derive(Debug)]
pub struct B3D {
    pub version: u32,
    pub textures: Vec<Texture>,
    pub brushes: Vec<Brush>,
    pub node: Node,
}

impl B3D {
    pub fn read(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data);

        let main_chunk = Chunk::read(&mut cursor)?;
        if main_chunk.tag != "BB3D" {
            return Err(B3dError::InvalidChunk(main_chunk).into());
        }
        let version = cursor.read_u32::<LittleEndian>()?;
        let mut textures = Vec::new();
        let mut brushes = Vec::new();
        let mut node = Node::default();

        while eof(&mut cursor, main_chunk.next)? {
            let chunk = Chunk::read(&mut cursor)?;
            match chunk.tag.as_str() {
                "TEXS" => textures = Self::read_textures(&mut cursor, chunk.next)?,
                "BRUS" => brushes = Self::read_brushes(&mut cursor, chunk.next)?,
                "NODE" => node = Node::read(&mut cursor, chunk.next)?,
                _ => return Err(B3dError::InvalidChunk(chunk).into()),
            }
        }

        Ok(Self {
            version,
            textures,
            brushes,
            node,
        })
    }

    pub fn read_textures<T>(data: &mut T, next: u64) -> Result<Vec<Texture>>
    where
        T: Read + Seek
    {
        let mut textures = vec![];
        while eof(data, next)? {
            textures.push(Texture::read(data)?);
        }
        Ok(textures)
    }

    pub fn read_brushes<T>(data: &mut T, next: u64) -> Result<Vec<Brush>>
    where
        T: Read + Seek
    {
        let mut brushes = vec![];
        let n_texs = data.read_u32::<LittleEndian>()?;
        while eof(data, next)? {
            brushes.push(Brush::read(data, n_texs as usize)?);
        }
        Ok(brushes)
    }
}
