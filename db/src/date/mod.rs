use log::warn;

pub fn parse(date: &str) -> Option<chrono::NaiveDateTime> {
    if let Some(date_time) = parse_with_time(date) {
        return Some(date_time);
    }
    if let Some(date_time) = parse_date(date) {
        return date_time.and_hms_opt(0, 0, 0); // Default time to 00:00:00
    }
    warn!("Unable to parse date: {}", date);

    None
}

pub fn parse_with_time(date: &str) -> Option<chrono::NaiveDateTime> {
    let common_date_formats = [
        "%Y/%m/%d %H:%M:%S", // 2023/05/12 14:30:00
        "%m/%d/%Y %H:%M:%S", // 05/12/2023 14:30:00
        "%d-%m-%Y %H:%M:%S", // 12-05-2023 14:30:00
        "%d/%m/%Y %H:%M:%S", // 12/05/2023 14:30:00
        "%Y-%m-%d %H:%M:%S", // 2023-05-12 14:30:00
        "%Y.%m.%d %H:%M:%S", // 2023.05.12 14:30:00
        "%d.%m.%Y %H:%M:%S", // 12.05.2023 14:30:00
    ];

    for format in common_date_formats.iter() {
        if let Ok(date_time) = chrono::NaiveDateTime::parse_from_str(date, format) {
            return Some(date_time);
        }
    }
    warn!("Unable to parse date with time: {}", date);
    None
}

pub fn parse_date(date: &str) -> Option<chrono::NaiveDate> {
    let common_date_formats = [
        "%Y/%m/%d", // 2023/05/12
        "%m/%d/%Y", // 05/12/2023
        "%d-%m-%Y", // 12-05-2023
        "%d/%m/%Y", // 12/05/2023
        "%Y-%m-%d", // 2023-05-12
        "%Y.%m.%d", // 2023.05.12
        "%d.%m.%Y", // 12.05.2023
    ];

    for format in common_date_formats.iter() {
        if let Ok(date_time) = chrono::NaiveDate::parse_from_str(date, format) {
            return Some(date_time);
        }
    }
    warn!("Unable to parse date: {}", date);
    None
}
