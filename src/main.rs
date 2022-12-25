// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{log::LogPlugin, prelude::*, window::WindowId, winit::WinitWindows, DefaultPlugins};
use bevy_rubiks::RubiksPlugin;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1280.,
                        height: 720.,
                        title: "Bevy Rubik's cube".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(LogPlugin {
                    #[cfg(debug_assertions)]
                    level: bevy::log::Level::DEBUG,
                    #[cfg(debug_assertions)]
                    filter: "wgpu=warn,bevy_ecs=info,naga=warn".to_string(),
                    ..default()
                }),
        )
        .add_plugin(RubiksPlugin)
        .add_startup_system(set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
