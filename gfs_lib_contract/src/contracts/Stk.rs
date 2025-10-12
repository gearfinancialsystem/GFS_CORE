use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::types::IsoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::traits::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_dividend::CycleAnchorDateOfDividendPayment::CycleAnchorDateOfDividendPayment;
use crate::terms::grp_dividend::CycleOfDividendPayment::CycleOfDividendPayment;
use crate::terms::grp_dividend::MarketObjectCodeOfDividends::MarketObjectCodeOfDividends;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::stk::pof::POF_DV_STK::POF_DV_STK;
use crate::functions::stk::pof::POF_PRD_STK::POF_PRD_STK;
use crate::functions::stk::pof::POF_TD_STK::POF_TD_STK;
use crate::functions::stk::stf::STF_DV_STK::STF_DV_STK;
use crate::functions::stk::stf::STF_TD_STK::STF_TD_STK;
use crate::functions::stk::stf::STK_PRD_STK::STF_PRD_STK;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::MarketValueObserved::MarketValueObserved;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::Quantity::Quantity;
use crate::traits::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;
use crate::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;


/// Represents the Principal At Maturity payoff algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct STK {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for STK {
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
        //let purchase_date = IsoDatetime::provide(sm, "purchaseDate");
        let purchase_date = PurchaseDate::provide_from_input_dict(sm, "purchaseDate");

        let calendar = Calendar::provide_rc(sm, "calendar");

        //VERIFIER PAS PRESENT DANS LA LISTE DES TERMES
        let cycle_of_dividend_payment = CycleOfDividendPayment::provide_from_input_dict(sm, "cycleOfDividendPayment");

        let business_day_adjuster = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "BusinessDayAdjuster",
                calendar_clone.expect("df")
            )
        };

        let cycle_anchor_date_of_dividend_payment = {
            let a = if cycle_of_dividend_payment.is_none() {
                None
            } else {
                let purchase_date_str = purchase_date.clone().unwrap().value().to_string();
                CycleAnchorDateOfDividendPayment::from_str(purchase_date_str.as_str()).ok()
            };
            let b = CycleAnchorDateOfDividendPayment::provide_from_input_dict(sm, "CycleAnchorDateOfDividendPayment");
            if b.is_none() { a } else { b }
        };

        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let ct = ContractTerms {
            contract_type: ContractType::provide_from_input_dict(sm, "contractType"),
            contract_id: ContractID::provide_from_input_dict(sm, "contractID"),
            status_date: StatusDate::provide_from_input_dict(sm, "statusDate"),
            contract_role: ContractRole::provide_from_input_dict(sm, "contractRole"),
            counterparty_id: CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            quantity: Quantity::provide_from_input_dict(sm, "quantity"),
            purchase_date: purchase_date,
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            market_object_code: MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            market_value_observed: MarketValueObserved::provide_from_input_dict(sm, "marketValueObserved"),
            calendar: calendar,
            business_day_adjuster: business_day_adjuster,
            end_of_month_convention: end_of_month_convention,
            cycle_of_dividend_payment: cycle_of_dividend_payment,
            cycle_anchor_date_of_dividend_payment: cycle_anchor_date_of_dividend_payment,
            market_object_code_of_dividends: MarketObjectCodeOfDividends::provide_from_input_dict(sm, "marketObjectCodeOfDividends"),
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

    /// Compute next events within the period up to `to` date based on the contract model
    fn schedule(&mut self, to: Option<IsoDatetime>) {
        let model = &self.contract_terms;
        let mut events = Vec::new();


        if model.purchase_date.is_some(){
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_STK)),
                Some(Rc::new(STF_PRD_STK)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }
        if model.cycle_of_dividend_payment.is_some(){
            if model.termination_date.is_none(){
                events.extend(
                    EventFactory::create_events(
                        &ScheduleFactory::create_schedule(
                            &model.cycle_anchor_date_of_dividend_payment,
                            &Some(model.cycle_anchor_date_of_dividend_payment.clone().unwrap().value() + IsoPeriod::of_years(10)), // definir les constantes
                            &model.cycle_of_dividend_payment,
                            &model.end_of_month_convention,
                            Some(true)
                        ),
                        &EventType::DV,
                        &model.currency,
                        Some(Rc::new(POF_DV_STK)),
                        Some(Rc::new(STF_DV_STK)),
                        &model.business_day_adjuster,
                        &model.contract_id)
                );
            }
            else {
                events.extend(
                    EventFactory::create_events(
                        &ScheduleFactory::create_schedule(
                            &model.cycle_anchor_date_of_dividend_payment,
                            &model.termination_date,
                            &model.cycle_of_dividend_payment,
                            &model.end_of_month_convention,
                            Some(true)),
                        &EventType::DV,
                        &model.currency,
                        Some(Rc::new(POF_DV_STK)),
                        Some(Rc::new(STF_DV_STK)),
                        &model.business_day_adjuster,
                        &model.contract_id)
                )
            }
        }
        if model.termination_date.is_some(){
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &model.termination_date,
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_STK)),
                Some(Rc::new(STF_TD_STK)),
                &None,
                &model.contract_id,
            );
            events.retain(|e| {
                e.compare_to(&termination.to_iso_datetime_event()) != 1
            });
            events.push(termination.to_iso_datetime_event());
        }
        events.retain(|e| {
            e.compare_to({
                &EventFactory::create_event(
                    &Some(model.status_date.clone().unwrap().value()),
                    &EventType::TD,
                    &model.currency,
                    None,
                    None,
                    &None,
                    &model.contract_id
                )
            }) != -1
        });
        events.retain(|e| {
            e.compare_to({
                &EventFactory::create_event(
                    &Some(to.clone().clone().unwrap()),
                    &EventType::AD,
                    &model.currency,
                    None,
                    None,
                    &None,
                    &model.contract_id
                )
            }) != 1
        });

        events.sort();
        self.contract_events = events.clone();
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
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
        // Sort events according to their time sequence
        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        let mut i: usize = 0;
        for event in events.iter_mut() {
            self.eval_pof_contract_event(i);
            self.eval_stf_contract_event(i);

            i+=1;
        }
        // Return evaluated events
        self.contract_events = events.clone();
    }

    /// Initialize the StatesSpace according to the model attributes
    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();
        states.status_date = model.status_date.clone();

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
impl fmt::Display for STK {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "STK")
    }
}