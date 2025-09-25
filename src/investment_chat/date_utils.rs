use crate::investment_chat::InvestmentChatError;
use chrono::{NaiveDate, Datelike};

/// Format date string to the format expected by the API (dd-mm-yyyy)
pub fn format_date_for_api(date_str: &str) -> Result<String, InvestmentChatError> {
    // Try to parse standard formats first
    if let Ok(date) = parse_flexible_date(date_str) {
        let day_padded = format!("{:02}", date.day());
        let month_padded = format!("{:02}", date.month());
        let year = date.year();
        
        return Ok(format!("{}-{}-{}", day_padded, month_padded, year));
    }
    
    Err(InvestmentChatError::InvalidInput(
        format!("Unrecognized date format: {}. Please use DD-MM-YYYY or DD Month YYYY.", date_str)
    ))
}

/// Parse flexible date formats including "15 September 2025"
pub fn parse_flexible_date(date_str: &str) -> Result<NaiveDate, InvestmentChatError> {
    // Try common formats
    let formats = [
        "%Y-%m-%d",     // YYYY-MM-DD
        "%d-%m-%Y",     // DD-MM-YYYY
        "%m-%d-%Y",     // MM-DD-YYYY
        "%d/%m/%Y",     // DD/MM/YYYY
        "%m/%d/%Y",     // MM/DD/YYYY
        "%d.%m.%Y",     // DD.MM.YYYY
        "%m.%d.%Y",     // MM.DD.YYYY
        "%d-%m-%y",     // DD-MM-YY
        "%m-%d-%y",     // MM-DD-YY
        "%d/%m/%y",     // DD/MM/YY
        "%m/%d/%y",     // MM/DD/YY
    ];
    
    for format in formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }
    
    // Try to parse formats like "15 September 2025"
    let parts: Vec<&str> = date_str.split_whitespace().collect();
    if parts.len() == 3 {
        // Try to parse the day
        let day = match parts[0].parse::<u32>() {
            Ok(d) => d,
            Err(_) => return Err(InvestmentChatError::InvalidInput(
                format!("Invalid day in date: {}", date_str)
            )),
        };
        
        // Try to parse the month name
        let month = match parts[1].to_lowercase().as_str() {
            "january" => 1,
            "february" => 2,
            "march" => 3,
            "april" => 4,
            "may" => 5,
            "june" => 6,
            "july" => 7,
            "august" => 8,
            "september" => 9,
            "october" => 10,
            "november" => 11,
            "december" => 12,
            _ => return Err(InvestmentChatError::InvalidInput(
                format!("Invalid month name in date: {}", date_str)
            )),
        };
        
        // Try to parse the year
        let year = match parts[2].parse::<i32>() {
            Ok(y) => y,
            Err(_) => return Err(InvestmentChatError::InvalidInput(
                format!("Invalid year in date: {}", date_str)
            )),
        };
        
        // Basic validation
        if day < 1 || day > 31 || year < 2000 || year > 2100 {
            return Err(InvestmentChatError::InvalidInput(
                format!("Invalid date values in: {}. Day must be 1-31, year 2000-2100.", date_str)
            ));
        }
        
        // Create a NaiveDate
        match NaiveDate::from_ymd_opt(year, month, day) {
            Some(date) => return Ok(date),
            None => return Err(InvestmentChatError::InvalidInput(
                format!("Invalid date: {}", date_str)
            )),
        }
    }
    
    Err(InvestmentChatError::InvalidInput(
        format!("Unrecognized date format: {}. Please use DD-MM-YYYY or DD Month YYYY.", date_str)
    ))
}
