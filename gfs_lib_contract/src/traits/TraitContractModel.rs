use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_types::types::Value::Value;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;

pub trait TraitContractModel {

    fn new() -> Self;
    fn init_contract_terms(&mut self, sm: HashMap<String, Value>);
    fn init_risk_factor_external_data(&mut self, risk_factor_external_data: Option<Arc<dyn TraitExternalData>>);
    fn init_risk_factor_external_event(&mut self, risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>);
    fn init_related_contracts(&mut self, sm: HashMap<String, Value>);
    fn init_status_date(&mut self);
    fn init_state_space(&mut self, maturity: &Option<Rc<MaturityDate>>);
    fn init_contract_event_timeline(&mut self, to: Option<PhantomIsoDatetimeW>); // , to: Option<IsoDatetime>// -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;
    fn set_status_date(&mut self, status_date: Option<StatusDate>);
    fn eval_pof_contract_event(&mut self, id_ce: usize);
    fn eval_stf_contract_event(&mut self, id_ce: usize);
    
    fn compute_payoff(&mut self); // eval pof


    fn next_day(&mut self, extract_results: bool) -> Option<Result<Vec<TestResult>, String>>; // eval stf, modifie statespaces

    fn next_event(&mut self, extract_results: bool) -> Option<Result<TestResult, String>>;
    //fn set_result_set(&mut self);

    fn add_event_to_contract_event_timeline(&mut self); // peut sexecuter nimporte quand
    fn reset(&mut self); // set to None at the end

    fn apply_until_date(&mut self, date: Option<PhantomIsoDatetimeW>, extract_results: bool) -> Option<Result<Vec<TestResult>, String>>; // utile pour l'analyse, du début a une date donnée

    // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;
    fn sort_events_timeline(&mut self);
    
}

