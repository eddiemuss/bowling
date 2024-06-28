pub mod throw;

use std::fmt;
use throw::Throw;

#[derive(Clone)]
pub enum Round {
    First,
    Second {
        remaining_pins: u8,
        first_throw: Throw,
    },
    Complete {
        throws: Vec<Throw>,
    },
}

const MAX_PINS: u8 = 10;

impl Round {
    pub fn new() -> Round {
        Round::First
    }

    pub fn role(self, rolled_pins: u8) -> Result<Round, String> {
        match self {
            Round::Complete { throws: _ } => return Err("Round already completed!".into()),
            Round::First => match rolled_pins {
                pins if pins == 0 => Ok(Round::Second {
                    remaining_pins: MAX_PINS,
                    first_throw: Throw::Miss,
                }),
                pins if pins < MAX_PINS => Ok(Round::Second {
                    remaining_pins: MAX_PINS - rolled_pins,
                    first_throw: Throw::Hit(rolled_pins),
                }),
                pins if pins == MAX_PINS => Ok(Round::Complete {
                    throws: vec![Throw::Strike],
                }),
                _ => Err("Sorry, but you can not role more than possible.".into()),
            },
            Round::Second {
                remaining_pins,
                first_throw,
            } => match rolled_pins {
                pins if pins == 0 => Ok(Round::Complete {
                    throws: vec![first_throw, Throw::Miss],
                }),
                pins if pins < remaining_pins => Ok(Round::Complete {
                    throws: vec![first_throw, Throw::Hit(rolled_pins)],
                }),
                pins if pins == remaining_pins => Ok(Round::Complete {
                    throws: vec![first_throw, Throw::Spare(rolled_pins)],
                }),
                _ => Err("Sorry, but you can not role more than remain.".into()),
            },
        }
    }

    pub fn last_throw(&self) -> Option<Throw> {
        match self {
            Round::First => None,
            Round::Second {
                remaining_pins: _,
                first_throw,
            } => Some(first_throw.clone()),
            Round::Complete { throws } => throws.iter().last().cloned(),
        }
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Round::First => write!(f, "[]"),
            Round::Second {
                remaining_pins: _,
                first_throw,
            } => write!(f, "[{}, ?]", first_throw.to_string()),
            Round::Complete { throws } => {
                let foo: Vec<String> = throws.iter().map(|x| x.to_string()).collect();

                write!(f, "[{}]", foo.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_pins_once() {
        assert_eq!(
            Round::new().role(0).unwrap().last_throw(),
            Some(Throw::Miss)
        );
        assert_eq!(
            Round::new().role(1).unwrap().last_throw(),
            Some(Throw::Hit(1))
        );
        assert_eq!(
            Round::new().role(5).unwrap().last_throw(),
            Some(Throw::Hit(5))
        );
        assert_eq!(
            Round::new().role(10).unwrap().last_throw(),
            Some(Throw::Strike)
        );
    }

    #[test]
    fn role_pins_twice() {
        let round = Round::new().role(1).unwrap();
        assert_eq!(
            round.clone().role(2).unwrap().last_throw(),
            Some(Throw::Hit(2))
        );
        assert_eq!(
            round.clone().role(0).unwrap().last_throw(),
            Some(Throw::Miss)
        );
        assert_eq!(
            round.clone().role(9).unwrap().last_throw(),
            Some(Throw::Spare(9))
        );
        assert_eq!(round.clone().role(10).is_err(), true);
    }

    #[test]
    fn display_empty_round() {
        assert_eq!(Round::new().to_string(), "[]");
    }

    #[test]
    fn display_fist_round() {
        assert_eq!(Round::new().role(2).unwrap().to_string(), "[2, ?]");
    }

    #[test]
    fn display_completed_rounds() {
        assert_eq!(
            Round::new().role(1).unwrap().role(2).unwrap().to_string(),
            "[1, 2]"
        );
        assert_eq!(
            Round::new().role(0).unwrap().role(10).unwrap().to_string(),
            "[-, /]"
        );
        assert_eq!(Round::new().role(10).unwrap().to_string(), "[X]");
        assert_eq!(
            Round::new().role(2).unwrap().role(8).unwrap().to_string(),
            "[2, /]"
        );
    }

    #[test]
    fn role_three_times() {
        assert_eq!(
            Round::new()
                .role(1)
                .unwrap()
                .role(2)
                .unwrap()
                .role(3)
                .is_err(),
            true
        );
    }

    #[test]
    fn role_not_more_then_ten() {
        assert_eq!(Round::new().role(11).is_err(), true);
    }
}
