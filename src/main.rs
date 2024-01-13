// Main Script

// Defined Modules
pub mod base;
pub mod detectors;
pub mod advanced_detectors;
pub mod types;
pub mod html;

use crate::advanced_detectors::tag_scope_detector;
#[allow(unused_imports)]
use crate::{
    base::*,
    detectors::{
        number_detector::{NumberDetector, NumberType},
        word_detector::WordDetector,
        scope_detector::ScopeDetector
    },
    types::*
};

// Main Function
fn main() {
    // Get cli arguments
    let args: Vec<String> = std::env::args().collect();

    // Match the joined arguments
    let joined_args = args[1..].join(" ");

    // Create a consumable from the joined arguments
    let mut consumable: Queue = Consumable::from_string(joined_args);

    // Create a detector
    let detector = Detector::TagScopeDetector(
        tag_scope_detector::TagScopeDetector::new(None, None, None, None, None, None)
    );

    // Consume the consumable
    let result = consumable.consume_any(
        &vec![
            detector
        ]
    );

    // Print the result
    println!("{}", result.to_json());
}