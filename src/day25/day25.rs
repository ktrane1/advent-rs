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

        self.val.clone().unwrap();

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
        }

        let mut next = &self.next;
        while next.is_some() {
            match next.as_ref() {
                Some(sd) => {
                    next = &sd.next;
                }
                None => {
                    next = &None;
                }
            };
        }

        next = &Some(Box::new(SnafuDigit::new_with_values(exponent, remainder)));
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
    let mut snaf = SnafuDigit::new();

    loop {
        let remainder = current % BASE;

        snaf.push(
            exponent,
            remainder.try_into().expect("could not coerce to i32"),
        );

        current = current / BASE;
        exponent += 1;
        if current == 0 {
            break;
        }
    }

    snaf.print_snaf();
}

pub fn execute(data: Vec<String>) {
    let result = data.iter().fold(0, |acc, snaf| {
        dbg!(acc.clone());
        return acc + parse(snaf.to_string());
    });
    dbg!(result);

    int_to_snafu();
}

// SNAFU
// base 5
// digits:
// 2 = 2
// 1 = 1
// 0 = 0
// = = -2
// - = -1
