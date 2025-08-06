use crate::attributes::ContractTerms::ContractTerms;

use crate::states_space::StatesSpace::StatesSpace;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::types::IsoDatetime::IsoDatetime;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::traits::TraitOptionExt::TraitOptionExt;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::attributes::ContractReference::ContractReference;


#[allow(non_camel_case_types)]
pub struct STF_PR_NAM;

impl TraitStateTransitionFunction for STF_PR_NAM {
    fn eval(
        &self,
        time: &IsoDatetime,
        states: &mut StatesSpace,
        contract_terms: &ContractTerms,
contract_structure: &Option<Vec<ContractReference>>,
        risk_factor_model: &Option<RiskFactorModel>,
        day_counter: &Option<DayCountConvention>,
        time_adjuster: &BusinessDayAdjuster,
    ) {
        let day_counter = day_counter.clone().expect("sould have day counter");
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");
        let next_principal_redemption_payment = states.next_principal_redemption_payment.clone().expect("nextPrincipalRedemptionPayment should always be Some");
        let accrued_interest = states.accrued_interest.clone().expect("accruedInterest should always be Some");

        let fee_rate_m = contract_terms.fee_rate.clone().expect("feeRateM should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.value()),
            time_adjuster.shift_sc(time)
        );
        states.accrued_interest.add_assign(nominal_interest_rate.value() *
            interest_calculation_base_amount.value() *
            time_from_last_event
        );

        states.fee_accrued.add_assign(fee_rate_m.value() * notional_principal.value() * time_from_last_event);


        let contract_role = contract_terms.contract_role.clone().expect("contractRole should always be Some");
        let role_sign = contract_role.role_sign();


        let redemption_amount = next_principal_redemption_payment.value() - role_sign * accrued_interest.value();

        let redemption = redemption_amount -
            0.0f64.max(redemption_amount - notional_principal.value().abs());


        states.notional_principal.sub_assign(role_sign * redemption);
        states.status_date = Some(StatusDate::from(*time));
    }
}
