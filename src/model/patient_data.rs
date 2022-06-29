use std::hash::{Hash, Hasher};
use std::fmt;


#[derive(Debug, Clone)] 
pub struct Patient {
    pub id: String,
    pub name: String,
}

impl PartialEq for Patient {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Patient {}

impl Hash for Patient {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(h)
    }
}


impl Patient {
    pub fn new(id: String, name: String) -> Patient {
        Patient {
            id: id,
            name: name
        }
    } 
}

#[derive(Debug, PartialEq, Clone)] 
pub struct Case {
    pub file: String,
}

impl Case {
    pub fn new(name: String) -> Case {
        Case {
            file: name
        }
    } 
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file)
    }
}