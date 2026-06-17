use gtk::prelude::*;

pub struct SoundPlayer {
    correct: gtk::MediaFile,
    wrong: gtk::MediaFile,
    music: gtk::MediaFile,
    settings: gio::Settings,
}

impl Default for SoundPlayer {
    fn default() -> Self {
        let prefix = "/io/github/nacho/mundi/sounds";
        let music = gtk::MediaFile::for_resource(&format!("{prefix}/quiz-music.oga"));
        music.set_loop(true);
        music.set_volume(0.5);
        Self {
            correct: gtk::MediaFile::for_resource(&format!("{prefix}/correct.oga")),
            wrong: gtk::MediaFile::for_resource(&format!("{prefix}/wrong.oga")),
            music,
            settings: gio::Settings::new("io.github.nacho.mundi"),
        }
    }
}

impl SoundPlayer {
    pub fn play_correct(&self) {
        self.play(&self.correct);
    }

    pub fn play_wrong(&self) {
        self.play(&self.wrong);
    }

    pub fn play_music(&self) {
        if self.settings.boolean("sound-effects") {
            self.music.pause();
            self.music.seek(0);
            self.music.play();
        }
    }

    pub fn stop_music(&self) {
        self.music.pause();
    }

    fn play(&self, media: &gtk::MediaFile) {
        if self.settings.boolean("sound-effects") {
            media.pause();
            media.seek(0);
            media.play();
        }
    }
}
