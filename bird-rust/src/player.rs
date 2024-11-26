use godot::prelude::*;
use godot::classes::{CharacterBody2D,ICharacterBody2D,AnimatedSprite2D};
use godot::classes::ProjectSettings;
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    start_play:bool,
    start_animation : bool,
    gravity: f32,
    jump_velocity:f32,
    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base:Base<CharacterBody2D>)->Self{
        Self{
            start_play:false,
            start_animation:true,
            gravity: ProjectSettings::singleton().get_setting("physics/2d/default_gravity").try_to::<f32>().unwrap_or(90.0),
            jump_velocity:-15.0,
            base
        }
    }
   fn enter_tree(&mut self) {
        let mut child_node = self.base().get_child(0).unwrap().cast::<AnimatedSprite2D>();
        child_node.set_autoplay("red");//无法在ready中使用，当node被添加进场景的时候，无法触发设置autoplay
        let mut gm =  self.base().get_node_as::<GameManager>("../GameManager");
        //开始游戏
        let callable = Callable::from_object_method(&self.base_mut(), "on_start_play");
        gm.connect("start_play", &callable);
        //修改皮肤
        let callable = Callable::from_object_method(&self.base_mut(), "change_skin");
        gm.connect("change_skin", &callable);
   }

    fn physics_process(&mut self, _delta: f64) {
        let stop_floor = self.base().is_on_floor_only();
        let mut velocity = self.base_mut().get_velocity();
        if self.start_animation && self.start_play {
            if !stop_floor{
                velocity = self.apply_gravity(velocity,_delta);
                if Input::singleton().is_action_just_pressed("ui_accept"){
                    velocity = self.jump(velocity);
                }
            }else{
                godot_print!("stop_floor");
                self.stop_animation();
            }
            
            let mut base = self.base_mut();
            if let Some(c) = base.move_and_collide(velocity) {
                velocity = velocity.slide(c.get_normal());
            }
            
            base.move_and_collide(velocity);
        }
       
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
    //此功能暂时有问题
    fn stop_animation(&mut self){
        let mut child_node = self.base().get_child(0).unwrap().cast::<AnimatedSprite2D>();
        godot_print!("{:?}",child_node.get_animation());
        child_node.pause();
        self.start_animation = false;
    }

    #[func]
    fn on_start_play(&mut self, fly_state: bool) {
        self.start_play = fly_state;
    }
    #[func]
    fn change_skin(&mut self, skin_index:u8){
       match skin_index{
        0=>{
            self.set_animation("yellow".to_string());
        }
        1=>{
            self.set_animation("blue".to_string());
        }
        2=>{
            self.set_animation("red".to_string());
        }
        _=>{
            godot_print!("skin_index error");
        }
       }
    }

    fn set_animation(&mut self, animation_name:String) { 
        let mut child_node: Gd<AnimatedSprite2D> = self.base().get_child(0).unwrap().cast::<AnimatedSprite2D>();
        child_node.set_animation(&animation_name);
        child_node.play(); // 开始播放动画
       
    }
}
