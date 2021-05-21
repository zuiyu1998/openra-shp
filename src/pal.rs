use anyhow::anyhow;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use image::Rgba;

#[derive(Debug, TypeUuid, Default)]
#[uuid = "ced32c23-7db4-4b9c-853d-8a020f81a6e4"]
pub struct Pal {
    pub colors: Vec<Rgba<u8>>,
}

//asset
impl Pal {
    pub fn new(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut pal = Pal::default();
        if bytes.len() != 256 * 3 {
            return Err(anyhow!(
                "The pal file is damaged or the file format is wrong"
            ));
        }

        for i in 0..256 {
            pal.colors
                .push([bytes[3 * i + 2], bytes[3 * i + 1], bytes[3 * i], 0xFF as u8].into());
        }

        Ok(pal)
    }
}

#[derive(Debug, Default)]
pub struct PalLoader;

//asset_server load

//Assets
impl AssetLoader for PalLoader {
    fn load(
        &self,
        bytes: &[u8],
        load_context: &mut LoadContext,
    ) -> BoxedFuture<Result<(), anyhow::Error>> {
        match Pal::new(bytes) {
            Ok(pal) => {
                load_context.set_default_asset(LoadedAsset::new(pal));
                Box::pin(async move { Ok(()) })
            }
            Err(e) => Box::pin(async move { Err(e) }),
        }
    }

    fn extensions(&self) -> &[&str] {
        &["pal"]
    }
}
