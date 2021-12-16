use std::{time::Duration, collections::HashMap};

use ggez::audio as gaudio;
use ggez::audio::SoundSource;

pub type SoundData = gaudio::SoundData;
pub type PlayableSound = gaudio::Source;
pub type SoundHandler = usize;

#[derive(Clone)]
pub struct SoundPlayFlags {
    fadein_mills: u64,
    pitch: f32,
    repeat: bool,
    volume: f32,
}

impl SoundPlayFlags {
    pub fn new(
	fadein_mills: u64,
	pitch: f32,
	repeat: bool,
	volume: f32,
    ) -> SoundPlayFlags {
	SoundPlayFlags {
	    fadein_mills: fadein_mills,
	    pitch: pitch,
	    repeat: repeat,
	    volume: volume,
	}
    }
}

impl Default for SoundPlayFlags {
    fn default() -> Self {
	SoundPlayFlags {
	    fadein_mills: 50,
	    pitch: 1.0,
	    repeat: false,
	    volume: 1.0,
	}
    }
}

pub struct SoundManager {
    playing_map: HashMap<SoundHandler, PlayableSound>,
    next_sound_handler: SoundHandler,
}

impl SoundManager {
    pub fn new() -> Self {
	SoundManager {
	    playing_map: HashMap::new(),
	    next_sound_handler: 0,
	}
    }

    pub fn play(
	&mut self,
	ctx: &mut ggez::Context,
	sound_data: SoundData,
	flags: Option<SoundPlayFlags>,
    ) -> SoundHandler {
	let mut sound = PlayableSound::from_data(ctx, sound_data).unwrap();

	if let Some(flags) = flags {
	    sound.set_fade_in(Duration::from_millis(flags.fadein_mills));
	    sound.set_pitch(flags.pitch);
	    sound.set_repeat(flags.repeat);
	    sound.set_volume(flags.volume);
	}
	
	let handler = self.issue_sound_handler();
	sound.play_later().unwrap();
	self.playing_map.insert(handler, sound);
	handler
    }

    fn issue_sound_handler(&mut self) -> SoundHandler {
	let ret = self.next_sound_handler;
	self.next_sound_handler += 1;
	ret
    }

    pub fn ref_sound(&self, handler: SoundHandler) -> &PlayableSound {
	self.playing_map.get(&handler).unwrap()
    }

    pub fn ref_sound_mut(&mut self, handler: SoundHandler) -> &mut PlayableSound {
	self.playing_map.get_mut(&handler).unwrap()
    }

    pub fn change_global_volume(&mut self, volume: f32) {
	for (_, bgm) in self.playing_map.iter_mut() {
	    bgm.set_volume(volume);
	}
    }

    pub fn stop(&mut self, ctx: &mut ggez::Context, handler: SoundHandler) {
	if let Some(sound) = self.playing_map.get_mut(&handler) {
	    sound.stop(ctx).expect("failed to stop sound");
	}
    }

    pub fn remove(&mut self, handler: SoundHandler) {
	self.playing_map.remove(&handler);
    }

    pub fn release_not_playing(&mut self) {
	let mut vec = Vec::new();
	for (handler, sound) in self.playing_map.iter() {
	    if sound.playing() {
		vec.push(*handler);
	    }
	}

	for handler in vec {
	    self.playing_map.remove(&handler);
	}
    }
}
