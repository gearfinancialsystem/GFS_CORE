use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use serde_json::to_string;
use gfs_lib_contract::attributes::ContractModel::ContractModel;
use gfs_lib_contract::traits::TraitExternalData::TraitExternalData;
use gfs_lib_contract::traits::TraitExternalEvent::TraitExternalEvent;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_types::types::Value::Value;
// statis qui n'implique pas l'execution des contrats
// pour les stats qui implique l'éxécution des contrats voir Measures

pub struct BasicStatistics {
    pub number_of_contracts: u32,
    pub number_of_assets_contracts: u32,
    pub number_of_liability_contracts: u32,
    pub number_of_fixed_rate_contracts: u32,
    pub number_of_floating_rate_contracts: u32,
    pub number_of_pam_contracts: u32,
    pub number_of_swppv_contracts: u32,
    pub number_of_stk_contracts: u32,
    pub number_of_nam_contracts: u32,
    pub number_of_lam_contracts: u32,
    pub number_of_fxout_contracts: u32,
    pub number_of_ann_contracts: u32,
}

pub struct BasicStatisticsAll {
    pub stats: BasicStatistics,
}

pub struct BasicStatisticsActiveAtStatusDate {
    pub stats: BasicStatistics,
}

pub struct Portfolio {
    pub portfolio_id: String, // creer un terme
    pub contracts: HashMap<ContractID, ContractModel>,
    pub risk_factor_external_data: Option<Arc<dyn TraitExternalData>>,
    pub risk_factor_external_event: Option<Arc<dyn TraitExternalEvent>>,
    pub curr_event_index: i32,
    pub status_date: Option<StatusDate>,
    pub basic_statistics_all: Option<BasicStatisticsAll>,
    pub basic_statistics_active_at_status_date: Option<BasicStatisticsActiveAtStatusDate>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: "init".to_string(),
            contracts: HashMap::new(),
            risk_factor_external_data: None,
            risk_factor_external_event: None,
            curr_event_index: -1,
            status_date: None,
            basic_statistics_all: None,
            basic_statistics_active_at_status_date: None,
        }
    }

    pub fn load_contract_constituent(&self, sm: HashMap<String, Value>) -> Result<(ContractID, ContractModel), String> {
        let cm = ContractModel::new(
            sm.clone(),
            self.risk_factor_external_data.clone(),
            self.risk_factor_external_event.clone(),
        );
        match cm {
            Ok(cmv) => Ok((cmv.get_contract_identifiant(), cmv)),
            Err(e) => Err(e),
        }
    }

    pub fn init_portfolio(&mut self, portfolio_id: String, sm: Vec<HashMap<String, Value>>) {
        self.portfolio_id = portfolio_id;
        for e in sm.iter() {

            let cm = self.load_contract_constituent(e.clone());
            let (id, cmv) = cm.unwrap();
            self.contracts.insert(id, cmv);

        }
    }

}

