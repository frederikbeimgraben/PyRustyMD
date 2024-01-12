// Detect numbers in a string
// -------------------------
// Detects valid numbers in a string.
// Four types of numbers are supported:
// - Integers
// - Floats
// Each positive or any

use crate::{base::*, types::{Dict, Value, Queue}};

#[derive(Debug, Clone)]
pub enum NumberType {
    Integer,
    Float
}

#[derive(Debug, Clone)]
pub struct NumberDetector {
    pub number_type: Option<NumberType>,
    pub positive: Option<bool>,
}

impl PartialEq for NumberType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumberType::Integer, NumberType::Integer) => true,
            (NumberType::Float, NumberType::Float) => true,
            _ => false
        }
    }
}

impl NumberDetector {
    pub fn new(number_type: Option<NumberType>, positive: Option<bool>) -> Self {
        Self {
            number_type,
            positive
        }
    }
}

impl Detectable for NumberDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        let mut inner: Queue = vec![];

        let factor;

        if queue[0] == '-' {
            if self.positive.unwrap_or(false) {
                return None;
            }

            inner.push(
                queue.remove(0)
            );

            factor = -1;
        } else {
            factor = 1;
        }

        let found_type;

        match &self.number_type {
            Some(number_type) => {
                found_type = number_type;

                match number_type {
                    NumberType::Integer => {
                        let mut found = false;

                        while !queue.is_empty() {
                            let queue_char = queue[0];

                            if queue_char.is_digit(10) {
                                found = true;

                                inner.push(
                                    queue.remove(0)
                                );
                            } else {
                                break;
                            }
                        }

                        if !found {
                            return None;
                        }
                    },
                    NumberType::Float => {
                        let mut found = false;

                        while !queue.is_empty() {
                            let queue_char = queue[0];

                            if queue_char.is_digit(10) {
                                found = true;

                                inner.push(
                                    queue.remove(0)
                                );
                            } else if queue_char == '.' {
                                found = true;

                                inner.push(
                                    queue.remove(0)
                                );
                            } else {
                                break;
                            }
                        }

                        if !found {
                            return None;
                        }
                    }
                }
            },
            None => {
                let mut found = false;
                let mut found_dot = false;

                while !queue.is_empty() {
                    let queue_char = queue[0];

                    if queue_char.is_digit(10) {
                        found = true;

                        inner.push(
                            queue.remove(0)
                        );
                    } else if queue_char == '.' {
                        if found_dot {
                            break;
                        }

                        found_dot = true;

                        inner.push(
                            queue.remove(0)
                        );
                    } else {
                        break;
                    }
                }

                if !found {
                    return None;
                }

                if found_dot {
                    found_type = &NumberType::Float;
                } else {
                    found_type = &NumberType::Integer;
                }
            }
        }

        let mut number = 0;
        let mut float_counter: i32 = 0;
        let mut passed_dot = false;

        for queue_char in inner.clone() {
            match queue_char {
                '.' => {
                    passed_dot = true;
                },
                '-' => {},
                _ => {
                    number *= 10;
                    number += queue_char.to_digit(10).unwrap() as i32;

                    if passed_dot {
                        float_counter += 1;
                    }
                }
            }
        }

        let float_factor = 10_i32.pow(float_counter as u32);

        let number: Value = match found_type {
            NumberType::Integer => Value::Integer((number * factor) as i64),
            NumberType::Float => Value::Float((number as f64) * (factor as f64) / (float_factor as f64))
        };

        let found_type_string: String = match found_type {
            NumberType::Integer => String::from("integer"),
            NumberType::Float => String::from("float")
        };

        Some(
            Result::new(
                Detector::NumberDetector(self.clone()),
                None,
                Some(
                    Dict::from_values(
                        vec![
                            (String::from("number_type"), Value::String(found_type_string)),
                            (String::from("number"), number)
                        ]
                    )
                ),
                None
            )
        )
    }
}

impl PartialEq for NumberDetector {
    fn eq(&self, other: &Self) -> bool {
        self.number_type == other.number_type &&
        self.positive == other.positive
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Queue, Value, Dict};

    use super::*;

    #[test]
    fn test_number_detector_int() {
        let mut queue = Queue::from_string(String::from("123"));

        let number_detector = NumberDetector::new(
            None,
            None
        );

        let (matched, _, result) = queue.consume(&Detector::NumberDetector(number_detector.clone()));

        assert_eq!(matched, true);
        match result {
            Some(result) => {
                assert_eq!(result.properties, Some(Dict::from_values(vec![
                    (String::from("number_type"), Value::String(String::from("integer"))),
                    (String::from("number"), Value::Integer(123))
                ])));
            },
            None => {
                assert_eq!(true, false);
            }
        }

        let mut queue = Queue::from_string(String::from("-123"));

        let (matched, _, result) = queue.consume(&Detector::NumberDetector(number_detector.clone()));

        assert_eq!(matched, true);
        match result {
            Some(result) => {
                assert_eq!(result.properties, Some(Dict::from_values(vec![
                    (String::from("number_type"), Value::String(String::from("integer"))),
                    (String::from("number"), Value::Integer(-123))
                ])));
            },
            None => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn test_number_detector_float() {
        let mut queue = Queue::from_string(String::from("123.456"));

        let number_detector = NumberDetector::new(
            None,
            None
        );

        let (matched, _, result) = queue.consume(&Detector::NumberDetector(number_detector));

        assert_eq!(matched, true);
        match result {
            Some(result) => {
                assert_eq!(result.properties, Some(Dict::from_values(vec![
                    (String::from("number_type"), Value::String(String::from("float"))),
                    (String::from("number"), Value::Float(123.456))
                ])));
            },
            None => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn test_number_detector_int_positive() {
        let mut queue = Queue::from_string(String::from("123"));

        let number_detector = NumberDetector::new(
            None,
            Some(true)
        );

        let (matched, _, result) = queue.consume(&Detector::NumberDetector(number_detector.clone()));

        assert_eq!(matched, true);
        match result {
            Some(result) => {
                assert_eq!(result.properties, Some(Dict::from_values(vec![
                    (String::from("number_type"), Value::String(String::from("integer"))),
                    (String::from("number"), Value::Integer(123))
                ])));
            },
            None => {
                assert_eq!(true, false);
            }
        }

        let mut queue = Queue::from_string(String::from("-123"));

        let (matched, _, result) = queue.consume(&Detector::NumberDetector(number_detector.clone()));

        assert_eq!(matched, false);
        assert_eq!(result, None);
    }

    #[test]
    fn test_number_detector_float_positive() {
        let mut queue = Queue::from_string(String::from("123.456"));

        let number_detector = NumberDetector::new(
            None,
            Some(true)
        );

        let (matched, _, result) = queue.consume(&Detector::NumberDetector(number_detector.clone()));

        assert_eq!(matched, true);
        match result {
            Some(result) => {
                assert_eq!(result.properties, Some(Dict::from_values(vec![
                    (String::from("number_type"), Value::String(String::from("float"))),
                    (String::from("number"), Value::Float(123.456))
                ])));
            },
            None => {
                assert_eq!(true, false);
            }
        }

        let mut queue = Queue::from_string(String::from("-123.456"));

        let (matched, _, result) = queue.consume(&Detector::NumberDetector(number_detector.clone()));

        assert_eq!(matched, false);
        assert_eq!(result, None);
    }
}