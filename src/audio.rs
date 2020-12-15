use rodio::{Sink};
use std::io::{BufReader, Read};
use std::fs::File;
macro_rules! sfx_buf {
    ($src:expr) => {{
        rodio::Decoder::new(::std::io::BufReader::new(::std::io::Cursor::new($src.clone()))).unwrap()
    }}
}

pub enum SFX {
    SHOOT = 1,
    EXPLOSION,
}

pub struct Audio {
    _stream: rodio::OutputStream,
    _handle: rodio::OutputStreamHandle,

    /// Sink for the background music.
    background_sink: rodio::Sink,
    background_file: File,

    sound_sinks: Vec<rodio::Sink>,

    sfx_explosion: Vec<u8>,
    sfx_shoot: Vec<u8>,
}

impl Audio {
    pub fn new() -> Self {
        let (_stream, _handle) = rodio::OutputStream::try_default().unwrap();
        let background_sink = rodio::Sink::try_new(&_handle).unwrap();

        let file = std::fs::File::open("Resources/Birds.ogg").unwrap();
        let background_file = file;

        let mut sfx_file = std::fs::File::open("Resources/explosion.ogg").unwrap();
        let mut sfx_explosion = vec![];
        sfx_file.read_to_end(&mut sfx_explosion).unwrap();

        let mut sfx_file = std::fs::File::open("Resources/shoot.ogg").unwrap();
        let mut sfx_shoot = vec![];
        sfx_file.read_to_end(&mut sfx_shoot).unwrap();


        let mut sound_sinks: Vec<Sink> = Vec::new();
        for _ in 0..10 {
            sound_sinks.push(
                rodio::Sink::try_new(&_handle).unwrap()
            );
        }

        Self {
            _stream,
            _handle,
            background_sink,
            background_file,
            sound_sinks,
            sfx_explosion,
            sfx_shoot
        }
    }

    pub fn play_background_music(&mut self) {
        // todo: this does not like it is very efficient resource management. Reconsider how to cache those.
        self.background_sink.append(
            rodio::Decoder::new(BufReader::new(self.background_file.try_clone().unwrap())).unwrap()
        );

        ::std::io::BufReader::new(self.background_file.try_clone().unwrap());
    }

    pub fn play_sfx(&mut self, choose: SFX) {
        let sink = self.sound_sinks.iter_mut().filter(|s| s.empty()).next();
        if let Some(s) = sink {
            match choose {
                SFX::SHOOT => {
                    s.append(
                        sfx_buf!(self.sfx_shoot)
                    );
                }
                SFX::EXPLOSION => {
                    s.append(
                        sfx_buf!(self.sfx_explosion)
                    );
                }
            }
        }
    }
}
