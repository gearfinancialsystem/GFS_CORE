use std::collections::HashSet;

use gfs_lib_terms::non_terms::EndTime::EndTime;
use gfs_lib_terms::non_terms::ScheduleFactoryStartTime::StartTime;

use gfs_lib_terms::phantom_terms::PhantomIsoCycle::PhantomIsoCycleW;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::phantom_terms::PhantomIsoPeriod::PhantomIsoPeriodW;

use gfs_lib_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoCycle::TraitMarkerIsoCycle;
use gfs_lib_terms::traits::types_markers::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use gfs_lib_types::traits::TraitConvert::{IsoDateTimeConvertTo, IsoDateTimeConvertToOption};
use gfs_lib_types::types::IsoCycle::LONG_STUB;

pub struct ScheduleFactory;

impl ScheduleFactory {

    pub fn create_schedule(
        start_time: &Option<StartTime>, // old T1
        end_time: &Option<EndTime>, // old T2
        cycle: &Option<PhantomIsoCycleW>, // old U
        end_of_month_convention: &EndOfMonthConvention,
        add_end_time: Option<bool>,
    ) -> HashSet<PhantomIsoDatetimeW> { // old T0
        //let stx = start_time.clone().unwrap().to_string();
        //let enx = end_time.clone().unwrap().to_string();

        let mut times_set: HashSet<PhantomIsoDatetimeW> = HashSet::new();
        //let ccccc = cycle.clone().unwrap().to_string();
        //let xxxx = end_of_month_convention.to_string();
        if cycle.is_none() {
            if start_time.is_some() {
                times_set.insert(start_time.convert_option::<PhantomIsoDatetimeW>().unwrap());
            }
            if add_end_time == Some(true) {
                times_set.insert(end_time.convert_option::<PhantomIsoDatetimeW>().unwrap());
            }
            else {
                if start_time.is_some() && end_time.is_some() {
                    if &start_time.unwrap().value() == &end_time.unwrap().value() {
                        times_set.remove(&start_time.convert_option::<PhantomIsoDatetimeW>().unwrap());
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
            ccycle.value()).expect("sd"); // attention vérifier

        let periodx = ccycle.value().extract_period().unwrap();
        let temp_year = periodx.years; 
        let temp_month = periodx.months; 
        let temp_day = periodx.days;
        
        let period = PhantomIsoPeriodW::new(temp_year, temp_month, temp_day);
        
        // Créez le calendrier en fonction de la convention de fin de mois
        let new_time = start_time.clone().unwrap();
        let mut counter = 1;
        let mut tmp1 = new_time.convert::<PhantomIsoDatetimeW>();
        let tmp2 = end_time.convert_option::<PhantomIsoDatetimeW>().unwrap();
        while tmp1 < tmp2 {
            let to_ins = tmp1.clone();
            times_set.insert(to_ins);
            let increment = period.multiplied_by(counter);
            tmp1 = PhantomIsoDatetimeW::new(shifter.shift(start_time.clone().unwrap().value() + increment)).expect("");
            
            counter += 1;
        }

        // Ajoutez (ou non) end_time au calendrier
        if add_end_time == Some(true) {
            times_set.insert(end_time.convert_option::<PhantomIsoDatetimeW>().unwrap());
        }
        else {
            if end_time.clone().unwrap().value() == start_time.clone().unwrap().value() {
                times_set.remove(&start_time.convert_option::<PhantomIsoDatetimeW>().unwrap());
            }
        }
        // Ajustez le dernier stub si nécessaire
        if stub.unwrap() == LONG_STUB && times_set.len() > 2 && tmp1 != tmp2.clone() {
            let last_stub_time = shifter.shift(start_time.clone().unwrap().value() + period.multiplied_by (counter - 2));
            let to_sup = PhantomIsoDatetimeW::new(last_stub_time).expect(""); // should be schedule time
            times_set.remove(&to_sup);
        }
        let mut test3: Vec<String> = vec![];

        for e in times_set.iter() {
            test3.push(e.to_string());
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
                let second_time: EndTime = start_times[i + 1].clone().convert();
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