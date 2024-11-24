use godot::{classes::Sprite2D, prelude::*};
use godot::classes::Node2D;
use godot::classes::INode2D;
use rand::Rng;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Pipe{
    pipe_sprite:Vec<Option<Gd<Sprite2D>>>,
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for Pipe {
    fn init(base:Base<Node2D>)->Self{
        Self{
            pipe_sprite:Vec::new(),
            base
        }
    }
    fn enter_tree(&mut self){
        let num = self.base().get_child_count();
        for i in 0..num{
            self.pipe_sprite.push(Some(self.base().get_child(i).unwrap().cast::<Sprite2D>()));
        }

        let random_index = rand::thread_rng().gen_range(0..100);
        godot_print!("random_index:{}",random_index);
        if random_index < 70 {
            self.pipe_sprite[1].as_mut().unwrap().set_visible(false);
        }else{
            self.pipe_sprite[0].as_mut().unwrap().set_visible(false);
        }
    }
}