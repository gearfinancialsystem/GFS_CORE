use gfs_lib_contract::attributes::ContractModel::ContractModel;
use gfs_lib_terms::phantom_terms::PhantomF64::PhantomF64W;
use gfs_lib_portfolio::portfolio::Portfolio::Portfolio;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;
use gfs_lib_terms::terms::grp_notional_principal::NotionalPrincipal::NotionalPrincipal;
use gfs_lib_types::traits::TraitConvert::IsoDateTimeConvertTo;
use crate::measures::core::core_structures::{DataTimeLineContract, DataTimeLineType, Measure, StructureType, TypeMeasure};

pub enum DailyNominalIndicator {
    SimpleTimeLineNominal
}
pub enum FinalNominalIndicator {
    AverageNominal
}

pub enum NominalIndicator {
    DailyMeasureIndicator(DailyNominalIndicator),
    FinalMeasureIndicator(FinalNominalIndicator),
}

// fn contract_dni_simple_time_line_nominal(dtl: DataTimeLineContract) -> Result<Measure, String> {
//     let res: Vec<(PhantomIsoDatetimeW, Option<f64>)> = vec![];
//     // faire un sort avant peut etre
//     for e in dtl.0.iter() {
//         res.push((e.0.convert::<PhantomIsoDatetimeW>(), {
//             // gerer le cas ou plusieurs events le meme jour
//             if e.1.len() == 1 {
//
//             }
//             else {
//                 for e in e.1.iter() {
//
//                 }
//             }
//         }))
//     }
// }


// pub fn nominal_measure(
//     structure_type: StructureType,
//     dtl_type: DataTimeLineType,
//     indicator: NominalIndicator,
// ) -> Result<Measure, String> {
//     match structure_type {
//         StructureType::Contract(cm) => {
//             match dtl_type {
//                 DataTimeLineType::DTLContract(v) => {
//                     match indicator {
//                         NominalIndicator::DailyMeasureIndicator(dni) => {
//                             match dni {
//                                 DailyNominalIndicator::SimpleTimeLineNominal => {
//
//                                 },
//                             }
//                         }
//                         NominalIndicator::FinalMeasureIndicator(fni) => {
//                             match fni {
//                                 FinalNominalIndicator::AverageNominal => {
//
//                                 }
//                             }
//                         }
//                     }
//
//                 },
//                 _ => Err("should be a DTLContract".to_string())
//             }
//         },
//         StructureType::Portfolio(pf) => {},
//         StructureType::SetPortfolio(spf) => {},
//     }
// }
//
//
