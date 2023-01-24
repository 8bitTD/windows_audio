#[derive(Debug, Clone)]
pub struct AudioInfo{
    pub name: String,
    pub path: String,
}
impl AudioInfo{
    pub fn new(p: std::fs::DirEntry) -> Self{
        let tmp_name = p.file_name().to_str().unwrap().to_string();
        let (name, _) = tmp_name.split_once(".").unwrap();
        let path  = p.path().to_str().unwrap().to_string();
        Self{name: String::from(name), path: path}
    }
}
#[derive(Debug, Clone)]
pub struct Audio{
    audios: Vec<AudioInfo>,
    pub is_mute: bool,
    pub volume: f32,
}
impl Audio{
	/// Sounds a wav file that can be used by default on windowsOS
	/// # Examples
	///
	/// ```
	/// let audio = windows_audio::Audio::new();
	/// let audios = audio.get_audios();
	/// audio.play(&audios[0].name);
	/// std::thread::sleep(std::time::Duration::from_secs(7));
	/// ```
    pub fn new() -> Self{
    	if !std::path::Path::new("C:/Windows/Media/").is_dir(){
    		return Self{ audios: vec![], is_mute: false, volume: 1.0 }
    	}
        let paths = std::fs::read_dir("C:/Windows/Media/").unwrap();
        let mut audios = Vec::new();
        for path in paths {
            let p = path.unwrap();
            if p.file_type().unwrap().is_dir(){continue;}
            let ext = p.path().extension().unwrap().to_str().unwrap().to_string();
            if ext != "wav"{continue;}
            let a = AudioInfo::new(p);
            audios.push(a);
        }
        Self{audios: audios, is_mute: false, volume: 1.0}
    }

    pub fn get_audios(&self) -> Vec<AudioInfo>{
        self.audios.clone()
    }

    pub fn play(&self, name: &str){
    	if self.is_mute {return;}
        let res = self.audios.iter().find(|&x| &x.name == name);
        if res.is_none(){return;}
        let file = std::fs::File::open(&res.unwrap().path).unwrap();
        let vol = self.volume;
        std::thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());
            sink.set_volume(val);
            sink.sleep_until_end();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let audio = Audio::new();
        for a in &audio.audios{
            println!("{:?}",a.path);
            println!("{:?}",a.name);
        }
    }
}
