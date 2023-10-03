use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

pub fn into_jst(utc: NaiveDateTime) -> DateTime<FixedOffset> {
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(utc, Utc);
    const JST: Option<FixedOffset> = FixedOffset::east_opt(9 * 3600);
    datetime.with_timezone(&JST.unwrap())
}
