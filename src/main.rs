use cacao::macos::window::Window;
use cacao::macos::{App, AppDelegate};
use cacao::notification_center::Dispatcher;
use cacao::view::View;

pub mod numerical_function;
pub mod display;

use display::interface::Interface;

pub struct BasicApp {
    window: Window,
    graph: View<Interface>,
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::activate();

        self.window.set_content_view(&self.graph);
        self.window.set_content_size(800, 800);
        self.window.set_minimum_content_size(800, 800);
        self.window.set_maximum_content_size(800, 800);
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

impl Dispatcher for BasicApp {
    type Message = String;

    fn on_ui_message(&self, msg: Self::Message) {
        self.graph.delegate.as_ref().unwrap().press_button(&msg);
    }
}

fn main() {
    App::new("", BasicApp {
        window: Window::default(),
        graph: View::with(Interface::new()),
    }).run();
}
