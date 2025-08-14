use crate::functions::pam::pof::POF_AD_PAM::POF_AD_PAM;
use crate::traits::TraitPayOffFunction::TraitPayOffFunction;

pub enum PayOffFunction {
    POF_AD_PAM(POF_AD_PAM)
}