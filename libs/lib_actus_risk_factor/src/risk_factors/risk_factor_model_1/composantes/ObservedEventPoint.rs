

#[derive(PartialEq, Debug, Clone)]
pub struct ObservedEventPoint {
    time: String,
    typex: String,
    value: f64,
    contract_id: String,
    states: StateSpace,
}

impl ObservedEventPoint {
    pub fn new(time: String,
               typex: String,
               value: f64,
               contract_id: String,
               states: StateSpace) -> Self {
        Self { time, typex, value, contract_id, states }
    }

    pub fn get_contract_id(&self) -> String {
        self.contract_id.clone()
    }

    pub fn set_contract_id(&mut self, contract_id: String) {
        self.contract_id = contract_id;
    }

    pub fn get_states(&self) -> StateSpace {
        self.states.clone()
    }

    pub fn set_states(&mut self, states: StateSpace) {
        self.states = states;
    }

    pub fn get_time(&self) -> String {
        self.time.clone()
    }
    pub fn set_time(&mut self, time: String) {
        self.time = time;
    }

    pub fn get_typex(&self) -> String {
        self.typex.clone()
    }

    pub fn set_typex(&mut self, typex: String) {
        self.typex = typex;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}