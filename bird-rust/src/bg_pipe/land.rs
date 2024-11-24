use godot::prelude::*;
use godot::classes::Sprite2D;


#[derive(GodotClass)]
#[class(base=Node2D)]
struct Land{
    #[export]
    max_speed:f32,
    left_bound:f32,
    land_sprites: Vec<Option<Gd<Sprite2D>>>,
    number_of_land:u8,
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for Land{
    fn init(base:Base<Node2D>)->Self{
        Self{
            max_speed:-50.0,
            left_bound:-336.0,
            land_sprites:Vec::new(),
            number_of_land:0,
            base
        }
    }
    fn enter_tree(&mut self){
        godot_print!("Land enter_tree,max_speed:{}",self.max_speed);
        let num = self.base().get_child_count();
        for i in 0..num{
            if let Some(child) = self.base().get_child(i){
                self.land_sprites.push(Some(child.cast::<Sprite2D>()));
            }
        }
        self.number_of_land = self.land_sprites.len() as u8;
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
                if sprite.get_position().x <= self.left_bound {
                    sprite.set_position(Vector2::new(target_position_x - self.left_bound, -56.0));
                }
                sprite.move_local_x(self.max_speed * _delta as f32);
            }
        }
    }

}