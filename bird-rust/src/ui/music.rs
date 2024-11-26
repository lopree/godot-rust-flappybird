use godot::global::linear_to_db;
use godot::prelude::*;
use godot::classes::{AudioServer, AudioStream};
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=AudioStreamPlayer)]
struct Music{
    audio_server:Gd<AudioServer>,
    music_bus_index:i32,
    sfx_bus_index:i32,
    music_stream:Gd<AudioStream>,
    base:Base<AudioStreamPlayer>
}
#[godot_api]
impl IAudioStreamPlayer for Music{
    fn init(base:Base<AudioStreamPlayer>)->Self{
        let music_stream:Gd<AudioStream> = load("res://assets/audio/music/Rolemusic - The White Frame.mp3");
        Self{
            audio_server:AudioServer::singleton(),
            music_bus_index:0,
            sfx_bus_index:0,
            music_stream,
            base
        }
    }
    fn enter_tree(&mut self){
        self.sfx_bus_index = AudioServer::get_bus_index(&self.audio_server,"sfx");
        self.music_bus_index = AudioServer::get_bus_index(&self.audio_server,"Music");
        let music_stream = self.music_stream.clone();
        self.base_mut().set_stream(&music_stream);
        self.base_mut().play();
        //find game manager
        let mut gm = self.base().get_node_as::<GameManager>("../GameManager");
        let callable = Callable::from_object_method(&self.base(), "change_music_state");
        let music_callable = Callable::from_object_method(&self.base(), "change_music_volume");
        let sfx_callable = Callable::from_object_method(&self.base(), "change_sfx_volume");
        gm.connect("music_state", &callable);
        gm.connect("music_volume", &music_callable);
        gm.connect("sfx_volume", &sfx_callable);

        
    }
}

#[godot_api]
impl Music{
    #[func]
    fn change_music_state(&mut self,state:bool){
       self.base_mut().set_stream_paused(state);
    }

    #[func]
    fn change_music_volume(&mut self,volume:f32){
        self.audio_server.set_bus_volume_db(self.music_bus_index,linear_to_db(volume as f64) as f32);
        self.audio_server.set_bus_mute(self.music_bus_index, volume < 0.05);
    }

    #[func]
    fn change_sfx_volume(&mut self,volume:f32){
        self.audio_server.set_bus_volume_db(self.sfx_bus_index,linear_to_db(volume as f64) as f32);
    }
}
