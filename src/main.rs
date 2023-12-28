use bevy::prelude::*;
use clap::{Parser, Subcommand};

mod map;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Default, Subcommand)]
enum Command {
    #[default]
    Launch,
    Update,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command.unwrap_or_default() {
        Command::Launch => App::new()
            .add_plugins((
                DefaultPlugins,
                #[cfg(feature = "editor")]
                bevy_editor_pls::EditorPlugin::default(),
            ))
            .insert_resource(map::MapConfig {
                path: "levels/test_movement.glb".into(),
                map_scene_root: None,
            })
            .add_systems(Update, map::map_init)
            .run(),
        Command::Update => {
            let status = self_update::backends::github::Update::configure()
                .repo_owner("washed-up-devs")
                .repo_name(env!("CARGO_PKG_NAME"))
                .bin_name("github")
                .show_download_progress(true)
                .current_version(self_update::cargo_crate_version!())
                .build()?
                .update()?;

            eprintln!("Update status: `{}`!", status.version());
        }
    }

    Ok(())
}
