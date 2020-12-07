struct RequiredFields {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl RequiredFields {
    pub fn new() -> RequiredFields {
        RequiredFields {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    pub fn reset(&mut self) {
        self.byr = None;
        self.iyr = None;
        self.eyr = None;
        self.hgt = None;
        self.hcl = None;
        self.ecl = None;
        self.pid = None;
        self.cid = None;
    }

    pub fn add_fields_from_string(&mut self, passport: &str) {
        let fields = passport.split(' ');
        for field in fields {
            let split_index = field.find(':').expect(
                format!(
                    "Expected the following string to contain a colon: {}",
                    field
                )
                .as_str(),
            );
            self.add_field(&field[0..split_index], &field[split_index + 1..])
        }
    }

    pub fn add_field(&mut self, field: &str, value: &str) {
        match field {
            "byr" => self.byr = Some(value.to_string()),
            "iyr" => self.iyr = Some(value.to_string()),
            "eyr" => self.eyr = Some(value.to_string()),
            "hgt" => self.hgt = Some(value.to_string()),
            "hcl" => self.hcl = Some(value.to_string()),
            "ecl" => self.ecl = Some(value.to_string()),
            "pid" => self.pid = Some(value.to_string()),
            "cid" => self.cid = Some(value.to_string()),
            _ => return,
        }
    }

    pub fn is_valid(&self) -> bool {
        // cid is missing on purpose
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

pub fn part1(input: &str) -> usize {
    let mut required_fields = RequiredFields::new();
    let mut valid_count = 0;
    for input_line in input.lines() {
        if input_line.is_empty() {
            if required_fields.is_valid() {
                valid_count += 1;
            }
            required_fields.reset();
            continue;
        }
        required_fields.add_fields_from_string(input_line);
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn part1_demo() {
        assert_eq!(2, part1(INPUT));
    }
}
