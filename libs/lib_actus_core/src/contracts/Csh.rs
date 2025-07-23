use std::collections::HashMap;

use std::fmt;
use std::rc::Rc;
use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;

use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_contract_identification::ContractRole::ContractRole;

use lib_actus_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;

use lib_actus_terms::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Nt::NT;
use lib_actus_terms::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::util::RedemptionUtils::RedemptionUtils;

use lib_actus_terms::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use lib_actus_terms::terms::grp_boundary::boundary_effect::Infil::INFIL;
use lib_actus_terms::terms::grp_boundary::boundary_effect::Insel::INSEL;
use lib_actus_terms::terms::grp_boundary::boundary_effect::Out::OUT;
use lib_actus_terms::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use lib_actus_terms::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use lib_actus_terms::terms::grp_boundary::BoundaryLegInitiallyActive::BoundaryLegInitiallyActive;
use lib_actus_terms::terms::grp_boundary::BoundaryMonitoringAnchorDate::BoundaryMonitoringAnchorDate;
use lib_actus_terms::terms::grp_boundary::BoundaryMonitoringCycle::BoundaryMonitoringCycle;
use lib_actus_terms::terms::grp_boundary::BoundaryMonitoringEndDate::BoundaryMonitoringEndDate;
use lib_actus_terms::terms::grp_boundary::BoundaryValue::BoundaryValue;
use lib_actus_terms::terms::grp_calendar::Calendar::Calendar;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use lib_actus_terms::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use lib_actus_terms::terms::grp_fees::CycleOfFee::CycleOfFee;


use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_fees::FeeBasis::FeeBasis;
use lib_actus_terms::terms::grp_fees::FeeRate::FeeRate;

use lib_actus_terms::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use lib_actus_terms::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use lib_actus_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use lib_actus_terms::terms::grp_interest::CycleOfInterestCalculationBase::CycleOfInterestCalculationBase;
use lib_actus_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use lib_actus_terms::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use lib_actus_terms::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
use lib_actus_terms::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use lib_actus_terms::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use lib_actus_terms::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use lib_actus_terms::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
use lib_actus_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use lib_actus_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;

use lib_actus_terms::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;

use lib_actus_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use lib_actus_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use lib_actus_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;

use lib_actus_terms::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use lib_actus_terms::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use lib_actus_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use lib_actus_terms::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use lib_actus_terms::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use lib_actus_terms::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use lib_actus_terms::terms::grp_optionality::PenaltyRate::PenaltyRate;
use lib_actus_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use lib_actus_terms::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use lib_actus_terms::terms::grp_reset_rate::LifeCap::LifeCap;
use lib_actus_terms::terms::grp_reset_rate::LifeFloor::LifeFloor;
use lib_actus_terms::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::NextResetRate::NextResetRate;
use lib_actus_terms::terms::grp_reset_rate::PeriodCap::PeriodCap;
use lib_actus_terms::terms::grp_reset_rate::PeriodFloor::PeriodFloor;
use lib_actus_terms::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use lib_actus_terms::terms::grp_reset_rate::RateSpread::RateSpread;
use lib_actus_types::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;

use lib_actus_terms::terms::grp_contract_identification::ContractType::ContractType;
use lib_actus_terms::terms::grp_contract_identification::CreatorID::CreatorID;
use lib_actus_terms::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use lib_actus_terms::terms::grp_counterparty::CoverageOfCreditEnhancement::CoverageOfCreditEnhancement;
use lib_actus_terms::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use lib_actus_terms::terms::grp_counterparty::DelinquencyPeriod::DelinquencyPeriod;
use lib_actus_terms::terms::grp_counterparty::DelinquencyRate::DelinquencyRate;
use lib_actus_terms::terms::grp_counterparty::GracePeriod::GracePeriod;
use lib_actus_terms::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use lib_actus_terms::terms::grp_counterparty::PrepaymentPeriod::PrepaymentPeriod;
use lib_actus_terms::terms::grp_counterparty::Seniority::Seniority;
use lib_actus_terms::terms::grp_dividend::CycleAnchorDateOfDividend::CycleAnchorDateOfDividend;
use lib_actus_terms::terms::grp_dividend::CycleOfDividend::CycleOfDividend;
use lib_actus_terms::terms::grp_dividend::ExDividendDate::ExDividendDate;
use lib_actus_terms::terms::grp_dividend::NextDividendPaymentAmount::NextDividendPaymentAmount;
use lib_actus_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use lib_actus_terms::terms::grp_settlement::ExerciseAmount::ExerciseAmount;

