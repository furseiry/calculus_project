use super::Function;

#[derive(Default)]
pub struct Polynomial {
    coefficients: Vec<f64>,
    derivative: Option<Box<Polynomial>>,
}

impl Function for Polynomial {
    fn value(&self, pos: f64) -> f64 {
        self.coefficients.iter().enumerate().fold(0., |acc, (i, e)| {
            acc + *e * pos.powf((self.coefficients.len() - i - 1) as f64)
        })
    }

    fn derive(&self, pos: f64) -> f64 {
        self.nth_derivative(1).value(pos)
    }
}

impl Polynomial {
    pub fn new(mut coefficients: Vec<f64>) -> Self {
        let mut found_nonzero = false;
        coefficients.retain(|e| {
            if *e != 0. {
                found_nonzero = true;
            }
            found_nonzero
        });
        let mut temp_poly = Self {
            coefficients,
            derivative: None,
        };
        temp_poly.take_derivative(temp_poly.coefficients.len());
        temp_poly
    }

    fn take_derivative(&mut self, depth: usize) -> () {
        if depth == 0 {return}

        let mut derivative = self.coefficients.clone();
        derivative.pop();

        self.derivative = Some(Box::new(Self { 
            coefficients: derivative.iter().enumerate().map(|(i, e)| {
                *e * (derivative.len() - i) as f64
            }).collect(),
            derivative: None,
        }));

        self.derivative.as_mut().unwrap().take_derivative(depth - 1);
    }

    pub fn nth_derivative(&self, mut depth: usize) -> &Self {
        depth = depth.min(self.coefficients.len());
        if depth == 0 {return self}

        let mut derivative = &self.derivative;
        for _ in 1..depth {
            derivative = &derivative.as_ref().unwrap().derivative;
        };

        derivative.as_ref().unwrap()
    }
}
