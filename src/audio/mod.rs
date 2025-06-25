use radio::{context::AudioContext, Sound, PlaybackSettings};
use std::collections::HashMap;
use std::path::PathBuf;
use parking_lot::Mutex;
use lazy_static::lazy_static;
mod integration;


lazy_static! {
    pub static ref AUDIO_CACHE: Mutex<AudioCache> = Mutex::new(AudioCache::new());
}

pub struct AudioCache {
    sounds: HashMap<String, Sound>,
}

impl AudioCache {
    pub fn new() -> Self {
        Self {
            sounds: HashMap::new(),
        }
    }

    // 预加载音频文件
    pub fn preload(&mut self, path: &str) -> radio::Result<()> {
        if self.sounds.contains_key(path) {
            return Ok(());
        }

        let sound = Sound::from_file(path)?;
        self.sounds.insert(path.to_string(), sound);
        Ok(())
    }

    // 获取音频引用
    pub fn get(&self, path: &str) -> Option<&Sound> {
        self.sounds.get(path)
    }

    // 清除未使用资源
    pub fn cleanup(&mut self) {
        self.sounds.retain(|_, sound| sound.instance_count() > 0);
    }
}

pub struct AudioSystem {
    manager: AudioManager,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            manager: AudioManager::new(),
        }
    }

    // 播放音乐（循环）
    pub fn play_music(&self, path: &str, volume: f32) {
        let cache = AUDIO_CACHE.lock();
        if let Some(sound) = cache.get(path) {
            let settings = PlaybackSettings::new()
                .loop_forever()
                .volume(volume);
            
            self.manager.play(sound, settings);
        }
    }

    // 播放音效
    pub fn play_sfx(&self, path: &str, volume: f32) {
        let cache = AUDIO_CACHE.lock();
        if let Some(sound) = cache.get(path) {
            let settings = PlaybackSettings::new()
                .volume(volume);
            
            self.manager.play(sound, settings);
        }
    }

    // 设置主音量
    pub fn set_master_volume(&self, volume: f32) {
        self.manager.set_volume(volume);
    }

    // 更新音频系统（每帧调用）
    pub fn update(&self) {
        self.manager.update();
    }
}