use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_events::traits::TraitPayOffFunction::TraitPayOffFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;

use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;

#[allow(non_camel_case_types)]
pub struct POF_PRD_LAM;

impl TraitPayOffFunction for POF_PRD_LAM {
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
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            model,
            time,
            states
        );

        settlement_currency_fx_rate
            * model.clone().contract_role.unwrap().role_sign()
            * (-1.0)
            * (model.price_at_purchase_date.clone().unwrap().value() + states.accrued_interest.clone().unwrap().value()
            + (day_counter.day_count_fraction(
                    time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
                    time_adjuster.shift_sc(time),
        ) * states.nominal_interest_rate.clone().unwrap().value()
            * states.interest_calculation_base_amount.clone().unwrap().value()))
    }
}
