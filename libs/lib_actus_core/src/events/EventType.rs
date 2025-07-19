use std::fmt;
use std::str::FromStr;
use crate::exceptions::ParseError::ParseError;
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
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

impl EventType {
    pub fn to_string(&self) -> String {
        match self {
            EventType::AD => "AD".to_string(),
            EventType::IED => "IED".to_string(),
            EventType::FP => "FP".to_string(),
            EventType::PR => "PR".to_string(),
            EventType::PD => "PD".to_string(),
            EventType::PRF => "PRF".to_string(),
            EventType::PY => "PY".to_string(),
            EventType::PP => "PP".to_string(),
            EventType::IP => "IP".to_string(),
            EventType::IPCI => "IPCI".to_string(),
            EventType::CE => "CE".to_string(),
            EventType::RRF => "RRF".to_string(),
            EventType::RR => "RR".to_string(),
            EventType::DV => "DV".to_string(),
            EventType::PRD => "PRD".to_string(),
            EventType::MR => "MR".to_string(),
            EventType::TD => "TD".to_string(),
            EventType::SC => "SC".to_string(),
            EventType::IPCB => "IPCB".to_string(),
            EventType::MD => "MD".to_string(),
            EventType::XD => "XD".to_string(),
            EventType::STD => "STD".to_string(),
            EventType::PI => "PI".to_string(),
            EventType::IPFX => "IPFX".to_string(),
            EventType::IPFL => "IPFL".to_string(),
            EventType::ME => "ME".to_string(),
        }
    }
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
impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}