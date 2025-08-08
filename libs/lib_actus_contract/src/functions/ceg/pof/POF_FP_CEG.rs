use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::fee_basis::A::A;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;

use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_FP_CEG;

impl TraitPayOffFunction for POF_FP_CEG {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always exist");
        let fee_rate = contract_terms.fee_rate.clone().expect("feeRate should always exist");

        let day_counter = day_counter.clone().expect("sould have day counter");
        
        let payoff = if FeeBasis::A(A) == contract_terms.fee_basis.clone().unwrap() {
            settlement_currency_fx_rate * contract_role.role_sign() * fee_rate.value()
        } else {
            let time_from_last_event = day_counter.day_count_fraction(
                time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
                time_adjuster.shift_sc(time)
            );
            settlement_currency_fx_rate * (
                states.fee_accrued.clone().unwrap().value() +
                    (states.notional_principal.clone().unwrap().value() * time_from_last_event * fee_rate.value())
            )
        };

        payoff
    }
}
