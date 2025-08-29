#![allow(non_camel_case_types)]

use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::pam::stf::STF_AD_PAM::STF_AD_PAM;
use crate::functions::pam::stf::STF_CD_PAM::STF_CD_PAM;
use crate::functions::pam::stf::STF_CE_PAM::STF_CE_PAM;
use crate::functions::pam::stf::STF_FP_PAM::STF_FP_PAM;
use crate::functions::pam::stf::STF_IED_PAM::STF_IED_PAM;
use crate::functions::pam::stf::STF_IP_PAM::STF_IP_PAM;
use crate::functions::pam::stf::STF_IPCI_PAM::STF_IPCI_PAM;
use crate::functions::pam::stf::STF_MD_PAM::STF_MD_PAM;
use crate::functions::pam::stf::STF_PP_PAM::STF_PP_PAM;
use crate::functions::pam::stf::STF_PRD_PAM::STF_PRD_PAM;
use crate::functions::pam::stf::STF_PY_PAM::STF_PY_PAM;
use crate::functions::pam::stf::STF_RR_PAM::STF_RR_PAM;
use crate::functions::pam::stf::STF_RRF_PAM::STF_RRF_PAM;
use crate::functions::pam::stf::STF_SC_PAM::STF_SC_PAM;
use crate::functions::pam::stf::STF_TD_PAM::STF_TD_PAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionPAM {

    STF_AD_PAM(STF_AD_PAM),
    STF_CD_PAM(STF_CD_PAM),
    STF_CE_PAM(STF_CE_PAM),
    STF_FP_PAM(STF_FP_PAM),
    STF_IED_PAM(STF_IED_PAM),
    STF_IP_PAM(STF_IP_PAM),
    STF_IPCI_PAM(STF_IPCI_PAM),
    STF_MD_PAM(STF_MD_PAM),
    STF_PP_PAM(STF_PP_PAM),
    STF_PRD_PAM(STF_PRD_PAM),
    STF_PY_PAM(STF_PY_PAM),
    STF_RR_PAM(STF_RR_PAM),
    STF_RRF_PAM(STF_RRF_PAM),
    //STF_RRY_PAM(STF_RRY_PAM),
    STF_SC_PAM(STF_SC_PAM),
    STF_TD_PAM(STF_TD_PAM),
}

impl StatesTransitionFunctionPAM {
    pub fn from_str(func: &str) -> StatesTransitionFunctionPAM {
        match func {
            "STF_AD_PAM" => Self::STF_AD_PAM(STF_AD_PAM::new()),
            "STF_CD_PAM" => Self::STF_CD_PAM(STF_CD_PAM::new()),
            "STF_CE_PAM" => Self::STF_CE_PAM(STF_CE_PAM::new()),
            "STF_FP_PAM" => Self::STF_FP_PAM(STF_FP_PAM::new()),
            "STF_IED_PAM" => Self::STF_IED_PAM(STF_IED_PAM::new()),
            "STF_IP_PAM" => Self::STF_IP_PAM(STF_IP_PAM::new()),
            "STF_IPCI_PAM" => Self::STF_IPCI_PAM(STF_IPCI_PAM::new()),
            "STF_MD_PAM" => Self::STF_MD_PAM(STF_MD_PAM::new()),
            "STF_PP_PAM" => Self::STF_PP_PAM(STF_PP_PAM::new()),
            "STF_PRD_PAM" => Self::STF_PRD_PAM(STF_PRD_PAM::new()),
            "STF_PY_PAM" => Self::STF_PY_PAM(STF_PY_PAM::new()),
            "STF_RR_PAM" => Self::STF_RR_PAM(STF_RR_PAM::new()),
            "STF_RRF_PAM" => Self::STF_RRF_PAM(STF_RRF_PAM::new()),
            "STF_SC_PAM" => Self::STF_SC_PAM(STF_SC_PAM::new()),
            "STF_TD_PAM" => Self::STF_TD_PAM(STF_TD_PAM::new()),
            _ => panic!("Unknown function {}", func)
        }
    }
    pub fn eval(&self,
                time: &PhantomIsoDatetimeW,
                states: &mut StatesSpace,
                contract_terms: &ContractTerms,
                contract_structure: &Option<RelatedContracts>,
                risk_factor_external_data: &Option<Box<dyn TraitExternalData>>,
                day_counter: &Option<DayCountConvention>,
                time_adjuster: &BusinessDayAdjuster,
    ) {
        match self {
            StatesTransitionFunctionPAM::STF_AD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_CD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_CE_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_FP_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_IED_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_IP_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_IPCI_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_MD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_PP_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_PRD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_PY_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_RR_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_RRF_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_SC_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionPAM::STF_TD_PAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}