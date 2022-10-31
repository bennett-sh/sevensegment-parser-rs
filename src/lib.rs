
use crate::utils::cmp_vec_unordered;
use std::str::FromStr;
use std::fmt::Debug;

mod utils;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum SSDPart {
  UpperTop = 0,
  UpperLeft = 1,
  UpperRight = 2,
  Middle = 3,
  LowerLeft = 4,
  LowerRight = 5,
  LowerBottom = 6,
  Dot = 7,
}

struct SSDChars {
  pub zero: Vec<SSDPart>,
  pub one: Vec<SSDPart>,
  pub two: Vec<SSDPart>,
  pub three: Vec<SSDPart>,
  pub four: Vec<SSDPart>,
  pub five: Vec<SSDPart>,
  pub six: Vec<SSDPart>,
  pub seven: Vec<SSDPart>,
  pub eight: Vec<SSDPart>,
  pub nine: Vec<SSDPart>,
  pub dot: Vec<SSDPart>,
  pub minus: Vec<SSDPart>,
}

impl SSDChars {
  pub fn default() -> Self {
    use SSDPart::*;

    return Self {
      zero: vec![ UpperTop, UpperLeft, UpperRight, LowerLeft, LowerRight, LowerBottom ],
      one: vec![ UpperRight, LowerRight ],
      two: vec![ UpperTop, UpperRight, Middle, LowerLeft, LowerBottom ],
      three: vec![ UpperTop, UpperRight, Middle, LowerRight, LowerBottom  ],
      four: vec![ UpperLeft, UpperRight, Middle, LowerRight ],
      five: vec![ UpperTop, UpperLeft, Middle, LowerRight, LowerBottom ],
      six: vec![ UpperTop, UpperLeft, Middle, LowerLeft, LowerRight, LowerBottom ],
      seven: vec![ UpperTop, UpperRight, LowerRight ],
      eight: vec![ UpperTop, UpperLeft, UpperRight, Middle, LowerLeft, LowerRight, LowerBottom ],
      nine: vec![ UpperTop, UpperLeft, UpperRight, Middle, LowerRight, LowerBottom ],
      dot: vec![ Dot ],
      minus: vec![ Middle ],
    }
  }
}

#[derive(Copy, Clone)]
pub struct SegmentValue {
  pub number: Option<u8>,
  pub character: Option<char>
}

impl SegmentValue {
  pub fn from_number(number: u8) -> Self {
    Self {
      number: Some(number),
      character: None
    }
  }

  pub fn from_char(character: char) -> Self {
    Self {
      number: None,
      character: Some(character)
    }
  }

  pub fn empty() -> Self {
    Self {
      number: None,
      character: None
    }
  }

  pub fn is_number(self: Self) -> bool {
      self.number.is_some()
  }

  pub fn is_char(self) -> bool {
    self.character.is_some()
  }
}

pub fn read_segments( segments: Vec<Vec<SSDPart>> ) -> std::io::Result<String> {
  let mut string = String::new();

  for segment in segments {
    let val = get_segment_value(segment).unwrap_or( SegmentValue::empty() );

    if val.is_char() {
      string.push(val.character.unwrap())
    }

    if val.is_number() {
      let num = val.number.unwrap();

      if num < 10 {
        string.push_str(num.to_string().as_str());
      }
    }
  }

  Ok(string)
}

pub struct UnknownNumber<T> {
  pub number: T,
  pub is_negative: bool
}

pub fn read_number<T>( segments: Vec<Vec<SSDPart>> ) -> std::io::Result<UnknownNumber<T>>
  where T: FromStr, <T as FromStr>::Err : Debug {
  let mut parsed_segments = read_segments(segments)?;
  let is_negative = parsed_segments.starts_with('-');

  if is_negative {
    parsed_segments = parsed_segments[1..].to_string()
  }
  
  Ok(
    UnknownNumber {
      number: parsed_segments.parse::<T>().unwrap(),
      is_negative
    }
  )
}

pub fn get_segment_value( segment: Vec<SSDPart> ) -> Option<SegmentValue> {
  let default_segments = SSDChars::default();

  if cmp_vec_unordered(&segment, default_segments.zero) {
    return Some(SegmentValue::from_number(0));
  }

  if cmp_vec_unordered(&segment, default_segments.one) {
    return Some(SegmentValue::from_number(1));
  }

  if cmp_vec_unordered(&segment, default_segments.two) {
    return Some(SegmentValue::from_number(2));
  }

  if cmp_vec_unordered(&segment, default_segments.three) {
    return Some(SegmentValue::from_number(3));
  }

  if cmp_vec_unordered(&segment, default_segments.four) {
    return Some(SegmentValue::from_number(4));
  }

  if cmp_vec_unordered(&segment, default_segments.five) {
    return Some(SegmentValue::from_number(5));
  }

  if cmp_vec_unordered(&segment, default_segments.six) {
    return Some(SegmentValue::from_number(6));
  }

  if cmp_vec_unordered(&segment, default_segments.seven) {
    return Some(SegmentValue::from_number(7));
  }

  if cmp_vec_unordered(&segment, default_segments.eight) {
    return Some(SegmentValue::from_number(8));
  }

  if cmp_vec_unordered(&segment, default_segments.nine) {
    return Some(SegmentValue::from_number(9));
  }
  
  if cmp_vec_unordered(&segment, default_segments.dot) {
    return Some(SegmentValue::from_char('.'))
  }
  
  if cmp_vec_unordered(&segment, default_segments.minus) {
    return Some(SegmentValue::from_char('-'))
  }

  None
}

