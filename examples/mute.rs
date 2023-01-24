fn main() {
    let mut audio = windows_audio::Audio::new();
    audio.play("Windows Error");
    std::thread::sleep(std::time::Duration::from_secs(1));
    audio.is_mute = true;
    audio.play("Windows Error");
    std::thread::sleep(std::time::Duration::from_secs(1));
}