// SPDX-License-Identifier: Apache-2.0
// Converted from the Java implementation of PrincipalAtMaturity in org.actus.contracts

use chrono::{DateTime, NaiveDateTime, Utc};
use std::error::Error;


use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
// use crate::externals::RiskFactorModelProvider::RiskFactorModelProvider;
use crate::externals::RiskFactorModel::RiskFactorModel;

use crate::functions::pam::pof::{
    POF_FP_PAM::POF_FP_PAM,
    POF_IED_PAM::POF_IED_PAM,
    POF_IP_PAM::POF_IP_PAM,
    POF_IPCI_PAM::POF_IPCI_PAM,
    POF_MD_PAM::POF_MD_PAM,
    POF_PRD_PAM::POF_PRD_PAM,
    POF_RR_PAM::POF_RR_PAM,
    POF_SC_PAM::POF_SC_PAM,
    POF_TD_PAM::POF_TD_PAM
};

use crate::functions::pam::stf::{
STF_FP_PAM::STF_FP_PAM,
    STF_IED_PAM::STF_IED_PAM,
    STF_IP_PAM::STF_IP_PAM,
    STF_IPCI_PAM::STF_IPCI_PAM,
    STF_MD_PAM::STF_MD_PAM,
    STF_PRD_PAM::STF_PRD_PAM,
    STF_RR_PAM::STF_RR_PAM,
    STF_RRF_PAM::STF_RRF_PAM,
    STF_SC_PAM::STF_SC_PAM,
    STF_TD_PAM::STF_TD_PAM,
};

use crate::state_space::StateSpace::StateSpace;
use crate::types::isoDatetime::IsoDatetime;
use crate::algorithmic::ScheduleFactory::ScheduleFactory;
use crate::util::CommonUtils::CommonUtils;
use crate::attributes::ContractModel::ContractModel;

/// Represents the Principal At Maturity payoff algorithm
pub struct PrincipalAtMaturity;

