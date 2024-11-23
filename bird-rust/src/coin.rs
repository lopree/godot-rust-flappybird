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
        let callable = Callable::from_object_method(&self.base(), "onEnter");
        self.base_mut().connect("body_entered", &callable);
    }
}

#[godot_api]
impl Coin{
    #[func]
    fn onEnter(&mut self,item:Gd<Node2D>){
       godot_print!("from impl");
       godot_print!("item:{}",item.get_name());
       self.base_mut().queue_free();
    }
}
