use cacao::color::Color;
use cacao::input::TextField;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::view::View;

use crate::numerical_function::polynomial::Polynomial;

pub struct TextPanel {
    pub view: View,
    inputs: Vec<TextField>
}

impl TextPanel {
    pub fn new(fields: usize) -> Self {
        let view = View::new();
        view.set_background_color(Color::rgb(0xff, 0x00, 0xff));

        let mut inputs = vec![];
        for i in 0..=fields {
            inputs.push(TextField::new());
            inputs[i].set_background_color(Color::rgb(0x00, 0x00, 0x00));
            view.add_subview(&inputs[i])
        }
        LayoutConstraint::activate(&[inputs[0].trailing.constraint_equal_to(&view.leading)]);

        Self { view, inputs }
    }

    pub fn set_shown(&self, num: usize) {
        for i in 1..self.inputs.len() {
            self.inputs[i].remove_from_superview();
        }

        let input_width = 72.;
        let offset = (800. - input_width * num as f64) / (num as f64 + 1.);
        for i in 1..=num {
            self.view.add_subview(&self.inputs[i]);
            LayoutConstraint::activate(&[
                self.inputs[i].center_y.constraint_equal_to(&self.view.center_y),
                self.inputs[i].leading.constraint_equal_to(&self.inputs[i - 1].trailing).offset(offset),
                self.inputs[i].width.constraint_equal_to_constant(input_width)
            ]);
        }
    }

    pub fn get_polynomial(&self, depth: usize) -> Polynomial {
        let mut coefficients = vec![];
        for i in 0..depth {
            coefficients.push(self.inputs[i + 1].get_value().parse().unwrap_or_default());
        }
        Polynomial::new(coefficients)
    }
}
