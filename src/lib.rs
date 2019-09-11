mod error;
mod output;
mod roller;

use error::ParseExpressionError;
use output::ExpressionResult;
use rand::Rng;
use roller::Roller;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Expression {
    num: usize,
    max: usize,
    modifier: i32,
}

impl Expression {
    pub fn execute(&self, rng: &mut impl Rng) -> ExpressionResult {
        let mut roller = Roller::new(rng, self.max);

        // Roll the vanilla result of the expression with extra dice per advantage/disadvantage.
        let values: Vec<_> = roller.sample_iter().take(self.total_dice()).collect();
        // Apply advantage/disadvantage
        let mut values = self.apply_modifier(values);
        // Explode.
        let explosions = values.iter().filter(|&&x| x == self.max).count();
        for _ in 0..explosions {
            values.extend(roller.explode(self.max));
        }

        // Sort one last time for the hell of it.
        values.sort_unstable_by(|a, b| b.cmp(&a));
        ExpressionResult(values)
    }

    fn total_dice(&self) -> usize {
        self.num + self.modifier.abs() as usize
    }

    fn apply_modifier(&self, mut values: Vec<usize>) -> Vec<usize> {
        values.sort_unstable();
        match self.modifier {
            0 => values,
            x if x > 0 => values.into_iter().skip(self.modifier as usize).collect(),
            x if x < 0 => values.into_iter().take(self.num).collect(),

            // For some stupid reason, the compiler doesn't get that x can be 0, greater than
            // zero, or less than zero. Whatever, rustc, you do you.
            _ => unreachable!(),
        }
    }
}

impl FromStr for Expression {
    type Err = ParseExpressionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rfind(|x: char| x == '+' || x == '-') {
            Some(idx) => {
                let (num, max) = read_segments(&s[..idx])?;
                Ok(Expression {
                    num,
                    max,
                    modifier: s[(idx)..].parse()?,
                })
            }

            None => {
                let (num, max) = read_segments(s)?;
                Ok(Expression {
                    num,
                    max,
                    modifier: 0,
                })
            }
        }
    }
}

fn read_segments(s: &str) -> Result<(usize, usize), ParseExpressionError> {
    let mut segments = s.split(|x: char| x == 'd' || x == 'D');
    let left: usize = segments
        .next()
        .ok_or(ParseExpressionError::Empty)?
        .parse()?;
    let right = segments.next().map(|x| x.parse::<usize>());

    if let Some(_) = segments.next() {
        return Err(ParseExpressionError::TooManySegments);
    }

    match right {
        Some(Ok(right)) => Ok((left, right)),
        Some(Err(e)) => Err(e.into()),
        _ => Ok((1, left)),
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;

    #[test]
    fn can_parse_segments() {
        assert_eq!(
            Ok(Expression {
                num: 1,
                max: 6,
                modifier: 2
            }),
            "6+2".parse()
        );
        assert_eq!(
            Ok(Expression {
                num: 2,
                max: 6,
                modifier: -2
            }),
            "2d6-2".parse()
        );
        assert_eq!(
            Ok(Expression {
                num: 2,
                max: 6,
                modifier: 0
            }),
            "2d6".parse()
        );
    }
}
