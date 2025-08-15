use lib_actus_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use lib_actus_terms::terms::grp_calendar::BusinessDayAdjuster::BusinessDayAdjuster;
use lib_actus_terms::terms::grp_interest::DayCountConvention::DayCountConvention;
use crate::attributes::ContractReference::ContractReference;
use crate::attributes::ContractTerms::ContractTerms;
use crate::attributes::RelatedContracts::RelatedContracts;
use crate::functions::lam::stf::STF_AD_LAM::STF_AD_LAM;
use crate::functions::lam::stf::STF_CD_LAM::STF_CD_LAM;
use crate::functions::lam::stf::STF_CE_LAM::STF_CE_LAM;
use crate::functions::lam::stf::STF_FP_LAM::STF_FP_LAM;
use crate::functions::lam::stf::STF_IED_LAM::STF_IED_LAM;
use crate::functions::lam::stf::STF_IP_LAM::STF_IP_LAM;
use crate::functions::lam::stf::STF_IPBC_LAM::STF_IPCB_LAM;
use crate::functions::lam::stf::STF_IPCI2_LAM::STF_IPCI2_LAM;
use crate::functions::lam::stf::STF_IPCI_LAM::STF_IPCI_LAM;
use crate::functions::lam::stf::STF_MD_LAM::STF_MD_LAM;
use crate::functions::lam::stf::STF_PP_LAM::STF_PP_LAM;
use crate::functions::lam::stf::STF_PR2_LAM::STF_PR2_LAM;
use crate::functions::lam::stf::STF_PR_LAM::STF_PR_LAM;
use crate::functions::lam::stf::STF_PRD_LAM::STF_PRD_LAM;
use crate::functions::lam::stf::STF_PY_LAM::STF_PY_LAM;
use crate::functions::lam::stf::STF_RR_LAM::STF_RR_LAM;
use crate::functions::lam::stf::STF_RRF_LAM::STF_RRF_LAM;
use crate::functions::lam::stf::STF_SC_LAM::STF_SC_LAM;
use crate::functions::lam::stf::STF_TD_LAM::STF_TD_LAM;
use crate::functions::pam::StatesTransitionFunctionPAM::StatesTransitionFunctionPAM;
use crate::states_space::StatesSpace::StatesSpace;
use crate::traits::TraitExternalData::TraitExternalData;
use crate::traits::TraitStateTransitionFunction::TraitStateTransitionFunction;

#[derive(Clone)]
pub enum StatesTransitionFunctionLAM {
    STF_AD_LAM(STF_AD_LAM),
    STF_CD_LAM(STF_CD_LAM),
    STF_CE_LAM(STF_CE_LAM),
    STF_FP_LAM(STF_FP_LAM),
    STF_IED_LAM(STF_IED_LAM),
    STF_IP_LAM(STF_IP_LAM),
    STF_IPCI_LAM(STF_IPCI_LAM),
    STF_IPCI2_LAM(STF_IPCI2_LAM),
    STF_IPCB_LAM(STF_IPCB_LAM),
    
    STF_MD_LAM(STF_MD_LAM),
    STF_PP_LAM(STF_PP_LAM),
    STF_PR_LAM(STF_PR_LAM),
    STF_PR2_LAM(STF_PR2_LAM),
    STF_PRD_LAM(STF_PRD_LAM),
    STF_PY_LAM(STF_PY_LAM),
    STF_RR_LAM(STF_RR_LAM),
    STF_RRF_LAM(STF_RRF_LAM),
    STF_SC_LAM(STF_SC_LAM),
    STF_TD_LAM(STF_TD_LAM),
}
impl StatesTransitionFunctionLAM {
    pub fn from_str(func: &str) ->  StatesTransitionFunctionLAM {
        match func {
            "STF_AD_LAM" => Self::STF_AD_LAM(STF_AD_LAM::new()),
            "STF_CD_LAM" => Self::STF_CD_LAM(STF_CD_LAM::new()),
            "STF_CE_LAM" => Self::STF_CE_LAM(STF_CE_LAM::new()),
            "STF_FP_LAM" => Self::STF_FP_LAM(STF_FP_LAM::new()),
            "STF_IED_LAM" => Self::STF_IED_LAM(STF_IED_LAM::new()),
            "STF_IP_LAM" => Self::STF_IP_LAM(STF_IP_LAM::new()),
            "STF_IPCI_LAM" => Self::STF_IPCI_LAM(STF_IPCI_LAM::new()),
            "STF_IPCI2_LAM" => Self::STF_IPCI2_LAM(STF_IPCI2_LAM::new()),
            "STF_IPCB_LAM" => Self::STF_IPCB_LAM(STF_IPCB_LAM::new()),
            "STF_MD_LAM" => Self::STF_MD_LAM(STF_MD_LAM::new()),
            "STF_PP_LAM" => Self::STF_PP_LAM(STF_PP_LAM::new()),
            "STF_PR_LAM" => Self::STF_PR_LAM(STF_PR_LAM::new()),
            "STF_PR2_LAM" => Self::STF_PR2_LAM(STF_PR2_LAM::new()),
            "STF_PRD_LAM" => Self::STF_PRD_LAM(STF_PRD_LAM::new()),
            "STF_PY_LAM" => Self::STF_PY_LAM(STF_PY_LAM::new()),
            "STF_RR_LAM" => Self::STF_RR_LAM(STF_RR_LAM::new()),
            "STF_RRF_LAM" => Self::STF_RRF_LAM(STF_RRF_LAM::new()),
            "STF_SC_LAM" => Self::STF_SC_LAM(STF_SC_LAM::new()),
            "STF_TD_LAM" => Self::STF_TD_LAM(STF_TD_LAM::new()),
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
            StatesTransitionFunctionLAM::STF_AD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_CD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_CE_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_FP_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_IED_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_IP_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_IPCI_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_IPCI2_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_IPCB_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_MD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_PP_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_PRD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_PR_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_PR2_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_PY_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_RR_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_RRF_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_SC_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
            StatesTransitionFunctionLAM::STF_TD_LAM(v) => v.eval(
                time, states, contract_terms, contract_structure,
                risk_factor_external_data, day_counter, time_adjuster
            ),
        }
    }
}