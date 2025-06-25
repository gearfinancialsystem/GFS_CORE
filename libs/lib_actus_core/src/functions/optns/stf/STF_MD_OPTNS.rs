use crate::attributes::ContractModel::ContractModel;
use crate::externals::RiskFactorModel::RiskFactorModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_optionality::OptionType::OptionType;
use crate::terms::grp_optionality::option_type::C::C;
use crate::terms::grp_optionality::option_type::P::P;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::isoDatetime::IsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_MD_OPTNS;

impl TraitStateTransitionFunction for STF_MD_OPTNS {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StateSpace,
        model: &ContractModel,
        _risk_factor_model: &RiskFactorModel,
        _day_counter: &DayCountConvention,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        if states.exerciseDate.is_none() {
            let mut x = 0.0;
            let st = 1.0; // Placeholder for risk_factor_model logic
            let option_type = model.optionType.as_ref().expect("optionType should always be Some");
            let option_strike1 = model.optionStrike1.unwrap_or(0.0);

            match option_type {
                OptionType::C(C) => {
                    x = (st - option_strike1).max(0.0);
                },
                OptionType::P(P) => {
                    x = (option_strike1 - st).max(0.0);
                },
                _ => {
                    let option_strike2 = model.optionStrike2.unwrap_or(0.0);
                    x = (st - option_strike1).max(0.0) + (option_strike2 - st).max(0.0);
                }
            }

            if x == 0.0 {
                states.exerciseDate = None;
            } else {
                states.exerciseDate = Some(*time);
            }
        }
        states.statusDate = Some(*time);
    }
}
