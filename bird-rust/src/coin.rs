use godot::{classes::{Area2D, IArea2D}, prelude::*};
#[derive(GodotClass)]
#[class(base=Area2D)]
struct Coin {
    base: Base<Area2D>
}
#[godot_api]
impl IArea2D for Coin {
    fn init(base:Base<Area2D>)->Self{
        Self{base}
    }

    fn enter_tree(&mut self){
        godot_print!("coin ready");
        let callable = Callable::from_object_method(&self.base(), "on_collision_enter");
        self.base_mut().connect("body_entered", &callable);
    }
}

#[godot_api]
impl Coin{
    #[func]
    fn on_collision_enter(&mut self,body:Gd<Node2D>){
       godot_print!("body:{}",item.get_name());
       self.base_mut().queue_free();
    }
}
