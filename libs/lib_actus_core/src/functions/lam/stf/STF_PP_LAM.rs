use lib_actus_terms::ContractTerms::ContractTerms;

use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use lib_actus_events::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use lib_actus_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_events::traits::TraitRiskFactorModel::TraitRiskFactorModel;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;

#[allow(non_camel_case_types)]
pub struct STF_PP_LAM;

impl TraitStateTransitionFunction for STF_PP_LAM {
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
        let status_date = states.status_date.clone().expect("statusDate should always be Some");
        let nominal_interest_rate = states.nominal_interest_rate.clone().expect("nominalInterestRate should always be Some");
        let interest_calculation_base_amount = states.interest_calculation_base_amount.clone().expect("interestCalculationBaseAmount should always be Some");
        let notional_principal = states.notional_principal.clone().expect("notionalPrincipal should always be Some");

        let time_from_last_event = day_counter.day_count_fraction(
            time_adjuster.shift_sc(&status_date.clone().value()),
            time_adjuster.shift_sc(time)
        );

        states.accrued_interest = AccruedInterest::new({
            states.accrued_interest.clone().unwrap().value() + nominal_interest_rate.value() * interest_calculation_base_amount.value() * time_from_last_event
        }).ok();


        states.fee_accrued = FeeAccrued::new({
            let fee_rate = {
                if model.fee_rate.is_none() {
                    0.0
                }
                else { model.fee_rate.clone().unwrap().value() }
            };
            states.fee_accrued.clone().unwrap().value() + fee_rate * {
                if states.notional_principal.is_none() {
                    0.0
                } else {states.notional_principal.clone().unwrap().value()}
            } * time_from_last_event
        }).ok();

        // let prepayment_factor = risk_factor_model.state_at(
        //     &model.objectCodeOfPrepaymentModel,
        //     time,
        //     states,
        //     model,
        //     false
        // );
        let prepayment_factor = 1.0;

        states.notional_principal = NotionalPrincipal::new({
            notional_principal.value() - prepayment_factor * notional_principal.value()
        }).ok();
        
        

        if let Some(interest_calculation_base) = &model.interest_calculation_base.clone() {
            if *interest_calculation_base == InterestCalculationBase::NTL(NTL) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
            }
        }

        states.status_date = Some(StatusDate::from(*time));
    }
}
