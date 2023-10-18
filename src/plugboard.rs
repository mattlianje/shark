pub struct Plugboard {
    plugboard_map: Vec<(char, char)>,
}

impl Plugboard {
    pub fn new(plugboard_map: Vec<(char, char)>) -> Self {
        Plugboard { plugboard_map }
    }

    pub fn pass_through(&self, c: char) -> Result<char, &'static str> {
        self.plugboard_map.iter()
            .find(|&&(a, b)| a == c || b == c)
            .map(|&(a, b)| if a == c { b } else { a })
            .ok_or("Character not found in plugboard map.")
    }
}

pub mod Plugboards {
    use super::Plugboard;

    pub fn test_plugboard() -> Plugboard {
        Plugboard::new(vec![('A', 'K'), ('Z', 'C')])
    }
}