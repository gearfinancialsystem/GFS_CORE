use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::types::IsoDatetime::IsoDatetime;
use crate::types::Value::Value;

use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::clm::pof::POF_IED_CLM::POF_IED_CLM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::pof::POF_TD_PAM::POF_TD_PAM;
use crate::functions::pam::stf::STF_FP_PAM::STF_FP_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::pam::stf::STF_IPCI_PAM::STF_IPCI_PAM;
use crate::functions::pam::stf::STF_RR_PAM::STF_RR_PAM;
use crate::functions::pam::stf::STF_RRF_PAM::STF_RRF_PAM;
use crate::functions::pam::stf::STF_TD_PAM::STF_TD_PAM;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;

use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use crate::terms::grp_fees::CycleOfFee::CycleOfFee;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
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
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::XDayNotice::XDayNotice;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct UMP {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for UMP {
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
        // Déclarations simples sans dépendances
        let calendar = Calendar::provide_rc(sm, "calendar");

        // doit etre None a priori
        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };

        let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
        let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfFee::from_str(&a).ok()
        } else {
            CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
        };

        let mut cycle_anchor_date_of_interest_payment = CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment");
        let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict (sm, "cycleOfInterestPayment");
        let cycle_anchor_date_of_interest_payment = if cycle_anchor_date_of_interest_payment.is_some() {
            cycle_anchor_date_of_interest_payment
        } else {
            if cycle_of_interest_payment.is_some() {
                let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value();
                CycleAnchorDateOfInterestPayment::new(a).ok()
            }
            else {
                None
            }
        };

        let day_count_convention =
            DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", maturity_date.clone(), Some(Rc::clone(&calendar)));
        // let day_count_convention = if let Some(maturity_date) = &maturity_date {
        //
        // } else {
        //     None
        // };

        let mut cycle_anchor_date_of_rate_reset = CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset");
        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict (sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_reset.is_some() {
            cycle_anchor_date_of_rate_reset
        } else {
            if cycle_of_rate_reset.is_some() {
                let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value();
                CycleAnchorDateOfRateReset::new(a).ok()
            }
            else {
                None
            }
        };

        let business_day_adjuster =  {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };
        let w = NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate");
        let nominal_interest_rate = if w.is_some() {
            w
        }
        else {
            NominalInterestRate::new(0.0).ok()
        };

        let w = FeeRate::provide_from_input_dict(sm, "feeRate");
        let fee_rate = if w.is_some() {
            w
        }
        else {
            FeeRate::new(0.0).ok()
        };

        let w = FeeAccrued::provide_from_input_dict(sm, "feeAccrued");
        let fee_accrued = if w.is_some() { w } else { FeeAccrued::new(0.0).ok() };


        let w = PeriodCap::provide_from_input_dict(sm, "periodCap");
        let period_cap = if w.is_some() { w } else { PeriodCap::new(f64::INFINITY).ok() };

        let w = PeriodFloor::provide_from_input_dict(sm, "periodFloor");
        let period_floor = if w.is_some() { w } else { PeriodFloor::new(f64::NEG_INFINITY).ok() };

        let w = LifeCap::provide_from_input_dict(sm, "lifeCap");
        let life_cap = if w.is_some() { w } else { LifeCap::new(f64::INFINITY).ok() };

        let w = LifeFloor::provide_from_input_dict(sm, "lifeFloor");
        let life_floor = if w.is_some() { w } else { LifeFloor::new(f64::NEG_INFINITY).ok() };



        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let ct = ContractTerms {
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            cycle_anchor_date_of_fee: cycle_anchor_date_of_fee,
            cycle_of_fee: CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
            fee_basis: FeeBasis::provide_from_input_dict(sm, "feeBasis"),
            fee_rate: fee_rate,
            fee_accrued: fee_accrued,
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment:cycle_of_interest_payment,
            nominal_interest_rate: nominal_interest_rate,
            day_count_convention: day_count_convention,
            accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            x_day_notice: XDayNotice::provide_from_input_dict(sm, "xDayNotice"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
            rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
            fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
            rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
            life_cap: life_cap,
            life_floor: life_floor,
            period_cap: period_cap,
            period_floor: period_floor,
            maturity_date: maturity_date,
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
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let model = &self.contract_terms;
        // Initial exchange event
        let e: ContractEvent<InitialExchangeDate, InitialExchangeDate> = EventFactory::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_CLM)),
            Some(Rc::new(STF_IED_PAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Interest payment capitalization events
        let s = ScheduleFactory::create_schedule(
            &model.cycle_anchor_date_of_interest_payment,
            &Some(to.clone().unwrap()),
            &model.cycle_of_interest_payment,
            &model.end_of_month_convention,
            Some(false),
        );
        let interest_events = EventFactory::create_events(
            &s,
            &EventType::IPCI,
            &model.currency,
            Some(Rc::new(POF_IPCI_PAM)),
            Some(Rc::new(STF_IPCI_PAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        events.extend(interest_events);

        // Rate reset events
        let s = ScheduleFactory::create_schedule(
            &model.cycle_anchor_date_of_rate_reset,
            &Some(to.clone().unwrap()),
            &model.cycle_of_rate_reset,
            &model.end_of_month_convention,
            Some(false),
        );
        let mut rate_reset_events = EventFactory::create_events(
            &s,
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::create_event(
                &model.status_date,
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

            let mut fixed_event = sorted_events.iter().find(|&e| e.event_time.clone() > status_event.event_time.clone()).unwrap().clone().clone();
            fixed_event.fstate = Some(Rc::new(STF_RRF_PAM));
            fixed_event.event_type = EventType::RRF;
            rate_reset_events.insert(fixed_event);

        }

        events.append(&mut rate_reset_events.into_iter().collect());

        // Fee events (if specified)
        if let Some(cycle_of_fee) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee.clone(),
                    &Some(to.clone().unwrap()),
                    &Some(cycle_of_fee.clone()),
                    &model.end_of_month_convention,
                    Some(false),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_PAM)),
                Some(Rc::new(STF_TD_PAM)),
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_time <= termination.event_time);
            events.push(termination.to_iso_datetime_event());
        }

        // Remove all pre-status date events
        let status_event = EventFactory::create_event(
            &model.status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time >= status_event.event_time);

        // Remove all post to-date events
        let to_event = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= to_event.event_time);

        // Sort events according to their time of occurrence
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        self.contract_events = events.clone();
    }

    fn apply(&mut self, result_set_toogle: bool) {

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        let model = &self.contract_terms;
        let _maturity = &model.maturity_date.clone();
        self.init_state_space(_maturity);
        let events = &mut self.contract_events.clone();

        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

        states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        states.interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();
        states.status_date = model.status_date.clone();

        if model.initial_exchange_date.clone().unwrap().value() <= model.status_date.clone().unwrap().value() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
            states.accrued_interest = AccruedInterest::new(role_sign * model.accrued_interest.clone().unwrap().value()).ok();
            states.fee_accrued = model.fee_accrued.clone();
        }

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
impl fmt::Display for UMP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UMP")
    }
}