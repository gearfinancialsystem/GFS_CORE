
use crate::terms::grp_calendar::EndOfMonthConvention::EndOfMonthConvention;
// use crate::algorithmic::EndOfMonthAdjuster::EndOfMonthAdjuster;

use chrono::Duration;

use std::collections::HashSet;
use crate::types::isoDatetime::IsoDatetime;
use crate::types::IsoPeriod::IsoPeriod;

pub struct ScheduleFactory;

impl ScheduleFactory {
    /// Crée un ensemble de dates selon un cycle et une convention de fin de mois.

    pub fn create_schedule_end_time_true(start_time: Option<IsoDatetime>,
                                         end_time: Option<IsoDatetime>,
                                         cycle: Option<String>,
                                         end_of_month_convention: EndOfMonthConvention) -> HashSet<IsoDatetime> {
        Self::create_schedule(start_time, end_time, cycle, end_of_month_convention, true)
    }

    pub fn create_schedule(
        start_time: Option<IsoDatetime>,
        end_time: Option<IsoDatetime>,
        cycle: Option<String>,
        end_of_month_convention: EndOfMonthConvention,
        add_end_time: bool,
    ) -> HashSet<IsoDatetime> {
        let mut times_set = HashSet::new();

        // Si aucun cycle n'est fourni, ajoutez uniquement start_time et end_time
        if cycle.is_none() {
            if !start_time.is_none() {
                times_set.insert(start_time.unwrap());
            }
            if !end_time.is_none() {
                times_set.insert(end_time.unwrap());
            }
            else {
                if start_time == end_time {
                    times_set.remove(&start_time.unwrap());
                }
            }
            return times_set;
        }

        let cycle = cycle.unwrap();
        // let shifter = EndOfMonthAdjuster::new(end_of_month_convention, start_time, cycle.clone());
        let shifter = EndOfMonthConvention::new_EOM(); // attention vérifier
        // Parsez le cycle pour obtenir une durée
        let period = Self::parse_period(&cycle);

        // Créez le calendrier en fonction de la convention de fin de mois
        let mut new_time = start_time;
        let mut counter = 1;

        while new_time <= end_time {
            times_set.insert(new_time.unwrap());
            let increment = period * counter;
            new_time = Some(shifter.shift(start_time.unwrap() + increment));
            counter += 1;
        }

        // Ajoutez (ou non) end_time au calendrier
        if add_end_time {
            times_set.insert(end_time.unwrap());
        } else if end_time == start_time {
            times_set.remove(&start_time.unwrap());
        }

        // Ajustez le dernier stub si nécessaire
        if cycle.ends_with('L') && times_set.len() > 2 && new_time != end_time {
            let last_stub_time = shifter.shift(start_time.unwrap() + period * (counter - 2));
            times_set.remove(&last_stub_time);
        }

        times_set
    }

    /// Crée un calendrier composé de sous-calendriers pour chaque paire start_time/cycle.
    pub fn create_array_schedule(
        start_times: Vec<IsoDatetime>,
        end_time: IsoDatetime,
        cycles: Vec<Option<String>>,
        end_of_month_convention: EndOfMonthConvention,
    ) -> HashSet<IsoDatetime> {
        let mut times_set = HashSet::new();

        // Ajoutez les sous-calendriers 1 à N-1
        for i in 0..start_times.len() - 1 {
            let sub_schedule = Self::create_schedule(
                Some(start_times[i]),
                Some(start_times[i + 1]),
                cycles[i].clone(),
                end_of_month_convention,
                true,
            );
            times_set.extend(sub_schedule);
        }

        // Ajoutez le dernier sous-calendrier
        let last_schedule = Self::create_schedule(
            Some(start_times[start_times.len() - 1]),
            Some(end_time),
            cycles[start_times.len() - 1].clone(),
            end_of_month_convention,
            true,
        );
        times_set.extend(last_schedule);

        times_set
    }

    /// Parse une chaîne de cycle en une durée.
    fn parse_period(cycle: &str) -> Duration {
        // Exemple de parsing simple : "1M" pour 1 mois, "2D" pour 2 jours, etc.
        let amount = cycle[..cycle.len() - 1].parse::<i64>().unwrap_or(1);
        match cycle.chars().last().unwrap() {
            'D' => Duration::days(amount),
            'M' => Duration::days(amount * 30), // Approximation pour les mois
            'Y' => Duration::days(amount * 365), // Approximation pour les années
            _ => Duration::days(1), // Par défaut, 1 jour
        }
    }
}