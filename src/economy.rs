use crate::funcs;
use crate::producer::ProducerNode;
use crate::stochastic::StochasticValue;
use std::time::SystemTime;

pub type Absolute = f64;
pub type Relative = f64;

#[derive(Debug, Clone)]
pub struct Economy {
    pub state: EconomyState,
    pub producers: Vec<ProducerNode>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EconomyState {
    // Resources
    pub world: World,
    pub actors: ActorGroup,

    pub velocity: StochasticValue,
    pub aggregate: StochasticValue,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct World {
    pub land: Absolute,
    pub capital: Absolute,
    
    pub depreciation: f64,
}

/// Comprises the human element of an economy.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ActorGroup {
    pub labour: Absolute,
    pub enterprise: Absolute,

    /// Marginal propencity to consume.
    pub mpc: f64,
}

#[derive(Debug, Clone)]
pub struct DataLog {
    pub snapshots: Vec<Snapshot>
}

/// An point in time in the economy.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Snapshot {
    pub time: f64,
    /// This can be considered an approximate measure of gdp
    pub aggregate: f64,
    pub firm_count: usize,
    pub state: EconomyState,
}

impl EconomyState {
    pub fn capacity(&self) -> Absolute {
        funcs::capacity(self.world.land, self.world.capital, self.actors.labour, self.actors.enterprise)
    }
}

impl Economy {
    pub fn tick(&mut self, delta: f64) {
        let da = self.state.aggregate.next(delta);
        let dc = funcs::delta_capital(self.capital(), self.mpc(), self.depreciation(), da);
        self.state.world.capital += dc;
        
        // Sum all the rps of every firm together.
        let potential_scale: f64 = self.producers.iter().map(|p| p.producer.rps).sum();
        let aggregate_scale: f64 = self.producers.iter().map(|p| p.producer.scale).sum();
        
        for p in self.producers.iter_mut() {
            let ps = p.producer.rps / potential_scale;
            let s = p.producer.scale / aggregate_scale;

            let target_delta = ps - s;

            let ds = target_delta.powf(2.0) / ps;



        }   

    }

    pub fn capital(&self) -> Absolute {
        self.state.world.capital
    }

    pub fn land(&self) -> Absolute {
        self.state.world.land
    }

    pub fn labour(&self) -> Absolute {
        self.state.actors.labour
    }

    pub fn enterprise(&self) -> Absolute {
        self.state.actors.enterprise
    }

    pub fn mpc(&self) -> f64 {
        self.state.actors.mpc
    }

    pub fn depreciation(&self) -> f64 {
        self.state.world.depreciation
    }
}

