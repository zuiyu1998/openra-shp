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
    pub state: ModState,
    pal: Handle<Pal>,
    shp: Handle<Shp>,
    textures: Vec<Handle<Texture>>,
    color_materials: Vec<Handle<ColorMaterial>>,
    pub current_image: Option<usize>,
    pub image_count: Option<usize>,
}

impl Mod {
    pub fn get_material(&self, index: usize) -> Option<Handle<ColorMaterial>> {
        if self.state == ModState::Loading {
            return None;
        } else if index >= self.color_materials.len() {
            println!("hand2");

            return None;
        } else {
            return Some(self.color_materials[index].clone_weak());
        }
    }

    pub fn new(pal: Handle<Pal>, shp: Handle<Shp>) -> Self {
        Mod {
            pal,
            shp,
            ..Default::default()
        }
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
    mut textures: ResMut<Assets<Texture>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
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

    for handle in loading_mods.into_iter() {
        mods.get_mut(handle).and_then(|one_mod| {
            pals.get(one_mod.pal.clone_weak()).and_then(|pal| {
                shps.get(one_mod.shp.clone_weak()).map(|shp| {
                    for i in 0..shp.image_count {
                        let texture: Texture = shp
                            .get_image(pal, i)
                            .and_then(|image| Some(image_to_texture(image)))
                            .map_or_else(|| Texture::default(), |texture| texture);

                        let handle_texture = textures.add(texture);
                        let hanle_color_material =
                            color_materials.add(handle_texture.clone_weak().into());

                        one_mod.textures.push(handle_texture);
                        one_mod.color_materials.push(hanle_color_material);
                    }
                    one_mod.image_count = Some(shp.image_count);
                    one_mod.current_image = Some(0);
                    one_mod.state = ModState::Loaded;
                })
            })
        });
    }
}
