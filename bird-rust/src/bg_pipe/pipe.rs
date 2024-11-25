use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Pipe{
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for Pipe{
    fn init(base:Base<Node2D>)->Self{
        Self{base}
    }
    //初始化三根管道之间的距离，以及位置，以及各个管道中间的间隔

}


