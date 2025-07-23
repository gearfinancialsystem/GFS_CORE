use std::fmt;
use std::rc::Rc;

use crate::types::IsoDatetime::IsoDatetime;
use chrono::Datelike;
use crate::terms::grp_notional_principal::MaturityDate::MaturityDate;
use crate::traits::TraitCountConvention::TraitDayCountConvention;

use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
use crate::types::IsoDatetime::TraitNaiveDateTimeExtension;


#[derive(Clone, PartialEq, Debug)]
pub struct E30360ISDA {
    pub maturity_date: Option<Rc<MaturityDate>>,
}

impl E30360ISDA {
    pub fn new(maturity_date: Option<Rc<MaturityDate>>) -> Self {
        E30360ISDA {maturity_date}
    }
}

impl TraitDayCountConvention for E30360ISDA {

    fn day_count(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        // d1
        let mut d1 = start_time.day();
        if start_time.is_last_day_of_month() {
            d1 = 30;
        }

        // d2
        let mut d2 = end_time.day();
        // Vérification du cas : si end_time == maturity_date et c'est un mois de février => pas d'ajustement
        let is_february = end_time.month() == 2;
        if self.maturity_date.is_some() {
            let a = self.maturity_date.clone().map(|rc| (*rc).clone()).unwrap();
            let maturity =  {
                // Vérifier end_time == maturityDate ET mois = 2 => on n'ajuste pas d2
                if end_time == a.value() && is_february {
                    // pas d'ajustement, on laisse d2
                }
                else if end_time.is_last_day_of_month() {
                    d2 = 30;
                }
            };
        }
        else {
    // Pas de maturité => la règle "dernier jour du mois => d2 = 30"
            if end_time.is_last_day_of_month() {
                d2 = 30;
            }
        }

        let del_d = (d2 as f64) - (d1 as f64);
        let del_m = (end_time.month() as i32) - (start_time.month() as i32);
        let del_y = end_time.year() - start_time.year();

        // Formule standard 30E/360
        360.0 * (del_y as f64) + 30.0 * (del_m as f64) + del_d
    }

    /// Calcule la fraction d'année (dayCount / 360.0)
    fn day_count_fraction(&self, start_time: IsoDatetime, end_time: IsoDatetime) -> f64 {
        self.day_count(start_time, end_time) / 360.0
    }
}
impl fmt::Display for E30360ISDA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "E30360ISDA")
    }
}



