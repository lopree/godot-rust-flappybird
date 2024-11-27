use godot::{classes::{Button, Control, IButton}, prelude::*};
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=Button)]
struct StartButton{
    #[export]
    play_ui:Option<Gd<Control>>,
    base:Base<Button>
}

#[godot_api]
impl IButton for StartButton {
    fn init(base:Base<Button>)->Self{
        Self{
            play_ui:None,
            base
        }
    }

    fn enter_tree(&mut self){
        self.play_ui = self.base().try_get_node_as("../../../play_ui");
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
        //隐藏mainUI
        let _ = self.base().get_node_as::<Control>("../../../main_ui").hide();
        
        //显示游玩的时候界面
        if let Some(play_ui) = &mut self.play_ui{
            play_ui.set_visible(true);
        }else{
            godot_error!("没有找到play_ui！！！！！！");
        }
    }
}
