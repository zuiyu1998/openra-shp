use std::time::Duration;

use bevy::prelude::*;
use openra_shp::{Mod, ModState, OpenRaAssetPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(OpenRaAssetPlugin)
        .add_startup_system(setup.system())
        .add_system(spawn_sprite.system())
        .init_resource::<FrameTimer>()
        .run();
}

pub struct Advpwr {
    m_mod: Handle<Mod>,
}

pub struct FrameTimer {
    timer: Timer,
}

impl Default for FrameTimer {
    fn default() -> Self {
        FrameTimer {
            timer: Timer::new(Duration::from_millis(200), true),
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mod_assets: ResMut<Assets<Mod>>,
) {
    let pal = asset_server.load("a_conyard.pal");
    let shp = asset_server.load("a_conyard.shp");
    let m_mod = mod_assets.add(Mod::new(pal, shp));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle::default())
        .insert(Advpwr { m_mod });
}

fn spawn_sprite(
    mut mod_assets: ResMut<Assets<Mod>>,
    mut sprite_query: Query<(&mut Handle<ColorMaterial>, &Advpwr), (With<Advpwr>, With<Sprite>)>,
    mut frame_timer: ResMut<FrameTimer>,
    time: Res<Time>,
) {
    for (mut handle_color_materia, advpwr) in sprite_query.iter_mut() {
        if let Some(mod_assets) = mod_assets.get_mut(advpwr.m_mod.clone_weak()) {
            if let ModState::Loaded = mod_assets.state {
                if frame_timer.timer.tick(time.delta()).finished() {
                    frame_timer.timer.reset();
                    if mod_assets.current_image.unwrap() < mod_assets.image_count.unwrap() {
                        *handle_color_materia = mod_assets
                            .get_material(mod_assets.current_image.unwrap())
                            .unwrap();
                        if let Some(ref mut current_image) = mod_assets.current_image {
                            *current_image += 1;
                        }
                    } else {
                        if let Some(ref mut current_image) = mod_assets.current_image {
                            *current_image = 0;
                        }
                    }
                }
            }
        }
    }
}
