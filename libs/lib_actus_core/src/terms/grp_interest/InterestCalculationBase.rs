use std::str::FromStr;
use crate::util::ParseError::ParseError;
use crate::terms::grp_interest::interestcalculationbase::Nt::NT;
use crate::terms::grp_interest::interestcalculationbase::Ntied::NTIED;
use crate::terms::grp_interest::interestcalculationbase::Ntl::NTL;
use crate::traits::TraitTermDescription::TraitTermDescription;

#[derive(PartialEq, Eq, Debug)]
pub enum InterestCalculationBase {
    NT(NT),
    NTIED(NTIED),
    NTL(NTL)
}

impl InterestCalculationBase {
    fn new_NT() -> Self {
        InterestCalculationBase::NT(NT::new())
    }
    fn new_NTIED() -> Self {
        InterestCalculationBase::NTIED(NTIED::new())
    }
    fn new_NTL() -> Self {
        InterestCalculationBase::NTL(NTL::new())
    }
}

impl Default for InterestCalculationBase {
    fn default() -> Self {
        InterestCalculationBase::new_NT()
    }
}

impl FromStr for InterestCalculationBase {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "NT" => Ok(InterestCalculationBase::new_NT()),
            "NTIED" => Ok(InterestCalculationBase::new_NTIED()),
            "NTL" => Ok(InterestCalculationBase::new_NTL()),
            "" => Ok(InterestCalculationBase::default()),
            _ => Err(ParseError { message: format!("Invalid BusinessDayConvention: {}", s)})
        }
    }
}

impl TraitTermDescription for InterestCalculationBase {
    fn get_identifier(&self) -> &str {
        "interestCalculationBase"
    }
    fn get_group(&self) -> &str {
        "Interest"
    }
    fn get_name(&self) -> &str {
        "Interest Calculation Base"
    }
    fn get_acronym(&self) -> &str {
        "IPCB"
    }
    fn get_type(&self) -> &str {
        "Enum"
    }
    fn get_allowed_values(&self) -> &str {
        "[{'option': '0', 'identifier': 'notioalOutstanding', 'name': 'Notional Outstanding', 'acronym': 'NT', 'description': 'Interest accrues on the basis of the notional outstanding.\r'}, {'option': '1', 'identifier': 'notionalAtInitialExchange', 'name': 'Notional at Initial Exchange', 'acronym': 'NTIED', 'description': 'Interest accrues on the basis of the notional value at initial exchange.\r'}, {'option': '2', 'identifier': 'notionalLagged', 'name': 'Notional Outstanding Lagged', 'acronym': 'NTL', 'description': 'Interest accrues on the basis of the lagged notional outstanding.'}]"
    }
    fn get_default_value(&self) -> &str {
        "NT"
    }
    fn get_description(&self) -> &str {
        "This is important for amortizing instruments. The basis of interest calculation is normally the notional outstanding amount as per SD. This is considered the fair basis and in many countries the only legal basis. If NULL or NTSD is selected, this is the case. 
Alternative bases (normally in order to favor the lending institution) are found. In the extreme case the original balance (PCDD=NT+PDCDD) never gets adjusted. In this case PCDD must be chosen. 
An intermediate case exist wherre balances do get adjusted, however with lags. In this case NTL mut be selected and anchor dates and cycles must be set."
    }
}    
   
