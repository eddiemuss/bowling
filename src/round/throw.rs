use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Throw {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_throw() {
        assert_eq!(Throw::Miss.to_string(), "-");
        assert_eq!(Throw::Strike.to_string(), "X");
        assert_eq!(Throw::Hit(2).to_string(), "2");
        assert_eq!(Throw::Spare(2).to_string(), "/");
    }
}
