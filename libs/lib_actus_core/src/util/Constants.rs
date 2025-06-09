use chrono::Duration;

pub struct Constants;
impl Constants {
    pub const MAX_LIFETIME: Duration = Duration::seconds(50 * 365 * 24 * 60 * 60); // 50 years in seconds
    pub const MAX_LIFETIME_STK: Duration = Duration::seconds(10 * 365 * 24 * 60 * 60); // 10 years in seconds
    pub const MAX_LIFETIME_UMP: Duration = Duration::seconds(10 * 365 * 24 * 60 * 60); // 10 years in seconds
}
