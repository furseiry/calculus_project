use calculus_project::numerical_function::*;

fn main() {
    let f = Polynomial::new(vec![1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
    println!("{:#?}", f);
    println!("{}", f.value(6.));
    println!("{}", f.derive(6.));
    println!("{}", f.nth_derivative(10).value(1.))
}
