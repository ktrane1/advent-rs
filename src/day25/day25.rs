#[derive(Debug, Clone)]
struct SnafuDigit {
    val: Option<String>,
    next: Option<Box<SnafuDigit>>,
    exponent: Option<i32>,
    decimal_value: Option<i32>,
}

impl SnafuDigit {
    pub fn new() -> SnafuDigit {
        SnafuDigit {
            val: None,
            next: None,
            exponent: None,
            decimal_value: None,
        }
    }

    pub fn new_with_values(exponent: i32, remainder: i32) -> SnafuDigit {
        SnafuDigit {
            val: None,
            next: None,
            exponent: Some(exponent),
            decimal_value: Some(remainder),
        }
    }

    fn print_snaf(&mut self) {
        let mut d: Vec<String> = Vec::new();

        d.push(self.val.clone().unwrap().to_string());

        let mut next = &self.next;

        while next.is_some() {
            match next.as_ref() {
                Some(sd) => {
                    d.push(sd.val.clone().unwrap().to_string());
                    next = &sd.next;
                }
                None => {
                    next = &None;
                }
            };
        }

        d.reverse();

        let s = d
            .iter()
            .fold(String::from(""), |acc, d| format!("{}{}", acc, d));

        dbg!(s);
    }

    fn print_decimal(&mut self) {
        let mut d: Vec<String> = Vec::new();

        d.push(self.decimal_value.clone().unwrap().to_string());

        let mut next = &self.next;

        while next.is_some() {
            match next.as_ref() {
                Some(sd) => {
                    d.push(sd.decimal_value.clone().unwrap().to_string());
                    next = &sd.next;
                }
                None => {
                    next = &None;
                }
            };
        }

        d.reverse();

        let s = d
            .iter()
            .fold(String::from(""), |acc, d| format!("{}{}", acc, d));

        dbg!(s);
    }

    pub fn push(&mut self, exponent: i32, remainder: i32) {
        if self.is_empty() {
            self.exponent = Some(exponent);
            self.decimal_value = Some(remainder);
            return;
        };

        let n = &mut None::<Box<SnafuDigit>>;

        let mut next = &mut self.next;
        while next.is_some() {
            match next {
                Some(sd) => {
                    next = &mut sd.next;
                }
                _ => {
                    next = n;
                }
            };
        }

        let _ = next.insert(Box::new(SnafuDigit::new_with_values(exponent, remainder)));
    }

    pub fn is_empty(&mut self) -> bool {
        if self.val.is_none()
            && self.exponent.is_none()
            && self.decimal_value.is_none()
            && self.next.is_none()
        {
            return true;
        }
        return false;
    }

    // make decimal values to snaf digits
    pub fn encode(&mut self) {
        process(self);

        if self.next.is_none() {
            return;
        }

        let mut next = self.next.as_deref_mut().unwrap();

        process(next);

        while next.next.is_some() {
            next = next.next.as_deref_mut().unwrap();
            process(next);
        }
    }

    pub fn inc_next(&mut self) {
        if self.next.as_deref_mut().is_none() {
            self.push(self.exponent.unwrap() + 1, 1);
        } else {
            match self.next.as_deref_mut().unwrap().decimal_value {
                Some(n) => {
                    self.next.as_deref_mut().unwrap().decimal_value = Some(n + 1);
                }
                _ => {}
            }
        }
    }
}

fn process(snafu_digit: &mut SnafuDigit) {
    match snafu_digit.decimal_value {
        Some(num) => match num {
            n if n < 3 => {
                snafu_digit.val = Some(n.to_string());
            }
            3 => {
                snafu_digit.val = Some("=".to_string());
                snafu_digit.inc_next();
            }
            4 => {
                snafu_digit.val = Some("-".to_string());
                snafu_digit.inc_next();
            }
            n if n > 4 => {
                snafu_digit.val = Some("0".to_string());
                snafu_digit.inc_next();
            }
            unknown => panic!("digit out of range: {:?}", unknown),
        },
        None => {
            dbg!("dec value is none");
        }
    }
}

fn parse(s: String) -> i64 {
    const BASE: i64 = 5;
    let mut cs = s.chars().collect::<Vec<char>>();
    cs.reverse();

    let mut sum: i64 = 0;

    for (i, c) in cs.iter().enumerate() {
        let multiplier = BASE.pow(i.try_into().expect("overflow usize -> u32"));

        match c {
            '1' => sum += 1 * multiplier,
            '2' => sum += 2 * multiplier,
            '0' => sum += 0 * multiplier,
            '=' => sum += -2 * multiplier,
            '-' => sum += -1 * multiplier,
            _ => todo!(),
        }
    }

    return sum;
}

fn int_to_snafu(decimal: i64) {
    const BASE: i64 = 5;
    let mut exponent: i32 = 0;
    let mut current = decimal / BASE;
    let mut remainder = decimal % BASE;
    let mut snaf = SnafuDigit::new();

    while current > 0 || remainder > 0 {
        snaf.push(
            exponent,
            remainder.try_into().expect("could not coerce to i32"),
        );

        remainder = current % BASE;
        current = current / BASE;
        exponent += 1;
    }

    snaf.print_decimal();
    snaf.encode();
    snaf.print_snaf();
}

pub fn execute(data: Vec<String>) {
    let result = data.iter().fold(0, |acc, snaf| {
        return acc + parse(snaf.to_string());
    });

    int_to_snafu(result);
}

// SNAFU
// base 5
// digits:
// 2 = 2
// 1 = 1
// 0 = 0
// = = -2
// - = -1
//
// 2=01 = 1301
// 1-12 = 412
