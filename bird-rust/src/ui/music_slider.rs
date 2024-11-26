use godot::{classes::{ISlider,Slider}, prelude::*};

#[derive(GodotClass)]
#[class(base=Slider)]
struct MusicSlider{
    base:Base<Slider>
}

#[godot_api]
impl ISlider for MusicSlider{
    fn init(base:Base<Slider>)->Self{
        Self{base}
    }
    fn enter_tree(&mut self){
        self.base_mut().set_min(0.0);
        self.base_mut().set_max(1.0);
        self.base_mut().set_value(1.0);
        let callable = Callable::from_object_method(&self.base(), "on_slider_value_changed");
        self.base_mut().connect("value_changed", &callable);
    }
}

#[godot_api]
impl MusicSlider{
    #[func]
    fn on_slider_value_changed(&mut self, value:f32){
        godot_print!("slider_value:{}",value);
    }
}
