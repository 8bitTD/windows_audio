# windows_audio
windows audio
```
fn main() {
    let audio = windows_audio::Audio::new();
    for a in audio.get_audios(){
        println!("{:?}", a.name);
    }
    audio.play("Windows Error");
    std::thread::sleep(std::time::Duration::from_secs(1));
}
```
