use statrs::function::factorial::factorial;
use rand::Rng;

/// We need a way 
/// of keeping track of the 'direction' that something is going.
/// To do this we use an object which contains derivative information about the curve.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Differential<const N: usize> {
    order: [f64; N],
}

impl<const N: usize> Differential<N> {
    pub fn new(base: [f64; N]) -> Self {
        Self {
            order: base,
        }
    }

    /// Sets the nth derivative to x.
    pub fn set_derivative(&mut self, n: usize, x: f64) {
        self.order[n - 1] = x;
    }

    pub fn clear_higher(&mut self) {
        for d in &mut self.order[1..N] {
            *d = 0.0;
        }
    }

    /// Integrates the function, giving the value at x, (assuming constant of integration is 0).
    /// Any required constant can be added on manually.
    pub fn eval(&self, x: f64) -> f64 {
        let mut summand: f64 = 0.0;
        for (i, d) in self.order.iter().enumerate() {
            let n = (i + 1) as i32;
            summand += (1.0 / (factorial(n as u64))) * d * x.powi(n);
        }
        summand
    }

    pub fn integrate_bounded(&self, a: f64, b: f64) -> f64 {
        self.eval(b) - self.eval(a)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StochasticAtom {
    pub differential: Differential<2>,
    volatility: f64,
    tendency: f64,
}

impl StochasticAtom {
    pub fn new(tendency: f64, volatility: f64) -> Self { 
        Self {
            differential: Differential::new([tendency, 0.0]),
            volatility,
            tendency,
        }
    }

    // Changes the randomness values.
    pub fn randomise(&mut self) {
        let r = rand::thread_rng().gen_range(0.0..std::f64::MAX);
        self.differential.set_derivative(2, r);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StochasticValue {
    pub atom: StochasticAtom,
    value: f64,
    t: f64,
}

impl StochasticValue {

    pub fn next(&mut self, dt: f64) -> f64 {
        self.atom.randomise();
        self.t += dt;
        self.atom.differential.eval(self.t)
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}