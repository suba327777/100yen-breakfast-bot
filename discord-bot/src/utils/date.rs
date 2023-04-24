use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Asia::Tokyo, Tz};

pub fn date_now_jst() -> DateTime<Tz> {
    let utc = Utc::now().naive_utc();
    Tokyo.from_utc_datetime(&utc)
}
