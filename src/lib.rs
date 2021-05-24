mod asset;
mod convert;
mod pal;
mod shp;

pub use asset::{openra_mod_system, Mod, ModState};
use bevy::{
    asset::create_platform_default_asset_io,
    prelude::{AddAsset, AppBuilder, AssetServer, CoreStage, IntoSystem, Plugin},
    tasks::IoTaskPool,
};
pub use pal::{Pal, PalLoader};
pub use shp::{Shp, ShpLoader};
pub struct OpenRaAssetPlugin;

impl Plugin for OpenRaAssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Pal>();
        app.add_asset::<Shp>();
        app.add_asset::<Mod>();
        let asset_server = app.world_mut().get_resource_mut::<AssetServer>();
        match asset_server {
            Some(asset_server) => {
                asset_server.add_loader(PalLoader::default());
                asset_server.add_loader(ShpLoader::default());
            }
            None => {
                let task_pool = app
                    .world()
                    .get_resource::<IoTaskPool>()
                    .expect("`IoTaskPool` resource not found.")
                    .0
                    .clone();

                let source = create_platform_default_asset_io(app);

                let mut asset_server = AssetServer::with_boxed_io(source, task_pool);
                asset_server.add_loader(PalLoader::default());
                asset_server.add_loader(ShpLoader::default());
                app.insert_resource(asset_server);
            }
        }

        app.add_system_to_stage(CoreStage::PostUpdate, openra_mod_system.system());
    }
}
