use std::collections::HashMap;
use std::fmt;
use std::io::ErrorKind::ResourceBusy;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractReference::ContractReference;
use crate::events::ContractEvent::{ContractEvent};
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;


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
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use crate::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use lib_actus_types::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::IsoDatetime;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractTerms::ContractTerms;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::ContractType::ContractType;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use crate::terms::grp_fees::CycleOfFee::CycleOfFee;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_interest::AccruedInterest2::AccruedInterest2;
use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate2::NominalInterestRate2;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use crate::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
//use crate::events::AnyContractEvent::AnyContractEvent;

use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NextPrincipalRedemptionPayment::NextPrincipalRedemptionPayment;
use crate::terms::grp_notional_principal::NotionalPrincipal2::NotionalPrincipal2;
use crate::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use crate::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use crate::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use crate::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use crate::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use crate::terms::grp_notional_principal::TerminationDate::TerminationDate;
use crate::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use crate::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use crate::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use crate::terms::grp_optionality::PenaltyRate::PenaltyRate;
use crate::terms::grp_optionality::PenaltyType::PenaltyType;
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
use crate::terms::grp_settlement::ExerciseAmount::ExerciseAmount;
use crate::terms::grp_settlement::ExerciseDate::ExerciseDate;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::util::Value::Value;
// use crate::util_tests::essai_load_results::ResultSet;

use crate::attributes::ResultSet::ResultSet;

#[derive(Debug, Clone, PartialEq)]
pub struct PAM {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for PAM { //

    fn new() -> Self {
        Self {
            contract_terms: ContractTerms::default(),
            contract_events: Vec::<ContractEvent<IsoDatetime, IsoDatetime>>::new(),
            contract_risk_factors: None,
            contract_structure: None,
            result_vec_toggle: false,
            result_vec: None,
        }
    }

    fn set_contract_terms(&mut self, sm: &HashMap<String, Value>) {
        //let mut cm = ContractModel::init();
        let maturity_date_tmp = MaturityDate::provide_from_input_dict(sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };
        let calendar = Calendar::provide_rc(sm, "calendar");

        let business_day_adjuster: Option<BusinessDayAdjuster> =  {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "BusinessDayAdjuster",
                calendar_clone.expect("te")
            )
        };

        let day_count_convention = if let Some(maturity_date) = &maturity_date {
            DayCountConvention::provide_from_input_dict(
                sm,
                "dayCountConvention",
                Some(Rc::clone(maturity_date)),
                Some(Rc::clone(&calendar))
            )
        } else {
            None
        };

        //map.put("cycleAnchorDateOfRateReset", (CommonUtils.isNull(attributes.get("cycleAnchorDateOfRateReset"))) ?
        //  ((CommonUtils.isNull(attributes.get("cycleOfRateReset"))) ? null : LocalDateTime.parse(attributes.get("initialExchangeDate"))) : LocalDateTime.parse(attributes.get("cycleAnchorDateOfRateReset")));

        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_resetxx = CycleAnchorDateOfRateReset::provide_from_input_dict(sm, "cycleAnchorDateOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_resetxx.is_none() {
            if cycle_of_rate_reset.is_none() {
                None
            }
            else {
                let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
                CycleAnchorDateOfRateReset::from_str(&a).ok()
            }
        } else {
            cycle_anchor_date_of_rate_resetxx
        };
        // une valeur par default non specifier dans la norme mais dans la base de code
        let mut accrued_interest = AccruedInterest::provide_from_input_dict(sm, "accruedInterest");
        if accrued_interest.is_none() {
            accrued_interest = AccruedInterest::new(0.0).ok();
        }

        let mut fee_rate = FeeRate::provide_from_input_dict(sm, "feeRate");
        if fee_rate.is_none() {
            fee_rate = FeeRate::new(0.0).ok();
        }

        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};

