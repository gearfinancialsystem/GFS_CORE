use std::rc::Rc;
use crate::attributes::ContractModel::ContractModel;
use crate::events::ContractEvent::ContractEvent;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::types::IsoDatetime::IsoDatetime;
use crate::util_tests::essai_data_observer::DataObserver;

pub trait TraitContractModel {
    fn schedule(to: Option<IsoDatetime>, model: &ContractModel) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;
    
    fn apply(events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
             model: &ContractModel,
             observer: &DataObserver) -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>;
    fn init_state_space(model: &ContractModel,
                        observer: &DataObserver, // mettre au pluriel pour quon puisse contruire plusieur different dataobserver
                        maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String>;

    // les erreurs doivent etre autre chose que String
    
    //fn get_as(&self, name: &str) -> Option<&dyn Any>;
}

