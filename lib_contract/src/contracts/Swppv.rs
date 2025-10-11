use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::types::IsoDatetime::IsoDatetime;

use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;

use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::fxout::pof::POF_PRD_FXOUT::POF_PRD_FXOUT;
use crate::functions::fxout::pof::POF_TD_FXOUT::POF_TD_FXOUT;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::swppv::pof::POF_IED_SWPPV::POF_IED_SWPPV;
use crate::functions::swppv::pof::POF_IP_SWPPV::POF_IP_SWPPV;
use crate::functions::swppv::pof::POF_IPFIx_SWPPV::POF_IPFix_SWPPV;
use crate::functions::swppv::pof::POF_IPFloat_SWPPV::POF_IPFloat_SWPPV;
use crate::functions::swppv::pof::POF_MD_SWPPV::POF_MD_SWPPV;
use crate::functions::swppv::stf::STF_IED_SWPPV::STF_IED_SWPPV;
use crate::functions::swppv::stf::STF_IP_SWPPV::STF_IP_SWPPV;
use crate::functions::swppv::stf::STF_IPFix_SWPPV::STF_IPFix_SWPPV;
use crate::functions::swppv::stf::STF_IPFloat_SWPPV::STF_IPFloat_SWPPV;
use crate::functions::swppv::stf::STF_MD_SWPPV::STF_MD_SWPPV;
use crate::functions::swppv::stf::STF_PRD_SWPPV::STF_PRD_SWPPV;
use crate::functions::swppv::stf::STF_RR_SWPPV::STF_RR_SWPPV;
use crate::functions::swppv::stf::STF_TD_SWPPV::STF_TD_SWPPV;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_settlement::DeliverySettlement::DeliverySettlement;
use crate::terms::grp_settlement::delivery_settlement::D::D;
use crate::types::Value::Value;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use crate::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use crate::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use crate::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use crate::terms::grp_reset_rate::NextResetRate::NextResetRate;
use crate::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use crate::terms::grp_reset_rate::RateSpread::RateSpread;
use crate::traits::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::CreatorID::CreatorID;
use crate::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use crate::traits::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

#[derive(Debug, Clone, PartialEq)]
pub struct SWPPV {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for SWPPV {
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
        let calendar = Calendar::provide_rc(sm, "calendar");

        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };


        // Champs qui dépendent d'autres champs
        let cycle_of_interest_payment = CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment");
        let cycle_anchor_date_of_interest_payment = if cycle_of_interest_payment.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfInterestPayment::from_str(&a).ok()
        } else {
            CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment")
        };

        let day_count_convention = if let Some(maturity_date) = &maturity_date {
            DayCountConvention::provide_from_input_dict(sm, "dayCountConvention", Some(Rc::clone(maturity_date)), Some(Rc::clone(&calendar)))
        } else {
            None
        };


        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfRateReset::from_str(&a).ok()
        } else {
            CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
        };

        let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide(sm, "cyclePointOfInterestPayment");
        let cycle_point_of_rate_reset = if let Some(point) = &cycle_point_of_interest_payment {
            if point.to_string() == "B" {
                Some(CyclePointOfRateReset::new("E").expect("d"))
            } else {
                CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
            }
        } else {
            CyclePointOfRateReset::provide(sm, "cyclePointOfRateReset")
        };

        let business_day_adjuster =  {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };
        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let ct = ContractTerms {
            accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
            accrued_interest2: AccruedInterest2::provide_from_input_dict(sm, "accruedInterest2"),
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            creator_id: CreatorID::provide_from_input_dict(sm, "creatorID"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            nominal_interest_rate2: NominalInterestRate2::provide_from_input_dict(sm, "nominalInterestRate2"),
            day_count_convention: day_count_convention,
            currency: Currency::provide_from_input_dict(sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
            maturity_date: maturity_date,
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
            rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
            cycle_point_of_rate_reset: cycle_point_of_rate_reset,
            cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
            fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
            rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
            delivery_settlement: DeliverySettlement::provide_from_input_dict(sm, "deliverySettlement"),
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
        let model = &self.contract_terms;
        let mut events = Vec::new();

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_FXOUT)),
                Some(Rc::new(STF_PRD_SWPPV)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Initial exchange event
        let e: ContractEvent<InitialExchangeDate, InitialExchangeDate> = EventFactory::create_event(
            &model.initial_exchange_date.clone(),
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_SWPPV)),
            Some(Rc::new(STF_IED_SWPPV)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption event
        let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &model.maturity_date.clone().map(|rc| (*rc).clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_SWPPV)),
            Some(Rc::new(STF_MD_SWPPV)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Interest payment events
        if model.delivery_settlement == Some(DeliverySettlement::D(D)) || model.delivery_settlement.is_none() {
            // In case of physical delivery (delivery of individual cash flows)
            let interest_schedule = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_interest_payment.clone(),
                &model.end_of_month_convention.clone(),
                Some(false)
            );

            // Fixed rate events
            let fixed_rate_events = EventFactory::create_events(
                &interest_schedule,
                &EventType::IPFX,
                &model.currency,
                Some(Rc::new(POF_IPFix_SWPPV)),
                Some(Rc::new(STF_IPFix_SWPPV)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            // Floating rate events
            let floating_rate_events = EventFactory::create_events(
                &interest_schedule,
                &EventType::IPFL,
                &model.currency,
                Some(Rc::new(POF_IPFloat_SWPPV)),
                Some(Rc::new(STF_IPFloat_SWPPV)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fixed_rate_events);
            events.extend(floating_rate_events);
        } else {
            // In case of cash delivery (cash settlement)
            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_payment.clone(),
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(true)
                ),
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_SWPPV)),
                Some(Rc::new(STF_IP_SWPPV)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_rate_reset,
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_rate_reset,
                &model.end_of_month_convention,
                Some(false),
            ),
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_SWPPV)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        events.extend(rate_reset_events);

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_FXOUT)),
                Some(Rc::new(STF_TD_SWPPV)),
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
            &Some(to.clone().clone().unwrap()),
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

        let _maturity = &self.contract_terms.maturity_date.clone();
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

        // Remove pre-purchase events if purchase date is set
        if let Some(purchase_date) = &self.contract_terms.purchase_date {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

        states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        states.status_date = model.status_date.clone();

        if model.initial_exchange_date.clone().unwrap().value() <= model.status_date.clone().unwrap().value() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
            states.nominal_interest_rate2 = model.nominal_interest_rate2.clone();
            states.accrued_interest = AccruedInterest::new(role_sign * model.accrued_interest.clone().unwrap().value()).ok();
            states.accrued_interest2 = AccruedInterest2::new(role_sign * model.accrued_interest2.clone().unwrap().value()).ok();
            states.last_interest_period = Some(0.0);
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
impl fmt::Display for SWPPV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMPPV")
    }
}