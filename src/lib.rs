// fn rgb(r: i32, g: i32, b: i32) -> String {
//     let validate_range  = |val| match Some(val) {
//         Some(x) if x > 255 => 255,
//         Some(x) if x < 0   => 0,
//           _                => val,
//     };
//     format!("{:02x}{:02x}{:02x}",validate_range(r),validate_range(g),validate_range(b)).to_ascii_uppercase()
// }

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{0:02X}{1:02X}{2:02X}", r.min(255).max(0), g.min(255).max(0), b.min(255).max(0))
}

macro_rules! compare {
  ( $got : expr, $expected : expr ) => {
    if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
  };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
