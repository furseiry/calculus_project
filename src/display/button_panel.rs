use cacao::button::Button;
use cacao::color::Color;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::View;

pub struct ButtonPanel {
    pub view: View,
    _buttons: Vec<Button>
}

impl ButtonPanel {
    pub fn new(mut buttons: Vec<Button>) -> Self {
        let view = View::new();
        view.set_background_color(Color::rgb(0x44, 0x44, 0x44));

        let button_width = 800. / buttons.len() as f64;

        let aligner = Button::new("");
        aligner.set_hidden(true);
        view.add_subview(&aligner);
        LayoutConstraint::activate(&[aligner.trailing.constraint_equal_to(&view.leading)]);

        buttons.insert(0, aligner);
        for i in 1..buttons.len() {
            view.add_subview(&buttons[i]);
            LayoutConstraint::activate(&[
                buttons[i].top.constraint_equal_to(&view.top),
                buttons[i].bottom.constraint_equal_to(&view.bottom),
                buttons[i].leading.constraint_equal_to(&buttons[i - 1].trailing),
                buttons[i].width.constraint_equal_to_constant(button_width),
            ]);
        }

        let _buttons = buttons;
        Self { view, _buttons }
    }
}
