// il faut peut etre l'implémenter au niveau du contrat :
// il faut extraire toutes les datas de chaque itération, next, or apply_to
// car certaine mesures se font seulement lorsque l'ensemble de l'écoulement contractuel s'est déroulé
// il faut creer un structure pour récupérer tout le states space, et ainsi que le payoff de l'event
// (et les Event Types (dans l'ordre) ayant amenées et ce nouveau StateSpace (pour le payoff et nominal surtout)
// IL FAUT NECESSAIREMENT DES SERIES JOURNALIERES (calendaires ! 365/366 jours !) ce sont les jours reels qui
// importent ici
// et il faut gerer que N event puisse generer plusieur payoff (en faire la somme par status_date)


use std::collections::HashMap;
use gfs_lib_contract::attributes::ContractModel::ContractModel;
use gfs_lib_contract::events::ContractEvent::ContractEvent;
use gfs_lib_contract::states_space::StatesSpace::StatesSpace;
use gfs_lib_portfolio::portfolio::Portfolio::Portfolio;
use gfs_lib_terms::phantom_terms::PhantomIsoDatetime::PhantomIsoDatetimeW;
use gfs_lib_terms::terms::grp_contract_identification::ContractID::ContractID;
use gfs_lib_terms::terms::grp_contract_identification::StatusDate::StatusDate;

// a déplacer qqpart peut etre
pub enum StructureType {
    Contract(ContractModel),
    Portfolio(Portfolio),
    SetPortfolio(Vec<Portfolio>), // prevoir une vrai struct
}

pub struct Data {
    pub curr_states_space: StatesSpace,
    pub curr_contract_event: ContractEvent,
}

pub struct DataTimeLineContract(pub Vec<(StatusDate, Vec<Data>)>); // vec car par status date, il peux y avoir plusieur events (Vec<Data>)

pub struct DataTimeLinePortfolio(pub HashMap<ContractID, DataTimeLineContract>);

pub struct DataTimeLineSetPortfolio(pub HashMap<String, DataTimeLinePortfolio>); // String pour PortfolioID

pub enum DataTimeLineType {
    DTLContract(DataTimeLineContract),
    DTLPortfolio(DataTimeLinePortfolio),
    DTLSetPortfolio(DataTimeLineSetPortfolio),
}

// Doit etre construit pour contract, portfolio, set of portfolio, pour chaque 'daily measures'
// les 'daily measures' SONT les termes issus de la struct Data,
// on peut presque l'implémenter ici
pub struct DailyMesure(Vec<(PhantomIsoDatetimeW, Option<f64>)>);
pub struct FinalMeasure(Option<f64>);
pub enum TypeMeasure {
    DailyMeasure(DailyMesure),
    FinalMeasure(FinalMeasure),
}

pub struct Measure {
    pub measure_id: String,
    pub measure: TypeMeasure,

}


// DataTimeLine est pour un contrat,
// c'est sur la base de DataTimeLine que toutes les mesures sont calculés
