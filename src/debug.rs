// Debug test script

pub mod types;
pub mod tag;
pub mod tags;

use types::*;

fn main() {
    let input = r#"<h1>hello world</h1>"#;

    let mut consumable: types::Queue = types::Queue::from(input);

    let _ = consumable.consume_all(
        vec![
            &tags::tag_scope_detector::TagScopeDetector::new(None, None, None, None, None),
        ]
    );
}