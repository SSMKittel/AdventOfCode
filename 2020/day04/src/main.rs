use std::str::FromStr;
use std::collections::HashMap;
use std::convert::TryFrom;

fn main() {
    let input = include_str!("input.txt");
    let candidates = input.split("\n\n")
        .filter(|&x| x.len() > 0)
        .filter_map(|x| PassportCandidate::try_from(x).ok())
        .collect::<Vec<PassportCandidate>>();

    let candidates_len = candidates.len();

    let pass = candidates.into_iter()
        .filter_map(|x| Passport::try_from(x).ok())
        .collect::<Vec<Passport>>();

    println!("{:#?}", pass);
    println!("{} => {}", candidates_len, pass.len());
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct BirthYear(i32);
impl FromStr for BirthYear {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<BirthYear, Self::Err> {
        let year = i32::from_str(s).map_err(|_| "not an i32")?;
        BirthYear::try_from(year)
    }
}
impl TryFrom<i32> for BirthYear {
    type Error = &'static str;

    fn try_from(year: i32) -> Result<Self, Self::Error> {
        if year >= 1920 && year <= 2002 {
            Ok(BirthYear(year))
        } else {
            Err("out of range")
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct IssueYear(i32);
impl FromStr for IssueYear {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<IssueYear, Self::Err> {
        let year = i32::from_str(s).map_err(|_| "not an i32")?;
        IssueYear::try_from(year)
    }
}
impl TryFrom<i32> for IssueYear {
    type Error = &'static str;

    fn try_from(year: i32) -> Result<Self, Self::Error> {
        if year >= 2010 && year <= 2020 {
            Ok(IssueYear(year))
        } else {
            Err("out of range")
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct ExpirationYear(i32);
impl FromStr for ExpirationYear {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<ExpirationYear, Self::Err> {
        let year = i32::from_str(s).map_err(|_| "not an i32")?;
        ExpirationYear::try_from(year)
    }
}
impl TryFrom<i32> for ExpirationYear {
    type Error = &'static str;

    fn try_from(year: i32) -> Result<Self, Self::Error> {
        if year >= 2020 && year <= 2030 {
            Ok(ExpirationYear(year))
        } else {
            Err("out of range")
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Height(u32, &'static str);
impl FromStr for Height {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Height, Self::Err> {
        if s.ends_with("cm") {
            let val = u32::from_str(&s[..s.len() - 2]).map_err(|_| "not an u32")?;
            if val >= 150 && val <= 193 {
                Ok(Height(val, "cm"))
            }
            else {
                Err("out of range")
            }
        }
        else if s.ends_with("in") {
            let val = u32::from_str(&s[..s.len() - 2]).map_err(|_| "not an u32")?;
            if val >= 59 && val <= 76 {
                Ok(Height(val, "in"))
            }
            else {
                Err("out of range")
            }
        }
        else {
            Err("invalid unit")
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct HairColour(String);
impl FromStr for HairColour {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<HairColour, Self::Err> {
        if s.len() == 7 && s.starts_with("#") {
            let s = &s[1..];
            for (_, c) in s.chars().enumerate() {
                match c {
                    '0'..='9' => (),
                    'a'..='f' => (),
                    _ => return Err("invalid characters")
                }
            }
            Ok(HairColour(s.to_owned()))
        }
        else {
            Err("missing #")
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum EyeColour {
    amb,
    blu,
    brn,
    gry,
    grn,
    hzl,
    oth
}

impl FromStr for EyeColour {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<EyeColour, Self::Err> {
        match s {
            "amb" => Ok(EyeColour::amb),
            "blu" => Ok(EyeColour::blu),
            "brn" => Ok(EyeColour::brn),
            "gry" => Ok(EyeColour::gry),
            "grn" => Ok(EyeColour::grn),
            "hzl" => Ok(EyeColour::hzl),
            "oth" => Ok(EyeColour::oth),
            _ => Err("invalid eye colour")
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct PassportId(u32);
impl FromStr for PassportId {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<PassportId, Self::Err> {
        if s.len() == 9 {
            let val = u32::from_str(s).map_err(|_| "not an u32")?;
            Ok(PassportId(val))
        } else {
            Err("invalid id")
        }
    }
}

#[derive(Debug)]
struct PassportCandidate<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: Option<&'a str>
}
impl<'a> TryFrom<&'a str> for PassportCandidate<'a> {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<PassportCandidate, Self::Error> {
        let components: Vec<_> = s.split_whitespace().collect();

        let mut fields = HashMap::new();
        for component in components {
            let parts: Vec<_> = component.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err("invalid component layout");
            }

            fields.insert(parts[0], parts[1]);
        }

        let byr = fields.remove("byr").ok_or("missing byr")?;
        let iyr = fields.remove("iyr").ok_or("missing iyr")?;
        let eyr = fields.remove("eyr").ok_or("missing eyr")?;
        let hgt = fields.remove("hgt").ok_or("missing hgt")?;
        let hcl = fields.remove("hcl").ok_or("missing hcl")?;
        let ecl = fields.remove("ecl").ok_or("missing ecl")?;
        let pid = fields.remove("pid").ok_or("missing pid")?;
        let cid = fields.remove("cid");

        Ok(PassportCandidate {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid
        })
    }
}


#[derive(Debug)]
struct Passport {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpirationYear,
    hgt: Height,
    hcl: HairColour,
    ecl: EyeColour,
    pid: PassportId,
    cid: Option<String>
}

impl<'a> TryFrom<PassportCandidate<'a>> for Passport {
    type Error = &'static str;

    fn try_from(pc: PassportCandidate) -> Result<Passport, Self::Error> {
        Ok(Passport {
            byr: BirthYear::from_str(pc.byr)?,
            iyr: IssueYear::from_str(pc.iyr)?,
            eyr: ExpirationYear::from_str(pc.eyr)?,
            hgt: Height::from_str(pc.hgt)?,
            hcl: HairColour::from_str(pc.hcl)?,
            ecl: EyeColour::from_str(pc.ecl)?,
            pid: PassportId::from_str(pc.pid)?,
            cid: pc.cid.map(|x| x.to_owned())
        })
    }
}