        let mut rate_multiplier = RateMultiplier::provide_from_input_dict(sm, "rateMultiplier");
        if rate_multiplier.is_none() {
            rate_multiplier = RateMultiplier::new(1.0).ok();
        }

        let mut notional_scaling_multiplier = NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier");
        if notional_scaling_multiplier.is_none() {
            notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        }

        let mut interest_scaling_multiplier = InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier");
        if interest_scaling_multiplier.is_none() {
            interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();
        }

        let ct = ContractTerms {
            accrued_interest:                       accrued_interest,
            business_day_adjuster:                  business_day_adjuster,
            calendar:                               calendar,
            capitalization_end_date:                CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
            contract_id:                            ContractID::provide_from_input_dict(sm, "contractID"),
            contract_performance:                   ContractPerformance::provide_from_input_dict(sm, "contractPerformance"),
            contract_role:                          ContractRole::provide_from_input_dict(sm, "contractRole"),
            contract_type:                          ContractType::provide_from_input_dict(sm, "contractType"),
            counterparty_id:                        CounterpartyID::provide_from_input_dict(sm, "CounterpartyID"),
            currency:                               Currency::provide_from_input_dict(sm, "currency"),
            cycle_anchor_date_of_fee:               CycleAnchorDateOfFee::provide_from_input_dict(sm, "cycleAnchorDateOfFee"),
            cycle_anchor_date_of_interest_payment:  CycleAnchorDateOfInterestPayment::provide_from_input_dict(sm, "cycleAnchorDateOfInterestPayment"),
            cycle_anchor_date_of_optionality:       CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality"),
            cycle_anchor_date_of_rate_reset:        cycle_anchor_date_of_rate_reset,
            cycle_anchor_date_of_scaling_index:     CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex"),
            cycle_of_fee:                           CycleOfFee::provide_from_input_dict(sm, "cycleOfFee"),
            cycle_of_interest_payment:              CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            cycle_of_optionality:                   CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
            cycle_of_rate_reset:                    cycle_of_rate_reset,
            cycle_of_scaling_index:                 CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
            cycle_point_of_interest_payment:        CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
            cycle_point_of_rate_reset:              CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset"),
            day_count_convention:                   day_count_convention,
            end_of_month_convention:                end_of_month_convention,
            fee_accrued:                            FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
            fee_basis:                              FeeBasis::provide_from_input_dict(sm, "feeBasis"),
            fee_rate:                               fee_rate,
            fixing_period:                          FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
            initial_exchange_date:                  InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
            interest_scaling_multiplier:            interest_scaling_multiplier,
            life_cap:                               LifeCap::provide_from_input_dict(sm, "lifeCap"),
            life_floor:                             LifeFloor::provide_from_input_dict(sm, "lifeFloor"),
            market_object_code:                     MarketObjectCode::provide_from_input_dict(sm, "marketObjectCode"),
            market_object_code_of_rate_reset:       MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
            market_object_code_of_scaling_index:    MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
            maturity_date:                          maturity_date,
            next_reset_rate:                        NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
            nominal_interest_rate:                  NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            notional_principal:                     NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            notional_scaling_multiplier:            notional_scaling_multiplier,
            object_code_of_prepayment_model:        ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
            penalty_rate:                           PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
            penalty_type:                           PenaltyType::provide_from_input_dict(sm, "penaltyType"),
            period_cap:                             PeriodCap::provide_from_input_dict(sm, "periodCap"),
            period_floor:                           PeriodFloor::provide_from_input_dict(sm, "periodFloor"),
            premium_discount_at_ied:                PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED"),
            price_at_purchase_date:                 PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            price_at_termination_date:              PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            purchase_date:                          PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            rate_multiplier:                        rate_multiplier,
            rate_spread:                            RateSpread::provide_from_input_dict(sm, "rateSpread"),
            scaling_effect:                         ScalingEffect::provide_from_input_dict(sm, "scalingEffect"),
            scaling_index_at_contract_deal_date:    ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
            status_date:                            StatusDate::provide_from_input_dict(sm, "statusDate"),
            termination_date:                       TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            ..Default::default()
        };

