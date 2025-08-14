use crate::traits::TraitPayOffFunction::TraitPayOffFunction;
use crate::attributes::ContractTerms::ContractTerms;
use crate::states_space::StatesSpace::StatesSpace;
use lib_actus_types::types::IsoDatetime::IsoDatetime;

use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_fees::fee_basis::A::A;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_fees::FeeBasis::FeeBasis;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractReference::ContractReference;
use crate::traits::_TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
#[allow(non_camel_case_types)]
pub struct POF_FP_PAM;

impl TraitPayOffFunction for POF_FP_PAM {
    fn eval(
        &self,
        time: &PhantomIsoDatetimeW,
        states: &StatesSpace,
        contract_terms: &ContractTerms,
        _contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<impl TraitRiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) -> f64 {

        let day_counter = day_counter.clone().expect("sould have day counter");
        let fee_basis = contract_terms.fee_basis.as_ref().expect("feebasis should always be some");
        let fee_rate = contract_terms.fee_rate.as_ref().expect("fee rate should always be some");
        
        let settlement_currency_fx_rate = crate::util::CommonUtils::CommonUtils::settlementCurrencyFxRate(
            risk_factor_model,
            contract_terms,
            time,
            states
        );
        if fee_basis.eq(&FeeBasis::A(A)) {
            let contract_role = contract_terms.contract_role.as_ref().expect("contract role should always be some");
            settlement_currency_fx_rate * contract_role.role_sign() * fee_rate.value()
        } 
        else {
            let notional_principal = contract_terms.notional_principal.as_ref().expect("notionalPrincipal should always be some");
            let fee_accrued = states.fee_accrued.as_ref().expect("fee accrued should always be some");
            let status_date = states.status_date.as_ref().expect("status date should always be some");
            
            settlement_currency_fx_rate * (fee_accrued.value() + day_counter.day_count_fraction(time_adjuster.shift_sc(&status_date.to_phantom_type()), time_adjuster.shift_sc(time))) * fee_rate.value() * notional_principal.value()
        }
        
   
    }

}

