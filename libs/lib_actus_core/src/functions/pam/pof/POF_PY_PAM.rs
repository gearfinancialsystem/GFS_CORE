use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;


#[allow(non_camel_case_types)]
pub struct POF_PY_PAM;

impl TraitPayOffFunction for POF_PY_PAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        model: &ContractTerms,
        risk_factor_model: Option<&dyn TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let penalty_type = model.penalty_type.as_ref().expect("penaltyType should be Some");
        let contract_role = model.contract_role.as_ref().expect("contract role should be Some");

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );
        match penalty_type {
            PenaltyType::A(_A) => {
                let penalty_rate = model.penalty_rate.as_ref().expect("penaltyRate should be Some");
                settlement_currency_fx_rate * contract_role.role_sign() * penalty_rate.value()
            }
            PenaltyType::N(_N) => {
                let penalty_rate = model.penalty_rate.as_ref().expect("penaltyRate should be Some");
                let status_date = states.status_date.as_ref().expect("status date should always exist");
                let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should be Some");

                settlement_currency_fx_rate * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()), time_adjuster.shift_sc(&time))
                * penalty_rate.value() * notional_principal.value()
            }
            _ => {
                let status_date = states.status_date.as_ref().expect("status date should always exist");
                let notional_principal = states.notional_principal.as_ref().expect("notionalPrincipal should always exist");
                let nominal_interest_rate = states.nominal_interest_rate.as_ref().expect("nominalInterestRate should be Some");
                //let market_object_code_of_rate_reset = model.marketObjectCodeOfRateReset.as_ref().expect("marketObjectCodeOfRateReset should be Some");
                
                settlement_currency_fx_rate * contract_role.role_sign()
                    * day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.value()), time_adjuster.shift_sc(&time))
                    * notional_principal.value()
                    * 0.0f64.max(nominal_interest_rate.value() - 1.0f64)
            }
        }
    }
}