impl PrincipalAtMaturity {
    /// Compute next events within the period up to `to` date based on the contract model
    pub fn schedule(
        to: &IsoDatetime,
        model: &ContractModel,
    ) -> Result<Vec<ContractEvent>, Box<dyn Error>> {
        let mut events: Vec<ContractEvent> = Vec::new();

        // Initial exchange (IED)
        events.push(EventFactory::create_event(
            *model.initialExchangeDate,
            EventType::IED,
            *model.Currency,
            POF_IED_PAM,
            STF_IED_PAM,
            *model.ContractID,
        ));

        // Principal redemption (MD)
        events.push(EventFactory::create_event(
            model.MaturityDate,
            EventType::MD,
            model.Currency,
            POF_MD_PAM,
            STF_MD_PAM,
            model.ContractID,
        ));

        // Purchase (PRD)
        if !CommonUtils::is_null(&model.PurchaseDate) {
            events.push(EventFactory::create_event(
                model.PurchaseDate,
                EventType::PRD,
                model.Currency,
                POF_PRD_PAM,
                STF_PRD_PAM,
                model.ContractID,
            ));
        }

        // Interest payment related events
        if !CommonUtils::is_null(model.NominalInterestRate)
            && (!CommonUtils::is_null(model.CycleOfInterestPayment)
                || !CommonUtils::is_null(model.CycleAnchorDateOfInterestPayment))
        {
            // Generate raw interest payment events (IP)
            let mut interest_events = EventFactory::create_events_with_convention(
                ScheduleFactory::create_schedule(
                    model.CycleAnchorDateOfInterestPayment,
                    model.MaturityDate,
                    model.CycleOfInterestPayment,
                    model.EndOfMonthConvention,
                    true,
                )?,
                EventType::IP,
                model.Currency,
                POF_IP_PAM,
                STF_IP_PAM,
                &model.BusinessDayConvention,
                model.ContractID,
            );

            // Adapt if interest capitalization is set
            if !CommonUtils::is_null(model.CapitalizationEndDate) {
                // Remove IP events at IPCED and add IPCI event instead
                let capitalization_end = EventFactory::create_event_with_convention(
                    *model.CapitalizationEndDate,
                    EventType::IPCI,
                    model.Currency,
                    POF_IPCI_PAM,
                    STF_IPCI_PAM,
                    &model.BusinessDayConvention,
                    model.ContractID,
                );

                // Remove IP events that occur at capitalization end date
                interest_events.retain(|e| {
                    !(e.event_type() == EventType::IP && e.event_time() == capitalization_end.event_time())
                });

                // Add capitalization end event
                interest_events.push(capitalization_end.clone());

                // Change events with time <= IPCED and cont_type IP to IPCI
                for e in interest_events.iter_mut() {
                    if e.event_type() == EventType::IP
                        && e.event_time() <= capitalization_end.event_time()
                    {
                        e.set_event_type(EventType::IPCI);
                        e.set_f_payoff(POF_IPCI_PAM);
                        e.set_f_state_trans(STF_IPCI_PAM);
                    }
                }
            }

            events.extend(interest_events);
        } else if !CommonUtils::is_null(model.CapitalizationEndDate) {
            // If no interest schedule set but capitalization end date, add single IPCI event
            events.push(EventFactory::create_event_with_convention(
                model.CapitalizationEndDate,
                EventType::IPCI,
                model.Currency,
                POF_IPCI_PAM,
                STF_IPCI_PAM,
                &model.BusinessDayConvention,
                model.ContractID,
            ));
        }

        // Rate reset events (RR)
        let mut rate_reset_events = EventFactory::create_event_with_convention(
            ScheduleFactory::create_schedule(
                model.CycleAnchorDateOfRateReset,
                model.MaturityDate,
                model.CycleOfRateReset,
                model.EndOfMonthConvention,
                false,
            )?,
            EventType::RR,
            model.Currency,
            POF_RR_PAM,
            STF_RR_PAM,
            &model.BusinessDayConvention,
            model.ContractID,
        )?;

        // Adapt fixed rate reset event
        if !CommonUtils::is_null(model.NextResetRate) {
            let status_date = model.StatusDate;
            let status_event = EventFactory::create_event(
                status_date,
                EventType::AD,
                model.Currency,
                None,
                None,
                model.ContractID,
            );
            rate_reset_events.sort();
            if let Some(fixed_event) = rate_reset_events
                .iter_mut()
                .filter(|e| **e > status_event)
                .next()
            {
                fixed_event.set_f_state_trans(STF_RRF_PAM);
                fixed_event.set_event_type(EventType::RRF);
            }
        }

        // Add all rate reset events
        events.extend(rate_reset_events);

        // Fee payment events (FP), if specified
        if !CommonUtils::is_null(model.CycleOfFee) {
            let fee_events = EventFactory::create_event_with_convention(
                ScheduleFactory::create_schedule(
                    model.CycleAnchorDateOfFee,
                    model.MaturityDate,
                    model.CycleOfFee,
                    model.EndOfMonthConvention,
                    true,
                )?,
                EventType::FP,
                model.Currency,
                POF_FP_PAM,
                STF_FP_PAM,
                &model.BusinessDayConvention,
                model.ContractID,
            )?;
            events.extend(fee_events);
        }

        // Scaling events (SC), if specified
        let scaling_effect: String = model.ScalingEffect.to_string();
        if !CommonUtils::is_null(&scaling_effect)
            && (scaling_effect.contains('I') || scaling_effect.contains('N'))
        {
            let scaling_events = EventFactory::create_event_with_convention(
                ScheduleFactory::create_schedule(
                    model.CycleAnchorDateOfScalingIndex,
                    model.MaturityDate,
                    model.CycleOfScalingIndex,
                    model.EndOfMonthConvention,
                    false,
                )?,
                EventType::SC,
                model.Currency,
                POF_SC_PAM,
                STF_SC_PAM,
                &model.BusinessDayConvention,
                model.ContractID,
            )?;
            events.extend(scaling_events);
        }

        // Termination event (TD)
        if !CommonUtils::is_null(model.TerminationDate) {
            let termination = EventFactory::create_event(
                model.TerminationDate,
                EventType::TD,
                model.Currency,
                POF_TD_PAM,
                STF_TD_PAM,
                model.ContractID,
            );

            // Remove all events occurring after termination date
            events.retain(|e| e <= &termination);
            events.push(termination);
        }

        // Remove all pre-status date events
        let status_date = model.StatusDate;
        let status_event = EventFactory::create_event(
            status_date,
            EventType::AD,
            model.Currency,
            None,
            None,
            model.ContractID,
        );
        events.retain(|e| e >= &status_event);

        // Remove all events after the `to` date
        let to_event = EventFactory::create_event(
            to,
            EventType::AD,
            model.Currency,
            None,
            None,
            model.ContractID,
        );
        events.retain(|e| e <= &to_event);

        // Sort events according to their time of occurrence
        events.sort();

        Ok(events)
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    pub fn apply(
        events: &mut Vec<ContractEvent>,
        model: &ContractModel,
        observer: &RiskFactorModel,
    ) -> Vec<ContractEvent> {
        // Initialize state space per status date
        let mut states = Self::init_StateSpace(model)?;

        // Sort events according to their time sequence
        events.sort();

        // Apply events according to their time sequence to current state
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                observer,
                model.DayCountConvention,
                model.BusinessDayConvention,
            );
        }

