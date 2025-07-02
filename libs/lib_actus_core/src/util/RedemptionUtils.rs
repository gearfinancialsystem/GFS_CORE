use std::collections::HashSet;
use crate::attributes::ContractModel::ContractModel;
use crate::state_space::StateSpace::StateSpace;
use crate::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::time::ScheduleFactory::ScheduleFactory;
use crate::types::IsoDatetime::IsoDatetime;

pub struct RedemptionUtils;

impl RedemptionUtils {
    pub fn redemptionAmount(model: &ContractModel, state: &StateSpace) -> f64 {
        let redemption_amount: f64;
        let status_date = state.statusDate.clone().unwrap();
        let maturity = if model.amortizationDate.is_none() {
            state.maturityDate.clone().unwrap()
        } else {
            model.amortizationDate.clone().unwrap()
        };

        let accrued_interest = state.accruedInterest.clone().unwrap();
        let outstanding_notional = state.notionalPrincipal.clone().unwrap();
        let interest_rate = state.nominalInterestRate.clone().unwrap();

        // extract day count convention
        let day_counter = model.dayCountConvention.clone().unwrap();

        // determine remaining PR schedule
        let mut event_times: HashSet<IsoDatetime> = ScheduleFactory::create_schedule(
            model.cycleAnchorDateOfPrincipalRedemption,
            Some(maturity),
            model.cycleOfPrincipalRedemption.clone(),
            model.endOfMonthConvention.unwrap(),
            true
        );
        event_times.retain(|e| e >= &status_date);
        event_times.remove(&status_date);

        redemption_amount = match model.contractType.clone().unwrap().as_str() {
            "LAM" => {
                model.notional_principal.clone().unwrap() / event_times.len() as f64 // on est sur que cest len ?
            },
            "ANN" => {
                0.0 // a implementer
            },
            "NAM" => {
                let mut event_times_sorted: Vec<IsoDatetime> = event_times.into_iter().collect();

                event_times_sorted.sort();
                let lb = 1;
                let ub = event_times_sorted.len();
                let scale = outstanding_notional + accrued_interest + day_counter.day_count(state.statusDate.clone().unwrap(),
                                                                                            event_times_sorted.get(0).unwrap().clone()) * interest_rate*outstanding_notional;
                let sum = RedemptionUtils::sumx(lb, ub as i32, event_times_sorted.clone(), interest_rate, day_counter.clone());
                let frac = RedemptionUtils::product(lb, ub as i32, event_times_sorted.clone(), interest_rate, day_counter.clone()) / (1.0 + sum);
                scale * frac
            },
            _ => 0.0
        };
        
        // finally, return the annuity payment
        redemption_amount
    }

    fn product(lb: i32, ub: i32, times: Vec<IsoDatetime>, ir: f64, day_counter: DayCountConvention) -> f64 {
        let mut prod = 1.0;
        for i in lb..ub {
            prod *= RedemptionUtils::effective_rate(i, times.clone(), ir, day_counter.clone());
        }

        prod
    }


    fn effective_rate(index: i32, times: Vec<IsoDatetime>, ir: f64, day_counter: DayCountConvention) -> f64 {
        let yf: f64 = day_counter.day_count_fraction (times[(index - 1) as usize],
                                                      times[index as usize]);

        1.0 + ir * yf
    }


    pub fn sumx(lb: i32, ub: i32, times: Vec<IsoDatetime>, ir: f64,
                day_counter: DayCountConvention) -> f64 {
        let mut sum = 0.0;
        for i in lb..ub {
            sum += RedemptionUtils::product(i, ub, times.clone(), ir, day_counter.clone());
        }
        sum
    }
}
