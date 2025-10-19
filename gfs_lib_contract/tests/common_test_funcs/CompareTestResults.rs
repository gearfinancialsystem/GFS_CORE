use std::ops::Mul;
use gfs_lib_contract::util::ResultsStruct::TestResult;

pub fn compare_test_results(computed_test_res: &Vec<TestResult>, expected_test_res: &Vec<TestResult>) -> bool {

    for (cr, er) in computed_test_res.iter().zip(expected_test_res.iter()){

        let a_payoff = cr.payoff.parse::<f64>().unwrap().mul(100000000.0).floor();
        let b_payoff = er.payoff.parse::<f64>().unwrap().mul(100000000.0).floor();
        let a_prin = cr.notionalPrincipal.parse::<f64>().unwrap().mul(100000000.0).floor();
        let b_prin = er.notionalPrincipal.parse::<f64>().unwrap().mul(100000000.0).floor();
        let a_inte = cr.nominalInterestRate.parse::<f64>().unwrap().mul(100000000.0).floor();
        let b_inte = er.nominalInterestRate.parse::<f64>().unwrap().mul(100000000.0).floor();
        let a_accr = cr.accruedInterest.parse::<f64>().unwrap().mul(100000000.0).floor();
        let b_accr = er.accruedInterest.parse::<f64>().unwrap().mul(100000000.0).floor();
        if
            cr.eventDate != er.eventDate ||
            cr.eventType != er.eventType ||
            a_payoff != b_payoff ||
            cr.currency != er.currency ||
            a_prin != b_prin ||
            a_inte != b_inte ||
            a_accr != b_accr  {

            println!("EventDate: {:?} | {:?} | {:?}", cr.eventDate, er.eventDate, cr.eventDate != er.eventDate);
            println!("EventType: {:?} | {:?} | {:?}", cr.eventType, er.eventType, cr.eventType != er.eventType);
            println!("Payoff: {:?} | {:?} | {:?}", a_payoff, b_payoff, a_payoff != b_payoff);
            println!("Currency: {:?} | {:?} | {:?}", cr.currency, er.currency, cr.currency != er.currency);
            println!("NotionalPrincipal: {:?} | {:?} | {:?}",a_prin ,b_prin, a_prin != b_prin );
            println!("NominalInterestRate: {:?} | {:?} | {:?}",a_inte ,b_inte, a_inte != b_inte );
            println!("AccruedInterest: {:?} | {:?} | {:?}", a_accr, b_accr, a_accr != b_accr );
            return false;
        }
    }
    true
}