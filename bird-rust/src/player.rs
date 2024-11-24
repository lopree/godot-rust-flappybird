use godot::prelude::*;
use godot::classes::{CharacterBody2D,ICharacterBody2D,AnimatedSprite2D};
use godot::classes::ProjectSettings;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    gravity: f32,
    jump_velocity:f32,
    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base:Base<CharacterBody2D>)->Self{
        Self{
            gravity: ProjectSettings::singleton().get_setting("physics/2d/default_gravity").try_to::<f32>().unwrap_or(90.0),
            jump_velocity:-15.0,
            base
        }
    }
   fn enter_tree(&mut self) {
        let mut child_node = self.base().get_child(0).unwrap().cast::<AnimatedSprite2D>();
        child_node.set_autoplay("fly");//无法在ready中使用，当node被添加进场景的时候，无法触发设置autoplay
   }
   fn ready(&mut self) { 
        let mut child_node = self.base().get_child(0).unwrap().cast::<AnimatedSprite2D>();
        // let animation_name = "fly02";
        // child_node.set_animation(animation_name);
        // child_node.play(); // 开始播放动画
    }
    fn physics_process(&mut self, _delta: f64) {
        let mut velocity = self.base_mut().get_velocity();
    
        if !self.base().is_on_floor(){
            velocity = self.apply_gravity(velocity,_delta);
        }
        if Input::singleton().is_action_just_pressed("ui_accept"){
            velocity = self.jump(velocity);
        }
        let mut base = self.base_mut();
        if let Some(c) = base.move_and_collide(velocity) {
            velocity = velocity.slide(c.get_normal());
        }

        base.move_and_collide(velocity);
       
    }

}

#[godot_api]
impl Player{
    fn apply_gravity(&mut self, mut velocity: Vector2,_delta:f64) -> Vector2 {
        velocity.y += self.gravity / 28.0 * _delta as f32;
        velocity
    }
    fn jump(&mut self,mut velocity:Vector2)->Vector2{
        velocity.y = self.jump_velocity;
        velocity
    }
}
