// HTML Detector Implementations
// ------------------------

pub mod div;
pub mod span;
pub mod img;
pub mod a;
pub mod p;
pub mod h;

use crate::base::*;
use crate::types::Queue;

use div::div_detector;
use span::span_detector;
use img::img_detector;
use a::a_detector;
use p::p_detector;
use h::h_detector;

// Base HTML Detector
// ------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum HTMLDetector {
    DivDetector,
    SpanDetector,
    ImgDetector,
    LinkDetector,
    ParagraphDetector,
    HeadingDetector
}

impl Detectable for HTMLDetector {
    fn detect(&self, queue: &mut Queue) -> Option<Result> {
        match self {
            HTMLDetector::DivDetector => div_detector().detect(queue),
            HTMLDetector::SpanDetector => span_detector().detect(queue),
            HTMLDetector::ImgDetector => img_detector().detect(queue),
            HTMLDetector::LinkDetector => a_detector().detect(queue),
            HTMLDetector::ParagraphDetector => p_detector().detect(queue),
            HTMLDetector::HeadingDetector => h_detector().detect(queue)
        }
    }
}