#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use chrono::NaiveDateTime;
    use crate::traits::TraitMarqueurIsoDatetime::TraitMarqueurIsoDatetime;
    use super::E30360ISDA;
    //use super::E30360ISDA;
    fn parse_date(date_str: &str) -> MaturityDate {

        let a = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse date");
        MaturityDate::new(IsoDatetime(a)).ok().unwrap()
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_1() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start1 = parse_date("2006-01-31T00:00:00");
        let end1 = parse_date("2006-02-28T00:00:00");
        let result = 30.0;
        assert_eq!(result, convention.day_count(start1.value(), end1.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_2() {

        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start2 = parse_date("2006-01-30T00:00:00");
        let end2 = parse_date("2006-02-28T00:00:00");
        let result = 30.0;
        assert_eq!(result, convention.day_count(start2.value(), end2.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_3() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start3 = parse_date("2006-02-28T00:00:00");
        let end3 = parse_date("2006-03-03T00:00:00");
        let result = 3.0;
        assert_eq!(result, convention.day_count(start3.value(), end3.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_4() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start4 = parse_date("2006-02-14T00:00:00");
        let end4 = parse_date("2006-02-28T00:00:00");
        let result = 16.0;
        assert_eq!(result, convention.day_count(start4.value(), end4.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_5() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start5 = parse_date("2006-09-30T00:00:00");
        let end5 = parse_date("2006-10-31T00:00:00");
        let result = 30.0;
        assert_eq!(result, convention.day_count(start5.value(), end5.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_6() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start6 = parse_date("2006-10-31T00:00:00");
        let end6 = parse_date("2006-11-28T00:00:00");
        let result = 28.0;
        assert_eq!(result, convention.day_count(start6.value(), end6.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_7() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start7 = parse_date("2007-08-31T00:00:00");
        let end7 = parse_date("2008-02-28T00:00:00");
        let result = 178.0;
        assert_eq!(result, convention.day_count(start7.value(), end7.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_8() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start8 = parse_date("2008-02-28T00:00:00");
        let end8 = parse_date("2008-08-28T00:00:00");
        let result = 180.0;
        assert_eq!(result, convention.day_count(start8.value(), end8.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_9() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start9 = parse_date("2008-02-28T00:00:00");
        let end9 = parse_date("2008-08-30T00:00:00");
        let result = 182.0;
        assert_eq!(result, convention.day_count(start9.value(), end9.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_10() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start10 = parse_date("2008-02-28T00:00:00");
        let end10 = parse_date("2008-08-31T00:00:00");
        let result = 182.0;
        assert_eq!(result, convention.day_count(start10.value(), end10.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_11() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));;

        let start11 = parse_date("2007-02-28T00:00:00");
        let end11 = parse_date("2008-02-28T00:00:00");
        let result = 358.0;
        assert_eq!(result, convention.day_count(start11.value(), end11.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_12() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));;

        let start12 = parse_date("2007-02-28T00:00:00");
        let end12 = parse_date("2008-02-29T00:00:00");
        let result = 359.0;
        assert_eq!(result, convention.day_count(start12.value(), end12.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_13() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start13 = parse_date("2008-02-29T00:00:00");
        let end13 = parse_date("2009-02-28T00:00:00");
        let result = 360.0;
        assert_eq!(result, convention.day_count(start13.value(), end13.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_14() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start14 = parse_date("2008-02-29T00:00:00");
        let end14 = parse_date("2008-03-30T00:00:00");
        let result = 30.0;
        assert_eq!(result, convention.day_count(start14.value(), end14.value()) as f64);
    }

    #[test]
    fn test_daycount_thirty_e_three_sixty_isda_15() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));;

        let start15 = parse_date("2008-02-29T00:00:00");
        let end15 = parse_date("2008-03-31T00:00:00");
        let result = 30.0;
        assert_eq!(result, convention.day_count(start15.value(), end15.value()) as f64);
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_1() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start1 = parse_date("2006-01-31T00:00:00");
        let end1 = parse_date("2006-02-28T00:00:00");
        let result = 30.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start1.value(), end1.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_2() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start2 = parse_date("2006-01-30T00:00:00");
        let end2 = parse_date("2006-02-28T00:00:00");
        let result = 30.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start2.value(), end2.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_3() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start3 = parse_date("2006-02-28T00:00:00");
        let end3 = parse_date("2006-03-03T00:00:00");
        let result = 3.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start3.value(), end3.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_4() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start4 = parse_date("2006-02-14T00:00:00");
        let end4 = parse_date("2006-02-28T00:00:00");
        let result = 16.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start4.value(), end4.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_5() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start5 = parse_date("2006-09-30T00:00:00");
        let end5 = parse_date("2006-10-31T00:00:00");
        let result = 30.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start5.value(), end5.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_6() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start6 = parse_date("2006-10-31T00:00:00");
        let end6 = parse_date("2006-11-28T00:00:00");
        let result = 28.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start6.value(), end6.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_7() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start7 = parse_date("2007-08-31T00:00:00");
        let end7 = parse_date("2008-02-28T00:00:00");
        let result = 178.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start7.value(), end7.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_8() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start8 = parse_date("2008-02-28T00:00:00");
        let end8 = parse_date("2008-08-28T00:00:00");
        let result = 180.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start8.value(), end8.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_9() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start9 = parse_date("2008-02-28T00:00:00");
        let end9 = parse_date("2008-08-30T00:00:00");
        let result = 182.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start9.value(), end9.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_10() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start10 = parse_date("2008-02-28T00:00:00");
        let end10 = parse_date("2008-08-31T00:00:00");
        let result = 182.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start10.value(), end10.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_11() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start11 = parse_date("2007-02-28T00:00:00");
        let end11 = parse_date("2008-02-28T00:00:00");
        let result = 358.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start11.value(), end11.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_12() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start12 = parse_date("2007-02-28T00:00:00");
        let end12 = parse_date("2008-02-29T00:00:00");
        let result = 359.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start12.value(), end12.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_13() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start13 = parse_date("2008-02-29T00:00:00");
        let end13 = parse_date("2009-02-28T00:00:00");
        let result = 360.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start13.value(), end13.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_14() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start14 = parse_date("2008-02-29T00:00:00");
        let end14 = parse_date("2008-03-30T00:00:00");
        let result = 30.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start14.value(), end14.value()));
    }

    #[test]
    fn test_fraction_thirty_e_three_sixty_isda_15() {
        let mat_date = parse_date("2008-02-29T00:00:00");
        let mut convention = E30360ISDA::new(Some(Rc::new(mat_date)));

        let start15 = parse_date("2008-02-29T00:00:00");
        let end15 = parse_date("2008-03-31T00:00:00");
        let result = 30.0 / 360.0;
        assert_eq!(result, convention.day_count_fraction(start15.value(), end15.value()));
    }
}
