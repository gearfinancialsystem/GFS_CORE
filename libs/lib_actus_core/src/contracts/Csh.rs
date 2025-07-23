use std::collections::HashMap;

use std::fmt;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::types::IsoDatetime::IsoDatetime;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;

use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;

use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;

use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::util::RedemptionUtils::RedemptionUtils;

use crate::terms::grp_boundary::BoundaryEffect::BoundaryEffect;
use crate::terms::grp_boundary::boundary_effect::Infil::INFIL;
use crate::terms::grp_boundary::boundary_effect::Insel::INSEL;
use crate::terms::grp_boundary::boundary_effect::Out::OUT;
use crate::terms::grp_boundary::BoundaryCrossedFlag::BoundaryCrossedFlag;
use crate::terms::grp_boundary::BoundaryDirection::BoundaryDirection;
use crate::terms::grp_boundary::BoundaryLegInitiallyActive::BoundaryLegInitiallyActive;
use crate::terms::grp_boundary::BoundaryMonitoringAnchorDate::BoundaryMonitoringAnchorDate;
use crate::terms::grp_boundary::BoundaryMonitoringCycle::BoundaryMonitoringCycle;
use crate::terms::grp_boundary::BoundaryMonitoringEndDate::BoundaryMonitoringEndDate;
use crate::terms::grp_boundary::BoundaryValue::BoundaryValue;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use crate::terms::grp_fees::CycleOfFee::CycleOfFee;


use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::FeeRate::FeeRate;

use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use crate::terms::grp_interest::CycleAnchorDateOfInterestCalculationBase::CycleAnchorDateOfInterestCalculationBase;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestCalculationBase::CycleOfInterestCalculationBase;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::AmortizationDate::AmortizationDate;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use crate::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use crate::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;

use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;

use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;

use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use crate::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use crate::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use crate::terms::grp_optionality::PenaltyRate::PenaltyRate;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use crate::terms::grp_reset_rate::LifeCap::LifeCap;
use crate::terms::grp_reset_rate::LifeFloor::LifeFloor;
use crate::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use crate::terms::grp_reset_rate::NextResetRate::NextResetRate;
use crate::terms::grp_reset_rate::PeriodCap::PeriodCap;
use crate::terms::grp_reset_rate::PeriodFloor::PeriodFloor;
use crate::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use crate::terms::grp_reset_rate::RateSpread::RateSpread;
use crate::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;

use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::CreatorID::CreatorID;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::CoverageOfCreditEnhancement::CoverageOfCreditEnhancement;
use crate::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use crate::terms::grp_counterparty::DelinquencyPeriod::DelinquencyPeriod;
use crate::terms::grp_counterparty::DelinquencyRate::DelinquencyRate;
use crate::terms::grp_counterparty::GracePeriod::GracePeriod;
use crate::terms::grp_counterparty::NonPerformingDate::NonPerformingDate;
use crate::terms::grp_counterparty::PrepaymentPeriod::PrepaymentPeriod;
use crate::terms::grp_counterparty::Seniority::Seniority;
use crate::terms::grp_dividend::CycleAnchorDateOfDividend::CycleAnchorDateOfDividend;
use crate::terms::grp_dividend::CycleOfDividend::CycleOfDividend;
use crate::terms::grp_dividend::ExDividendDate::ExDividendDate;
use crate::terms::grp_dividend::NextDividendPaymentAmount::NextDividendPaymentAmount;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;

use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;

use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::types::Value::Value;
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
                &self.contract_structure,
                &self.contract_risk_factors,
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
                &self.contract_structure,
                &self.contract_risk_factors,
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