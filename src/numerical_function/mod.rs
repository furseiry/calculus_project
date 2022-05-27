pub mod polynomial;

pub trait Function {
    fn value(&self, pos: f64) -> f64;

    fn derive(&self, pos: f64) -> f64 {
        let delta_x = 1e-6; // good small value
        let delta_y = (self.value(pos + delta_x) - self.value(pos - delta_x)).abs() / 2.;
        delta_y / delta_x
    }
}
