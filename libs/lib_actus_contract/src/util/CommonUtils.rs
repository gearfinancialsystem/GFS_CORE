
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::events::ContractEvent::TraitContractEvent;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use crate::traits::TraitExternalData::TraitExternalData;

pub struct CommonUtils;


impl CommonUtils {


    pub fn settlementCurrencyFxRate(
        riskFactorModel: &Option<Box<dyn TraitExternalData>>, 
        model: &ContractTerms, 
        time: &PhantomIsoDatetimeW,
        state: &StatesSpace) -> f64 
    {
        let settlement_currency = model.settlement_currency.clone();
        let currency = model.currency.clone();
        
        if settlement_currency.is_none()  || currency == Some(settlement_currency.clone().unwrap().to_currency()) {
            1.0
        }
        else {
            let strings = vec![currency.unwrap(), settlement_currency.clone().unwrap().to_currency()]; // refaire plus proprement pour pas melanger Currency et setllment currency

            let str_slices: Vec<String> = strings.iter().map(|s| s.value()).collect();
            let joined = str_slices.join(" ");
            
            if riskFactorModel.is_none() {
                1.0
                //riskFactorModel.clone().unwrap().state_at(joined, time).expect("expect curr value")
            } else { 
                1.0 // a verifier
            }


            
        }
        
    }
}