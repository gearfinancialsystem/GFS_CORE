use std::collections::HashSet;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use crate::states_space::StatesSpace::StatesSpace;
use crate::attributes::ContractTerms::ContractTerms;
use gfs_lib_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use gfs_lib_terms::terms::grp_notional_principal::MaturityDate::MaturityDate;
use gfs_lib_terms::traits::types_markers::TraitMarkerF64::TraitMarkerF64;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
use crate::time::ScheduleFactory::ScheduleFactory;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;


pub struct RedemptionUtils;

impl RedemptionUtils {
    pub fn redemptionAmount(model: &ContractTerms, state: &StatesSpace) -> f64 {
        let redemption_amount: f64;
        let status_date = state.status_date.clone().unwrap();
        let maturity: MaturityDate = if model.amortization_date.is_none() {
            state.maturity_date.clone().unwrap()
        } else {
            let md = MaturityDate::new(model.amortization_date.clone().unwrap().value()).unwrap();
            md
        };

        let accrued_interest = state.accrued_interest.clone().unwrap();
        let outstanding_notional = state.notional_principal.clone().unwrap();
        let interest_rate = state.nominal_interest_rate.clone().unwrap();

        // extract day count convention
        let day_counter = model.day_count_convention.clone().unwrap();

        // determine remaining PR schedule
        let mut event_times: HashSet<PhantomIsoDatetimeW> = ScheduleFactory::create_schedule(
            &model.cycle_anchor_date_of_principal_redemption.unwrap().to_start_time(),
            &Some(maturity.to_end_time().unwrap()),
            &Some(model.cycle_of_principal_redemption.clone().unwrap().to_phantom_type()),
            &model.end_of_month_convention,
            Some(true)
        );
        event_times.retain(|e| e >= &status_date.to_phantom_type());
        event_times.remove(&status_date.to_phantom_type());

        redemption_amount = match model.contract_type.clone().unwrap().to_string().as_str() {
            "LAM" => {
                model.notional_principal.clone().unwrap().value() / event_times.len() as f64 // on est sur que cest len ?
            },
            "ANN" => {
                0.0 // a implementer
            },
            "NAM" => {
                let mut event_times_sorted: Vec<PhantomIsoDatetimeW> = event_times.into_iter().collect();

                event_times_sorted.sort();
                let lb = 1;
                let ub = event_times_sorted.len();
                let scale = outstanding_notional.value() + 
                    accrued_interest.value() + 
                    day_counter.day_count(state.status_date.clone().unwrap().to_phantom_type(),
                                          event_times_sorted.get(0).unwrap().to_phantom_type()) 
                        * interest_rate.value() * outstanding_notional.value();
                let sum = RedemptionUtils::sumx(lb, ub as i32, event_times_sorted.clone(), interest_rate.value(), day_counter.clone());
                let frac = RedemptionUtils::product(lb, ub as i32, event_times_sorted.clone(), interest_rate.value(), day_counter.clone()) / (1.0 + sum);
                scale * frac
            },
            _ => 0.0
        };
        
        // finally, return the annuity payment
        redemption_amount
    }

    fn product(lb: i32, ub: i32, times: Vec<PhantomIsoDatetimeW>, ir: f64, day_counter: DayCountConvention) -> f64 {
        let mut prod = 1.0;
        for i in lb..ub {
            prod *= RedemptionUtils::effective_rate(i, times.clone(), ir, day_counter.clone());
        }

        prod
    }


    fn effective_rate(index: i32, times: Vec<PhantomIsoDatetimeW>, ir: f64, day_counter: DayCountConvention) -> f64 {
        let yf: f64 = day_counter.day_count_fraction (times[(index - 1) as usize],
                                                      times[index as usize]);

        1.0 + ir * yf
    }


    pub fn sumx(lb: i32, ub: i32, times: Vec<PhantomIsoDatetimeW>, ir: f64,
                day_counter: DayCountConvention) -> f64 {
        let mut sum = 0.0;
        for i in lb..ub {
            sum += RedemptionUtils::product(i, ub, times.clone(), ir, day_counter.clone());
        }
        sum
    }
}
