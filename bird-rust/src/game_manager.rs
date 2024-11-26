use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager{
    score:u16,
    base:Base<Node>
}

#[godot_api]
impl INode for GameManager{
    fn init(base:Base<Node>)->Self{
        Self{
            score:0,
            base
        }    
    }
}
#[godot_api]
impl GameManager {
    pub fn add_score(&mut self){
        self.score += 1;
    }

    pub fn get_score(&self)->u16{
        godot_print!("score:{}",self.score);
        self.score
    }

    #[signal]
    pub fn start_play();
    /// 音乐状态
    #[signal]
    pub fn music_state();
}

