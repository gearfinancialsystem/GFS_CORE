use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum EventType {
    AD,
    IED,
    FP,
    PR,
    PD,
    PRF,
    PY,
    PP,
    IP,
    IPCI,
    CE,
    RRF,
    RR,
    DV,
    PRD,
    MR,
    TD,
    SC,
    IPCB,
    MD,
    XD,
    STD,
    PI,
    IPFX,
    IPFL,
    ME
}


impl FromStr for EventType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AD" => Ok(EventType::AD),
            "IED" => Ok(EventType::IED),
            "FP" => Ok(EventType::FP),
            "PR" => Ok(EventType::PR),
            "PD" => Ok(EventType::PD),
            "PRF" => Ok(EventType::PRF),
            "PY" => Ok(EventType::PY),
            "PP" => Ok(EventType::PP),
            "IP" => Ok(EventType::IP),
            "IPCI" => Ok(EventType::IPCI),
            "CE" => Ok(EventType::CE),
            "RRF" => Ok(EventType::RRF),
            "RR" => Ok(EventType::RR),
            "DV" => Ok(EventType::DV),
            "PRD" => Ok(EventType::PRD),
            "MR" => Ok(EventType::MR),
            "TD" => Ok(EventType::TD),
            "SC" => Ok(EventType::SC),
            "IPCB" => Ok(EventType::IPCB),
            "MD" => Ok(EventType::MD),
            "XD" => Ok(EventType::XD),
            "STD" => Ok(EventType::STD),
            "PI" => Ok(EventType::PI),
            "IPFX" => Ok(EventType::IPFX),
            "IPFL" => Ok(EventType::IPFL),
            "ME" => Ok(EventType::ME),
            _ => Err(ParseError {
                message: format!("Invalid Event cont_type: {}", s),
            }),
        }
    }
}

impl Default for EventType {
    fn default() -> Self {
        EventType::PI
    }
}