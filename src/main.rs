use std::{env, io, option::Option, thread, time};

use windows::Media::{
    Core::MediaSource,
    Playback::{MediaPlayer, MediaPlayerAudioCategory},
    SpeechSynthesis::{SpeechSynthesizer, VoiceInformation},
};

pub struct Speaker {
    synth: SpeechSynthesizer,
    player: MediaPlayer,
    rate: f32,
    pitch: f32,
    volume: f32,
    voice: VoiceInformation,
}

impl Speaker {
    fn new() -> Result<Self, io::Error> {
        let synth = SpeechSynthesizer::new()?;
        let player = MediaPlayer::new()?;
        player.SetRealTimePlayback(true)?;
        player.SetAudioCategory(MediaPlayerAudioCategory::Speech)?;

        Ok(Self {
            synth,
            player,
            rate: 1.,
            pitch: 1.,
            volume: 1.,
            voice: SpeechSynthesizer::DefaultVoice()?,
        })
    }

    fn say(&self, text: &str) -> Result<Option<bool>, io::Error> {
        self.synth.Options()?.SetSpeakingRate(self.rate.into())?;
        self.synth.Options()?.SetAudioPitch(self.pitch.into())?;
        self.synth.Options()?.SetAudioVolume(self.volume.into())?;
        self.synth.SetVoice(&self.voice)?;

        let stream = self
            .synth
            .SynthesizeTextToStreamAsync(&text.into())?
            .get()?;

        let content_type = stream.ContentType()?;
        let source = MediaSource::CreateFromStream(&stream, &content_type)?;
        self.player.SetSource(&source)?;

        let _ = self.player.Play()?;

        Ok(Some(true))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut text = String::new();
    if args.len() > 1 {
        text = args[1].clone();
    } else {
        io::stdin().read_line(&mut text).unwrap_or_else(|error| {
            panic!("Error reading stdin {error:?}.");
        });
    }

    if text.is_empty() {
        println!("No text to say.");
        return;
    } else {
        println!("Saying: {text}");
    }

    let speaker = Speaker::new().unwrap_or_else(|error| {
        panic!("Error making speaker {error:?}.");
    });

    match speaker.say(&text) {
        Ok(_) => {}
        Err(error) => {
            panic!("Error saying {text:?} {error:?}");
        }
    }

    let time = time::Duration::from_secs(10);
    thread::sleep(time);
}
