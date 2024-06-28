use std::fmt;

#[derive(Clone)]
struct Round {
    remaining_pins: u8,
    throws: Vec<Throw>,
}

impl Round {
    fn new() -> Round {
        Round {
            remaining_pins: 10,
            throws: vec![],
        }
    }

    fn role(&mut self, rolled_pins: u8) -> &mut Self {
        if rolled_pins > self.remaining_pins {
            return self;
        };
        if self.throws.len() >= 2 {
            return self;
        }
        let new_remaining_pins = self.remaining_pins - rolled_pins;

        let throw = match new_remaining_pins {
            pins if pins == 0 => {
                if self.throws.len() == 0 {
                    Some(Throw::Strike)
                } else {
                    Some(Throw::Spare(rolled_pins))
                }
            }
            pins if pins < self.remaining_pins => Some(Throw::Hit(rolled_pins)),
            pins if pins == self.remaining_pins => Some(Throw::Miss),
            _ => None,
        };

        if let Some(throw) = throw.clone() {
            self.throws.push(throw);
            self.remaining_pins = new_remaining_pins
        };
        self
    }

    fn last_throw(&self) -> Option<Throw> {
        self.throws.last().cloned()
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Throw {
    Hit(u8),
    Spare(u8),
    Strike,
    Miss,
}

impl fmt::Display for Throw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            Throw::Hit(x) => x.to_string(),
            Throw::Spare(_) => "/".into(),
            Throw::Strike => "X".into(),
            Throw::Miss => "-".into(),
        };
        write!(f, "{}", status_str)
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let foo: Vec<String> = self.throws.iter().map(|x| x.to_string()).collect();

        write!(f, "[{}]", foo.join(", "))
    }
}

fn main() {
    let mut round = Round::new();
    round.role(1).role(9);

    let last_throw = round.last_throw();

    println!("number {round} have bin rolled.");
    if let Some(last_throw) = last_throw {
        println!("number {last_throw} have bin rolled.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_pins_once() {
        assert_eq!(Round::new().role(0).last_throw(), Some(Throw::Miss));
        assert_eq!(Round::new().role(1).last_throw(), Some(Throw::Hit(1)));
        assert_eq!(Round::new().role(5).last_throw(), Some(Throw::Hit(5)));
        assert_eq!(Round::new().role(10).last_throw(), Some(Throw::Strike));
    }

    #[test]
    fn role_pins_twice() {
        let mut round = Round::new();
        round.role(1);
        assert_eq!(round.clone().role(2).last_throw(), Some(Throw::Hit(2)));
        assert_eq!(round.clone().role(0).last_throw(), Some(Throw::Miss));
        assert_eq!(round.clone().role(9).last_throw(), Some(Throw::Spare(9)));
        // I'm not sure yet how I want to handle errors while bowling?
        // assert_eq!(round.clone().role(10).last_throw(), None);
    }

    #[test]
    fn display_throw() {
        assert_eq!(Throw::Miss.to_string(), "-");
        assert_eq!(Throw::Strike.to_string(), "X");
        assert_eq!(Throw::Hit(2).to_string(), "2");
        assert_eq!(Throw::Spare(2).to_string(), "/");
    }

    #[test]
    fn display_round() {
        assert_eq!(Round::new().role(1).role(2).to_string(), "[1, 2]");
        assert_eq!(Round::new().role(0).role(10).to_string(), "[-, /]");
        assert_eq!(Round::new().role(10).role(1).to_string(), "[X]");
        assert_eq!(Round::new().role(2).role(8).to_string(), "[2, /]");
        assert_eq!(Round::new().role(8).role(0).to_string(), "[8, -]");
    }

    #[test]
    fn role_three_times() {
        assert_eq!(
            Round::new().role(1).role(2).role(3).last_throw(),
            Some(Throw::Hit(2))
        );
    }

    #[test]
    fn role_not_more_then_ten() {
        assert_eq!(Round::new().role(11).last_throw(), None);
    }
}
