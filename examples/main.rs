
use sevensegment_parser::{read_number, get_segment_value, parse_parts, parts_to_number};

fn main() -> std::io::Result<()> {
  use sevensegment_parser::SSDPart::*;

  println!("test: {}", parts_to_number(vec![
    Middle, UpperLeft, UpperRight, LowerRight
  ]));

  for part in parse_parts(102334) {
    print!("{:?}, ", part)
  }

  println!(
    "\n{}",
    get_segment_value(vec![UpperLeft, UpperTop, UpperRight, Middle, LowerLeft, LowerRight, LowerBottom])
      .unwrap()
      .number
      .unwrap()
  );

  println!(
    "{}",
    get_segment_value(vec![UpperLeft, UpperRight, LowerRight, Middle])
      .unwrap()
      .number
      .unwrap()
  );

  let result = read_number::<i32>(
    vec![
      vec![ Middle ],
      vec![UpperLeft, UpperTop, UpperRight, Middle, LowerLeft, LowerRight, LowerBottom],
      vec![UpperLeft, UpperRight, LowerRight, Middle],
    ]
  ).unwrap();

  println!(
    "{}",
    result.number * if result.is_negative { -1 } else { 1 }
  );

  let result1 = read_number::<f32>(
    vec![
      vec![ UpperLeft, UpperTop, UpperRight, Middle, LowerLeft, LowerRight, LowerBottom ],
      vec![ Dot ],
      vec![UpperLeft, UpperRight, LowerRight, Middle],
    ]
  ).unwrap();

  println!(
    "{}",
    result1.number
  );

  Ok(())
}
