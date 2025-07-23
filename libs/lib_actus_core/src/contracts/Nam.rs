use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;
use crate::events::ContractEvent::ContractEvent;
use crate::events::EventFactory::EventFactory;
use crate::events::EventType::EventType;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use crate::types::IsoDatetime::IsoDatetime;
use crate::attributes::ContractModel::ContractModel;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ResultSet::ResultSet;
use crate::external::RiskFactorModel::RiskFactorModel;
use crate::functions::lam::pof::POF_IP_LAM::POF_IP_LAM;
use crate::functions::lam::pof::POF_IPCB_LAM::POF_IPCB_LAM;
use crate::functions::lam::pof::POF_PRD_LAM::POF_PRD_LAM;
use crate::functions::lam::pof::POF_TD_LAM::POF_TD_LAM;
use crate::functions::lam::stf::STF_FP_LAM::STF_FP_LAM;
use crate::functions::lam::stf::STF_IED_LAM::STF_IED_LAM;
use crate::functions::lam::stf::STF_IPBC_LAM::STF_IPCB_LAM;
use crate::functions::lam::stf::STF_IPCI2_LAM::STF_IPCI2_LAM;
use crate::functions::lam::stf::STF_IPCI_LAM::STF_IPCI_LAM;
use crate::functions::lam::stf::STF_MD_LAM::STF_MD_LAM;
use crate::functions::lam::stf::STF_PRD_LAM::STF_PRD_LAM;
use crate::functions::lam::stf::STF_RR_LAM::STF_RR_LAM;
use crate::functions::lam::stf::STF_RRF_LAM::STF_RRF_LAM;
use crate::functions::lam::stf::STF_SC_LAM::STF_SC_LAM;
use crate::functions::nam::pof::POF_PR_NAM::POF_PR_NAM;
use crate::functions::nam::stf::STF_PR2_NAM::STF_PR2_NAM;
use crate::functions::nam::stf::STF_PR_NAM::STF_PR_NAM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IED_PAM::POF_IED_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::pof::POF_SC_PAM::POF_SC_PAM;
use crate::functions::pam::stf::STF_IP_PAM::STF_IP_PAM;
use crate::functions::pam::stf::STF_TD_PAM::STF_TD_PAM;
use crate::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use crate::terms::grp_contract_identification::ContractID::ContractID;
use crate::terms::grp_contract_identification::ContractRole::ContractRole;
use crate::terms::grp_contract_identification::MarketObjectCode::MarketObjectCode;
use crate::terms::grp_calendar::Calendar::Calendar;
use crate::terms::grp_counterparty::CounterpartyID::CounterpartyID;
use crate::terms::grp_fees::CycleAnchorDateOfFee::CycleAnchorDateOfFee;
use crate::terms::grp_fees::CycleOfFee::CycleOfFee;
use crate::terms::grp_fees::FeeBasis::FeeBasis;
use crate::terms::grp_fees::FeeRate::FeeRate;
use crate::terms::grp_interest::CyclePointOfInterestPayment::CyclePointOfInterestPayment;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::terms::grp_notional_principal::Currency::Currency;
use crate::terms::grp_notional_principal::CycleAnchorDateOfScalingIndex::CycleAnchorDateOfScalingIndex;
use crate::terms::grp_notional_principal::CycleOfScalingIndex::CycleOfScalingIndex;
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
use crate::terms::grp_counterparty::CreditEventTypeCovered::CreditEventTypeCovered;
use crate::terms::grp_contract_identification::StatusDate::StatusDate;
use crate::terms::grp_fees::FeeAccrued::FeeAccrued;
use crate::terms::grp_interest::AccruedInterest::AccruedInterest;
use crate::terms::grp_interest::CapitalizationEndDate::CapitalizationEndDate;
use crate::terms::grp_interest::CycleAnchorDateOfInterestPayment::CycleAnchorDateOfInterestPayment;
use crate::terms::grp_interest::CycleOfInterestPayment::CycleOfInterestPayment;
use crate::terms::grp_interest::InterestCalculationBase::InterestCalculationBase;
use crate::terms::grp_interest::interest_calculation_base::Nt::NT;
use crate::terms::grp_interest::interest_calculation_base::Ntl::NTL;
use crate::terms::grp_interest::InterestCalculationBaseAmount::InterestCalculationBaseAmount;
use crate::terms::grp_interest::NominalInterestRate::NominalInterestRate;
use crate::terms::grp_notional_principal::CycleAnchorDateOfPrincipalRedemption::CycleAnchorDateOfPrincipalRedemption;
use crate::terms::grp_notional_principal::CycleOfPrincipalRedemption::CycleOfPrincipalRedemption;
use crate::terms::grp_notional_principal::InitialExchangeDate::InitialExchangeDate;
use crate::terms::grp_notional_principal::InterestScalingMultiplier::InterestScalingMultiplier;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use crate::terms::grp_notional_principal::PurchaseDate::PurchaseDate;
use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::Value::Value;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::traits::TraitContractModel::TraitContractModel;

