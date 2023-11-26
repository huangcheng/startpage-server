use chrono::prelude::*;
use chrono::Duration;
use regex::Regex;

use crate::errors::ServiceError;

pub fn parse_duration(expires: &str) -> Result<Duration, ServiceError> {
    let re = Regex::new(r"^(\d+)([smhdwMy])$")?;

    let caps = match re.captures(expires) {
        Some(caps) => caps,
        None => {
            return Err(ServiceError::FormatError(
                "Invalid expires format".to_string(),
            ))
        }
    };

    let num = match caps.get(1) {
        Some(num) => num.as_str().parse::<usize>()?,
        None => {
            return Err(ServiceError::FormatError(
                "Invalid expires format".to_string(),
            ))
        }
    };

    let unit = match caps.get(2) {
        Some(unit) => unit.as_str(),
        None => {
            return Err(ServiceError::FormatError(
                "Invalid expires format".to_string(),
            ))
        }
    };

    let offset = match unit {
        "s" => Duration::seconds(num as i64),
        "m" => Duration::minutes(num as i64),
        "h" => Duration::hours(num as i64),
        "d" => Duration::days(num as i64),
        "w" => Duration::weeks(num as i64),
        "M" => Duration::days(num as i64 * 30),
        "y" => Duration::days(num as i64 * 365),
        _ => {
            return Err(ServiceError::FormatError(
                "Invalid expires format".to_string(),
            ))
        }
    };

    Ok(offset)
}

pub fn calculate_expires(expires: &str) -> Result<i64, ServiceError> {
    let now = Utc::now();
    let offset = parse_duration(expires)?;

    Ok((now + offset).timestamp())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_expires() {
        assert_eq!(
            calculate_expires("10s").unwrap(),
            (Utc::now() + Duration::seconds(10)).timestamp()
        );
        assert_eq!(
            calculate_expires("1m").unwrap(),
            (Utc::now() + Duration::minutes(1)).timestamp()
        );
        assert_eq!(
            calculate_expires("1h").unwrap(),
            (Utc::now() + Duration::hours(1)).timestamp()
        );
        assert_eq!(
            calculate_expires("1d").unwrap(),
            (Utc::now() + Duration::days(1)).timestamp()
        );
        assert_eq!(
            calculate_expires("1w").unwrap(),
            (Utc::now() + Duration::weeks(1)).timestamp()
        );
        assert_eq!(
            calculate_expires("1M").unwrap(),
            (Utc::now() + Duration::days(30)).timestamp()
        );
        assert_eq!(
            calculate_expires("1y").unwrap(),
            (Utc::now() + Duration::days(365)).timestamp()
        );
    }
}
