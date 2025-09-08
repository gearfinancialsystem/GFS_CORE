
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use lib_actus_terms::non_terms::EndTime::EndTime;
use lib_actus_terms::non_terms::ScheduleTime::ScheduleTime;
use lib_actus_terms::non_terms::StartTime::StartTime;

use lib_actus_terms::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;

use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
use lib_actus_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use lib_actus_types::types::IsoCycle::LONG_STUB;


pub struct ScheduleFactory;


impl ScheduleFactory
{

    pub fn create_schedule(
        start_time: &Option<StartTime>, // old T1
        end_time: &Option<EndTime>, // old T2
        cycle: &Option<PhantomIsoCycleW>, // old U
        end_of_month_convention: &EndOfMonthConvention,
        add_end_time: Option<bool>,
    ) -> HashSet<PhantomIsoDatetimeW> { // old T0
        let mut times_set: HashSet<PhantomIsoDatetimeW> = HashSet::new();
        
        if cycle.is_none() {
            if start_time.is_some() {
                let to_ins = start_time.clone().unwrap().to_phantom_type();
                times_set.insert(to_ins);
            }
            if add_end_time == Some(true) {
                let to_ins = end_time.clone().unwrap().to_phantom_type();
                times_set.insert(to_ins);
            }
            else {
                if start_time.is_some() && end_time.is_some() {
                    if start_time.clone().unwrap().value() == end_time.clone().unwrap().value() {
                        let to_sup = start_time.clone().unwrap().to_phantom_type();
                        times_set.remove(&to_sup);
                        times_set.remove(&to_sup);
                    }
                }
            }
            return times_set;
        }

        let ccycle = cycle.clone().unwrap();
        // let stub = CycleUtils::parse_stub(cycle.as_ref().unwrap()).expect("Invalid stub");
        let stub = cycle.clone().unwrap().value().extract_stub();
        // let shifter = EndOfMonthAdjuster::new(end_of_month_convention, start_time, cycle.clone());
        let shifter = EndOfMonthConvention::new(
            end_of_month_convention.clone(),
            start_time.clone().unwrap().value(), 
            ccycle.clone().value().clone()).expect("sd"); // attention vérifier

        let periodx = ccycle.value().extract_period().unwrap();
        let temp_year = periodx.years; 
        let temp_month = periodx.months; 
        let temp_day = periodx.days;
        
        let period = PhantomIsoPeriodW::new(temp_year, temp_month, temp_day);
        
        // Créez le calendrier en fonction de la convention de fin de mois
        let mut new_time = start_time.clone().unwrap();
        let mut counter = 1;
        while new_time.to_phantom_type() < end_time.unwrap().to_phantom_type() {
            let to_ins = new_time.clone().to_phantom_type();
            times_set.insert(to_ins);
            let increment = period.multiplied_by(counter);
            new_time = StartTime::new(shifter.shift(start_time.clone().unwrap().value() + increment)).expect("");
            
            counter += 1;
        }

        // Ajoutez (ou non) end_time au calendrier
        if add_end_time == Some(true) {
            let to_ins = end_time.clone().unwrap().to_phantom_type();
            times_set.insert(to_ins);
        } else {
            if end_time.clone().unwrap().value() == start_time.clone().unwrap().value() {
                let to_sup = start_time.clone().unwrap().to_phantom_type();
                times_set.remove(&to_sup);
            }
        }
        // Ajustez le dernier stub si nécessaire
        if stub.unwrap() == LONG_STUB && times_set.len() > 2 && new_time.to_phantom_type() != end_time.clone().unwrap().to_phantom_type() {
            let last_stub_time = shifter.shift(start_time.clone().unwrap().value() + period.multiplied_by (counter - 2));
            let to_sup = PhantomIsoDatetimeW::new(last_stub_time).expect(""); // should be schedule time
            times_set.remove(&to_sup);
        }

        times_set
    }

    /// Crée un calendrier composé de sous-calendriers pour chaque paire start_time/cycle.
    pub fn create_array_schedule(
        start_times: &Vec<StartTime>, // old T1
        end_time: &Option<EndTime>, // old T2
        cycles: &Vec<PhantomIsoCycleW>, // old U
        end_of_month_convention: &EndOfMonthConvention,
    ) -> HashSet<PhantomIsoDatetimeW> { // old T0 // a modifier avec EventTime ou ScheduleTime
        let mut times_set: HashSet<PhantomIsoDatetimeW> = HashSet::new();

        // Ajoutez les sous-calendriers 1 à N-1
        for i in 0..start_times.len() - 1 {
            let sub_schedule = {
                let second_time = start_times[i + 1].clone().to_end_time().expect("");
                Self::create_schedule(
                    &Some(start_times[i].clone()),
                    &Some(second_time),
                    &(if cycles.is_empty() { None } else { Some(cycles[i].clone()) }),
                    end_of_month_convention,
                    Some(true),
                )
            };
            times_set.extend(sub_schedule);
        }

        // Ajoutez le dernier sous-calendrier
        let last_schedule = {
            let second_time = end_time.clone().unwrap();
                Self::create_schedule(
                    &Some(start_times[start_times.len() - 1].clone()),
                    &Some(second_time), 
                    &(if cycles.is_empty() { None } else { Some(cycles[start_times.len() - 1].clone()) }),
                    end_of_month_convention, 
                    Some(true),
                )
            };
        times_set.extend(last_schedule);

        times_set
    }
    
}