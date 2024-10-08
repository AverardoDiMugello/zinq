use super::features::*;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Version {
    Armv8p0a,
    Armv8p1a,
    Armv8p2a,
    Armv8p3a,
    Armv8p4a,
    Armv8p5a,
    Armv8p6a,
    Armv8p7a,
    Armv8p8a,
    Armv8p9a,
    Armv9p0a,
    Armv9p1a,
    Armv9p2a,
    Armv9p3a,
    Armv9p4a,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Version::*;
        match self {
            Armv8p0a => write!(f, "Armv8.0-a"),
            Armv8p1a => write!(f, "Armv8.1-a"),
            Armv8p2a => write!(f, "Armv8.2-a"),
            Armv8p3a => write!(f, "Armv8.3-a"),
            Armv8p4a => write!(f, "Armv8.4-a"),
            Armv8p5a => write!(f, "Armv8.5-a"),
            Armv8p6a => write!(f, "Armv8.6-a"),
            Armv8p7a => write!(f, "Armv8.7-a"),
            Armv8p8a => write!(f, "Armv8.8-a"),
            Armv8p9a => write!(f, "Armv8.9-a"),
            Armv9p0a => write!(f, "Armv9.0-a"),
            Armv9p1a => write!(f, "Armv9.1-a"),
            Armv9p2a => write!(f, "Armv9.2-a"),
            Armv9p3a => write!(f, "Armv9.3-a"),
            Armv9p4a => write!(f, "Armv9.4-a"),
        }
    }
}

impl Version {
    pub fn is_feat_impl(&self, feat: Feat) -> bool {
        use Version::*;
        match self {
            Armv8p0a => panic!("Feature list not supported yet for {self}"),
            Armv8p1a => panic!("Feature list not supported yet for {self}"),
            Armv8p2a => panic!("Feature list not supported yet for {self}"),
            Armv8p3a => panic!("Feature list not supported yet for {self}"),
            Armv8p4a => panic!("Feature list not supported yet for {self}"),
            Armv8p5a => panic!("Feature list not supported yet for {self}"),
            Armv8p6a => panic!("Feature list not supported yet for {self}"),
            Armv8p7a => panic!("Feature list not supported yet for {self}"),
            Armv8p8a => panic!("Feature list not supported yet for {self}"),
            Armv8p9a => panic!("Feature list not supported yet for {self}"),
            Armv9p0a => panic!("Feature list not supported yet for {self}"),
            Armv9p1a => panic!("Feature list not supported yet for {self}"),
            Armv9p2a => panic!("Feature list not supported yet for {self}"),
            Armv9p3a => panic!("Feature list not supported yet for {self}"),
            Armv9p4a => v9p4a_is_feat_impl(feat),
        }
    }
}
