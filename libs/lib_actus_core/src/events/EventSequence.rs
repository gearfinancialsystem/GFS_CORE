use crate::events::EventType::EventType;
pub struct EventSequence;

impl EventSequence {
    /// Returns the upper bound on the time offsets for the event sequence
    pub fn upper_bound() -> i32 {
        900
    }

    /// Returns the time offset according to the event sequence for a particular event cont_type
    pub fn time_offset(event_typex: EventType) -> i64 {
        match event_typex {
            EventType::IED => 20,
            EventType::PR => 30,
            EventType::IP => 40,
            EventType::IPFX => 40,
            EventType::IPFL => 45,
            EventType::IPCI => 40,
            EventType::FP => 60,
            EventType::DV => 70,
            EventType::MR => 80,
            EventType::RRF => 100,
            EventType::RR => 100,
            EventType::PRF => 105,
            EventType::SC => 110,
            EventType::IPCB => 120,
            EventType::PRD => 130,
            EventType::TD => 140,
            EventType::MD => 150,
            EventType::XD => 160,
            EventType::STD => 170,
            EventType::AD => 950,
            EventType::ME => 180,
            EventType::CE => 0,
            EventType::PD => 0,
            EventType::PI => 0,
            EventType::PP => 0,
            EventType::PY => 0
        }
    }
}
