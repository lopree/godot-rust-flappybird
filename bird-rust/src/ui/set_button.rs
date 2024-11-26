use godot::{classes::{Button, Control, IButton}, prelude::*};

#[derive(GodotClass)]
#[class(base=Button)]
struct SetButton{
    set_ui:Option<Gd<Control>>,
    base:Base<Button>
}
#[godot_api]
impl IButton for SetButton{
    fn init(base:Base<Button>)->Self{
        Self{
            set_ui:None,
            base
        }
    }

    fn enter_tree(&mut self){
        self.set_ui = self.base().try_get_node_as::<Control>("../../set_ui");
       
        let callable = Callable::from_object_method(&self.base(), "on_button_pressed");
        self.base_mut().connect("pressed", &callable);
    }
}

#[godot_api]
impl SetButton{
    #[func]
    fn on_button_pressed(&mut self){
        if let Some(set_ui) = &mut self.set_ui{
            set_ui.set_visible(true);
        }
    }
}
