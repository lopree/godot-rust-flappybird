/// 管理设置面板的UI：
/// 1. 语言选择
/// 2. 保存数据，返回主面板按钮
/// 3. 退出游戏按钮

use godot::prelude::*;
use godot::classes::{Control, IControl, OptionButton, Button};


#[derive(GodotClass)]
#[class(base=Control)]
struct SetUIManager{
    #[export]//根节点
    root_control:Option<Gd<Control>>,
    #[export]
    language_option_button:Option<Gd<OptionButton>>,
    #[export]
    save_button:Option<Gd<Button>>,
    #[export]
    exit_button:Option<Gd<Button>>,
    base:Base<Control>
}

#[godot_api]
impl IControl for SetUIManager{
    fn init(base:Base<Control>)->Self{
        Self{
            root_control:None,
            language_option_button:None,
            save_button:None,
            exit_button:None,
            base
        }
    }

    fn enter_tree(&mut self){
        //下拉菜单，添加语言选项
        let language_callable = Callable::from_object_method(&self.base(), "on_language_option_button_item_selected");
        if let Some(language_option_button) = &mut self.language_option_button{
            language_option_button.add_item("Deutsche");
            language_option_button.connect("item_selected", &language_callable);
            
        }else{
            godot_error!("语言下拉菜单没有正确放置！！！！！！")
        }
        //返回主界面以及保存数据按钮
        let save_callable = Callable::from_object_method(&self.base(), "on_save_button_pressed");
        self.save_button.as_mut().unwrap().connect("pressed", &save_callable);
        
        //退出游戏按钮
        let exit_callable = Callable::from_object_method(&self.base(), "on_exit_button_pressed");
        self.exit_button.as_mut().unwrap().connect("pressed", &exit_callable);
    }
}

#[godot_api]
impl SetUIManager{
    #[func]
    fn on_language_option_button_item_selected(&mut self,index:i32){
        godot_print!("language_option_button_item_selected: {}",index);
    }
    #[func]
    fn on_save_button_pressed(&mut self){
        //先保存数据
        self.root_control.as_mut().unwrap().set_visible(false);
        //返回主界面

    }
}
