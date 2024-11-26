use godot::{classes::{Control, IControl, Slider}, prelude::*};
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=Control)]
struct SliderManager{
    #[export]
    music_slider:Option<Gd<Slider>>,
    #[export]
    sfx_slider:Option<Gd<Slider>>,
    game_manager:Option<Gd<GameManager>>,
    base:Base<Control>
}

#[godot_api]
impl IControl for SliderManager{
    fn init(base:Base<Control>)->Self{
        Self{
            music_slider:None,
            sfx_slider:None,
            game_manager:None,
            base
        }
    }
    fn enter_tree(&mut self){
        self.game_manager = self.base().try_get_node_as::<GameManager>("../../GameManager");
        
        let music_callable = Callable::from_object_method(&self.base(), "on_music_slider_value_changed");
        let sfx_callable = Callable::from_object_method(&self.base(), "on_sfx_slider_value_changed");
        
        if let Some(music_slider) = &mut self.music_slider {
            music_slider.connect("value_changed", &music_callable);
        }
        if let Some(sfx_slider) = &mut self.sfx_slider {
            sfx_slider.connect("value_changed", &sfx_callable);
        }
    }
}

#[godot_api]
impl SliderManager{
    #[func]
    fn on_music_slider_value_changed(&mut self, value:f32){
        if let Some(game_manager) = &mut self.game_manager{
            game_manager.emit_signal("music_volume", &[value.to_variant()]);
            
        }else{
            godot_print!("game_manager is none");
        }
    }
    #[func]
    fn on_sfx_slider_value_changed(&mut self, value:f32){
        if let Some(game_manager) = &mut self.game_manager{
            game_manager.emit_signal("sfx_volume", &[value.to_variant()]);
        }else{
            godot_print!("game_manager is none");
        }
    }
}
