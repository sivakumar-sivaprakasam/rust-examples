use std::time::SystemTime;
use chrono::{prelude::*, Days, Months};
use chrono_tz::Asia::Singapore;

fn main() {
    // Get current time using Standard library's SystemTime
    let curr_time = SystemTime::now();
    let dt: DateTime<Utc> = curr_time.clone().into();
    println!("Date/Time created using SystemTime: {}", dt.format("%d-%b-%Y %H:%M:%S %P %z"));

    // Get current date time using Chrono crate 
    let now = Utc::now();
    println!("Date/Time created using UTC: {}", now.format("%d-%b-%Y %H:%M:%S %P %z"));
    
    // Get current date time with specific timezone (ex: Singapore)
    let now = Utc::now().with_timezone(&Singapore);
    println!("Date/Time with specific timezone created using UTC: {}", now.format("%d-%b-%Y %H:%M:%S %P %z"));

    // Construct date time with given inputs
    let now = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
    println!("Custom Date/Time created using UTC: {}", now.format("%d-%b-%Y %H:%M:%S %P %z"));

    let now = Singapore.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
    println!("Custom Date/Time with specific timezone created using UTC: {}", now.format("%d-%b-%Y %H:%M:%S %P %z"));

    // String to Date/Time conversion
    let now = DateTime::parse_from_str("01-Jan-2023 12:00:00 pm +0800", "%d-%b-%Y %H:%M:%S %P %z").unwrap();
    println!("Parse Date/Time from String: {now:?}");

    let curr_date = NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").unwrap();
    println!("Parse Date from String: {curr_date:?}");

    let curr_date = NaiveDate::parse_from_str("01-01-2023", "%d-%m-%Y").unwrap();
    println!("Parse Date from String: {curr_date:?}");

    let curr_date = NaiveDate::parse_from_str("Jan-01-2023", "%b-%d-%Y").unwrap();
    println!("Parse Date from String: {curr_date:?}");

    let curr_time= NaiveTime::parse_from_str("12:00:00", "%H:%M:%S").unwrap();
    println!("Parse Time from String: {curr_time:?}");

    let curr_time= NaiveTime::parse_from_str("12:00:00+0800", "%H:%M:%S%z").unwrap();
    println!("Parse Time from String: {curr_time:?}");

    let now = DateTime::parse_from_rfc2822("Sun, 1 Jan 2023 12:00:00 +0800").unwrap();
    println!("Parse RFC2822 formatted Date/Time from String: {now:?}");

    let now = DateTime::parse_from_rfc3339("2023-01-01T12:00:00+08:00").unwrap();
    println!("Parse RFC3339 formatted Date/Time from String: {now:?}");

    // Timestamp to Date/Time conversion
    let now = DateTime::parse_from_str("01-Jan-2023 12:00:00 pm +0800", "%d-%b-%Y %H:%M:%S %P %z").unwrap();
    let now_from_timestamp = NaiveDateTime::from_timestamp_millis(now.timestamp_millis());
    println!("Create Date/Time from Timestamp: {now_from_timestamp:?}");
    
    // Date addition
    let new_date = now.checked_add_days(Days::new(10)).and_then(|i| i.checked_add_months(Months::new(1)));
    println!("Add days and months to a Date/Time: {new_date:?}");

    // Date subtraction
    let new_date = now.checked_sub_days(Days::new(10)).and_then(|i| i.checked_sub_months(Months::new(1)));
    println!("Subtract days and months from a Date/Time: {new_date:?}");

    // Date comparison
    let dt1 = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2023, 2, 1, 12, 0, 0).unwrap();
    if dt1 == dt2 {
        println!("{dt1:?} and {dt2:?} are same");
    } else {
        println!("{dt1:?} and {dt2:?} are NOT same");
    }

    // Formatting DateTime
    let curr_time = SystemTime::now();
    let dt: DateTime<Utc> = curr_time.clone().into();
    println!("Date in mm/dd/yy format: {}", dt.format("%D"));
    println!("Date in yyyy-mm-dd format: {}", dt.format("%F"));
    println!("Date in d-MMM-yyyy format: {}", dt.format("%v"));
    println!("Date in Day, dd-mm-yyyy format: {}", dt.format("%a, %d-%M-%Y"));
    println!("Date in Day, dd-mm-yyyy format: {}", dt.format("%A, %d-%M-%Y"));
    println!("Date in Full Week Day, dd-MMM-yyyy format: {}", dt.format("%A, %d-%b-%Y"));
    println!("Date in Full Week Day, dd-MMMM-yyyy format: {}", dt.format("%A, %d-%B-%Y"));
    println!("Day of the year: {}", dt.format("%j"));
    println!("Day of the year: {}, Week day (Starting with Sunday): {}", dt.format("%j"), dt.format("%u"));
    println!("Day of the year: {}, Week day (Starting with Monday): {}", dt.format("%j"), dt.format("%w"));
    println!("Time in 24-hour format HH:mm:ss: {}", dt.format("%H:%M:%S"));
    println!("Time in 12-hour format HH:mm:ss: {}", dt.format("%I:%M:%S"));
    println!("Time in 12-hour format HH:mm:ss <AM/PM>: {}", dt.format("%I:%M:%S %p"));
    println!("Time in 12-hour format HH:mm:ss <AM/PM> TZ: {}", dt.format("%I:%M:%S %p %:z"));
    println!("Date and Time: {}", dt.format("%c"));
    println!("Date and Time in ISO 8601/RFC 3339 format: {}", dt.format("%+"));
}