        self.contract_terms = ct;
    }

    fn set_contract_risk_factors(&mut self, risk_factors: &Option<RiskFactorModel>) {
        self.contract_risk_factors = None; // RiskFactorModel::new();
    }

    fn set_contract_structure(&mut self, sm: &HashMap<String, Value>)  {

        self.contract_structure = None;

    }

    fn set_result_vec(&mut self) {
        self.result_vec = Some(Vec::<ResultSet>::new()) //ResultSet::new()
    }
    /// Compute next events within the period up to `to` date based on the contract model
    fn schedule(&mut self, to: Option<IsoDatetime>) { // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>
        let model = &self.contract_terms;
        let events = &mut self.contract_events;
        //let mut events: Vec<Box< dyn TraitContractEvent>> = Vec::new();
        //let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let maturity_date = model.maturity_date.clone().unwrap().deref().clone();

        ////////////////////////////
        // Initial exchange (IED) //
        ////////////////////////////
        let e = EventFactory::<InitialExchangeDate, InitialExchangeDate>::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_PAM)),
            &None,
            &model.contract_id);

        events.push(e.to_iso_datetime_event());

        ///////////////////////////////
        // Principal redemption (MD) //
        /////////////////////////////// 
        let e = EventFactory::<MaturityDate, MaturityDate>::create_event(
            &Some(maturity_date.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_PAM)),
            &None,
            &model.contract_id);
        events.push(e.to_iso_datetime_event());

        ///////////////////////////////
        //       Purchase (PRD)      //
        ///////////////////////////////
        //let aa = model.purchase_date.is_some();
        if model.purchase_date.is_some() {
            //let a = false;
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_PAM)),
                Some(Rc::new(STF_PRD_PAM)),
                &None,
                &model.contract_id);
            events.push(e.to_iso_datetime_event());
        }
        
        /////////////////////////////////////
        // Interest payment related events //
        /////////////////////////////////////
        if model.nominal_interest_rate.is_some() && 
            (model.cycle_of_interest_payment.is_some() || 
            model.cycle_anchor_date_of_interest_payment.is_some()){

            // Generate raw interest payment events (IP)
            let z = &ScheduleFactory::
                <CycleAnchorDateOfInterestPayment, 
                MaturityDate, 
                CycleOfInterestPayment,
                IsoDatetime>::create_schedule(  
                    &model.cycle_anchor_date_of_interest_payment,
                    &Some(maturity_date.clone()),
                    &model.cycle_of_interest_payment,
                    &model.end_of_month_convention,
                    Some(true));

            let mut interest_events = EventFactory::create_events(
                z,
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_PAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id);

            // Adapt if interest capitalization is set
            if model.capitalization_end_date.is_some() {
                // Remove IP events at IPCED and add IPCI event instead
                let a = model.capitalization_end_date.clone().unwrap();
                let b : Option<IsoDatetime> = a.try_into().ok();
                let c : IsoDatetime = IsoDatetime::new(b.unwrap().date(), b.unwrap().time());

                let capitalization_end = EventFactory::create_event(
                    &Some(c),
                    &EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(Rc::new(STF_IPCI_PAM)),
                    &model.business_day_adjuster,
                    &model.contract_id);

                // Remove IP events that occur at capitalization end date
                interest_events.retain(|e| {
                    !(e.event_type != EventType::IP || e.event_time != Some(capitalization_end.get_event_time()))
                }); // A REVOIR

                // Add capitalization end event
                interest_events.insert(capitalization_end.clone() );
                let mut vec: Vec<_> = interest_events.clone().into_iter().collect();
                // Change events with time <= IPCED and cont_type IP to IPCI


                vec.iter_mut()
                    .filter(|e| e.event_type == EventType::IP &&
                        e.get_event_time() <= capitalization_end.get_event_time())
                    .for_each(|e| {
                        e.chg_event_type(EventType::IPCI);
                        e.set_f_pay_off(Some(Rc::new(POF_IPCI_PAM)));
                        e.set_f_state_trans(Some(Rc::new(STF_IPCI_PAM)));
                    });

                // interest_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = vec.into_iter().collect();
                
            }
            let w: Vec<Box<ContractEvent<IsoDatetime, IsoDatetime>>> = interest_events.into_iter().map(|ce| Box::new(ce)).collect();
            for el in w.into_iter(){
                events.push(el.to_iso_datetime_event());
            }
            
            //events.extend(w);
        }
        else if model.capitalization_end_date.is_some() {
            // If no interest schedule set but capitalization end date, add single IPCI event
            let a: ContractEvent<CapitalizationEndDate, CapitalizationEndDate> = EventFactory::create_event( // lannotation est peut etre fausse a verifier
                &model.capitalization_end_date,
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(Rc::new(STF_IPCI_PAM)),
                &model.business_day_adjuster,
                &model.contract_id);

            events.push(a.to_iso_datetime_event());
        }

        ////////////////////////////
        // Rate reset events (RR) //
        ////////////////////////////
        let a = &ScheduleFactory::
                <CycleAnchorDateOfRateReset, 
                MaturityDate,
                CycleOfRateReset,
                IsoDatetime>::create_schedule(
                &model.cycle_anchor_date_of_rate_reset,
                &Some(maturity_date),
                &model.cycle_of_rate_reset,
                &model.end_of_month_convention,
                Some(false),
            );
        
        let mut rate_reset_events = EventFactory::create_events(
            a,
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_PAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::<StatusDate, StatusDate>::create_event(
                &model.status_date,
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );
            let mut vec: Vec<_> = rate_reset_events.clone().into_iter().collect();
            vec.sort();
            let fixed_event = vec.iter_mut().filter(|e| e.compare_to(&status_event.to_iso_datetime_event())  == 1 ).next();

            if let Some(fixed_event_val) = fixed_event {
                fixed_event_val.set_f_state_trans(Some(Rc::new(STF_RRF_PAM)));
                fixed_event_val.chg_event_type(EventType::RRF);
                rate_reset_events.insert(fixed_event_val.clone());
            }
        }

        // Add all rate reset events
        //events.extend(rate_reset_events);

        let w: Vec<Box<ContractEvent<IsoDatetime, IsoDatetime>>> = rate_reset_events.into_iter().map(|ce| Box::new(ce)).collect();
        for el in w.into_iter(){
            events.push(el.to_iso_datetime_event());
        }

        ///////////////////////////////////////////
        // Fee payment events (FP), if specified //
        ///////////////////////////////////////////
        if model.cycle_of_fee.is_some() {
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee,
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &model.cycle_of_fee.clone(),
                    &model.end_of_month_convention,
                    Some(true),
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
        
        ///////////////////////////////////////
        // Scaling events (SC), if specified //
        ///////////////////////////////////////
        if model.scaling_effect.is_some() && 
            (model.scaling_effect.clone().unwrap().to_string().contains('I') || 
             model.scaling_effect.clone().unwrap().to_string().contains('N'))
        {
            let scaling_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_scaling_index,
                    &model.maturity_date.clone().map(|rc| (*rc).clone()),
                    &model.cycle_of_scaling_index.clone(),
                    &model.end_of_month_convention,
                    Some(false),
                ),
                &EventType::SC,
                &model.currency,
                Some(Rc::new(POF_SC_PAM)),
                Some(Rc::new(STF_SC_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(scaling_events);
        }

        ////////////////////////////
        // Termination event (TD) //
        ////////////////////////////
        if model.termination_date.is_some() {
            let termination: ContractEvent<TerminationDate, TerminationDate> = EventFactory::create_event(
                &model.termination_date,
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_PAM)),
                Some(Rc::new(STF_TD_PAM)),
                &None,
                &model.contract_id,
            );

            // Remove all events occurring after termination date
            events.retain(|e| e <= &termination.to_iso_datetime_event());
            events.push(termination.to_iso_datetime_event());
        }

        
        ///////////////////////////////////////
        // Remove all pre-status date events //
        ///////////////////////////////////////
        let status_date = model.status_date.clone().unwrap();
        let status_event : ContractEvent<StatusDate, StatusDate> = EventFactory::create_event(
            &Some(status_date),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id);
        events.retain(|e| e >= &status_event.to_iso_datetime_event());

        ///////////////////////////////////////////
        // Remove all events after the `to` date //
        ///////////////////////////////////////////
        let to_event: ContractEvent<IsoDatetime, IsoDatetime> = EventFactory::create_event(
            &to.clone(),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );
        events.retain(|e| e <= &to_event.to_iso_datetime_event());

        ///////////////////////////////////////////////////////
        // Sort events according to their time of occurrence //
        ///////////////////////////////////////////////////////
        events.sort();

        self.contract_events = events.clone();
    }

    /// Apply a set of events to the current state of a contract and return the post-event states
    fn apply(&mut self, result_set_toogle: bool) { // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>

        // faut pas le mettre apres les borrow immutable ci dessous, lordre compte
        if result_set_toogle == true {
            self.result_vec_toggle = true;
            self.set_result_vec();
        }

        let model = &self.contract_terms;
        let risk_factors = &self.contract_risk_factors;
        let events = &mut self.contract_events.clone();

        ////////////////////////////////////////////
        // Initialize state space per status date //
        ////////////////////////////////////////////
        let _maturity = &model.maturity_date.clone();
        let mut states = &mut self.init_state_space(_maturity).expect("uncorrect state space initialization !");



        let mut events = events.clone();

        //////////////////////////////////////////////////
        // Sort events according to their time sequence //
        //////////////////////////////////////////////////
        events.sort_by(|a, b|
            a.epoch_offset.cmp(&b.epoch_offset));

        ////////////////////////////////////////////////////////////////////
        // Apply events according to their time sequence to current state //
        ////////////////////////////////////////////////////////////////////
        for event in events.iter_mut() {
            event.eval(
                &mut states,
                model,
                &risk_factors.clone().unwrap(),
                &model.day_count_convention.clone(),
                &model.business_day_adjuster.clone().unwrap(),
            );
            if self.result_vec_toggle == true {
                if let Some(rv) = &mut self.result_vec {
                    let mut a = ResultSet::new();
                    a.set_result_set(&states, &event);

                    rv.push(a)
                }
            }
        }

        ////////////////////////////////////////////////////////
        // Remove pre-purchase events if purchase date is set //
        ////////////////////////////////////////////////////////
        if model.purchase_date.is_some() {
            // let purchase_date = model.purchase_date;
            let purchase_event: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &model.purchase_date,
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );
            events.retain(|e| {
                e.get_event_type() == EventType::AD || e >= &purchase_event.to_iso_datetime_event()
            });
        }
        /////////////////////////////
        // Return evaluated events //
        /////////////////////////////
        //Ok(events)
        self.contract_events = events.clone();
    }

    /// Initialize the StateSpace according to the model attributes
    fn init_state_space(&self, _maturity: &Option<Rc<MaturityDate>>) -> Result<StateSpace, String> {

        let model = &self.contract_terms;
        let risk_factors = &self.contract_risk_factors;

        let mut states = StateSpace::default();

        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = model.interest_scaling_multiplier.clone();
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date.clone();

        let initial_exchange_date: IsoDatetime = model.initial_exchange_date.clone().unwrap().value();
        if initial_exchange_date > states.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok();
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok()
        } else {

            let role_sign = model.contract_role.as_ref().map_or(1.0, |a| a.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();
        }

        // Initialize accrued interest
        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = AccruedInterest::new(0.0).ok();
        } else if model.accrued_interest.is_some() {
            states.accrued_interest = model.accrued_interest.clone();
        } else {
            // GERER CE CAS : Il y a UNE ERREUR
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.business_day_adjuster.as_ref().unwrap();


            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment,
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_interest_payment.clone(),
                &model.end_of_month_convention,
                Some(true),
            ).into_iter().collect();

            ip_schedule.sort();

            let sd = states.status_date.clone().unwrap().value();
            let date_earlier_than_t0: Vec<IsoDatetime> = ip_schedule
                .into_iter()
                .filter(|&date| date < sd )
                .collect();

            let t_minus = date_earlier_than_t0.last();

            states.accrued_interest = AccruedInterest::new(
                day_counter.day_count_fraction(
                    time_adjuster.shift_bd(t_minus.unwrap()),
                    time_adjuster.shift_bd(&states.status_date.clone().unwrap().value()))
                * states.notional_principal.clone().unwrap().value()
                * states.nominal_interest_rate.clone().unwrap().value()
                ).ok()
        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();
        } else if model.fee_accrued.is_some() {
            states.fee_accrued = model.fee_accrued.clone();
        }
        // TODO: Implement last two possible initializations if needed

        Ok(states)
    }


}


