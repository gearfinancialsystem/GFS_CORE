use std::collections::HashMap;
use std::rc::Rc;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_types::types::Value::Value;
use crate::events::ContractEvent::TraitContractEvent;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

pub trait TraitContractModel<CE>
where
    CE: TraitContractEvent
{

    fn new() -> Self;

    fn set_contract_terms(&mut self, sm: &HashMap<String, Value>);

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<impl TraitRiskFactorModel<CE>>);

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>);

    fn set_result_vec(&mut self, result_set_toogle: bool);
    
    fn set_init_state_space(&mut self);

    fn schedule(&mut self, to: Option<IsoDatetime>); // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;

    fn apply(&mut self, stop_states_space_date: Option<IsoDatetime>); // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;

    fn next(&mut self);

    fn sort_events(&mut self);

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) ; // -> Result<StatesSpace, String>

    fn eval_pof_contract_event(&mut self, id_ce: usize);

    fn eval_stf_contract_event(&mut self, id_ce: usize);
    
}

