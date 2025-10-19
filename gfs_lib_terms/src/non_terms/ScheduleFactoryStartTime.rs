use crate::define_struct_isodatetime;


define_struct_isodatetime!(StartTime);

// use gfs_lib_types::types::IsoDatetime::IsoDatetime;
// use crate::terms::grp_reset_rate::CycleAnchorDateOfRateReset::CycleAnchorDateOfRateReset;
//
// pub trait IntoScheduleFactoryStartTime {
//     fn into_schedule_factory_start_time(self) -> ScheduleFactoryStartTime;
// }
//
// impl IntoScheduleFactoryStartTime for CycleAnchorDateOfRateReset {
//     fn into_schedule_factory_start_time(self) -> ScheduleFactoryStartTime {
//         ScheduleFactoryStartTime::CycleAnchorDateOfRateReset(self)
//     }
// }
//
// pub enum ScheduleFactoryStartTime {
//     CycleAnchorDateOfRateReset(CycleAnchorDateOfRateReset)
// }
//
// impl ScheduleFactoryStartTime {
//     pub fn new<T>(authorized_isodatetime_type: T) -> Self
//     where
//         T: IntoScheduleFactoryStartTime,
//     {
//         authorized_isodatetime_type.into_schedule_factory_start_time()
//     }
// }