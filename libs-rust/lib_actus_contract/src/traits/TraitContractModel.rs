use std::collections::HashMap;
use std::rc::Rc;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;

use lib_actus_types::types::Value::Value;
use crate::events::ContractEvent::TraitContractEvent;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;

pub trait TraitContractModel {

    fn new() -> Self;
    fn init_contract_terms(&mut self, sm: &HashMap<String, Value>);
    fn init_risk_factor_external_data(&mut self, risk_factor_external_data: Option<Box<dyn TraitExternalData>>);
    fn init_risk_factor_external_event(&mut self, risk_factor_external_event: Option<Box<dyn TraitExternalEvent>>);
    fn init_related_contracts(&mut self, sm: &HashMap<String, Value>);
    
    fn init_status_date(&mut self);

    fn init_state_space(&mut self, maturity: &Option<Rc<MaturityDate>>);


    fn init_contract_event_timeline(&mut self); // , to: Option<IsoDatetime>// -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;


    fn set_status_date(&mut self, status_date: Option<StatusDate>);
    
    fn eval_pof_contract_event(&mut self, id_ce: usize);
    fn eval_stf_contract_event(&mut self, id_ce: usize);
    
    fn compute_payoff(&mut self); // eval pof
    fn next(&mut self); // eval stf, modifie statespaces
    //fn set_result_set(&mut self);

    fn add_event_to_contract_event_timeline(&mut self); // peut sexecuter nimporte quand
    fn reset(&mut self); // set to None at the end

    fn apply_until_date(&mut self, date: Option<PhantomIsoDatetimeW>); // utile pour l'analyse, du début a une date donnée

    // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;
    fn sort_events_timeline(&mut self);
    
}

