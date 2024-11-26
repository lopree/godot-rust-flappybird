use godot::prelude::*;
use godot::classes::AudioStream;
use crate::game_manager::GameManager;
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
        let music_stream = self.music_stream.clone();
        self.base_mut().set_stream(&music_stream);
        self.base_mut().play();
        //find game manager
        let mut gm = self.base().get_node_as::<GameManager>("../GameManager");
        let callable = Callable::from_object_method(&self.base(), "change_music_state");
        gm.connect("music_state", &callable);
        
    }
}

#[godot_api]
impl Music{
    #[func]
    fn change_music_state(&mut self,state:bool){
       self.base_mut().set_stream_paused(state);
    }
}
