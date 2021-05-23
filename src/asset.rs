use bevy::{
    asset::HandleId,
    prelude::{Assets, Handle, ResMut, Texture},
    reflect::TypeUuid,
    sprite::ColorMaterial,
    utils::HashMap,
};

use crate::{convert::image_to_texture, Pal, Shp};

#[derive(Debug, TypeUuid, Default)]
#[uuid = "3928340f-dae8-4924-805b-3298c70b9c38"]
pub struct Mod {
    state: ModState,
    pal: Handle<Pal>,
    shp: Handle<Shp>,
    textures: Vec<Handle<Texture>>,
    color_materials: Vec<Handle<ColorMaterial>>,
}

impl Mod {
    fn get_material(index: usize) -> Handle<ColorMaterial> {
        todo!()
    }

    fn new(pal: Handle<Pal>, shp: Handle<Shp>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum ModState {
    Loading,
    Loaded,
}

impl Default for ModState {
    fn default() -> Self {
        ModState::Loading
    }
}

//该系统通过获取pal和shp生成需要的mod
pub fn openra_mod_system(
    pals: ResMut<Assets<Pal>>,
    shps: ResMut<Assets<Shp>>,
    mut mods: ResMut<Assets<Mod>>,
    textures: ResMut<Assets<Texture>>,
    color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut loading_mods = Vec::new();
    for (handle, one_mod) in mods.iter() {
        if one_mod.state == ModState::Loading {
            loading_mods.push(handle);
        }
    }

    if loading_mods.len() == 0 {
        return;
    }

    for handle in loading_mods.iter() {
        mods.get_mut(handle).and_then(|one_mod| {
            pals.get(one_mod.pal).and_then(|pal| {
                shps.get(one_mod.shp).and_then(|shp| {
                    for i in 0..shp.image_count {
                        let texture = shp
                            .get_image(pal, i)
                            .and_then(|image| image_to_texture(image))
                            .map_or_else(Texture::default(), |texture| texture);

                        let handle_texture = textures.add(texture);
                        let hanle_color_materials =
                            color_materials.add(handle_texture.clone_weak());

                        one_mod.textures.push(handle_texture);
                        one_mod.color_materials.push(color_materials);
                    }

                    one_mod.state = ModState::Loaded;
                })
            })
        });
    }
}
