use bevy::prelude::*;

#[derive(Resource)]
pub struct MusicController(pub Handle<AudioSink>);
