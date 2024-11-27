use godot::prelude::*;
use godot::classes::{Button, IButton, Texture2D};
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=Button)]
struct MusicButton{
    gm:Option<Gd<GameManager>>,
    music_open_texture:Gd<Texture2D>,
    music_close_texture:Gd<Texture2D>,
    start_music_state:bool,
    base:Base<Button>
}

#[godot_api]
impl IButton for MusicButton{
    fn init(base:Base<Button>)->Self{
        let music_open_texture = match try_load("res://assets/sprite/bg/music_fill.png") {
            Ok(texture) => texture,
            Err(_) => {
                godot_print!("cant find resources");
                load("res://icon.svg")
            }
        };
        let music_close_texture = load("res://assets/sprite/bg/music_forbid_fill.png");
        Self{
            gm:None,
            music_open_texture,
            music_close_texture,
            start_music_state:true,
            base
        }
    }

    fn enter_tree(&mut self){
        let callable = Callable::from_object_method(&self.base(), "on_button_pressed");
        self.base_mut().connect("pressed", &callable);
        
        self.gm = self.base().try_get_node_as::<GameManager>("../../GameManager");
        if self.gm.is_none() {
            self.gm = self.base().try_get_node_as::<GameManager>("../../../../GameManager");
        }
    }
}
#[godot_api]
impl MusicButton{
    #[func]
    fn on_button_pressed(&mut self){
        //切换图标
        let music_close_texture = self.music_close_texture.clone();
        let music_open_texture: Gd<Texture2D> = self.music_open_texture.clone();
         
         if self.start_music_state {
            self.base_mut().set_button_icon(&music_close_texture);
        }else{
            self.base_mut().set_button_icon(&music_open_texture);
        }
        //发送信号
        let args = &[self.start_music_state.to_variant()];
        if let Some(gm) = &mut self.gm{
            gm.emit_signal("music_state", args);
        }else{
            godot_error!("cant find gm");
        };
        
        self.start_music_state = !self.start_music_state;


    }
}
