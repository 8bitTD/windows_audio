fn main() {
    let audio = windows_audio::Audio::new();
    for a in audio.get_audios(){
        println!("{:?}", a.name);
    }
}