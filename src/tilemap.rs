use std::{
    fs::File,
    io::{BufRead, BufReader}
};

use bevy::prelude::*;

use crate::{ascii::{AsciiSheet, spawn_ascii_sprite}, TILESIZE};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;


impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map);
    }
}

fn create_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILESIZE, -(y as f32) * TILESIZE, 800.0),
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(Visibility::visible())
        .push_children(&tiles);
}