use std::fmt::{self, Display};

pub struct ExpressionResult(pub Vec<usize>);

impl ExpressionResult {
    pub fn total(&self) -> usize {
        self.0.iter().sum()
    }
}

impl Display for ExpressionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.len() {
            1 => write!(f, "{:3}", self.total()),
            _ => write!(f, "{:3} = {}", self.total(), SliceFormatter(&self.0)),
        }
    }
}

struct SliceFormatter<'s>(&'s [usize]);

impl Display for SliceFormatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = self.0.iter();
        if let Some(item) = items.next() {
            write!(f, "({}", item)?;
        }

        for item in items {
            write!(f, " + {}", item)?;
        }

        f.write_str(")")
    }
}
