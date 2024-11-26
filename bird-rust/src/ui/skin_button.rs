//修改小鸟的皮肤=>直接修改动画
use godot::{classes::{Button, IButton}, prelude::*};
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=Button)]
struct SkinButton{
    #[export]
    skin_index:u8,
    gm:Option<Gd<GameManager>>,
    base:Base<Button>
}

#[godot_api]
impl IButton for SkinButton{
    fn init(base:Base<Button>)->Self{
        Self{
            skin_index:0,
            gm:None,
            base
        }
    }

    fn enter_tree(&mut self){
        //获取game_manager
        self.gm = self.base().try_get_node_as::<GameManager>("../../../GameManager");
        let callable = Callable::from_object_method(&self.base(), "change_skin");
        self.base_mut().connect("pressed", &callable);
    }
}
#[godot_api]
impl SkinButton{
    #[func]
    fn change_skin(&mut self){
        if let Some(gm) = &mut self.gm{
            godot_print!("change_skin:{}",self.skin_index);
            gm.emit_signal("change_skin", &[self.skin_index.to_variant()]);
        }
    }
}

