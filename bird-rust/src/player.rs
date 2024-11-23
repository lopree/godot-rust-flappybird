use godot::prelude::*;
use godot::classes::{CharacterBody2D,ICharacterBody2D,AnimatedSprite2D};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("init");
        
        Self { base }
   }
   fn enter_tree(&mut self) {
        godot_print!("enter_tree");
        let mut child_node = self.base().get_child(0).unwrap().cast::<AnimatedSprite2D>();
        child_node.set_autoplay("fly");//无法在ready中使用，当node被添加进场景的时候，无法触发设置autoplay
   }
   fn ready(&mut self) { 
        godot_print!("ready");
        let mut child_node = self.base().get_child(0).unwrap().cast::<AnimatedSprite2D>();
        let animation_name = "fly02";
        child_node.set_animation(animation_name);
        child_node.play(); // 开始播放动画
    }

}