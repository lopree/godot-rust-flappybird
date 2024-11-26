use godot::prelude::*;
use godot::classes::Button;
use crate::ui::music::Music;
#[derive(GodotClass)]
#[class(base=Button)]
struct MusicButton{
    base:Base<Button>
}

#[godot_api]
impl IButton for MusicButton{
    fn init(base:Base<Button>)->Self{
        Self{base}
    }

    fn enter_tree(&mut self){
        let callable = Callable::from_object_method(&self.base(), "on_button_pressed");
        self.base_mut().connect("pressed", &callable);
    }
}
#[godot_api]
impl MusicButton{
    #[func]
    fn on_button_pressed(&mut self){
        let mut gm = self.base().get_node_as::<GameManager>("../../../GameManager");
    }
}
