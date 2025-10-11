use std::collections::HashMap;
use crate::events::EventType::EventType;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::types::IsoDatetime::IsoDatetime;

#[derive(PartialEq, Debug, Clone)]
pub struct ResultSet {
    values: HashMap<String, String>,
    event_date: IsoDatetime,
    exercise_date: IsoDatetime,
    event_type: EventType,
    currency: Currency,
    payoff: f64,
    accrued_interest: f64,
    accrued_interest2: f64,
    exercise_mount: f64,
    fee_accrued: f64,
    interest_calculation_base_amount: f64,
    interest_scaling_multiplier: f64,
    next_principal_redemption_payment: f64,
    nominal_interest_rate: f64,
    nominal_interest_rate2: f64,
    notional_principal: f64,
    notional_principal2: f64,
    notional_scaling_multiplier: f64,
}
impl ResultSet {
    pub fn get_values(&self) -> &HashMap<String, String> {
        &self.values
    }
    pub fn set_values(&mut self, values: HashMap<String, String>) {
        self.values = values;
    }
    pub fn get_event_date(&self) -> &IsoDatetime {
        &self.event_date
    }
    pub fn set_event_date(&mut self, event_date: IsoDatetime) {
        self.event_date = event_date;
    }
    pub fn get_exercise_date(&self) -> &IsoDatetime {
        &self.exercise_date
    }
    pub fn set_exercise_date(&mut self, exercise_date: IsoDatetime) {
        self.exercise_date = exercise_date;
    }
    pub fn get_event_type(&self) -> &EventType {
        &self.event_type
    }
    pub fn set_event_type(&mut self, event_type: EventType) {
        self.event_type = event_type;
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency;
    }
    pub fn get_payoff(&self) -> f64 {
        self.payoff
    }
    pub fn set_payoff(&mut self, payoff: f64) {
        self.payoff = payoff;
    }
    pub fn get_accrued_interest(&self) -> f64 {
        self.accrued_interest
    }
    pub fn set_accrued_interest(&mut self, accrued_interest: f64) {
        self.accrued_interest = accrued_interest;
    }
    pub fn get_accrued_interest2(&self) -> f64 {
        self.accrued_interest2
    }
    pub fn set_accrued_interest2(&mut self, accrued_interest2: f64) {
        self.accrued_interest2 = accrued_interest2;
    }
    pub fn get_exercise_mount(&self) -> f64 {
        self.exercise_mount
    }
    pub fn set_exercise_mount(&mut self, exercise_mount: f64) {
        self.exercise_mount = exercise_mount;
    }
    pub fn get_fee_accrued(&self) -> f64 {
        self.fee_accrued
    }
    pub fn set_fee_accrued(&mut self, fee_accrued: f64) {
        self.fee_accrued = fee_accrued;
    }
    pub fn get_interest_calculation_base_amount(&self) -> f64 {
        self.interest_calculation_base_amount
    }
    pub fn set_interest_calculation_base_amount(&mut self, interest_calculation_base_amount: f64) {
        self.interest_calculation_base_amount = interest_calculation_base_amount;
    }
    pub fn get_interest_scaling_multiplier(&self) -> f64 {
        self.interest_scaling_multiplier
    }
    pub fn set_interest_scaling_multiplier(&mut self, interest_scaling_multiplier: f64) {
        self.interest_scaling_multiplier = interest_scaling_multiplier;
    }
    pub fn get_next_principal_redemption_payment(&self) -> f64 {
        self.next_principal_redemption_payment
    }
    pub fn get_nominal_interest_rate(&self) -> f64 {
        self.nominal_interest_rate
    }
    pub fn set_nominal_interest_rate(&mut self, nominal_interest_rate: f64) {
        self.nominal_interest_rate = nominal_interest_rate;
    }
    pub fn get_notional_principal(&self) -> f64 {
        self.notional_principal
    }
    pub fn set_notional_principal(&mut self, notional_principal: f64) {
        self.notional_principal = notional_principal;
    }
    pub fn get_notional_principal2(&self) -> f64 {
        self.notional_principal2
    }
    pub fn set_notional_principal2(&mut self, notional_principal2: f64) {
        self.notional_principal2 = notional_principal2;
    }
    pub fn get_notional_scaling_multiplier(&self) -> f64 {
        self.notional_scaling_multiplier
    }
    pub fn set_notional_scaling_multiplier(&mut self, notional_scaling_multiplier: f64) {
        self.notional_scaling_multiplier = notional_scaling_multiplier;
    }
    
}   