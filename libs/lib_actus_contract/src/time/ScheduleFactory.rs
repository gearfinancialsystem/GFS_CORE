
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use lib_actus_terms::traits::TraitMarqueurIsoCycle::TraitMarqueurIsoCycle;
use lib_actus_terms::traits::TraitMarkerIsoDatetime::TraitMarkerIsoDatetime;
use lib_actus_types::types::IsoCycle::{IsoCycle, LONG_STUB};
use lib_actus_types::types::IsoDatetime::IsoDatetime;

pub struct ScheduleFactory<T1, T2,U, TO, > {
    marker: PhantomData<(T1, T2,U, TO)>,
}


impl<T1, T2, U, TO> ScheduleFactory<T1, T2,U, TO>
where
    T1: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash + From<IsoDatetime>,
    T2: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash + From<IsoDatetime>,
    U: TraitMarqueurIsoCycle + Clone + PartialEq + Debug + Hash + From<IsoCycle>,
    TO: TraitMarqueurIsoDatetime + Clone + PartialEq + Debug + Hash + From<IsoDatetime> + Eq,
{

    pub fn create_schedule(
        start_time: &Option<T1>,
        end_time: &Option<T2>,
        cycle: &Option<U>,
        end_of_month_convention: &EndOfMonthConvention,
        add_end_time: Option<bool>,
    ) -> HashSet<TO> {
        let mut times_set: HashSet<TO> = HashSet::new();
        
        if cycle.is_none() {
            if start_time.is_some() {
                let to_ins = TO::from(start_time.clone().unwrap().value());
                times_set.insert(to_ins);
            }
            if add_end_time == Some(true) {
                let to_ins = TO::from(end_time.clone().unwrap().value());
                times_set.insert(to_ins);
            }
            else {
                if start_time.is_some() && end_time.is_some() {
                    if start_time.clone().unwrap().value() == end_time.clone().unwrap().value() {
                        let to_sup = TO::from(start_time.clone().unwrap().value());
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
        // CHANGER EoM new (prendre cycle as ref, pas en valeur
        // Parsez le cycle pour obtenir une durée
        //let period = CycleUtils::parse_period(&ccycle).expect("et");
        let period = ccycle.value().extract_period().unwrap();

        // Créez le calendrier en fonction de la convention de fin de mois
        let mut new_time = start_time.clone().unwrap().value();
        let mut counter = 1;
        while new_time < end_time.clone().unwrap().value() {
            let to_ins = TO::from(new_time.clone().value());
            times_set.insert(to_ins);
            let increment = period.multiplied_by(counter);
            new_time = shifter.shift(start_time.clone().unwrap().value() + increment);
            
            counter += 1;
        }

        // Ajoutez (ou non) end_time au calendrier
        if add_end_time == Some(true) {
            let to_ins = TO::from(end_time.clone().unwrap().value());
            times_set.insert(to_ins);
        } else {
            if end_time.clone().unwrap().value() == start_time.clone().unwrap().value() {
                let to_sup = TO::from(start_time.clone().unwrap().value());
                times_set.remove(&to_sup);
            }
        }
        // Ajustez le dernier stub si nécessaire
        if stub.unwrap() == LONG_STUB && times_set.len() > 2 && new_time != end_time.clone().unwrap().value() {
            let last_stub_time = shifter.shift(start_time.clone().unwrap().value() + period.multiplied_by (counter - 2));
            let to_sup = TO::from(last_stub_time);
            times_set.remove(&to_sup);
        }

        times_set
    }

    /// Crée un calendrier composé de sous-calendriers pour chaque paire start_time/cycle.
    pub fn create_array_schedule(
        start_times: &Vec<T1>,
        end_time: &Option<T2>,
        cycles: &Vec<U>,
        end_of_month_convention: &EndOfMonthConvention,
    ) -> HashSet<TO> {
        let mut times_set = HashSet::new();

        // Ajoutez les sous-calendriers 1 à N-1
        for i in 0..start_times.len() - 1 {
            let sub_schedule = {
                let second_time = T2::from(start_times[i + 1].clone().value());
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
            let second_time = T2::from(end_time.clone().unwrap().value());
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