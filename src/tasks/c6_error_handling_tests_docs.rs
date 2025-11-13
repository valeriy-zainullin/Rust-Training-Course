// This chapter is dedicated to the error handling, tests and documentation.

// RESULT
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `first_char(text: &str) -> Result<char, String>` that returns the first
// character of a string or an error message "Empty string" if the string is empty.

pub fn first_char(text: &str) -> Result<char, String> {
    match text.chars().next() {
        None => Result::Err(String::from("Empty string")),
        Some(chr) => Result::Ok(chr),
    }
}

// ----- 2 --------------------------------------
// Write a function `read_numbers_from_str(line: &str) -> Result<Vec<i32>, String>` that reads a
// line of integers separated by whitespace and parses each integer as i32. In case the value cannot
// be parsed (if it is not an integer) return the `Err("Invalid number")` result.

pub fn read_numbers_from_str(line: &str) -> Result<Vec<i32>, String> {
    let numbers_iter = line
        .split_whitespace()
        .map(|word| word.parse::<i32>());
    let mut numbers = Vec::<i32>::new();
    for number in numbers_iter {
        match number {
            Ok(num) => {
                numbers.push(num);
            }

            Err(_) => {
                return Err(String::from("Invalid number"));
            }
        }
    }

    Ok(numbers)
}

// OPTION
// ================================================================================================

// ----- 3 --------------------------------------
// You have a struct `UserProfile` with fields `username: String` and `email: Option<String>`.
//
// Implement a method `get_email_domain(&self) -> Option<String>` that:
// - If the email exists, extracts the domain (the part after @).
// - If the email is missing, returns `None`.

// IMPLEMENT HERE:
pub struct UserProfile {
    #[allow(dead_code)]
    username: String,
    email: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        UserProfile { username, email }
    }

    pub fn get_email_domain(&self) -> Option<String> {
        match &self.email {
            None => None,
            Some(address) => Some(address.split_once('@')?.1.to_string()),
        }
    }
}

// WRITING TESTS
// ================================================================================================

// ----- 4 --------------------------------------
// Write unit tests for the `factorial(n: u32) -> u64` function.

fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod factorial_tests {
    // IMPLEMENT HERE:

    use crate::tasks::c6_error_handling_tests_docs::factorial;

    pub fn test_factorial() {
        assert!(factorial(0) == 1);
        assert!(factorial(1) == 1);
        assert!(factorial(2) == 2);
        assert!(factorial(3) == 6);
        assert!(factorial(4) == 24);
        assert!(factorial(5) == 120);
        assert!(factorial(6) == 720);
    }
}

// ----- 5 --------------------------------------
// Write unit tests for the `is_prime(n: u64) -> bool` function checking both prime and non-prime
// numbers.

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod prime_tests {
    // IMPLEMENT HERE:

    use crate::tasks::c6_error_handling_tests_docs::is_prime;

    pub fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(is_prime(11));
    }
}

// WRITING DOCS
// ================================================================================================

// ----- 6 --------------------------------------
// You have an implemented `TemperatureLog` struct below, which stores a city name and a list of
// daily temperature readings. This struct have a constructor, an `add_reading` method which just
// ads a new value to the `readings` vector and an `average` method which returns an average value
// of the readings of there are some.
//
// Your task is to add doc comments:
// - High-level purpose of the struct.
// - Inline docs for each field and method.
//
// In case you want something more than хор(5):
// - Additionally white the usage example for the `TemperatureLog` in the high-level docs.
// - For the `average` method additionally write an example of its usage.

/// Stores daily history of tempreature (in degrees Celsius) for a city
/// 
/// #### Examples
/// 
/// ```
/// use crate::tasks::c6_error_handling_tests_docs::TemperatureLog;
/// let mut log = TemperatureLog::new("Cheboksary");
/// log.add_reading(12.0);
/// log.add_reading(14.0);
/// log.add_reading(16.0);
/// log.add_reading(18.0);
/// println!("{}", log.average().unwrap_or(f64::NAN));
/// ```
#[allow(dead_code)]
pub struct TemperatureLog {
    /// Name of the city readings are made for
    pub city: String,
    
    /// Reading of temperature for the city in degrees Celsius
    pub readings: Vec<f64>,
}

#[allow(dead_code)]
impl TemperatureLog {
    /// Creates a new log for the specified city
    /// 
    /// #### Examples
    /// 
    /// ```
    /// use crate::tasks::c6_error_handling_tests_docs::TemperatureLog;
    /// let mut log = TemperatureLog::new("Cheboksary");
    /// ```
    pub fn new(city: &str) -> Self {
        Self {
            city: city.to_string(),
            readings: Vec::new(),
        }
    }

    /// Adds a daily reading in degrees Celsius into log
    /// 
    /// #### Examples
    /// 
    /// ```
    /// use crate::tasks::c6_error_handling_tests_docs::TemperatureLog;
    /// let mut log = TemperatureLog::new("Cheboksary");
    /// log.add_reading(12.0);
    /// ```
    pub fn add_reading(&mut self, value: f64) {
        self.readings.push(value);
    }

    /// Computes average daily temperature in degrees Celsius
    /// 
    /// #### Examples
    /// 
    /// ```
    /// use crate::tasks::c6_error_handling_tests_docs::TemperatureLog;
    /// let mut log = TemperatureLog::new("Cheboksary");
    /// log.add_reading(12.0);
    /// log.add_reading(14.0);
    /// log.add_reading(16.0);
    /// log.add_reading(18.0);
    /// println!("{}", log.average().unwrap_or(f64::NAN)); // Prints 15
    /// ```
    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum_of_readings: f64 = self.readings.iter().sum();
        Some(sum_of_readings / self.readings.len() as f64)
    }
}

mod test {
    use core::f64;

    #[test]
    pub fn test_temperature_log_docs() {
        use crate::tasks::c6_error_handling_tests_docs::TemperatureLog;
        let mut log = TemperatureLog::new("Cheboksary");
        log.add_reading(12.0);
        log.add_reading(14.0);
        log.add_reading(16.0);
        log.add_reading(18.0);
        assert!((log.average().unwrap_or(f64::NAN) - 15.0).abs() < 1e-8);
    }
}
