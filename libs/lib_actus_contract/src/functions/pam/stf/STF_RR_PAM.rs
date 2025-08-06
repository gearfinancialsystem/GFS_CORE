use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;

use lib_actus_types::types::IsoDatetime::IsoDatetime;

use crate::external::RiskFactorModel::RiskFactorModel;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct STF_RR_PAM;

impl TraitStateTransitionFunction for STF_RR_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let rate_multiplier = contract_terms.rate_multiplier.as_ref().expect("rate_multiplier should be some");
        let rate_spread = contract_terms.rate_spread.as_ref().expect("rate_spread should be some");
        let status_date = states.status_date.as_ref().expect("status date should be some");
        let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be some");
        let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be some");
        let period_floor = contract_terms.period_floor.as_ref().expect("period floor should be some");
        let period_cap = contract_terms.period_cap.as_ref().expect("period cap should be some");
        let life_floor = contract_terms.life_floor.as_ref().expect("lifeFloor should be some");
        let life_cap = contract_terms.life_cap.as_ref().expect("lifeCap should be some");

        let mut cbv = None;
        if let Some(rfm) = risk_factor_model {
            cbv = rfm.state_at(
                contract_terms.market_object_code_of_rate_reset.clone().unwrap().value(),
                time,
                states,
                contract_terms,
                true
            );
        } else {
            cbv = None
        }

        let mut rate = cbv.unwrap() * rate_multiplier.value() + rate_spread.value();
        let mut delta_rate = rate - nominal_interest_rate.value();

        delta_rate = delta_rate.max(period_floor.value()).min(period_cap.value());
        rate = nominal_interest_rate.value() + delta_rate;
        rate = rate.max(life_floor.value()).min(life_cap.value());

        let time_from_last_event = day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()),
                                                                  time_adjuster.shift_sc(time));
        
        states.accrued_interest.add_assign(nominal_interest_rate.value() * notional_principal.value() * time_from_last_event);
        states.nominal_interest_rate = NominalInterestRate::new(rate).ok(); //Some(rate);

        states.status_date = Some(StatusDate::from(*time));


    }
}
