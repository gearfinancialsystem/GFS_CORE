use std::collections::HashMap;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::types::IsoDatetime::IsoDatetime;
use crate::types::Value::Value;
use crate::external::RiskFactorModel::RiskFactorModel;




pub trait TraitContractModel {

    fn new() -> Self;

    fn set_contract_terms(&mut self, sm: &HashMap<String, Value>);

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<RiskFactorModel>);

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>);

    fn set_result_vec(&mut self);

    fn schedule(&mut self, to: Option<IsoDatetime>); // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;

    fn apply(&mut self, result_set_toogle: bool); // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) ; // -> Result<StatesSpace, String>

    fn eval_pof_contract_event(&mut self, id_ce: usize);

    fn eval_stf_contract_event(&mut self, id_ce: usize);
    
}

