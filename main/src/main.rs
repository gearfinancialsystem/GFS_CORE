// use lib1::hello_from_lib1;
// use lib2::hello_from_lib2;
use lib_datetime_period::local_datetime::local_datetime::IsoDateTime;
use lib_datetime_period::period::period::IsoPeriod;
fn main() {
    println!("test");
    // println!("{}", hello_from_lib1());
    // println!("{}", hello_from_lib2());
    let startDate = IsoDateTime::new(2020, 1, 1,1,1,1).unwrap();
    let endDate = IsoDateTime::new(2023, 10, 8,1,1,1).unwrap();

    println!("Periode entre {:?} et {:?}", startDate, endDate);
    let p = IsoPeriod::between(startDate.0.date(), endDate.0.date());

        // Period.between(startDate.toLocalDate(), endDate.toLocalDate());
    println!("{:?} annees, {:?} mois, et {:?} jours.", p.get_years(), p.get_months(), p.get_days());
}



