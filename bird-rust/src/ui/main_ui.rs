use godot::{classes::{Button,IButton}, prelude::*};
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=Button)]
struct StartButton{
    base:Base<Button>
}

#[godot_api]
impl IButton for StartButton {
    fn init(base:Base<Button>)->Self{
        Self{base}
    }

    fn enter_tree(&mut self){
        let callable = Callable::from_object_method(&self.base(), "on_button_pressed");
        self.base_mut().connect("pressed", &callable);
       
        
    }
}

#[godot_api]
impl StartButton{
    #[func]
    fn on_button_pressed(&mut self){
        let mut gm = self.base().get_node_as::<GameManager>("../../../GameManager");
        let args = &[true.to_variant()];
        gm.emit_signal("start_play", args);
        self.base_mut().queue_free();
    }
}