impl fmt::Display for PAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PAM")
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::{Value as ValueS, Map};
//     use std::fs::File;
//     use std::io::Read;
//     use std::error::Error;
//     use std::collections::HashMap;
//     use crate::exceptions::ContractTypeUnknownException::ContractError;
//     use c>rate::utils::Value::Value;
//     use crate::util_tests::TestsUtils::test_read_and_parse_json;
//     use crate::util_tests::TestsUtils::json_to_dico;

//     fn load_dico_tests() -> Vec<Value> {
//         let pathx = "/home/cet/Projects/ACTUS-CORE/actus-core-master-rust-project-v2/libs/lib_actus_core/tests_sets/actus-tests-pam.json";
//         let json_value = test_read_and_parse_json(pathx).unwrap();
//         let dico_from_json = json_to_dico(json_value);
//         dico_from_json
//     }

//     #[test]
//     fn test_pam_contracts(){
//         let dico_tests = load_dico_tests();

//         //let dico_tests: Vec<HashMap<String, Value>> = vec![load_dico_tests()];
//         for el in dico_tests.iter() {

//             let curr_test = el.as_hashmap().unwrap();

//             let curr_identifier = curr_test.get("identifier").unwrap().as_string();
//             let curr_terms = curr_test.get("terms").unwrap().as_hashmap();
//             let curr_to = curr_test.get("to").unwrap().as_string();
//             let curr_data_observed = curr_test.get("dataObserved").unwrap().as_hashmap(); // verifier si cest None
//             let curr_events_observed = curr_test.get("eventsObserved").unwrap().as_vec();
//             let curr_results = curr_test.get("results").unwrap().as_vec().unwrap();
//             //let a = curr_results.get(0).unwrap().get("notionalPrincipal").unwrap().as_string().unwrap();
//             let to_date = if let Some(curr_to) = curr_to {
//                 IsoDatetime::parse_from_str(&curr_to, "%Y-%m-%dT%H:%M:%S").ok()
//             } else {
//                 None
//             };

//             let mut contract_model: Box<Result<ContractModel, ContractError>> = if let Some(ref curr_terms) = curr_terms {
//                 // Supposons que ContractModel::new retourne Result<ContractModel, String>
//                 match ContractModel::new(&curr_terms) {
//                     Ok(model) => Box::new(Ok(model)),
//                     Err(e) => Box::new(Err(ContractError::from(e))),
//                 }
//             } else {
//                 Box::new(Err(ContractError::MissingTerms))
//             };

//             let risk_factor_model = RiskFactorModel;


//             let mut vec_results: Vec<HashMap<String, Value>> = vec![];
//         }
//         true
//     }
// }