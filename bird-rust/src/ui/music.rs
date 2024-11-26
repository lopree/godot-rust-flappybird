use godot::prelude::*;
use godot::classes::AudioStream;
#[derive(GodotClass)]
#[class(base=AudioStreamPlayer)]
struct Music{
    music_stream:Gd<AudioStream>,
    base:Base<AudioStreamPlayer>
}
#[godot_api]
impl IAudioStreamPlayer for Music{
    fn init(base:Base<AudioStreamPlayer>)->Self{
        let music_stream:Gd<AudioStream> = load("res://assets/audio/music/Rolemusic - The White Frame.mp3");
        Self{
            music_stream,
            base
        }
    }
    fn enter_tree(&mut self){
        //self.base_mut().set_autoplay(true);
        let music_stream = self.music_stream.clone();
        self.base_mut().set_stream(&music_stream);
        self.base_mut().play();
    }
}

#[godot_api]
impl Music{
    fn play_music(&mut self){
        self.base_mut().play();
    }
    fn stop_music(&mut self){
        self.base_mut().stop();
    }
}
