use serde::{Deserialize, Serialize};

/// A parsed single-syllable shape, not a second mutable copy of its input string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SyllablePattern {
    onset: usize,
    nucleus: usize,
    coda: usize,
}
impl SyllablePattern {
    pub fn parse(input: &str) -> Result<Self, &'static str> {
        let raw = input.to_ascii_uppercase();
        let flat = if raw.contains(['(', ')']) {
            let mut groups = vec![];
            let mut current = String::new();
            let mut inside = false;
            for c in raw.chars() {
                match c {
                    '(' if !inside => {
                        inside = true;
                        current.clear();
                    }
                    ')' if inside && !current.is_empty() => {
                        if !(current.chars().all(|c| c == 'C') || current.chars().all(|c| c == 'V'))
                        {
                            return Err("explicit groups must contain only C or only V");
                        }
                        groups.push(current.clone());
                        inside = false;
                    }
                    'C' | 'V' if inside => current.push(c),
                    _ => return Err("malformed explicit syllable pattern"),
                }
            }
            if inside || groups.is_empty() || groups.len() > 3 {
                return Err("malformed explicit syllable pattern");
            }
            // Each component has one group; adjacent duplicate groups are ambiguous.
            if groups
                .windows(2)
                .any(|g| g[0].starts_with(g[1].chars().next().unwrap()))
            {
                return Err("duplicate syllable component");
            }
            groups.concat()
        } else {
            raw
        };
        if flat.is_empty() || !flat.chars().all(|c| c == 'C' || c == 'V') {
            return Err("patterns contain only C and V");
        }
        let start = flat.find('V').ok_or("a syllable requires a nucleus")?;
        let end = flat.rfind('V').unwrap() + 1;
        if !flat[start..end].chars().all(|c| c == 'V') {
            return Err("a pattern describes one contiguous nucleus");
        }
        Ok(Self {
            onset: start,
            nucleus: end - start,
            coda: flat.len() - end,
        })
    }
    pub fn onset_size(&self) -> usize {
        self.onset
    }
    pub fn nucleus_size(&self) -> usize {
        self.nucleus
    }
    pub fn coda_size(&self) -> usize {
        self.coda
    }
    pub fn matches(&self, onset: usize, nucleus: usize, coda: usize) -> bool {
        (self.onset, self.nucleus, self.coda) == (onset, nucleus, coda)
    }
}
impl TryFrom<String> for SyllablePattern {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
impl From<SyllablePattern> for String {
    fn from(value: SyllablePattern) -> Self {
        value.to_string()
    }
}
impl std::fmt::Display for SyllablePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            "C".repeat(self.onset),
            "V".repeat(self.nucleus),
            "C".repeat(self.coda)
        )
    }
}
