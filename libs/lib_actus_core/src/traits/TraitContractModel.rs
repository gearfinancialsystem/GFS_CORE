use std::rc::Rc;
use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::types::IsoDatetime::IsoDatetime;

pub trait TraitContractModel {
    fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;
    fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
             model: &ContractModel,
             observer: &RiskFactorModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;
    fn init_state_space(model: &ContractModel, 
                        observer: &RiskFactorModel,
                        maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String>;

    // les erreurs doivent etre autre chose que String
    
    //fn get_as(&self, name: &str) -> Option<&dyn Any>;
}

