use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use gfs_lib_terms::non_terms::EndTime::EndTime;
use gfs_lib_terms::non_terms::EventTime::EventTime;
use gfs_lib_terms::non_terms::PayOff::PayOff;
use gfs_lib_terms::non_terms::ScheduleTime::ScheduleTime;
use gfs_lib_terms::non_terms::ScheduleFactoryStartTime::StartTime;
use gfs_lib_terms::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventType::EventType;

use crate::states_space::StatesSpace::StatesSpace;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_fees::FeeAccrued::FeeAccrued;
use gfs_lib_terms::terms::grp_interest::AccruedInterest::AccruedInterest;
use gfs_lib_terms::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use gfs_lib_terms::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::CycleOfRateReset::CycleOfRateReset;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;

use crate::time::ScheduleFactory::ScheduleFactory;
use crate::attributes::ContractTerms::ContractTerms;
use gfs_lib_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use gfs_lib_terms::terms::grp_calendar::Calendar::Calendar;
use gfs_lib_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::ContractRole::ContractRole;
use gfs_lib_terms::terms::grp_contract_identification::ContractType::ContractType;
use gfs_lib_terms::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use gfs_lib_terms::terms::grp_counterparty::ContractPerformance::ContractPerformance;
use gfs_lib_terms::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use gfs_lib_terms::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use gfs_lib_terms::terms::grp_fees::CycleOfFee::CycleOfFee;
use gfs_lib_terms::terms::grp_fees::FeeBasis::FeeBasis;
use gfs_lib_terms::terms::grp_fees::FeeRate::FeeRate;
use gfs_lib_terms::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use gfs_lib_terms::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_notional_principal::Currency::Currency;
use gfs_lib_terms::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use gfs_lib_terms::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::MarketObjectCodeOfScalingIndex::MarketObjectCodeOfScalingIndex;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::terms::grp_notional_principal::NotionalScalingMultiplier::NotionalScalingMultiplier;
use gfs_lib_terms::terms::grp_notional_principal::PremiumDiscountAtIED::PremiumDiscountAtIED;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtPurchaseDate::PriceAtPurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::PriceAtTerminationDate::PriceAtTerminationDate;
use gfs_lib_terms::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use gfs_lib_terms::terms::grp_notional_principal::ScalingEffect::ScalingEffect;
use gfs_lib_terms::terms::grp_notional_principal::ScalingIndexAtContractDealDate::ScalingIndexAtContractDealDate;
use gfs_lib_terms::terms::grp_notional_principal::TerminationDate::TerminationDate;
use gfs_lib_terms::terms::grp_optionality::CycleAnchorDateOfOptionality::CycleAnchorDateOfOptionality;
use gfs_lib_terms::terms::grp_optionality::CycleOfOptionality::CycleOfOptionality;
use gfs_lib_terms::terms::grp_optionality::ObjectCodeOfPrepaymentModel::ObjectCodeOfPrepaymentModel;
use gfs_lib_terms::terms::grp_optionality::PenaltyRate::PenaltyRate;
use gfs_lib_terms::terms::grp_optionality::PenaltyType::PenaltyType;
use gfs_lib_terms::terms::grp_reset_rate::CyclePointOfRateReset::CyclePointOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::FixingPeriod::FixingPeriod;
use gfs_lib_terms::terms::grp_reset_rate::LifeCap::LifeCap;
use gfs_lib_terms::terms::grp_reset_rate::LifeFloor::LifeFloor;
use gfs_lib_terms::terms::grp_reset_rate::MarketObjectCodeOfRateReset::MarketObjectCodeOfRateReset;
use gfs_lib_terms::terms::grp_reset_rate::NextResetRate::NextResetRate;
use gfs_lib_terms::terms::grp_reset_rate::PeriodCap::PeriodCap;
use gfs_lib_terms::terms::grp_reset_rate::PeriodFloor::PeriodFloor;
use gfs_lib_terms::terms::grp_reset_rate::RateMultiplier::RateMultiplier;
use gfs_lib_terms::terms::grp_reset_rate::RateSpread::RateSpread;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_types::traits::TraitConvert::{IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::traits::TraitConvert::IsoCycleConvertToOption;
use gfs_lib_types::types::IsoDatetime::IsoDatetime;
use gfs_lib_types::types::Value::Value;
use crate::traits::TraitContractModel::TraitContractModel;
use crate::attributes::RelatedContracts::RelatedContracts;
// use crate::contracts::Swaps::SWAPS;
use crate::events::EventFactory::EventFactory;
use crate::events::EventSequence::EventSequence;
use crate::functions::PayOffFunction::PayOffFunction;
use crate::functions::StatesTransitionFunction::StatesTransitionFunction;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitExternalEvent::TraitExternalEvent;
use crate::util::ResultsStruct::TestResult;

pub struct PAM {
    pub contract_id: ContractID,
    pub contract_terms: ContractTerms,
    pub risk_factor_external_data: Option<Arc<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>,
    pub related_contracts: Option<RelatedContracts>,
    pub event_timeline: Vec<ContractEvent>, //Vec<ContractEvent>, ScheduleTime doit être plus précis qu'event time
    pub curr_event_index: i32,
    pub states_space: StatesSpace,
    pub status_date: Option<StatusDate>,
    pub first_event_date: Option<PhantomIsoDatetimeW>,
    pub last_event_date: Option<PhantomIsoDatetimeW>,
}

impl TraitContractModel for PAM { //

    fn new() -> Self {
        Self {
            contract_id: ContractID::new("init".to_string()).expect("init contract ID"),
            contract_terms: ContractTerms::default(),
            risk_factor_external_data: None,
            risk_factor_external_event: None,
            related_contracts: None,
            event_timeline: Vec::new(),
            curr_event_index: -1,
            states_space: StatesSpace::default(),
            status_date: None,
            first_event_date: None,
            last_event_date: None,
        }
    }

    fn init_contract_terms(&mut self, sm: HashMap<String, Value>) {
        //let mut cm = ContractModel::init();
        let maturity_date_tmp = MaturityDate::provide_from_input_dict(&sm, "maturityDate");
        let maturity_date = if let Some(a) = maturity_date_tmp {
            Some(Rc::new(a))
        } else {
            None
        };
        let calendar = Calendar::provide_rc(&sm, "calendar");

        let business_day_adjuster: Option<BusinessDayAdjuster> = {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                &sm,
                "businessDayConvention",
                calendar_clone.expect("te")
            )
        };

        let day_count_convention = if let Some(maturity_date) = &maturity_date {
            DayCountConvention::provide_from_input_dict(
                &sm,
                "dayCountConvention",
                Some(Rc::clone(maturity_date)),
                Some(Rc::clone(&calendar))
            )
        } else {
            None
        };

        //map.put("cycleAnchorDateOfRateReset", (CommonUtils.isNull(attributes.get("cycleAnchorDateOfRateReset"))) ?
        //  ((CommonUtils.isNull(attributes.get("cycleOfRateReset"))) ? null : LocalDateTime.parse(attributes.get("initialExchangeDate"))) : LocalDateTime.parse(attributes.get("cycleAnchorDateOfRateReset")));

        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(&sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_resetxx = CycleAnchorDateOfRateReset::provide_from_input_dict(&sm, "cycleAnchorDateOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_anchor_date_of_rate_resetxx.is_none() {
            if cycle_of_rate_reset.is_none() {
                None
            } else {
                let a = InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate").unwrap().value().to_string();
                CycleAnchorDateOfRateReset::from_str(&a).ok()
            }
        } else {
            cycle_anchor_date_of_rate_resetxx
        };
        // une valeur par default non specifier dans la norme mais dans la base de code
        let mut accrued_interest = AccruedInterest::provide_from_input_dict(&sm, "accruedInterest");
        if accrued_interest.is_none() {
            accrued_interest = AccruedInterest::new(0.0).ok();
        }

        let mut fee_rate = FeeRate::provide_from_input_dict(&sm, "feeRate");
        if fee_rate.is_none() {
            fee_rate = FeeRate::new(0.0).ok();
        }

        let eomc = EndOfMonthConvention::provide_from_input_dict(&sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else { eomc.unwrap() };

        let mut rate_multiplier = RateMultiplier::provide_from_input_dict(&sm, "rateMultiplier");
        if rate_multiplier.is_none() {
            rate_multiplier = RateMultiplier::new(1.0).ok();
        }

        let mut notional_scaling_multiplier = NotionalScalingMultiplier::provide_from_input_dict(&sm, "notionalScalingMultiplier");
        if notional_scaling_multiplier.is_none() {
            notional_scaling_multiplier = NotionalScalingMultiplier::new(1.0).ok();
        }

        let mut interest_scaling_multiplier = InterestScalingMultiplier::provide_from_input_dict(&sm, "interestScalingMultiplier");
        if interest_scaling_multiplier.is_none() {
            interest_scaling_multiplier = InterestScalingMultiplier::new(1.0).ok();
        }

        // Life cap
        let mut life_cap = LifeCap::provide_from_input_dict(&sm, "lifeCap");
        if life_cap.is_none() {
            life_cap = LifeCap::new(f64::INFINITY).ok();
        }
        // Life floor
        let mut life_floor = LifeFloor::provide_from_input_dict(&sm, "lifeFloor");
        if life_floor.is_none() {
            life_floor = LifeFloor::new(f64::NEG_INFINITY).ok();
        }
        // PeriodCap
        let mut period_cap = PeriodCap::provide_from_input_dict(&sm, "periodCap");
        if period_cap.is_none() {
            period_cap = PeriodCap::new(f64::INFINITY).ok();
        }
        // PeriodFloor
        let mut period_floor = PeriodFloor::provide_from_input_dict(&sm, "periodFloor");
        if period_floor.is_none() {
            period_floor = PeriodFloor::new(f64::NEG_INFINITY).ok();
        }


        let contract_id = ContractID::provide_from_input_dict(&sm, "contractID");
        self.contract_id =  contract_id.clone().expect("contract ID not provided");


        let status_date = StatusDate::provide_from_input_dict(&sm, "statusDate");
        let ct = ContractTerms {
            accrued_interest: accrued_interest,
            business_day_adjuster: business_day_adjuster,
            calendar: calendar,
            capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(&sm, "capitalizationEndDate"),
            contract_id: contract_id,
            contract_performance: ContractPerformance::provide_from_input_dict(&sm, "contractPerformance"),
            contract_role: ContractRole::provide_from_input_dict(&sm, "contractRole"),
            contract_type: ContractType::provide_from_input_dict(&sm, "contractType"),
            counterparty_id: CounterpartyID::provide_from_input_dict(&sm, "CounterpartyID"),
            currency: Currency::provide_from_input_dict(&sm, "currency"),
            cycle_anchor_date_of_fee: CycleAnchorDateOfFee::provide_from_input_dict(&sm, "cycleAnchorDateOfFee"),
            cycle_anchor_date_of_interest_payment: CycleAnchorDateOfInterestPayment::provide_from_input_dict(&sm, "cycleAnchorDateOfInterestPayment"),
            cycle_anchor_date_of_optionality: CycleAnchorDateOfOptionality::provide_from_input_dict(&sm, "cycleAnchorDateOfOptionality"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_anchor_date_of_scaling_index: CycleAnchorDateOfScalingIndex::provide_from_input_dict(&sm, "cycleAnchorDateOfScalingIndex"),
            cycle_of_fee: CycleOfFee::provide_from_input_dict(&sm, "cycleOfFee"),
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(&sm, "cycleOfInterestPayment"),
            cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(&sm, "cycleOfOptionality"),
            cycle_of_rate_reset: cycle_of_rate_reset,
            cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(&sm, "cycleOfScalingIndex"),
            cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(&sm, "cyclePointOfInterestPayment"),
            cycle_point_of_rate_reset: CyclePointOfRateReset::provide_from_input_dict(&sm, "cyclePointOfRateReset"),
            day_count_convention: day_count_convention,
            end_of_month_convention: end_of_month_convention,
            fee_accrued: FeeAccrued::provide_from_input_dict(&sm, "feeAccrued"),
            fee_basis: FeeBasis::provide_from_input_dict(&sm, "feeBasis"),
            fee_rate: fee_rate,
            fixing_period: FixingPeriod::provide_from_input_dict(&sm, "fixingPeriod"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(&sm, "initialExchangeDate"),
            interest_scaling_multiplier: interest_scaling_multiplier,
            life_cap: life_cap,
            life_floor: life_floor,
            market_object_code: MarketObjectCode::provide_from_input_dict(&sm, "marketObjectCode"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(&sm, "marketObjectCodeOfRateReset"),
            market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(&sm, "marketObjectCodeOfScalingIndex"),
            maturity_date: maturity_date,
            next_reset_rate: NextResetRate::provide_from_input_dict(&sm, "nextResetRate"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(&sm, "nominalInterestRate"),
            notional_principal: NotionalPrincipal::provide_from_input_dict(&sm, "notionalPrincipal"),
            notional_scaling_multiplier: notional_scaling_multiplier,
            object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(&sm, "objectCodeOfPrepaymentModel"),
            penalty_rate: PenaltyRate::provide_from_input_dict(&sm, "penaltyRate"),
            penalty_type: PenaltyType::provide_from_input_dict(&sm, "penaltyType"),
            period_cap: period_cap,
            period_floor: period_floor,
            premium_discount_at_ied: PremiumDiscountAtIED::provide_from_input_dict(&sm, "premiumDiscountAtIED"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(&sm, "priceAtPurchaseDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(&sm, "priceAtTerminationDate"),
            purchase_date: PurchaseDate::provide_from_input_dict(&sm, "purchaseDate"),
            rate_multiplier: rate_multiplier,
            rate_spread: RateSpread::provide_from_input_dict(&sm, "rateSpread"),
            scaling_effect: ScalingEffect::provide_from_input_dict(&sm, "scalingEffect"),
            scaling_index_at_contract_deal_date: ScalingIndexAtContractDealDate::provide_from_input_dict(&sm, "scalingIndexAtContractDealDate"),
            status_date: status_date,
            termination_date: TerminationDate::provide_from_input_dict(&sm, "terminationDate"),
            ..Default::default()
        };

        self.contract_terms = ct;
    }

    fn init_risk_factor_external_data(&mut self, risk_factor_external_data: Option<Arc<dyn TraitExternalData>>) {
        self.risk_factor_external_data = risk_factor_external_data;
    }

    fn init_risk_factor_external_event(&mut self, risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>) {
        self.risk_factor_external_event = risk_factor_external_event;
    }

    fn init_related_contracts(&mut self, _sm: HashMap<String, Value>) {
        self.related_contracts = None;
    }

    fn init_status_date(&mut self) {
        self.status_date = self.contract_terms.status_date;
    }
    
    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) { // -> Result<StatesSpace, String>

        let model = &self.contract_terms;

        let mut states = StatesSpace::default();

        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = model.interest_scaling_multiplier.clone();
        states.contract_performance = model.contract_performance;
        states.status_date = model.status_date.clone();

        let initial_exchange_date = model.initial_exchange_date.clone().unwrap();
        if initial_exchange_date.value() > states.status_date.clone().unwrap().value() {
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

            let start_time = model.cycle_anchor_date_of_interest_payment.convert_option::<StartTime>();
            let end_time = model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>();
            let mut ip_schedule: Vec<PhantomIsoDatetimeW> = ScheduleFactory::create_schedule(
                &start_time, // unwrap().to_start_time(),
                &end_time,
                &model.cycle_of_interest_payment.clone().convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(true),
            ).into_iter().collect();

            ip_schedule.sort();

            let sd = PhantomIsoDatetimeW::new(states.status_date.clone().unwrap().value()).expect("d");
            let date_earlier_than_t0: Vec<PhantomIsoDatetimeW> = ip_schedule
                .into_iter()
                .filter(|&date| date < sd)
                .collect();

            let t_minus = date_earlier_than_t0.last();

            states.accrued_interest = AccruedInterest::new(
                day_counter.day_count_fraction(
                    time_adjuster.shift_bd(t_minus.unwrap()),
                    time_adjuster.shift_bd(&states.status_date.clone().unwrap().convert::<PhantomIsoDatetimeW>()))
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

        self.states_space = states
    }

    fn init_contract_event_timeline(&mut self, _to : Option<PhantomIsoDatetimeW>) { // to: Option<IsoDatetime> // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>

        let model = &self.contract_terms;
        let events = &mut self.event_timeline;
        //let mut events: Vec<Box< dyn TraitContractEvent>> = Vec::new();
        //let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let maturity_date = model.maturity_date.clone().unwrap().deref().clone();

        ////////////////////////////
        // Initial exchange (IED) //
        ////////////////////////////
        let e = EventFactory::create_event(
            &model.initial_exchange_date.convert_option::<ScheduleTime>(),
            &EventType::IED,
            &model.currency,
            Some(PayOffFunction::from_str("POF_IED_PAM")),
            Some(StatesTransitionFunction::from_str("STF_IED_PAM")),
            &None,
            &model.contract_id);

        events.push(e);

        ///////////////////////////////
        // Principal redemption (MD) //
        ///////////////////////////////
        let e = EventFactory::create_event(
            &Some(maturity_date.clone().convert::<ScheduleTime>()),
            &EventType::MD,
            &model.currency,
            Some(PayOffFunction::from_str("POF_MD_PAM")),
            Some(StatesTransitionFunction::from_str("STF_MD_PAM")),
            &None,
            &model.contract_id);
        events.push(e);

        ///////////////////////////////
        //       Purchase (PRD)      //
        ///////////////////////////////
        //let aa = model.purchase_date.is_some();
        if model.purchase_date.is_some() {
            //let a = false;
            let e: ContractEvent = EventFactory::create_event(
                & {
                    let tmp: ScheduleTime = model.purchase_date.unwrap().convert();
                    Some(tmp)
                },
                &EventType::PRD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_PRD_PAM")),
                Some(StatesTransitionFunction::from_str("STF_PRD_PAM")),
                &None,
                &model.contract_id);
            events.push(e);
        }

        /////////////////////////////////////
        // Interest payment related events //
        /////////////////////////////////////
        if model.nominal_interest_rate.is_some() &&
            (model.cycle_of_interest_payment.is_some() ||
                model.cycle_anchor_date_of_interest_payment.is_some()) {

            // Generate raw interest payment events (IP)
            let sd = model.cycle_anchor_date_of_interest_payment.convert_option::<StartTime>();
            let ed = Some(maturity_date.clone().convert::<EndTime>());
            let cy = model.cycle_of_interest_payment.convert_option::<PhantomIsoCycleW>();
            let z = ScheduleFactory::create_schedule(
                &sd,
                &ed,
                &cy,
                &model.end_of_month_convention,
                Some(true));

            let x: HashSet<ScheduleTime> = z.iter().map(|e| e.convert::<ScheduleTime>()).collect();

            let mut interest_events = EventFactory::create_events(
                &x,
                &EventType::IP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_IP_PAM")),
                Some(StatesTransitionFunction::from_str("STF_IP_PAM")),
                &model.business_day_adjuster,
                &model.contract_id);


            let mut vec: Vec<ContractEvent> = vec![];
            // Adapt if interest capitalization is set
            if model.capitalization_end_date.is_some() {
                // Remove IP events at IPCED and add IPCI event instead
                let a = model.capitalization_end_date.clone().unwrap();
                let b = a.convert::<ScheduleTime>();
                // let c: PhantomIsoDatetimeW = PhantomIsoDatetimeW::from_str(b.to_string().as_str()).ok().unwrap();

                let capitalization_end = EventFactory::create_event(
                    &Some(b),
                    &EventType::IPCI,
                    &model.currency,
                    Some(PayOffFunction::from_str("POF_IPCI_PAM")),
                    Some(StatesTransitionFunction::from_str("STF_IPCI_PAM")),
                    &model.business_day_adjuster,
                    &model.contract_id);

                // Remove IP events that occur at capitalization end date
                // interest_events.retain(|e| {
                //     e.event_type != EventType::IP || e.event_time != Some(
                //         capitalization_end.get_event_time()
                //     )
                // }); // A REVOIR
                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP) || e.compare_to(&capitalization_end) != 0
                });

                // Add capitalization end event
                interest_events.insert(capitalization_end.clone());
                vec = interest_events.clone().into_iter().collect();
                // Change events with time <= IPCED and cont_type IP to IPCI


                vec.iter_mut().for_each(|e| {
                    if e.event_type == EventType::IP &&
                        e.compare_to(&capitalization_end) != 1 {
                        e.chg_event_type(EventType::IPCI);
                        e.set_f_pay_off(Some(PayOffFunction::from_str("POF_IPCI_PAM")));
                        e.set_f_state_trans(Some(StatesTransitionFunction::from_str("STF_IPCI_PAM")));
                    }
                });
            }
            //let w: Vec<ContractEvent> = interest_events.into_iter().map(|ce| ce).collect();
            // ATTENTION ICI A REVOIR
            if !vec.is_empty() {
                for el in vec.into_iter() {
                    events.push(el);
                }
            }
            else {
                events.extend(interest_events);
            }

            //events.extend(w);
        }
        else if model.capitalization_end_date.is_some() {
            // If no interest schedule set but capitalization end date, add single IPCI event
            let a: ContractEvent = EventFactory::create_event( // lannotation est peut etre fausse a verifier
                       &model.capitalization_end_date.convert_option::<ScheduleTime>(),
                       &EventType::IPCI,
                       &model.currency,
                       Some(PayOffFunction::from_str("POF_IPCI_PAM")),
                       Some(StatesTransitionFunction::from_str("STF_IPCI_PAM")),
                       &model.business_day_adjuster,
                       &model.contract_id);
            events.push(a);
        }


        ////////////////////////////
        // Rate reset events (RR) //
        ////////////////////////////

        let start_time: Option<StartTime> = model.cycle_anchor_date_of_rate_reset.convert_option();
        let end_time: Option<EndTime> = Some(maturity_date.convert());
        // let cycle = model.cycle_of_rate_reset.convert_option::<PhantomIsoCycleW>();
        let a = &ScheduleFactory::create_schedule(
            &start_time,
            &end_time,
            &model.cycle_of_rate_reset.convert_option::<PhantomIsoCycleW>(),
            &model.end_of_month_convention,
            Some(false),
        );

        let x: HashSet<ScheduleTime> = a.iter().map(|e| e.convert::<ScheduleTime>()).collect();

        let mut rate_reset_events = EventFactory::create_events(
            &x,
            &EventType::RR,
            &model.currency,
            Some(PayOffFunction::from_str("POF_RR_PAM")),
            Some(StatesTransitionFunction::from_str("STF_RR_PAM")),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        // Adapt fixed rate reset event
        if model.next_reset_rate.is_some() {
            let status_event = EventFactory::create_event(
                &model.status_date.convert_option::<ScheduleTime>(),
                &EventType::AD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );
            let mut vec: Vec<_> = rate_reset_events.clone().into_iter().collect();
            vec.sort();
            let fixed_event = vec.iter_mut().filter(|e| e.compare_to(&status_event) == 1).next();

            if let Some(fixed_event_val) = fixed_event {
                fixed_event_val.set_f_state_trans(Some(StatesTransitionFunction::from_str("STF_RRF_PAM")));
                fixed_event_val.chg_event_type(EventType::RRF);
                rate_reset_events.insert(fixed_event_val.clone());
            }
        }

        // Add all rate reset events
        //events.extend(rate_reset_events);

        let w: Vec<ContractEvent>
            = rate_reset_events.into_iter().map(|ce| ce).collect();
        for el in w.into_iter() {
            events.push(el);
        }

        ///////////////////////////////////////////
        // Fee payment events (FP), if specified //
        ///////////////////////////////////////////

        if model.cycle_of_fee.is_some() {
            let q = &ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_fee.convert_option::<StartTime>(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>(),
                &model.cycle_of_fee.clone().convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(true),
            );

            let x: HashSet<ScheduleTime> = q.iter().map(|e| e.convert::<ScheduleTime>()).collect();
            let fee_events = EventFactory::create_events(
                &x,
                &EventType::FP,
                &model.currency,
                Some(PayOffFunction::from_str("POF_FP_PAM")),
                Some(StatesTransitionFunction::from_str("STF_FP_PAM")),
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
            let q = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_scaling_index.convert_option::<StartTime>(),
                &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>(),
                &model.cycle_of_scaling_index.clone().convert_option::<PhantomIsoCycleW>(),
                &model.end_of_month_convention,
                Some(false),
            );
            let x: HashSet<ScheduleTime> = q.iter().map(|e| e.convert::<ScheduleTime>()).collect();
            let scaling_events = EventFactory::create_events(
                &x,
                &EventType::SC,
                &model.currency,
                Some(PayOffFunction::from_str("POF_SC_PAM")),
                Some(StatesTransitionFunction::from_str("STF_SC_PAM")),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.extend(scaling_events);
        }

        ////////////////////////////
        // Termination event (TD) //
        ////////////////////////////
        if model.termination_date.is_some() {
            let termination: ContractEvent = EventFactory::create_event(
                &model.termination_date.convert_option::<ScheduleTime>(),
                &EventType::TD,
                &model.currency,
                Some(PayOffFunction::from_str("POF_TD_PAM")),
                Some(StatesTransitionFunction::from_str("STF_TD_PAM")),
                &None,
                &model.contract_id,
            );

            // Remove all events occurring after termination date
            events.retain(|e| e.compare_to(&termination) != 1);
            events.push(termination);
        }

        events.sort();
        ///////////////////////////////////////
        // Remove all pre-status date events //
        ///////////////////////////////////////
        let status_date = model.status_date.clone().unwrap();
        // let w = status_date.to_string();
        let status_event: ContractEvent = EventFactory::create_event(
            &Some(status_date.convert::<ScheduleTime>()),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id);
        events.retain(|e| e.compare_to(&status_event) != -1);

        ///////////////////////////////////////////
        // Remove all events after the `to` date //
        ///////////////////////////////////////////
        let to = &model.maturity_date.clone().map(|rc| (*rc).clone()).convert_option::<EndTime>().convert_option::<ScheduleTime>();
        //let a = to.unwrap().to_string();
        let to_event: ContractEvent = EventFactory::create_event(
            &to,
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );
        events.retain(|e| e.compare_to(&to_event) != 1);

        ///////////////////////////////////////////////////////
        // Sort events according to their time of occurrence //
        ///////////////////////////////////////////////////////
        events.sort();
        // let fxx1 = events.get(1).unwrap().event_time.unwrap().to_string();
        // let fxx2 = events.get(2).unwrap().event_time.unwrap().to_string();
        // let fxx3 = events.get(3).unwrap().event_time.unwrap().to_string();
        // let fxx4 = events.get(4).unwrap().event_time.unwrap().to_string();
        self.event_timeline = events.clone();
        self.first_event_date = self.event_timeline.first().unwrap().event_time.convert_option::<PhantomIsoDatetimeW>();
        self.last_event_date = self.event_timeline.last().unwrap().event_time.convert_option::<PhantomIsoDatetimeW>();

    }

    fn set_status_date(&mut self, status_date: Option<StatusDate>) {
        self.status_date = status_date;
    }

    fn eval_pof_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.event_timeline.get(id_ce).expect("ca marche forcement");

        if curr_ce.fpayoff.is_some() {
            let a = curr_ce.fpayoff.clone().unwrap().eval(
                &curr_ce.get_schedule_time().convert::<PhantomIsoDatetimeW>(),
                &self.states_space,
                &self.contract_terms,
                &self.related_contracts,
                &self.risk_factor_external_data,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            );
            //println!("{:?}\n", a);
            self.event_timeline[id_ce].payoff = Some(a.expect("ok"));
            //println!("payoff0{:?}\n", self.event_timeline[id_ce].payoff);
        }

        // on peut la retravailler pour etre plus direct et efficace
    }

    fn eval_stf_contract_event(&mut self, id_ce: usize) {
        let curr_ce = self.event_timeline.get(id_ce).expect("ca marche forcement");

        if curr_ce.fstate.is_some() {
            curr_ce.fstate.clone().unwrap().eval(
                &curr_ce.get_schedule_time().convert::<PhantomIsoDatetimeW>(),
                &mut self.states_space,
                &self.contract_terms,
                &self.related_contracts,
                &self.risk_factor_external_data,
                &self.contract_terms.day_count_convention,
                &self.contract_terms.business_day_adjuster.clone().unwrap(),
            )
            //self.contract_events[id_ce].payoff = Some(a);
            //let b = curr_ce.set_payoff(a);
            // self.contract_events[id_ce] = a;

        }
        // on peut la retravailler pour etre plus direct et efficace
    }

    fn compute_payoff(&mut self) {
        let id_ce: usize = 0;
        self.eval_pof_contract_event(id_ce);
    }

    fn next_day(&mut self, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> {
        // ici on met Vec<TestResult> car il peut y avoir plusieur event le meme jour
        // itere un jour apres lautre
        let period1d = *PhantomIsoPeriodW::new(0,0,1);
        let next_status_date = self.status_date.convert_option::<PhantomIsoDatetimeW>().unwrap().value()
            + period1d;
        let next_event_index = (self.curr_event_index + 1) as usize;
        let mut next_event_date = self.event_timeline.get(next_event_index).unwrap().get_schedule_time();

        if next_status_date < next_event_date.value() {
            self.status_date = StatusDate::new(next_status_date).ok();
            let oo = self.status_date.clone()?.to_string();
            None
        }
        else { // case >=, seul = doit etre matche
            let mut result_vec: Vec<TestResult> = Vec::new();
            let mut curr_next_event_index = next_event_index;
            while next_status_date == next_event_date.value() {
                let ww = next_status_date.to_string();
                let www = next_event_date.to_string();

                result_vec.push(self.next_event(extract_results).expect("ok").expect("ok"));
                curr_next_event_index += 1;
                if curr_next_event_index == self.event_timeline.len() {
                    break;
                }
                next_event_date = self.event_timeline.get(curr_next_event_index).unwrap().get_schedule_time();
            }
            self.status_date = StatusDate::new(next_status_date).ok();
            Some(Ok(result_vec))
        }

    }

    fn next_event(&mut self, extract_results: bool) -> Option<Result<TestResult, String>> {

        let next_event_index = (self.curr_event_index + 1) as usize;
        if next_event_index < self.event_timeline.len() {

            self.eval_pof_contract_event(next_event_index);
            self.eval_stf_contract_event(next_event_index);

            self.curr_event_index += 1;
            if extract_results == true {
                let curr_testresult = TestResult {
                    eventDate: self.event_timeline[next_event_index].event_time.expect("fe").to_string(),
                    eventType: self.event_timeline[next_event_index].event_type.to_string(),
                    payoff: self.event_timeline[next_event_index].payoff.clone().expect("ok").to_string(),
                    currency: self.event_timeline[next_event_index].currency.clone().expect("ef").0,
                    notionalPrincipal: self.states_space.notional_principal.clone().expect("ok").to_string(),
                    nominalInterestRate: self.states_space.nominal_interest_rate.clone().expect("ok").to_string(),
                    accruedInterest: self.states_space.accrued_interest.clone().expect("ok").to_string(),
                };

                Some(Ok(curr_testresult))
            }
            else {
                Some(Err("Err ave TestResult".to_string()))
            }

        } else {
            None
        }
    }

    fn add_event_to_contract_event_timeline(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        // reflechir a quoi pourrait bien servir reset
        self.contract_terms = ContractTerms::default();
        self.risk_factor_external_data = None;
        self.risk_factor_external_event = None;
        self.related_contracts = None;
        self.event_timeline = Vec::new();
        self.states_space = StatesSpace::default();
        self.status_date = None;
    }

    fn apply_until_date(&mut self, date: Option<PhantomIsoDatetimeW>, extract_results: bool) -> Option<Result<Vec<TestResult>, String>> { // -> Result<Vec<ContractEvent<IsoDatetime, IsoDatetime>>, String>
        self.sort_events_timeline();
        let events_len = self.event_timeline.len();
        let mut result_vec: Vec<TestResult> = Vec::new();

        while self.curr_event_index + 1 < events_len as i32 { // i < events_len {
            if self.curr_event_index > -1 {
                if date.is_some() {
                    if self.event_timeline[self.curr_event_index as usize].event_time.expect("fd") > EventTime::new(date.expect("fo").value()).expect("ok") {
                        break
                    }
                }
            }

            let curr_testresult: Option<Result<TestResult, String>> = self.next_event(extract_results);
            if extract_results == true {
                if curr_testresult.clone().unwrap().is_ok() {
                    result_vec.push(curr_testresult.clone().unwrap().unwrap());
                }
            }
        }


        ////////////////////////////////////////////////////////
        // Remove pre-purchase events if purchase date is set //
        ////////////////////////////////////////////////////////

        if self.contract_terms.purchase_date.is_some() {
            // let purchase_date = model.purchase_date;
            let purchase_event: ContractEvent = EventFactory::create_event(
                &self.contract_terms.purchase_date.convert_option::<ScheduleTime>(),
                &EventType::PRD,
                &self.contract_terms.currency,
                None,
                None,
                &None,
                &self.contract_terms.contract_id,
            );
            self.event_timeline.retain(|e| {
                e.get_event_type() == EventType::AD || e >= &purchase_event
            });
        }
        /////////////////////////////
        // Return evaluated events //
        /////////////////////////////
        //Ok(events)
        // self.event_timeline = events.clone();

        // recup des resultats
        if extract_results == false {

            None
        }
        else {
            ////////////////////////////////////////////////////////
            // Remove pre-purchase events if purchase date is set //
            ////////////////////////////////////////////////////////
            result_vec.retain(|e| {
                if self.contract_terms.purchase_date.is_some() {
                    let purchase_event: ContractEvent = EventFactory::create_event(
                        &self.contract_terms.purchase_date.convert_option::<ScheduleTime>(),
                        &EventType::PRD,
                        &self.contract_terms.currency,
                        None,
                        None,
                        &None,
                        &self.contract_terms.contract_id,
                    );
                    let epoch_millis = IsoDatetime::from_str(e.eventDate.as_str()).clone().unwrap().value().and_utc().timestamp_millis(); //.and_utc().timestamp_millis();
                    let epoch_offset = epoch_millis + EventSequence::time_offset(&EventType::from_str(e.eventType.as_str()).expect("exist"));
                    EventType::from_str(e.eventType.as_str()).expect("exist") == EventType::AD || epoch_offset as f64 >= purchase_event.epoch_offset.unwrap().value()
                } else { true }
            });
            Some(Ok(result_vec))
        }



    }

    fn sort_events_timeline(&mut self) {
        self.event_timeline.sort_by(|a, b| a.epoch_offset.partial_cmp(&b.epoch_offset).unwrap_or(Ordering::Less));
    }

}



impl fmt::Display for PAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PAM")
    }
}


impl fmt::Debug for PAM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PAM")
            .field("contract_id", &self.contract_id)
            .field("contract_terms", &self.contract_terms)
            .field("event_timeline", &self.event_timeline)
            .field("states_space", &self.states_space)
            .field("status_date", &self.status_date)
            .finish()
    }
}

impl Clone for PAM {
    fn clone(&self) -> Self {
        PAM {
            contract_id: self.contract_id.clone(),
            contract_terms: self.contract_terms.clone(),
            risk_factor_external_data: None, // faire qqchose specifique ici ?
            risk_factor_external_event: None, // faire qqchose specifique ici ?
            related_contracts: None, // faire qqchose specifique ici ?
            event_timeline: self.event_timeline.clone(),
            curr_event_index: self.curr_event_index.clone(),
            states_space: self.states_space.clone(),
            status_date: self.status_date.clone(),
            first_event_date: self.first_event_date.clone(),
            last_event_date: self.last_event_date.clone(),
        }
    }
}

// Implémentation manuelle de PartialEq
impl PartialEq for PAM {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id &&
            self.contract_terms == other.contract_terms
    }
}

impl Eq for PAM {}

impl Hash for PAM {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // ça veut dire que le contract ID doit etre absolument unique
        self.contract_id.hash(state);
    }
}
