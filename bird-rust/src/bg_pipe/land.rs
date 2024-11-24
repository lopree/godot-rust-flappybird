use godot::prelude::*;
use godot::classes::Sprite2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Land{
    land_sprites: Vec<Option<Gd<Sprite2D>>>,
    base:Base<Node2D>
}

#[godot_api]
impl INode2D for Land{
    fn init(base:Base<Node2D>)->Self{
        Self{
            land_sprites:Vec::new(),
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
    }
    fn process(&mut self, _delta:f64){
        let mut target_position_x = 0.0;
        for i in 0..self.land_sprites.len() {
            let sprite = self.land_sprites[i].as_mut().unwrap();
            if sprite.get_position().x <= -336.0 {
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
                if sprite.get_position().x <= -336.0 {
                    sprite.set_position(Vector2::new(target_position_x + 336.0, -56.0));
                }
                sprite.move_local_x(-50.0 * _delta as f32);
            }
        }
    }

}

#[godot_api]
impl Land{
    
}