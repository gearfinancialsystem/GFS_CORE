// SPDX-License-Identifier: Apache-2.0
// Converted from the Java implementation of RedemptionUtils in org.actus.util

use crate::AttributeConversionException::AttributeConversionException;

use crate::traits::ContractModelTrait::ContractModelTrait;

// use crate::algorithmic::DayCountCalculator::DayCountCalculator;
use crate::states::StateSpace::StateSpace;
// use crate::algorithmic::ScheduleFactory::ScheduleFactory;
use crate::terms::grp_contract_identification::ContractType::ContractType;

use chrono::prelude::*;



pub struct RedemptionUtils;

// impl RedemptionUtils {
    // Calculate the NextPrincipalRedemption amount
    // pub fn redemption_amount(
    //     model: &dyn ContractModelTrait,
    //     state: &StateSpace,
    // ) -> Result<f64, AttributeConversionException> {
    //     let redemption_amount: f64;
    //     let status_date = state.statusDate;
    //     let maturity = if model.get_as("AmortizationDate").is_none() {
    //         state.maturityDate
    //     } else {
    //         model.get_as("AmortizationDate")
    //     };

    //     let accrued_interest = state.accruedInterest;
    //     let outstanding_notional = state.notionalPrincipal;
    //     let interest_rate = state.nominalInterestRate;

    //     // Extract day count convention
    //     let day_counter: DayCountCalculator = model.get_as("DayCountConvention")?;

    //     // Determine remaining PR schedule
    //     let mut event_times = ScheduleFactory::create_schedule_with_end_time(
    //         model.get_as("CycleAnchorDateOfPrincipalRedemption"),
    //         maturity,
    //         model.get_as("CycleOfPrincipalRedemption"),
    //         model.get_as("EndOfMonthConvention"),
    //         true,
    //     );

    //     event_times.retain(|&d| d >= status_date);
    //     event_times.remove(&status_date);

    //     // Compute redemption amount for different contracts
    //     match model.get_as("ContractType") {
    //         ContractTypeEnum::LAM => {
    //             redemption_amount =
    //                 model.get_as::<f64>("NotionalPrincipal") / event_times.len() as f64;
    //         }
    //         ContractTypeEnum::ANN | ContractTypeEnum::NAM => {
    //             let mut event_times_sorted: Vec<DateTime<Utc>> =
    //                 event_times.into_iter().collect();
    //             event_times_sorted.sort();
    //             let lb = 1;
    //             let ub = event_times_sorted.len();
    //             let scale = outstanding_notional
    //                 + accrued_interest
    //                 + day_counter
    //                     .day_count_fraction(state.statusDate, event_times_sorted[0])?
    //                     * interest_rate
    //                     * outstanding_notional;
    //             let sum = Self::sum(lb, ub, &event_times_sorted, interest_rate, &day_counter)?;
    //             let frac = Self::product(lb, ub, &event_times_sorted, interest_rate, &day_counter)?
    //                 / (1.0 + sum);
    //             redemption_amount = scale * frac;
    //         }
    //         _ => {
    //             redemption_amount = 0.0;
    //         }
    //     }

    //     // Return the annuity payment
    //     Ok(redemption_amount)
    // }

    // fn product(
    //     lb: usize,
    //     ub: usize,
    //     times: &[DateTime<Utc>],
    //     ir: f64,
    //     day_counter: &DayCountCalculator,
    // ) -> Result<f64, AttributeConversionException> {
    //     let mut prod = 1.0;
    //     for i in lb..ub {
    //         prod *= Self::effective_rate(i, times, ir, day_counter)?;
    //     }
    //     Ok(prod)
    // }

    // fn sum(
    //     lb: usize,
    //     ub: usize,
    //     times: &[DateTime<Utc>],
    //     ir: f64,
    //     day_counter: &DayCountCalculator,
    // ) -> Result<f64, AttributeConversionException> {
    //     let mut sum = 0.0;
    //     for i in lb..ub {
    //         sum += Self::product(i, ub, times, ir, day_counter)?;
    //     }
    //     Ok(sum)
    // }

    // fn effective_rate(
    //     index: usize,
    //     times: &[DateTime<Utc>],
    //     ir: f64,
    //     day_counter: &DayCountCalculator,
    // ) -> Result<f64, AttributeConversionException> {
    //     let yf = day_counter.day_count_fraction(times[index - 1], times[index]);
    //     Ok(1.0 + ir * yf)
    // }
// }
