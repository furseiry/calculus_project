use cacao::button::Button;
use cacao::color::Color;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::macos::App;
use cacao::macos::FocusRingType;
use cacao::text::Font;
use cacao::view::{View, ViewDelegate};

use std::sync::Mutex;

use super::button_panel::ButtonPanel;
use super::text_panel::TextPanel;
use crate::BasicApp;

pub struct Interface {
    inputs: TextPanel,
    buttons: ButtonPanel,
    poly_depth: Mutex<usize>
}

impl Interface {
    pub fn new() -> Self {
        let mut buttons = vec![];
        for name in vec![
            String::from("Add field"),
            String::from("Subtract field"),
            String::from("Process graph")
        ] {
            let mut button = Button::new(&name);
            button.set_bordered(false);
            button.set_focus_ring_type(FocusRingType::None);
            button.set_font(Font::system(16.));
            button.set_text_color(Color::SystemWhite);
            button.set_action(move || {
                App::<BasicApp, String>::dispatch_main(name.clone());
            });
            buttons.push(button);
        }

        Self {
            inputs: TextPanel::new(10),
            buttons: ButtonPanel::new(buttons),
            poly_depth: Mutex::new(0)
        }
    }

    pub fn press_button(&self, msg: &str) {
        let mut depth = self.poly_depth.lock().unwrap();
        match msg {
            "Add field" => {
                if *depth != 10 {
                    *depth += 1;
                }
            }
            "Subtract field" => {
                if *depth != 0 {
                    *depth -= 1;
                }
            }
            "Process graph" => {
                println!("{:#?}", self.inputs.get_polynomial(*depth));
            }
            _ => ()
        }
        self.inputs.set_shown(*depth);
        println!("{}", depth);
    }
}

impl ViewDelegate for Interface {
    const NAME: &'static str = "Interface";

    fn did_load(&mut self, view: View) {
        view.set_background_color(Color::rgb(0xc2, 0xb2, 0x80));
        view.add_subview(&self.buttons.view);
        view.add_subview(&self.inputs.view);
        LayoutConstraint::activate(&[
            self.buttons.view.leading.constraint_equal_to(&view.leading),
            self.buttons.view.trailing.constraint_equal_to(&view.trailing),
            self.buttons.view.top.constraint_equal_to(&view.top).offset(25.),
            self.buttons.view.height.constraint_equal_to_constant(40.),

            self.inputs.view.leading.constraint_equal_to(&view.leading),
            self.inputs.view.trailing.constraint_equal_to(&view.trailing),
            self.inputs.view.bottom.constraint_equal_to(&view.bottom),
            self.inputs.view.height.constraint_equal_to_constant(40.),
        ]);
    }
}
