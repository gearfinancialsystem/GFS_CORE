
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::traits::TraitExternalData::TraitExternalData;

pub struct CommonUtils;


impl CommonUtils {


    pub fn settlementCurrencyFxRate(
        risk_factor_model: &Option<Box<dyn TraitExternalData>>,
        model: &ContractTerms, 
        time: &PhantomIsoDatetimeW,
        _state: &StatesSpace) -> f64 
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

            let a = risk_factor_model.as_ref().expect("should exist").as_ref().state_at(joined, time);

            a.unwrap()

        }
        
    }
}