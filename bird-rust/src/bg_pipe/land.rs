use godot::prelude::*;
use godot::classes::Sprite2D;
use crate::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=Node2D)]
struct Land{
    #[export]
    land_y_position:f32,//地板的y轴位置
    #[export]
    max_speed:f32,//移动速度
    land_move_state:bool,//移动状态
    left_bound:f32,//左侧的边界距离
    land_sprites: Vec<Option<Gd<Sprite2D>>>,//地面的精灵集合
    number_of_land:u8,
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for Land{
    fn init(base:Base<Node2D>)->Self{
        Self{
            land_y_position : -56.0,
            max_speed:-50.0,
            land_move_state:false,
            left_bound:-336.0,
            land_sprites:Vec::new(),
            number_of_land:0,
            base
        }
    }
    fn enter_tree(&mut self){
        let num = self.base().get_child_count();
        for i in 0..num{
            if let Some(child) = self.base().get_child(i){
                self.land_sprites.push(Some(child.cast::<Sprite2D>()));
            }
        }
        self.number_of_land = self.land_sprites.len() as u8;
        //绑定信号
        let mut gm = self.base().get_node_as::<GameManager>("../GameManager");
        let callable = Callable::from_object_method(&self.base(), "set_land_move_state");
        gm.connect("start_play", &callable);
    }
    fn process(&mut self, _delta:f64){
        let mut target_position_x = 0.0;
        for i in 0..self.number_of_land {
            let sprite = self.land_sprites[i as usize].as_mut().unwrap();
            if sprite.get_position().x <= self.left_bound {
                let query_index = match i {
                    0 => 2,
                    1 => 0,
                    2 => 1,
                    _ => unreachable!(),
                };
                // 获取目标精灵的位置
                target_position_x = self.land_sprites[query_index].as_mut().unwrap().get_position().x;
            }
        }
        for sprite in &mut self.land_sprites {
            if let Some(sprite) = sprite {
                if self.land_move_state {
                    if sprite.get_position().x <= self.left_bound {
                        sprite.set_position(Vector2::new(target_position_x - self.left_bound, self.land_y_position));
                    }
                    sprite.move_local_x(self.max_speed * _delta as f32);
                }
            }
        }
    }
}

#[godot_api]
impl Land{
    #[func]
    fn set_land_move_state(&mut self,state:bool){
        self.land_move_state = state;
    }
}
