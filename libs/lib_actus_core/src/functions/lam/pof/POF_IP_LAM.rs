use std::os::linux::raw::stat;
use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_IP_LAM;

impl TraitPayOffFunction for POF_IP_LAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StateSpace,
        model: &ContractModel,
        risk_factor_model: &RiskFactorModel,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        let status_date = states.status_date.clone().expect("No status date");
        let interest_scaling_multiplier = states.interest_scaling_multiplier.clone().expect("interest_scaling_multiplier should exist");
        let accrued_interest = states.accrued_interest.clone().expect("accrued_interest should exist");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominal_interest_rate should exist");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interest_calculation_base_amount should exist");
        
        
        let timadj = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        );
        println!("pof ip lam : {:?}", time);
        settlement_currency_fx_rate * interest_scaling_multiplier.value()
            * (accrued_interest.value() + (timadj * nominal_interest_rate.value() * 
            interest_calculation_base_amount.value())
        )

    }
}
