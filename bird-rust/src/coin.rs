use godot::{classes::{Area2D, IArea2D}, prelude::*};
use crate::game_manager::GameManager;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Coin {
    gm :Option<Gd<GameManager>>,
    base: Base<Area2D>
}
#[godot_api]
impl IArea2D for Coin {
    fn init(base:Base<Area2D>)->Self{
        Self{
            gm:None,
            base
        }
    }

    fn enter_tree(&mut self){
        let callable = Callable::from_object_method(&self.base(), "on_collision_enter");
        self.base_mut().connect("body_entered", &callable);
        self.gm = self.base().try_get_node_as::<GameManager>("../GameManager");
        
    }
}

#[godot_api]
impl Coin{
    #[func]
    fn on_collision_enter(&mut self,body:Gd<Node2D>){
       match &mut self.gm{
        Some(gm)=>{
            gm.bind_mut().add_score();
        }
        None=>{
            godot_print!("gm is none");
        }
    
    }
       self.base_mut().queue_free();
    }
}
