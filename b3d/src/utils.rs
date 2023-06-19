use std::io::{Read, Seek};
use std::fmt;
use byteorder::{ReadBytesExt, LittleEndian};

use crate::Error;

/// The `Vec2` is used for 2D dimensions.
pub type Vec2 = [f32; 2];

/// The `Vec3` is used by postions, normals, etc.
pub type Vec3 = [f32; 3];

/// The `Vec4` is used by Quats and Colors.
pub type Vec4 = [f32; 4];

pub fn read_null_term_string<T>(data: &mut T) -> String
where
    T: Read + Seek
{
    let mut string = vec![];
    loop {
        let byte = data.read_u8().unwrap();
        if byte == 0 {
            break;
        }
        string.push(byte);
    }
    String::from_utf8(string).unwrap()
}

pub fn eof<T>(data: &mut T, next: u64) -> Result<bool, Error>
where
    T: Seek
{
    Ok(data.stream_position()? < next)
}

#[derive(Debug)]
pub struct Chunk {
    pub tag: String,
    pub size: u32,
    
    pub position: u64,
    pub next: u64,
}

impl Chunk {
    pub fn read<T>(data: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek
    {
        let position = data.stream_position()?;
        let mut tag_buf = vec![0; 4];
        data.read_exact(&mut tag_buf)?;
        let tag = String::from_utf8(tag_buf)?;

        let size = data.read_u32::<LittleEndian>()?;
        let next = position + (size as u64) + 8;

        Ok(Self {
            tag,
            size,
            position,
            next,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tag: {}, Size: {}, Position: {}, Next: {}", self.tag, self.size, self.position, self.next)
    }
}