#[derive(Debug, Clone, PartialEq)]
pub struct NAM {
    pub contract_terms: ContractTerms,
    pub contract_risk_factors: Option<RiskFactorModel>,
    pub contract_structure: Option<Vec<ContractReference>>,
    pub contract_events: Vec<ContractEvent<IsoDatetime, IsoDatetime>>,
    pub states_space: StatesSpace,
    pub result_vec_toggle: bool,
    pub result_vec: Option<Vec<ResultSet>>,
}

impl TraitContractModel for NAM {
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

        let cycle_point_of_interest_payment = CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment");
        let cycle_point_of_rate_reset =
            if let Some(point) = &cycle_point_of_interest_payment {
                if point.to_string() == "B" {
                    CyclePointOfRateReset::from_str("E").ok()
                } else {
                    CyclePointOfRateReset::provide_from_input_dict(sm, "cyclePointOfRateReset")
                }
            } else {
                None
            };


        let cycle_of_scaling_index = CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex");
        let cycle_anchor_date_of_scaling_index = if cycle_of_scaling_index.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfScalingIndex::from_str(&a).ok()
        } else {
            CycleAnchorDateOfScalingIndex::provide_from_input_dict(sm, "cycleAnchorDateOfScalingIndex")
        };


        let cycle_of_optionality = CycleOfOptionality::provide_from_input_dict (sm, "cycleOfOptionality");
        let cycle_anchor_date_of_optionality = if cycle_of_optionality.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfOptionality::from_str(&a).ok()
        } else {
            CycleAnchorDateOfOptionality::provide_from_input_dict(sm, "cycleAnchorDateOfOptionality")
        };

        let cycle_of_rate_reset = CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset");
        let cycle_anchor_date_of_rate_reset = if cycle_of_rate_reset.is_none() {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate").unwrap().value().to_string();
            CycleAnchorDateOfRateReset::from_str(&a).ok()
        } else {
            CycleAnchorDateOfRateReset::provide_from_input_dict(sm,"cycleAnchorDateOfRateReset" )
        };

        let business_day_adjuster =  {
            let calendar_clone = Some(Rc::clone(&calendar));
            BusinessDayAdjuster::provide(
                sm,
                "businessDayAdjuster",
                calendar_clone.unwrap()
            )
        };

        let credit_event_type_covered_tmp  = CreditEventTypeCovered::provide_from_input_dict(sm, "creditEventTypeCovered");
        let credit_event_type_covered = if credit_event_type_covered_tmp.is_none() {
            Some(CreditEventTypeCovered::default())
        } else {
            credit_event_type_covered_tmp
        };
        let eomc = EndOfMonthConvention::provide_from_input_dict(sm, "endOfMonthConvention");
        let end_of_month_convention = if eomc.is_none() {
            EndOfMonthConvention::default()
        } else {eomc.unwrap()};


        let w = AccruedInterest::provide_from_input_dict(sm, "accruedInterest");
        let accrued_interest = if w.is_some() {
            w
        }
        else {
            AccruedInterest::new(0.0).ok()
        };


        let w = FeeRate::provide_from_input_dict(sm, "feeRate");
        let fee_rate = if w.is_some() {
            w
        }
        else {
            FeeRate::new(0.0).ok()
        };


        let w = PeriodCap::provide_from_input_dict(sm, "periodCap");
        let period_cap = if w.is_some() {
            w
        }
        else {
            PeriodCap::new(f64::INFINITY).ok()
        };

        let w = PeriodFloor::provide_from_input_dict(sm, "periodFloor");
        let period_floor = if w.is_some() {
            w
        }
        else {
            PeriodFloor::new(f64::NEG_INFINITY).ok()
        };


        let w = LifeCap::provide_from_input_dict(sm, "lifeCap");
        let life_cap = if w.is_some() {
            w
        }
        else {
            LifeCap::new(f64::INFINITY).ok()
        };

        let w = LifeFloor::provide_from_input_dict(sm, "lifeFloor");
        let life_floor = if w.is_some() {
            w
        }
        else {
            LifeFloor::new(f64::NEG_INFINITY).ok()
        };

        let mut cycle_anchor_date_of_principal_redemption = CycleAnchorDateOfPrincipalRedemption::provide_from_input_dict(sm, "cycleAnchorDateOfPrincipalRedemption");
        cycle_anchor_date_of_principal_redemption = if cycle_anchor_date_of_principal_redemption.is_some() {
            cycle_anchor_date_of_principal_redemption
        }
        else {
            let a = InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate");
            CycleAnchorDateOfPrincipalRedemption::new(a.unwrap().value()).ok()
        };


        let mut scaling_effect = ScalingEffect::provide_from_input_dict(sm, "scalingEffect");
        scaling_effect = if scaling_effect.is_some() {
            scaling_effect
        }
        else {
            Some(ScalingEffect::new("OOO").unwrap())
        };

        let mut premium_discount_at_ied= PremiumDiscountAtIED::provide_from_input_dict(sm, "premiumDiscountAtIED");
        premium_discount_at_ied = if premium_discount_at_ied.is_some() {
            premium_discount_at_ied
        }
        else {
            PremiumDiscountAtIED::new(0.0).ok()
        };


        let mut next_principal_redemption_payment= NextPrincipalRedemptionPayment::provide_from_input_dict(sm, "nextPrincipalRedemptionPayment");
        next_principal_redemption_payment = if next_principal_redemption_payment.is_some() {
            next_principal_redemption_payment
        }
        else {
            None
        };

        let ct = ContractTerms {
            next_principal_redemption_payment: next_principal_redemption_payment,
            premium_discount_at_ied: premium_discount_at_ied,
            cycle_anchor_date_of_principal_redemption: cycle_anchor_date_of_principal_redemption,
            cycle_of_principal_redemption: CycleOfPrincipalRedemption::provide_from_input_dict(sm, "cycleOfPrincipalRedemption"),
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
            fee_accrued: FeeAccrued::provide_from_input_dict(sm, "feeAccrued"),
            cycle_anchor_date_of_interest_payment: cycle_anchor_date_of_interest_payment,
            cycle_of_interest_payment: CycleOfInterestPayment::provide_from_input_dict(sm, "cycleOfInterestPayment"),
            nominal_interest_rate: NominalInterestRate::provide_from_input_dict(sm, "nominalInterestRate"),
            day_count_convention: day_count_convention,
            accrued_interest: accrued_interest,
            capitalization_end_date: CapitalizationEndDate::provide_from_input_dict(sm, "capitalizationEndDate"),
            cycle_point_of_rate_reset: cycle_point_of_rate_reset,
            cycle_point_of_interest_payment: CyclePointOfInterestPayment::provide_from_input_dict(sm, "cyclePointOfInterestPayment"),
            currency: Currency::provide_from_input_dict(sm, "currency"),
            initial_exchange_date: InitialExchangeDate::provide_from_input_dict(sm, "initialExchangeDate"),
            notional_principal: NotionalPrincipal::provide_from_input_dict(sm, "notionalPrincipal"),
            purchase_date: PurchaseDate::provide_from_input_dict(sm, "purchaseDate"),
            price_at_purchase_date: PriceAtPurchaseDate::provide_from_input_dict(sm, "priceAtPurchaseDate"),
            termination_date: TerminationDate::provide_from_input_dict(sm, "terminationDate"),
            price_at_termination_date: PriceAtTerminationDate::provide_from_input_dict(sm, "priceAtTerminationDate"),
            market_object_code_of_scaling_index: MarketObjectCodeOfScalingIndex::provide_from_input_dict(sm, "marketObjectCodeOfScalingIndex"),
            scaling_index_at_contract_deal_date: ScalingIndexAtContractDealDate::provide_from_input_dict(sm, "scalingIndexAtContractDealDate"),
            notional_scaling_multiplier: NotionalScalingMultiplier::provide_from_input_dict(sm, "notionalScalingMultiplier"),
            interest_scaling_multiplier: InterestScalingMultiplier::provide_from_input_dict(sm, "interestScalingMultiplier"),
            cycle_anchor_date_of_scaling_index: cycle_anchor_date_of_scaling_index,
            cycle_of_scaling_index: CycleOfScalingIndex::provide_from_input_dict(sm, "cycleOfScalingIndex"),
            scaling_effect: scaling_effect,
            cycle_anchor_date_of_optionality: cycle_anchor_date_of_optionality,
            cycle_of_optionality: CycleOfOptionality::provide_from_input_dict(sm, "cycleOfOptionality"),
            penalty_type: PenaltyType::provide_from_input_dict(sm, "penaltyType"),
            penalty_rate: PenaltyRate::provide_from_input_dict(sm, "penaltyRate"),
            object_code_of_prepayment_model: ObjectCodeOfPrepaymentModel::provide_from_input_dict(sm, "objectCodeOfPrepaymentModel"),
            cycle_anchor_date_of_rate_reset: cycle_anchor_date_of_rate_reset,
            cycle_of_rate_reset: CycleOfRateReset::provide_from_input_dict(sm, "cycleOfRateReset"),
            rate_spread: RateSpread::provide_from_input_dict(sm, "rateSpread"),
            market_object_code_of_rate_reset: MarketObjectCodeOfRateReset::provide_from_input_dict(sm, "marketObjectCodeOfRateReset"),
            life_cap: life_cap,
            life_floor: life_floor,
            period_cap: period_cap,
            period_floor: period_floor,
            fixing_period: FixingPeriod::provide_from_input_dict(sm, "fixingPeriod"),
            next_reset_rate: NextResetRate::provide_from_input_dict(sm, "nextResetRate"),
            rate_multiplier: RateMultiplier::provide_from_input_dict(sm, "rateMultiplier"),
            maturity_date: maturity_date,
            credit_event_type_covered: credit_event_type_covered,
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
        let mut events: Vec<ContractEvent<IsoDatetime, IsoDatetime>> = Vec::new();
        let maturity = Self::maturity(model);

        // Initial exchange
        let e: ContractEvent<InitialExchangeDate, InitialExchangeDate> = EventFactory::create_event(
            &model.initial_exchange_date,
            &EventType::IED,
            &model.currency,
            Some(Rc::new(POF_IED_PAM)),
            Some(Rc::new(STF_IED_LAM)),
            &None,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Principal redemption schedule
        let pr_schedule = ScheduleFactory
            ::<CycleAnchorDateOfPrincipalRedemption,
             MaturityDate,
             CycleOfPrincipalRedemption,
            IsoDatetime
            >::create_schedule(
            &model.cycle_anchor_date_of_principal_redemption,
            &Some(maturity.clone()),
            &model.cycle_of_principal_redemption,
            &model.end_of_month_convention,
            Some(false),
        );

        // Choose the right state transition function depending on ipcb attributes
        let stf: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base != Some(InterestCalculationBase::NT(NT)) {
            Rc::new(STF_PR_NAM)
        } else {
            Rc::new(STF_PR2_NAM)
        };

        // Regular principal redemption events
        let mut pr_events = EventFactory::create_events(
            &pr_schedule,
            &EventType::PR,
            &model.currency,
            Some(Rc::new(POF_PR_NAM)),
            Some(stf),
            &model.business_day_adjuster,
            &model.contract_id,
        );

        events.extend(pr_events);

        // Maturity event
        let e: ContractEvent<MaturityDate, MaturityDate> = EventFactory::create_event(
            &Some(maturity.clone()),
            &EventType::MD,
            &model.currency,
            Some(Rc::new(POF_MD_PAM)),
            Some(Rc::new(STF_MD_LAM)),
            &model.business_day_adjuster,
            &model.contract_id,
        );
        events.push(e.to_iso_datetime_event());

        // Purchase event
        if let Some(purchase_date) = &model.purchase_date {
            let e: ContractEvent<PurchaseDate, PurchaseDate> = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                Some(Rc::new(POF_PRD_LAM)),
                Some(Rc::new(STF_PRD_LAM)),
                &None,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        }

        // Choose the right state transition function for IPCI depending on ipcb attributes
        let stf_ipci: Rc<dyn TraitStateTransitionFunction> = if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            Rc::new(STF_IPCI_LAM)
        } else {
            Rc::new(STF_IPCI2_LAM)
        };

        // Interest payment related events
        if model.cycle_of_interest_payment.is_some() || model.cycle_anchor_date_of_interest_payment.is_some() {
            let s = ScheduleFactory::<
                CycleAnchorDateOfInterestPayment,
                MaturityDate,
                CycleOfInterestPayment,
                IsoDatetime
            >::create_schedule(
                &model.cycle_anchor_date_of_interest_payment.clone(),
                &Some(maturity.clone()),
                &model.cycle_of_interest_payment,
                &model.end_of_month_convention,
                Some(true),
            );
            let mut interest_events = EventFactory::create_events(
                &s,
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            // Check if the cycle anchor dates and cycle periods for interest payments and principal payments are different
            if model.cycle_anchor_date_of_interest_payment.clone().unwrap().value() != model.cycle_anchor_date_of_principal_redemption.clone().unwrap().value()
                || model.cycle_of_interest_payment.clone().unwrap().value() != model.cycle_of_principal_redemption.clone().unwrap().value() {
                // Calculate the next principal redemption date by subtracting the cycle period from the anchor date
                //let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap());
                let prcl = &model.cycle_of_principal_redemption.clone().unwrap().value().extract_period();
                let pranxm = model.cycle_anchor_date_of_principal_redemption.clone().unwrap() - prcl.clone().unwrap();

                // Remove any interest payment events that occur on or after the calculated next principal redemption date
                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && (e.event_time.clone().unwrap() > pranxm.value() || e.event_time.clone().unwrap() == pranxm.value()))
                });

                // Create a new interest payment event at the adjusted principal redemption date
                let ipanxm: ContractEvent<CycleAnchorDateOfPrincipalRedemption, CycleAnchorDateOfPrincipalRedemption> = EventFactory::create_event(
                    &Some(pranxm),
                    &EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.insert(ipanxm.to_iso_datetime_event());

                // Generate new interest payment events based on the updated principal redemption schedule
                let new_interest_events = EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        &model.cycle_anchor_date_of_principal_redemption.clone(),
                        &Some(maturity.clone()),
                        &model.cycle_of_principal_redemption,
                        &model.end_of_month_convention,
                        Some(true),
                    ),
                    &EventType::IP,
                    &model.currency,
                    Some(Rc::new(POF_IP_LAM)),
                    Some(Rc::new(STF_IP_PAM)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.extend(new_interest_events);
            }

            // Adapt if interest capitalization set
            if let Some(capitalization_end_date) = &model.capitalization_end_date {
                let capitalization_end = EventFactory::create_event(
                    &Some(capitalization_end_date.clone()),
                    &EventType::IPCI,
                    &model.currency,
                    Some(Rc::new(POF_IPCI_PAM)),
                    Some(stf_ipci.clone()),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                interest_events.retain(|e| {
                    !(e.event_type == EventType::IP && e.event_time == capitalization_end.event_time)
                });

                interest_events.insert(capitalization_end.clone().to_iso_datetime_event());

                for mut e in &mut interest_events.clone().into_iter() {
                    if e.event_type == EventType::IP && e.event_time <= capitalization_end.event_time.clone() {
                        e.event_type = EventType::IPCI;
                        e.fpayoff = Some(Rc::new(POF_IPCI_PAM));
                        e.fstate = Some(stf_ipci.clone());
                    }
                }
            }

            events.append(&mut interest_events.clone().into_iter().collect());
        } else if model.capitalization_end_date.is_some() {
            // If no extra interest schedule set but capitalization end date, add single IPCI event
            let e: ContractEvent<CapitalizationEndDate, CapitalizationEndDate> = EventFactory::create_event(
                &model.capitalization_end_date.clone(),
                &EventType::IPCI,
                &model.currency,
                Some(Rc::new(POF_IPCI_PAM)),
                Some(stf_ipci),
                &model.business_day_adjuster,
                &model.contract_id,
            );
            events.push(e.to_iso_datetime_event());
        } else if model.cycle_of_interest_payment.is_none() && model.cycle_anchor_date_of_interest_payment.is_none() {
            // If no IPCL or IPANX is provided, IP events are set to PR cycle
            let interest_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_principal_redemption,
                    &Some(maturity.clone()),
                    &model.cycle_of_principal_redemption,
                    &model.end_of_month_convention,
                    Some(true),
                ),
                &EventType::IP,
                &model.currency,
                Some(Rc::new(POF_IP_LAM)),
                Some(Rc::new(STF_IP_PAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(interest_events);
        }

        // Rate reset events
        let mut rate_reset_events = EventFactory::create_events(
            &ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_rate_reset.clone(),
                &Some(maturity.clone()),
                &model.cycle_of_rate_reset.clone(),
                &model.end_of_month_convention,
                Some(false),
            ),
            &EventType::RR,
            &model.currency,
            Some(Rc::new(POF_RR_PAM)),
            Some(Rc::new(STF_RR_LAM)),
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

            let mut fixed_eventa = sorted_events.iter_mut().find(|e| e.event_time.clone().unwrap() > status_event.event_time.clone().unwrap()).unwrap().clone();
            fixed_eventa.fstate = Some(Rc::new(STF_RRF_LAM)); // Ensure the field name is correct
            fixed_eventa.event_type = EventType::RRF;
            rate_reset_events.insert(fixed_eventa.clone());

            // if let Some(mut fixed_event) = sorted_events.iter().find(|&e| e.event_time > status_event.event_time) {
            //     let mut fixed_event = fixed_event.clone(); // Clone the event to get an owned value
            //     fixed_eventxfstate = Some(Rc::new(STF_RRF_LAM)); // Ensure the field name is correct
            //     fixed_eventxeventType = EventType::RRF;
            //     rate_reset_events.insert(fixed_eventx.clone()); // Use push to add to the vector
            // }


        }

        events.append(&mut rate_reset_events.into_iter().collect());

        // Fee events (if specified)
        if let Some(cycle_of_fee) = &model.cycle_of_fee {
            let fee_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_fee.clone(),
                    &Some(maturity.clone()),
                    &Some(cycle_of_fee.clone()),
                    &model.end_of_month_convention.clone(),
                    Some(true),
                ),
                &EventType::FP,
                &model.currency,
                Some(Rc::new(POF_FP_PAM)),
                Some(Rc::new(STF_FP_LAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(fee_events);
        }

        // Scaling events (if specified)
        if let scaling_effect = &model.scaling_effect.clone().unwrap().to_string() {
            if scaling_effect.contains('I') || scaling_effect.contains('N') {
                let scaling_events = EventFactory::create_events(
                    &ScheduleFactory::create_schedule(
                        &model.cycle_anchor_date_of_scaling_index.clone(),
                        &Some(maturity.clone()),
                        &model.cycle_of_scaling_index.clone(),
                        &model.end_of_month_convention.clone(),
                        Some(false),
                    ),
                    &EventType::SC,
                    &model.currency,
                    Some(Rc::new(POF_SC_PAM)),
                    Some(Rc::new(STF_SC_LAM)),
                    &model.business_day_adjuster,
                    &model.contract_id,
                );

                events.extend(scaling_events);
            }
        }

        // Interest calculation base events (if specified)
        if model.interest_calculation_base == Some(InterestCalculationBase::NTL(NTL)) {
            let icb_events = EventFactory::create_events(
                &ScheduleFactory::create_schedule(
                    &model.cycle_anchor_date_of_interest_calculation_base.clone(),
                    &Some(maturity.clone()),
                    &model.cycle_of_interest_calculation_base.clone(),
                    &model.end_of_month_convention.clone(),
                    Some(false),
                ),
                &EventType::IPCB,
                &model.currency,
                Some(Rc::new(POF_IPCB_LAM)),
                Some(Rc::new(STF_IPCB_LAM)),
                &model.business_day_adjuster,
                &model.contract_id,
            );

            events.extend(icb_events);
        }

        // Termination event
        if let Some(termination_date) = &model.termination_date {
            let termination = EventFactory::create_event(
                &Some(termination_date.clone()),
                &EventType::TD,
                &model.currency,
                Some(Rc::new(POF_TD_LAM)),
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
        let to_date = maturity.clone(); //to.unwrap_or(maturity);
        let post_date = EventFactory::create_event(
            &Some(to_date),
            &EventType::AD,
            &model.currency,
            None,
            None,
            &None,
            &model.contract_id,
        );

        events.retain(|e| e.event_time <= post_date.event_time);

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
        let _maturity = &model.maturity_date.clone() ;
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
        if let Some(purchase_date) = &model.purchase_date {
            let purchase_event = EventFactory::create_event(
                &Some(purchase_date.clone()),
                &EventType::PRD,
                &model.currency,
                None,
                None,
                &None,
                &model.contract_id,
            );

            events.retain(|e| e.event_type == EventType::AD || e.event_time >= purchase_event.event_time);
        }

        self.contract_events = events.clone();
    }

    fn init_state_space(&mut self, _maturity: &Option<Rc<MaturityDate>>) {
        let model = &self.contract_terms;
        let mut states = StatesSpace::default();

        states.notional_scaling_multiplier = model.notional_scaling_multiplier.clone();
        states.interest_scaling_multiplier = InterestScalingMultiplier::new(self.contract_terms.notional_scaling_multiplier.clone().unwrap().value()).ok();
        states.contract_performance = model.contract_performance.clone();
        states.status_date = model.status_date.clone();
        states.next_principal_redemption_payment = model.next_principal_redemption_payment.clone();

        if model.initial_exchange_date.clone().unwrap().value() > model.status_date.clone().unwrap().value() {
            states.notional_principal = NotionalPrincipal::new(0.0).ok();//Some(0.0);
            states.nominal_interest_rate = NominalInterestRate::new(0.0).ok();;
            states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(0.0).ok();
        } else {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.notional_principal = NotionalPrincipal::new(role_sign * model.notional_principal.clone().unwrap().value()).ok();
            states.nominal_interest_rate = model.nominal_interest_rate.clone();

            if model.interest_calculation_base == Some(InterestCalculationBase::NT(NT)) {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(states.notional_principal.clone().unwrap().value()).ok();
            } else {
                states.interest_calculation_base_amount = InterestCalculationBaseAmount::new(role_sign * &self.contract_terms.interest_calculation_base_amount.clone().unwrap().value()).ok();
            }
        }

        if model.nominal_interest_rate.is_none() {
            states.accrued_interest = AccruedInterest::new(0.0).ok();// Some(0.0);
        } else if model.accrued_interest.is_some() {
            let role_sign = model.contract_role.as_ref().map_or(1.0, |role| role.role_sign());
            states.accrued_interest = AccruedInterest::new(role_sign * model.accrued_interest.clone().unwrap().value()).ok();
        } else {
            let day_counter = model.day_count_convention.as_ref().unwrap();
            let time_adjuster = model.business_day_adjuster.as_ref().unwrap();
            let mut ip_schedule: Vec<IsoDatetime> = ScheduleFactory::create_schedule(
                &model.cycle_anchor_date_of_interest_payment,
                &model.maturity_date.clone().map(|rc| (*rc).clone()),
                &model.cycle_of_interest_payment,
                &model.end_of_month_convention,
                Some(true),
            ).into_iter().collect();

            ip_schedule.sort();
            let date_earlier_than_t0: Vec<IsoDatetime> = ip_schedule.into_iter().filter(|date| date.clone() < states.status_date.clone().unwrap().value()).collect();
            let t_minus = date_earlier_than_t0.last().unwrap();

            states.accrued_interest = AccruedInterest::new(day_counter.day_count_fraction(
                time_adjuster.shift_sc(t_minus),
                time_adjuster.shift_sc(&states.status_date.clone().unwrap().value()),
            ) * states.notional_principal.clone().unwrap().value() * states.nominal_interest_rate.clone().unwrap().value()).ok();

        }

        if model.fee_rate.is_none() {
            states.fee_accrued = FeeAccrued::new(0.0).ok();// Some(0.0);
        } else if model.fee_accrued.is_some() {
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

impl NAM {
    fn maturity(&self) -> MaturityDate {
        if let maturity = &self.contract_terms.maturity_date.clone().unwrap() {
            return maturity.as_ref().clone();
        }

        let t0 = &self.contract_terms.status_date.clone().unwrap();
        let pranx = &self.contract_terms.cycle_anchor_date_of_principal_redemption.clone().unwrap();
        let ied = &self.contract_terms.initial_exchange_date.clone().unwrap();
        //let prcl = CycleUtils::parse_period(&model.cycle_of_principal_redemption.clone().unwrap()).unwrap();
        let prcl = &self.contract_terms.cycle_of_principal_redemption.clone().unwrap().value().extract_period().unwrap();
        let last_event: IsoDatetime;

        if pranx.value() >= t0.value() || pranx.value() == t0.value() {
            last_event = pranx.value();
        } else if (ied.clone().value() + prcl.clone()) > t0.value() || (ied.clone().value() + prcl.clone()) == t0.value() {
            last_event = ied.clone().value() + prcl.clone();
        } else {
            let mut previous_events = ScheduleFactory::<
                CycleAnchorDateOfPrincipalRedemption,
                StatusDate,
                CycleOfPrincipalRedemption,
                IsoDatetime
            >::create_schedule(
                &self.contract_terms.cycle_anchor_date_of_principal_redemption,
                &self.contract_terms.status_date,
                &self.contract_terms.cycle_of_principal_redemption,
                &self.contract_terms.end_of_month_convention,
                Some(true)
            );

            previous_events.retain(|d| d.clone() < t0.value());
            previous_events.remove(&t0.value());

            let mut prev_events_list: Vec<_> = previous_events.into_iter().collect();
            prev_events_list.sort();

            last_event = prev_events_list.last().unwrap().clone();
        }

        let time_from_last_event_plus_one_cycle = &self.contract_terms.day_count_convention.as_ref().unwrap().day_count_fraction(
            last_event,
            last_event + prcl.clone(),
        );

        let redemption_per_cycle = &self.contract_terms.next_principal_redemption_payment.clone().unwrap().value();
        - (time_from_last_event_plus_one_cycle * &self.contract_terms.nominal_interest_rate.clone().unwrap().value()
            * &self.contract_terms.notional_principal.clone().unwrap().value());

        let remaining_periods = ((&self.contract_terms.notional_principal.clone().unwrap().value() / redemption_per_cycle).ceil() - 1.0) as i32;

        let new_mat_date = last_event + prcl.multiplied_by(remaining_periods);
        MaturityDate::new(&self.contract_terms.business_day_adjuster.clone().as_ref().unwrap().shift_bd(
            &new_mat_date
        )).ok().unwrap()
    }

}

impl fmt::Display for NAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NAM")
    }
}