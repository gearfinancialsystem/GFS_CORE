use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;

use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitOptionExt::TraitOptionExt;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractReference::ContractReference;
#[allow(non_camel_case_types)]
pub struct STF_RR_SWPPV;

impl TraitStateTransitionFunction for STF_RR_SWPPV {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        );

        let model_nominal_interest_rate = contract_terms.nominal_interest_rate.clone().itself_or(0.0);
        let delivery_settlement = contract_terms.delivery_settlement.clone().expect("deliverySettlement should always be Some");

        let interest_rate = match delivery_settlement {
            DeliverySettlement::D(D) => model_nominal_interest_rate,
            _ => NominalInterestRate::new(model_nominal_interest_rate.value() - nominal_interest_rate.value()).expect(""),
        };

        states.accrued_interest = states.accrued_interest.clone().map(|mut accrued_interest| {
            accrued_interest += interest_rate.value() * notional_principal.value() * time_from_last_event;
            accrued_interest
        });

        states.accrued_interest2 = states.accrued_interest2.clone().map(|mut accrued_interest2| {
            accrued_interest2 += (-1.0) * nominal_interest_rate.value() * notional_principal.value() * time_from_last_event;
            accrued_interest2
        });

        // Placeholder for risk factor calculation
        //let market_object_code_of_rate_reset = contract_terms.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should always be Some");
        let rate_multiplier = contract_terms.rate_multiplier.clone().itself_or(1.0);
        let rate_spread = contract_terms.rate_spread.clone().itself_or(0.0);

        // Simplified calculation as a placeholder
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

        states.nominal_interest_rate = NominalInterestRate::new(cbv.unwrap() * rate_multiplier.value() + rate_spread.value()).ok();

        states.status_date = StatusDate::new(time.value()).ok();;
    }
}