        // Remove pre-purchase events if purchase date is set
        if !CommonUtils::is_null(model.PurchaseDate) {
            let purchase_date = model.PurchaseDate;
            let purchase_event = EventFactory::create_event(
                purchase_date,
                EventType::PRD,
                model.Currency,
                None,
                None,
                model.ContractID,
            );
            events.retain(|e| {
                e.event_type() == EventType::AD || e >= &purchase_event
            });
        }

        // Return evaluated events
        events
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_StateSpace(
        model: &ContractModel,
    ) -> StateSpace {
        let mut states = StateSpace::new();
        

        states.notional_scaling_multiplier = model.NotionalScalingMultiplier;
        states.interest_scaling_multiplier = model.InterestScalingMultiplier;
        states.contract_performance = model.ContractPerformance;
        states.status_date = model.StatusDate;

        let initial_exchange_date: IsoDatetime = model.InitialExchangeDate;
        if initial_exchange_date > states.status_date {
            states.notional_principal = 0.0;
            states.nominal_interest_rate = 0.0;
        } else {
            let role_sign = model.ContractRole.role_sign();
            states.notional_principal = role_sign * model.NotionalPrincipal;
            states.nominal_interest_rate = model.NominalInterestRate;
        }

        // Initialize accrued interest
        if CommonUtils::is_null(model.NominalInterestRate) {
            states.accrued_interest = 0.0;
        } else if !CommonUtils::is_null(model.AccruedInterest) {
            states.accrued_interest = model.AccruedInterest;
        } else {
            let day_counter = model.DayCountConvention;
            let time_adjuster = model.BusinessDayConvention;
            let mut ip_schedule = ScheduleFactory::create_schedule(
                model.CycleAnchorDateOfInterestPayment,
                model.MaturityDate,
                model.CycleOfInterestPayment,
                model.EndOfMonthConvention,
                true,
            )?;

            ip_schedule.sort();
            let dates_earlier_than_status: Vec<&NaiveDateTime> = ip_schedule
                .iter()
                .filter(|&&date| date < states.status_date)
                .collect();

            if let Some(&t_minus) = dates_earlier_than_status.last() {
                let adjusted_t_minus = time_adjuster.shift_calc_time(t_minus);
                let adjusted_status_date = time_adjuster.shift_calc_time(states.status_date);
                let day_fraction = day_counter.day_count_fraction(adjusted_t_minus, adjusted_status_date);
                states.accrued_interest = day_fraction * states.notional_principal * states.nominal_interest_rate;
            }
        }

        // Initialize fee accrued
        if CommonUtils::is_null(model.FeeRate) {
            states.fee_accrued = 0.0;
        } else if !CommonUtils::is_null(model.FeeAccrued) {
            states.fee_accrued = model.FeeAccrued;
        }
        // TODO: Implement last two possible initializations if needed

        // Return the initialized state space
        states
    }
}
