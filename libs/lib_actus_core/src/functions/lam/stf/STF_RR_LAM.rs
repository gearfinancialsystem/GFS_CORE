use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;

use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_RR_LAM;

impl TraitStateTransitionFunction for STF_RR_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let accruedInterest = states.accrued_interest.clone().expect("accruedInterest should always be Some");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let rate_multiplier_m = model.rate_multiplier.clone().expect("rateMultiplier should always be Some");
        let rate_spread_m = model.rate_spread.clone().expect("rateSpread should always be Some");
        let period_floor_m = model.period_floor.clone().expect("periodFloor should always be Some");
        let period_cap_m = model.period_cap.clone().expect("periodCap should always be Some");
        let life_floor_m = model.life_floor.clone().expect("lifeFloor should always be Some");
        let life_cap_m = model.life_cap.clone().expect("lifeCap should always be Some");
        let market_object_code_of_rate_reset_m = model.market_object_code_of_rate_reset.clone().expect("model.market_object_code_of_rate_reset should be some");
        let fee_rate_m = model.fee_rate.clone().expect("feeRate should always be Some");


        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().value()),
            time_adjuster.shift_sc(time)
        );

        //let rf = risk_factor_model.state_at(&market_object_code_of_rate_reset_m.value(), time, states, model, true).expect("riskFactorModel.state_at shoould be some");.
        let rf = 1.0;

        let mut rate = ( rf * rate_multiplier_m.value())
            + rate_spread_m.clone().value() - nominal_interest_rate.clone().value();

        let delta_rate = rate.max(period_floor_m.value()).min(period_cap_m.value());
        rate = (nominal_interest_rate.value() + delta_rate).max(life_floor_m.value()).min(life_cap_m.value());

        states.accrued_interest.add_assign(nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event);
        states.fee_accrued.add_assign(fee_rate_m.value() * notional_principal.value() * time_from_last_event);


        states.nominal_interest_rate = NominalInterestRate::new(rate).ok();
        states.status_date = Some(StatusDate::from(*time));
    }
}
