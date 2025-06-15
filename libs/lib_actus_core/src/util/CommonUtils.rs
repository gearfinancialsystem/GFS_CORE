use std::collections::HashMap;
// use crate::terms::grp_settlement::DeliverySettlement::S;



pub struct CommonUtils;


impl CommonUtils {

    // pub fn provide_box_f64(string_map: &HashMap<String, String>, key: &str) -> Box<Option<f64>>{
    //     string_map.get(key).cloned().map(|c| Box::new(c.parse::<f64>().ok())).unwrap_or_else(|| Box::new(None))
    // }
    pub fn provide_box_f64(string_map: &HashMap<String, String>, key: &str) -> Option<Box<f64>> {
        string_map.get(key).and_then(|s| s.parse::<f64>().ok()).map(Box::new)
    }
    pub fn provide_f64(string_map: &HashMap<String, String>, key: &str) -> Option<f64> {
        string_map.get(key).and_then(|s| s.parse::<f64>().ok())
    }
    // pub fn provide_box_string(string_map: &HashMap<String, String>, key: &str) -> Box<Option<String>> {
    //     string_map.get(key).cloned().map(|c| Box::new(Some(c))).unwrap_or_else(|| Box::new(None))
    // }
    pub fn provide_box_string(string_map: &HashMap<String, String>, key: &str) -> Option<Box<String>> {
        string_map.get(key).cloned().map(Box::new)
    }
    pub fn provide_string(string_map: &HashMap<String, String>, key: &str) -> Option<String> {
        string_map.get(key).cloned()
    }

    
    // pub fn is_none(value: &Option<AnyBox>) -> bool {
    //     match value {
    //         None => true,
    //         Some(v) => false,
    //     }
    // }
    // pub fn is_none_string(value: &Option<&String>) -> bool {
    //     match value {
    //         None => true,
    //         Some(v) => false,
    //     }
    // }

    // pub fn settlement_currency_fx_rate(
    //     risk_factor_model: &dyn RiskFactorModelTrait,
    //     model: &dyn ContractModelTrait,
    //     time: IsoDatetime,
    //     state: &StateSpace,
    // ) -> f64 {
    //     let settlement_currency = model.get_as("SettlementCurrency");
    //     let currency = model.get_as("Currency");

    //     let are_equal = match(settlement_currency, currency){
    //         (Some(a), Some(b)) => a.downcast_ref::<&str>() == b.downcast_ref::<&str>(), _ => false,
    //     };

    //     if CommonUtils::is_none(settlement_currency) || are_equal {
    //         1.0
    //     } else {
    //         let currency_pair = format!("{:?}/{:?}", currency.unwrap(), settlement_currency.unwrap());
    //         risk_factor_model.state_at(&currency_pair, &time, state, model)
    //     }
    // }
}