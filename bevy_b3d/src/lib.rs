pub use b3d;
pub use loader::*;

mod loader;

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{renderer::RenderDevice, texture::CompressedImageFormats},
};

/// Adss support for b3d file loading to the app.
#[derive(Default)]
pub struct B3DPlugin;

impl Plugin for B3DPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<B3D>()
            .init_asset::<B3DNode>()
            .init_asset::<B3DMesh>()
            .preregister_asset_loader::<B3DLoader>(&["b3d"]);
    }

    fn finish(&self, app: &mut App) {
        let supported_compressed_formats = match app.world().get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),

            None => CompressedImageFormats::NONE,
        };
        app.register_asset_loader(B3DLoader {
            supported_compressed_formats,
        });
    }
}

/// Representation of a loaded b3d file.
#[derive(Asset, Debug, TypePath)]
pub struct B3D {
    pub scene: Handle<Scene>,
    pub meshes: Vec<Handle<B3DMesh>>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub nodes: Vec<Handle<B3DNode>>,
}

/// A b3d node with all of its child nodes, its [`B3DMesh`] and [`Transform`]
#[derive(Asset, Debug, TypePath)]
pub struct B3DNode {
    pub children: Vec<B3DNode>,
    pub mesh: Option<Handle<B3DMesh>>,
    pub transform: Transform,
}

/// A b3d mesh, which may contists of a [`Mesh`] and an optional [`StandardMaterial`].
#[derive(Asset, Debug, TypePath)]
pub struct B3DMesh {
    pub mesh: Handle<Mesh>,
    pub material: Option<Handle<StandardMaterial>>,
}
