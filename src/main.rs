use cacao::macos::{App, AppDelegate};
use cacao::macos::window::Window;
use calculus_project::numerical_function::polynomial;

#[derive(Default)]
struct BasicApp {
    window: Window
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::activate();

        self.window.set_content_size(400., 400.);
        self.window.set_title("Hello World!");
        self.window.show();
    }
}

fn main() {
    let p = polynomial::Polynomial::default();

    App::new("com.hello.world", BasicApp::default()).run();
}
