use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use lib_actus_events::events::ContractEvent::ContractEvent;
use lib_actus_events::events::EventFactory::EventFactory;
use lib_actus_events::events::EventType::EventType;
use lib_actus_states_space::states_space::StatesSpace::StatesSpace;
use lib_actus_terms::ContractTerms::ContractTerms;
use lib_actus_types::types::IsoDatetime::IsoDatetime;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::terms::grp_contract_identification::ContractID::ContractID;
use lib_actus_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use lib_actus_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use lib_actus_terms::terms::grp_calendar::Calendar::Calendar;
use lib_actus_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use lib_actus_terms::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use lib_actus_terms::terms::grp_fees::CycleOfFee::CycleOfFee;
use lib_actus_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use lib_actus_terms::terms::grp_fees::FeeBasis::FeeBasis;
use lib_actus_terms::terms::grp_fees::FeeRate::FeeRate;
use lib_actus_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use lib_actus_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use lib_actus_terms::terms::grp_notional_principal::Currency::Currency;
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
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::clm::pof::POF_IED_CLM::POF_IED_CLM;
use crate::functions::clm::pof::POF_IP_CLM::POF_IP_CLM;
use crate::functions::clm::stf::STF_IP_CLM::STF_IP_CLM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::stf::STF_FP_PAM::STF_FP_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::pam::stf::STF_IPCI_PAM::STF_IPCI_PAM;
use crate::functions::pam::stf::STF_MD_PAM::STF_MD_PAM;
use crate::functions::pam::stf::STF_RR_PAM::STF_RR_PAM;
use crate::functions::pam::stf::STF_RRF_PAM::STF_RRF_PAM;
use lib_actus_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use lib_actus_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use lib_actus_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use lib_actus_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use lib_actus_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use lib_actus_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use lib_actus_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use lib_actus_terms::terms::grp_notional_principal::XDayNotice::XDayNotice;
use lib_actus_terms::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use lib_actus_terms::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use lib_actus_types::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct CLM {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for CLM {
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


        let cycle_of_fee = CycleOfFee::provide_from_input_dict(sm, "cycleOfFee");
        let cycle_anchor_date_of_fee = if cycle_of_fee.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfFee::from_str(&a).ok()
        } else {
            CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee")
        };

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

        let business_day_adjuster = {
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
            fee_rate: FeeRate::provide_from_input_dict(sm, "feeRate"),
            fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            day_count_convention: day_count_convention,
            accrued_interest: AccruedInterest::provide_from_input_dict(sm, "accruedInterest"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            maturity_date: maturity_date,
            x_day_notice: XDayNotice::provide_from_input_dict(sm, "xDayNotice"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
            rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
            fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
            rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
            life_cap: LifeCap::provide_from_input_dict(sm, "lifeCap"),
            life_floor: LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
            period_cap: PeriodCap::provide_from_input_dict(sm, "periodCap"),
            period_floor: PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
            ..Default::default()
        };


        self.contract_terms = ct;
    }

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<RiskFactorModel>) {
        self.contract_risk_factors = None; // RiskFactorModel::new();
    }

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>) {
        self.contract_structure = None;
    }

    fn set_result_vec(&mut self) {
        self.result_vec = Some(Vec::<ResultSet>::new());
    }

    fn schedule(&mut self, to: Option<IsoDatetime>) {
        let model= &self.contract_terms;
        let mut events : Vec<ContractEvent<IsoDatetime, IsoDatetime>>= Vec::new();

        // Determine maturity of the contract
        self.maturity(&to.unwrap());

        // Initial exchange
        let e = EventFactory::<InitialExchangeDate, InitialExchangeDate>::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_CLM)),
            Some(Rc::new(STF_IED_PAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Interest payment event
        let e = EventFactory::<MaturityDate, MaturityDate>::create_event(
            &Some(maturity.clone()),
            &EventType::IP,
            &model.currency,
            Some(Rc::new(POF_IP_CLM)),
            Some(Rc::new(STF_IP_CLM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption
        let e:ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_PAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Interest payment capitalization (if specified)
        if model.cycle_of_interest_payment.is_some() {
            let cycle_anchor_date_of_interest_payment = if model.cycle_anchor_date_of_interest_payment.is_none() {
                //model.initial_exchange_date.clone().unwrap() + CycleUtils::parse_period(&model.cycle_of_interest_payment.clone().unwrap()).unwrap()
                CycleAnchorDateOfInterestPayment::new({
                    model.initial_exchange_date.clone().unwrap().value() + model.cycle_of_interest_payment.clone().unwrap().value().extract_period().unwrap()
                }).ok(
                ).unwrap()
            } else {
                model.cycle_anchor_date_of_interest_payment.clone().unwrap()
            };

            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &Some(cycle_anchor_date_of_interest_payment),
                    &Some(maturity.clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(false),
                ),
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(Rc::new(STF_IPCI_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::<
            CycleAnchorDateOfRateReset,
                MaturityDate,
                CycleOfRateReset,
                IsoDatetime
            >::create_schedule(
                &model.cycle_anchor_date_of_rate_reset,
                &Some(maturity.clone()),
                &model.cycle_of_rate_reset,
                &model.end_of_month_convention,
                Some(false),
            ),
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
                &model.status_date,
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            let mut sorted_events: Vec<_> = rate_reset_events.iter().collect();
            sorted_events.sort();

            if let Some(fixed_event) = sorted_events.iter().find(|&&e| e.compare_to(&status_event.to_iso_datetime_event()) == 1).cloned() {
                let mut fixed_event_clone = fixed_event.clone();
                fixed_event_clone.set_f_state_trans(Some(Rc::new(STF_RRF_PAM)));
                fixed_event_clone.chg_event_type(EventType::RRF);
                rate_reset_events.insert(fixed_event_clone);
            }
        }

        events.extend(rate_reset_events);

        // Fees (if specified)
        if model.cycle_of_fee.is_some() {
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &Some(maturity.clone()),
                    &model.cycle_of_fee,
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

        // Remove all pre-status date events
        let status_event: ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &model.status_date.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&status_event.to_iso_datetime_event()) != -1);

        // Remove all post to-date events
        let to_event: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
            &Some(to.clone().unwrap()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.compare_to(&to_event) != 1);

        // Sort the events according to their time of occurrence
        events.sort();

        self.contract_events = events.clone();
    }

    fn apply(&mut self, result_set_toogle: bool) {
        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        // Initialize state space per status date
        let _maturity = &self.contract_terms.maturity_date.clone();
        self.init_state_space(_maturity);
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

        states.notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        states.interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();
        states.status_date = model.status_date.clone();

        if model.initial_exchange_date.clone().unwrap().value() <= model.status_date.clone().unwrap().value() {
            let role_sign = model.contract_role.as_ref().unwrap().role_sign();
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



impl CLM {
    fn maturity(&self, to: &IsoDatetime) -> MaturityDate {
        MaturityDate::from_str(self.contract_terms.maturity_date.clone().unwrap().value().to_string().as_str()).unwrap()
    }
}

impl fmt::Display for CLM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLM")
    }
}