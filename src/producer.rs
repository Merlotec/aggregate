use crate::funcs;
use crate::stochastic::StochasticAtom;

#[derive(Debug, Copy, Clone)]
pub struct Producer {
    /// Relative potential growth. This is the theoretical optimum market share of this firm at quasi-economic efficiency.
    /// This changes very slowly.
    pub rps: f64,

    /// The current scale of the firm (the value of all its assets).
    /// This may be different to the actual 'market share', however they are closely related.
    pub scale: f64, 

    /// Marginal productivity of captial coefficient (defines growth rate of firm).
    pub mpk_coeff: f64,
}

impl Producer {
    pub fn mpk(&self) -> f64 {
        1.0 - std::f64::consts::E.powf(self.mpk_coeff * self.scale)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProducerNode {
    pub producer: Producer,
    pub growth: StochasticAtom,
    pub market_share: StochasticAtom,
}

impl ProducerNode {
    pub fn tick(&mut self, delta: f64, potential_scale: f64, aggregate_scale: f64) {
        let ps = self.producer.rps / potential_scale;
        let s = self.producer.scale / aggregate_scale;

        let target_delta = ps - s;

        let ds = target_delta.powf(2.0) / ps;

        
    }
}