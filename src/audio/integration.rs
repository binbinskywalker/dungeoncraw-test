use legion::*;
use crate::audio::manager::{AudioSystem, AUDIO_CACHE};
use std::path::PathBuf;

// 资源定义
#[derive(Clone, Debug)]
pub struct AudioPlayer(pub AudioSystem);

// 音频事件
pub enum AudioEvent {
    PlayMusic {
        path: String,
        volume: f32,
    },
    PlaySfx {
        path: String,
        volume: f32,
    },
    SetVolume(f32),
    Preload(Vec<String>),
}

// 背景音乐组件
#[derive(Clone, Debug, PartialEq)]
pub struct BackgroundMusic {
    pub path: String,
    pub volume: f32,
    pub playing: bool,
}

// 音效组件
#[derive(Clone, Debug, PartialEq)]
pub struct SoundEffect {
    pub path: String,
    pub volume: f32,
    pub play: bool, // 设置为true时播放
}

// 音频预加载系统
#[system]
pub fn audio_preload(
    #[resource] events: &mut EventChannel<AudioEvent>,
) {
    for event in events.iter() {
        if let AudioEvent::Preload(paths) = event {
            let mut cache = AUDIO_CACHE.lock();
            for path in paths {
                if let Err(e) = cache.preload(path) {
                    eprintln!("Failed to preload {}: {}", path, e);
                }
            }
        }
    }
}

// 音乐播放系统
#[system]
pub fn music_system(
    #[resource] audio: &AudioPlayer,
    query: &mut Query<&mut BackgroundMusic>,
) {
    for mut music in query.iter_mut() {
        if music.playing {
            audio.0.play_music(&music.path, music.volume);
            music.playing = false; // 重置播放状态
        }
    }
}

// 音效播放系统
#[system]
pub fn sfx_system(
    #[resource] audio: &AudioPlayer,
    query: &mut Query<&mut SoundEffect>,
) {
    for mut sfx in query.iter_mut() {
        if sfx.play {
            audio.0.play_sfx(&sfx.path, sfx.volume);
            sfx.play = false; // 重置播放状态
        }
    }
}

// 音频事件处理系统
#[system]
pub fn audio_event_system(
    #[resource] audio: &AudioPlayer,
    #[resource] events: &mut EventChannel<AudioEvent>,
) {
    for event in events.iter() {
        match event {
            AudioEvent::PlayMusic { path, volume } => {
                audio.0.play_music(path, *volume);
            }
            AudioEvent::PlaySfx { path, volume } => {
                audio.0.play_sfx(path, *volume);
            }
            AudioEvent::SetVolume(vol) => {
                audio.0.set_master_volume(*vol);
            }
            _ => {} // 预加载事件由其他系统处理
        }
    }
}