struct Year {
    pub year: usize,
}
impl Year {
    pub fn is_valid_birth_year(&self) -> bool {
        1920 <= self.year && self.year <= 2002
    }
    pub fn is_valid_issue_year(&self) -> bool {
        2010 <= self.year && self.year <= 2020
    }
    pub fn is_valid_expiration_year(&self) -> bool {
        2020 <= self.year && self.year <= 2030
    }
}
enum MeasureLength {
    IN,
    CM,
}
struct Field {
    pub field: String,
}
impl Field {
    pub fn is_height(&self) -> bool {
        let index;
        let measure;
        match self.field.find("in") {
            Some(index_some) => {
                index = index_some;
                measure = MeasureLength::IN
            }
            None => match self.field.find("cm") {
                Some(index_some) => {
                    index = index_some;
                    measure = MeasureLength::CM
                }
                None => return false,
            },
        }
        let amount_result = self.field[0..index].parse::<usize>();
        if amount_result.is_err() {
            return false;
        }
        let amount = amount_result.unwrap();
        match measure {
            MeasureLength::IN => 59 <= amount && amount <= 76,
            MeasureLength::CM => 150 <= amount && amount <= 193,
        }
    }
}
struct RequiredFields {
    pub byr: Option<Year>,
    pub iyr: Option<Year>,
    pub eyr: Option<Year>,
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
            let split_index = field.find(':').unwrap_or_else(|| {
                panic!(
                    "Expected the following string to contain a colon: {}",
                    field
                )
            });
            self.add_field(&field[0..split_index], &field[split_index + 1..])
        }
    }

    pub fn add_field(&mut self, field: &str, value: &str) {
        match field {
            "byr" => {
                self.byr = Some(Year {
                    year: value.parse().expect("Expected value to be parsable"),
                })
            }
            "iyr" => {
                self.iyr = Some(Year {
                    year: value.parse().expect("Expected value to be parsable"),
                })
            }
            "eyr" => {
                self.eyr = Some(Year {
                    year: value.parse().expect("Expected value to be parsable"),
                })
            }
            "hgt" => self.hgt = Some(value.to_string()),
            "hcl" => self.hcl = Some(value.to_string()),
            "ecl" => self.ecl = Some(value.to_string()),
            "pid" => self.pid = Some(value.to_string()),
            "cid" => self.cid = Some(value.to_string()),
            _ => panic!("Invalid field: {}", field),
        }
    }

    pub fn has_all_fields(&self) -> bool {
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
            if required_fields.has_all_fields() {
                valid_count += 1;
            }
            required_fields.reset();
            continue;
        }
        required_fields.add_fields_from_string(input_line);
    }
    if required_fields.has_all_fields() {
        valid_count += 1;
    }
    required_fields.reset();
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
