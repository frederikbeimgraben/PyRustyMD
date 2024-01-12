// Word Detector
// -------------------
// Detects either a given word or any sequence at the start of the queue that only contains letters from a given alphabet.
// -------------------

use crate::{base::*, types::{Dict, Queue}};

#[derive(Debug, Clone)]
pub struct WordDetector {
    pub word: Option<String>,
    pub alphabet: Option<Vec<char>>,
    pub inverted: Option<bool>
}

impl WordDetector {
    pub fn new(word: Option<String>, alphabet: Option<Vec<char>>, inverted: Option<bool>) -> Self {
        Self {
            word,
            alphabet,
            inverted
        }
    }
}

impl Detectable for WordDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        match (&self.word, &self.alphabet) {
            (Some(word), None) => {
                let mut word_queue = Queue::from_string(word.to_string());

                let mut result = Result::new(Detector::WordDetector(self.clone()), None, None, None);

                let mut inner: Queue = vec![];

                while !word_queue.is_empty() {
                    if queue.is_empty() {
                        return None;
                    }

                    let word_char = word_queue.remove(0);
                    let queue_char = queue.remove(0);

                    if word_char != queue_char {
                        return None;
                    }

                    inner.push(queue_char);
                }

                result.properties = Some(
                    Dict::from(
                        vec![
                            ("word".to_string(), &word.clone())
                        ]
                    )
                );

                Some(result)
            },
            (None, Some(alphabet)) => {
                let mut result = Result::new(Detector::WordDetector(self.clone()), None, None, None);

                let mut inner: Queue = vec![];

                while !queue.is_empty() {
                    if self.inverted.unwrap_or(false) == alphabet.contains(&queue[0]) {
                        break;
                    }

                    inner.push(queue.remove(0));
                }

                if inner.is_empty() {
                    return None;
                }

                result.content = Some(inner);

                result.properties = Some(
                    Dict::from(
                        vec![
                            ("alphabet".to_string(), &alphabet.clone())
                        ]
                    )
                );

                Some(result)
            },
            _ => None
        }
    }
}

impl PartialEq for WordDetector {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word && self.alphabet == other.alphabet && self.inverted == other.inverted
    }
}

// Alphabets
pub const WHITESPACES: [char; 4] = [' ', '\n', '\t', '\r'];

pub const VALID_IDENTIFIER_TOKENS: [char; 63] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'X', 'Y', 'Z', '-',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

pub const VALID_NUMBER_TOKENS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

pub const STRING_DELIMITERS: [char; 1] = [
    '"'
];

// Predefined Detectors
pub fn whitespace_detector() -> WordDetector {
    WordDetector::new(None, Some(WHITESPACES.to_vec()), None)
}

pub fn identifier_detector() -> WordDetector {
    WordDetector::new(None, Some(VALID_IDENTIFIER_TOKENS.to_vec()), None)
}

pub fn numeric_detector() -> WordDetector {
    WordDetector::new(None, Some(VALID_NUMBER_TOKENS.to_vec()), None)
}

#[cfg(test)]
mod tests {
    use crate::types::Queue;

    use super::*;

    #[test]
    fn test_word_detector() {
        let queue = Queue::from_string("Hello World".to_string());

        let (matched, word, _) = queue.clone().consume(&Detector::WordDetector(WordDetector::new(Some("Hello".to_string()), None, None)));

        assert_eq!(matched, true);
        assert_eq!(word.unwrap(), "Hello".to_string());

        let (matched, _, _) = queue.clone().consume(&Detector::WordDetector(WordDetector::new(Some("World".to_string()), None, None)));

        assert_eq!(matched, false);

        let (matched, word, _) = queue.clone().consume(&Detector::WordDetector(WordDetector::new(None, Some(VALID_IDENTIFIER_TOKENS.to_vec()), None)));

        assert_eq!(matched, true);
        assert_eq!(word.unwrap(), "Hello".to_string());

        let queue = Queue::from_string("   ".to_string());

        let (matched, word, _) = queue.clone().consume(&Detector::WordDetector(WordDetector::new(None, Some(WHITESPACES.to_vec()), None)));

        assert_eq!(matched, true);
        assert_eq!(word.unwrap(), "   ".to_string());

        let queue = Queue::from_string("".to_string());

        let (matched, _, _) = queue.clone().consume(&Detector::WordDetector(WordDetector::new(None, Some(WHITESPACES.to_vec()), None)));

        assert_eq!(matched, false);

        let (matched, _, _) = queue.clone().consume(&Detector::WordDetector(WordDetector::new(None, Some(VALID_IDENTIFIER_TOKENS.to_vec()), None)));

        assert_eq!(matched, false);
    }
}