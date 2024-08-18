use b3d::{B3D, Error};

fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    let _ = args.next();
    let bytes = std::fs::read(args.next().expect("No b3d file provided")).unwrap();
    let b3d = B3D::read(&bytes)?;

    let mut min_z = f32::INFINITY;
    let mut max_z = -f32::INFINITY;

    for vertex in &b3d.node.mesh.vertices.vertices {
        let z = vertex.position[2];
        min_z = min_z.min(z);
        max_z = max_z.max(z);
    }

    let depth = max_z - min_z;

    println!("{:#?}", b3d);
    println!("Mesh Depth: {depth}");

    Ok(())
}