use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;

use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_types::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;


#[derive(Debug, Clone, PartialEq)]
pub struct CSH {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for  CSH {
    fn new() -> Self {
        Self {
            contract_terms: ContractTerms::default(),
            contract_events: Vec::<ContractEvent<IsoDatetime, IsoDatetime>>::new(),
            contract_risk_factors: None,
            contract_structure: None,
            states_space: StatesSpace::default(),
            result_vec_toggle: false,
            result_vec: None,
        }
    }

    fn set_contract_terms(&mut self, sm: &HashMap<String, Value>) {
        let ct = ContractTerms {
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            ..Default::default()
        };

        self.contract_terms = ct;
    }

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<RiskFactorModel>) {
        self.contract_risk_factors = None;
    }

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>) {
        self.contract_structure = None;
    }

    fn set_result_vec(&mut self) {
        self.result_vec = Some(Vec::<ResultSet>::new());
    }

    fn schedule(&mut self, to: Option<IsoDatetime>) {
        Ok(Vec::new())
    }

    fn apply(&mut self, result_set_toogle: bool) {

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        // Initialize state space per status date
        
        self.init_state_space(&None);
        self.states_space.status_date = self.contract_terms.status_date.clone();
        self.states_space.notional_principal = NotionalPrincipal::new(&self.contract_terms.contract_role.clone().unwrap().role_sign() * &self.contract_terms.notional_principal.clone().unwrap().value()).ok();
        let events = &mut self.contract_events.clone();
        // Sort the events according to their time sequence
        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        // Apply events according to their time sequence to current state
        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();
        self.states_space = states;
    }

    fn eval_pof_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.contract_events.get(id_ce).expect("ca marche forcement");

        if curr_ce.fpayoff.is_some() {
            let a = curr_ce.fpayoff.clone().unwrap().eval(
                &curr_ce.get_schedule_time(),
                &self.states_space,
                &self.contract_terms,
                {
                    let a = &self.contract_risk_factors;
                    if let Some(rfm) = a {
                        Some(rfm)
                    } else {
                        None
                    }
                },
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            );
            println!("{:?}", a);


            self.contract_events[id_ce].payoff = Some(a);
            // let curr_ce_clone = &curr_ce.clone();
            if self.result_vec_toggle == true {
                if let Some(rv) = &mut self.result_vec {
                    let mut a = ResultSet::new();
                    a.set_result_set(&self.states_space, &self.contract_events[id_ce]);

                    rv.push(a)
                }
            }
        }

        // on peut la retravailler pour etre plus direct et efficace
    }


    fn eval_stf_contract_event(&mut self, id_ce: usize) {
        let mut curr_ce= self.contract_events.get(id_ce).expect("ca marche forcement");

        if curr_ce.fstate.is_some() {
            curr_ce.fstate.clone().unwrap().eval(
                &curr_ce.get_schedule_time(),
                &mut self.states_space,
                &self.contract_terms,
                {
                    let a = &self.contract_risk_factors;
                    if let Some(rfm) = a {
                        Some(rfm)
                    } else {
                        None
                    }
                }
                ,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            )
            //self.contract_events[id_ce].payoff = Some(a);
            //let b = curr_ce.set_payoff(a);
            // self.contract_events[id_ce] = a;

        }
        // on peut la retravailler pour etre plus direct et efficace
    }

}

impl fmt::Display for CSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSH")
    }
}