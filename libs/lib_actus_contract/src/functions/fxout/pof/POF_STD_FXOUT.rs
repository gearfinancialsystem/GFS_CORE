use crate::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

#[allow(non_camel_case_types)]
pub struct POF_STD_FXOUT;

impl TraitPayOffFunction for POF_STD_FXOUT {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        _day_counter: &Option<DayCountConvention>,
        _time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {
        let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always exist");
        let contract_role_sign = contract_role.role_sign();
        let notional_principal = contract_terms.notional_principal.clone().expect("notionalPrincipal should always exist");
        let notional_principal_2 = contract_terms.notional_principal2.clone().expect("notionalPrincipal2 should always exist");
        let maturity_date = contract_terms.maturity_date.clone().expect("maturity date should always exist");

        let strings = vec![
                            contract_terms.currency2.clone().unwrap().to_currency(),
                            contract_terms.currency.clone().unwrap()
        ];

        let str_slices: Vec<String> = strings.iter().map(|s| s.value().clone().to_string()).collect();
        let joined = str_slices.join("/");


        let mut cbv = None;
        if let Some(rfm) = risk_factor_model {
            cbv = rfm.state_at(
                joined,
                &maturity_date.value(),
                states,
                contract_terms,
                true
            );
        } else {
            cbv = None
        }

        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );

        let payoff = settlement_currency_fx_rate *
                          contract_role_sign *
                          (
                                  notional_principal.value() -
                                      cbv.unwrap() *
                                      notional_principal_2.value()
                          );

        payoff
    }
}
