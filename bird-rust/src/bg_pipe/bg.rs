use godot::{classes::Sprite2D, prelude::*};
use godot::classes::Node2D;
use godot::classes::INode2D;
use rand::Rng;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Background{
    #[export]
    inter_time :f32,
    day_night_time:f32,
    day_or_night:bool,
    pipe_sprite:Vec<Option<Gd<Sprite2D>>>,
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for Background {
    fn init(base:Base<Node2D>)->Self{
        Self{
            inter_time:7.0,
            day_night_time:0.0,
            day_or_night:false,
            pipe_sprite:Vec::new(),
            base
        }
    }
    fn enter_tree(&mut self){
        let num = self.base().get_child_count();
        for i in 0..num{
            self.pipe_sprite.push(Some(self.base().get_child(i).unwrap().cast::<Sprite2D>()));
        }
        self.day_night_time = self.inter_time;

        let random_index = rand::thread_rng().gen_range(0..100);
        godot_print!("random_index:{}",random_index);
        if random_index < 70 {
            self.pipe_sprite[1].as_mut().unwrap().set_visible(false);
            self.day_or_night = true;
        }else{
            self.pipe_sprite[0].as_mut().unwrap().set_visible(false);
            self.day_or_night = false;
        }
    }

    fn process(&mut self,_delta:f64){
        self.day_night_time -= _delta as f32;
        if self.day_night_time <= 0.0 {
            self.day_night_time =rand::thread_rng().gen_range(23.0..self.inter_time);
            self.day_or_night = !self.day_or_night;
            self.set_background(self.day_or_night);
        }
    }
}
#[godot_api]
impl Background{
    fn set_background(&mut self,day_or_night:bool){
        if day_or_night {
            self.pipe_sprite[0].as_mut().unwrap().set_visible(true);
            self.pipe_sprite[1].as_mut().unwrap().set_visible(false);
        }else{
            self.pipe_sprite[0].as_mut().unwrap().set_visible(false);
            self.pipe_sprite[1].as_mut().unwrap().set_visible(true);
        }
    }
}
