mod loader;

use std::time::Duration;

use bevy::prelude::*;
use loader::AviLoader;

#[derive(Default)]
pub struct AviPlugin;

impl Plugin for AviPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Video>();
        app.register_type::<VideoClip>()
            .init_asset::<VideoClip>()
            .register_asset_reflect::<VideoClip>();
        app.world
            .resource_mut::<Assets<VideoClip>>()
            .insert(Handle::default(), VideoClip::default());
        
        app.preregister_asset_loader::<AviLoader>(&["avi"]);
    }

    fn finish(&self, app: &mut App) {
        app.register_asset_loader(AviLoader);
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Video {
    pub handle: Handle<VideoClip>,
    pub is_playing: bool,
    pub timer: Timer,
    pub current_frame_index: usize,
}

#[derive(Asset, Reflect, Debug, Default, Clone)]
#[reflect_value]
pub struct VideoClip {
    pub frames: Vec<Handle<Image>>,
    pub frame_rate: f32,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            is_playing: true,
            timer: Timer::from_seconds(0.016666, TimerMode::Repeating),
            current_frame_index: 1,
        }
    }
}

impl Video {
    pub fn update(&mut self, delta: Duration) {
        if self.is_playing {
            self.timer.tick(delta);

            if self.timer.just_finished() {
                self.current_frame_index += 1;
            }
        }
    }

    pub fn current_frame(&self, video_clips: &Res<Assets<VideoClip>>) -> Option<Handle<Image>> {
        if let Some(video_clip) = video_clips.get(&self.handle) {
            Some(video_clip.frames[self.current_frame_index].clone())
        } else {
            None
        }
    }
}
