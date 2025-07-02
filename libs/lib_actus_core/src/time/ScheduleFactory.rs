use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
use chrono::Duration;
use std::collections::HashSet;
use crate::types::IsoCycle::{IsoCycle, LONG_STUB};
use crate::types::IsoCycle::SHORT_STUB;
use crate::types::IsoDatetime::IsoDatetime;


pub struct ScheduleFactory;

impl ScheduleFactory {
    /// Crée un ensemble de dates selon un cycle et une convention de fin de mois.

    pub fn create_schedule_end_time_true(start_time: Option<IsoDatetime>,
                                         end_time: Option<IsoDatetime>,
                                         cycle: Option<IsoCycle>,
                                         end_of_month_convention: EndOfMonthConvention) -> HashSet<IsoDatetime> {
        Self::create_schedule(start_time, end_time, cycle, end_of_month_convention, true)
    }

    pub fn create_schedule(
        start_time: Option<IsoDatetime>,
        end_time: Option<IsoDatetime>,
        cycle: Option<IsoCycle>,
        end_of_month_convention: EndOfMonthConvention,
        add_end_time: bool,
    ) -> HashSet<IsoDatetime> {
        let mut times_set = HashSet::new();

        // Si aucun cycle n'est fourni, ajoutez uniquement start_time et end_time
        if cycle.is_none() {
            if start_time.is_some() {
                times_set.insert(start_time.unwrap());
            }
            if add_end_time == true {
                times_set.insert(end_time.unwrap());
            }
            else {
                if start_time == end_time {
                    times_set.remove(&start_time.unwrap());
                }
            }
            return times_set;
        }
        let ccycle = cycle.clone().unwrap();
        let stub = CycleUtils::parse_stub(cycle.as_ref().unwrap()).expect("Invalid stub");
        // let shifter = EndOfMonthAdjuster::new(end_of_month_convention, start_time, cycle.clone());
        let shifter = EndOfMonthConvention::new(end_of_month_convention, start_time.unwrap(), ccycle.clone()).expect("sd"); // attention vérifier
        // CHANGER EoM new (prendre cycle as ref, pas en valeur
        // Parsez le cycle pour obtenir une durée
        let period = CycleUtils::parse_period(&ccycle).expect("et");

        // Créez le calendrier en fonction de la convention de fin de mois
        let mut new_time = start_time.clone();
        let mut counter = 1;

        while new_time < end_time {
            times_set.insert(new_time.clone().unwrap());
            let increment = period.multiplied_by(counter);
            new_time = Some(shifter.shift(start_time.clone().unwrap() + increment));
            counter += 1;
        }

        // Ajoutez (ou non) end_time au calendrier
        if add_end_time == true {
            times_set.insert(end_time.unwrap());
        } else {
            if end_time == start_time {
                times_set.remove(&start_time.unwrap());
            }
        }
        // Ajustez le dernier stub si nécessaire
        if stub == LONG_STUB && times_set.len() > 2 && new_time != end_time {
            let last_stub_time = shifter.shift(start_time.unwrap() + period.multiplied_by (counter - 2));
            times_set.remove(&last_stub_time);
        }

        times_set
    }

    /// Crée un calendrier composé de sous-calendriers pour chaque paire start_time/cycle.
    pub fn create_array_schedule(
        start_times: Vec<IsoDatetime>,
        end_time: Option<IsoDatetime>,
        cycles: Vec<IsoCycle>,
        end_of_month_convention: EndOfMonthConvention,
    ) -> HashSet<IsoDatetime> {
        let mut times_set = HashSet::new();

        // Ajoutez les sous-calendriers 1 à N-1
        for i in 0..start_times.len() - 1 {
            let sub_schedule = Self::create_schedule(
                Some(start_times[i]),
                Some(start_times[i + 1]),
                if cycles.is_empty() { None } else { cycles[i].clone() },
                end_of_month_convention,
                true,
            );
            times_set.extend(sub_schedule);
        }

        // Ajoutez le dernier sous-calendrier
        let last_schedule = Self::create_schedule(
            Some(start_times[start_times.len() - 1]),
            Some(end_time),
            if cycles.is_empty() { None } else { cycles[start_times.len() - 1].clone() },
            end_of_month_convention,
            true,
        );
        times_set.extend(last_schedule);

        times_set
    }
    
}