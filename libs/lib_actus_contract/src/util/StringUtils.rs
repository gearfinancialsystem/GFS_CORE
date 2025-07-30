

pub struct StringUtils;
// Contains constants representing String-conventions
impl StringUtils {
    // cycle stubs
    pub const LONG_STUB: char = '0';
    pub const SHORT_STUB: char = '1';

    // day count conventions
    pub const DAY_COUNT_CONVENTION_AAISDA: &'static str = "AA";
    pub const DAY_COUNT_CONVENTION_A360: &'static str = "A360";
    pub const DAY_COUNT_CONVENTION_A365: &'static str = "A365";
    pub const DAY_COUNT_CONVENTION_B252: &'static str = "B252";
    pub const DAY_COUNT_CONVENTION_30E360: &'static str = "30E360";
    pub const DAY_COUNT_CONVENTION_30E360ISDA: &'static str = "30E360ISDA";
    pub const DAY_COUNT_CONVENTION_A336: &'static str = "A336";
    pub const DAY_COUNT_CONVENTION_28336: &'static str = "28336";

    // calc/shift conventions
    pub const CALC_SHIFT_CONVENTION_CS: &'static str = "CS";
    pub const CALC_SHIFT_CONVENTION_SC: &'static str = "SC";
}
