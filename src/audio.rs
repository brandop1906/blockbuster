use bevy::prelude::*;
use bevy::audio::Volume;

fn play_background_audio(
    asset_server: Res<AssetServer>, 
    mut commands: Commands,
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("disco.mp3")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.2)),
    ));
}

pub fn play_block_destroyed_audio(
    asset_server: &AssetServer,
    commands: &mut Commands,
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("pottery_clang.wav")),
        PlaybackSettings::ONCE,
    ));
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, play_background_audio);
    }
}