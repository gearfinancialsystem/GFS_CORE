use std::str::FromStr;
use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::functions::pam::pof::POF_AD_PAM::POF_AD_PAM;
use crate::functions::pam::pof::POF_CD_PAM::POF_CD_PAM;
use crate::functions::pam::pof::POF_CE_PAM::POF_CE_PAM;
use crate::functions::pam::pof::POF_FP_PAM::POF_FP_PAM;
use crate::functions::pam::pof::POF_IED_PAM::POF_IED_PAM;
use crate::functions::pam::pof::POF_IP_PAM::POF_IP_PAM;
use crate::functions::pam::pof::POF_IPCI_PAM::POF_IPCI_PAM;
use crate::functions::pam::pof::POF_MD_PAM::POF_MD_PAM;
use crate::functions::pam::pof::POF_PP_PAM::POF_PP_PAM;
use crate::functions::pam::pof::POF_PRD_PAM::POF_PRD_PAM;
use crate::functions::pam::pof::POF_PY_PAM::POF_PY_PAM;
use crate::functions::pam::pof::POF_RR_PAM::POF_RR_PAM;
use crate::functions::pam::pof::POF_RRF_PAM::POF_RRF_PAM;
use crate::functions::pam::pof::POF_RRY_PAM::POF_RRY_PAM;
use crate::functions::pam::pof::POF_SC_PAM::POF_SC_PAM;
use crate::functions::pam::pof::POF_TD_PAM::POF_TD_PAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;

#[derive(Clone)]
pub enum PayOffFunctionPAM {
    POF_AD_PAM(POF_AD_PAM),
    POF_CD_PAM(POF_CD_PAM),
    POF_CE_PAM(POF_CE_PAM),
    POF_FP_PAM(POF_FP_PAM),
    POF_IED_PAM(POF_IED_PAM),
    POF_IP_PAM(POF_IP_PAM),
    POF_IPCI_PAM(POF_IPCI_PAM),
    POF_MD_PAM(POF_MD_PAM),
    POF_PP_PAM(POF_PP_PAM),
    POF_PRD_PAM(POF_PRD_PAM),
    POF_PY_PAM(POF_PY_PAM),
    POF_RR_PAM(POF_RR_PAM),
    POF_RRF_PAM(POF_RRF_PAM),
    POF_RRY_PAM(POF_RRY_PAM),
    POF_SC_PAM(POF_SC_PAM),
    POF_TD_PAM(POF_TD_PAM),
}


impl PayOffFunctionPAM {
    pub fn from_str(func: &str) -> PayOffFunctionPAM     {
        match func {
            "POF_AD_PAM" => Self::POF_AD_PAM(POF_AD_PAM::new()),
            "POF_CD_PAM" => Self::POF_CD_PAM(POF_CD_PAM::new()),
            "POF_CE_PAM" => Self::POF_CE_PAM(POF_CE_PAM::new()),
            "POF_FP_PAM" => Self::POF_FP_PAM(POF_FP_PAM::new()),
            "POF_IED_PAM" => Self::POF_IED_PAM(POF_IED_PAM::new()),
            "POF_IP_PAM" => Self::POF_IP_PAM(POF_IP_PAM::new()),
            "POF_IPCI_PAM" => Self::POF_IPCI_PAM(POF_IPCI_PAM::new()),
            "POF_MD_PAM" => Self::POF_MD_PAM(POF_MD_PAM::new()),
            "POF_PP_PAM" => Self::POF_PP_PAM(POF_PP_PAM::new()),
            "POF_PRD_PAM" => Self::POF_PRD_PAM(POF_PRD_PAM::new()),
            "POF_PY_PAM" => Self::POF_PY_PAM(POF_PY_PAM::new()),
            "POF_RR_PAM" => Self::POF_RR_PAM(POF_RR_PAM::new()),
            "POF_RRF_PAM" => Self::POF_RRF_PAM(POF_RRF_PAM::new()),
            "POF_RRY_PAM" => Self::POF_RRY_PAM(POF_RRY_PAM::new()),
            "POF_SC_PAM" => Self::POF_SC_PAM(POF_SC_PAM::new()),
            "POF_TD_PAM" => Self::POF_TD_PAM(POF_TD_PAM::new()),
            _ => panic!("Unknown function {}", func)
        }
    }



    pub fn eval(&self,
            time: &PhantomIsoDatetimeW,
            states: &StatesSpace,
            contract_terms: &ContractTerms,
            contract_structure: &Option<RelatedContracts>,
            risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
            day_counter: &Option<DayCountConvention>,
            time_adjuster: &BusinessDayAdjuster,
        ) -> f64 {
        match self {
            PayOffFunctionPAM::POF_AD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_CD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_CE_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_FP_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_IED_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_IP_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_IPCI_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_MD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_PP_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_PRD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_PY_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_RR_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_RRF_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_RRY_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_SC_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            PayOffFunctionPAM::POF_TD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}
