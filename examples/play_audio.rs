fn main() {
    let audio = windows_audio::Audio::new();
    let audios = audio.get_audios();
    audio.play(&audios[0].name);
    std::thread::sleep(std::time::Duration::from_secs(7));
}