mod error;
mod output;
mod roller;

use error::ParseExpressionError;
use output::ExpressionResult;
use rand::Rng;
use roller::Roller;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Options {
    pub destructive_trance: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Expression {
    num: usize,
    max: usize,
    modifier: i32,
}

impl Expression {
    pub fn new(num: usize, max: usize, modifier: i32) -> Self {
        Self { num, max, modifier }
    }

    pub fn execute(&self, rng: &mut impl Rng, options: Options) -> ExpressionResult {
        let mut roller = Roller::new(rng, self.max, options);

        // Roll the vanilla result of the expression with extra dice per advantage/disadvantage.
        let mut values: Vec<_> = roller.sample_iter().take(self.total_dice()).collect();
        values.sort_unstable();
        let values = self.apply_modifier_window(&values);

        // Explode.
        let explosions = values
            .iter()
            .filter(|&&x| {
                if options.destructive_trance {
                    x >= self.max - 1
                } else {
                    x == self.max
                }
            })
            .count();

        let mut values: Vec<_> = values.iter().cloned().collect();
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

    fn apply_modifier_window<'a>(&self, values: &'a [usize]) -> &'a [usize] {
        match self.modifier {
            0 => values,
            x if x > 0 => &values[(x as usize)..],
            x if x < 0 => &values[..(values.len() - (x.abs() as usize))],

            _ => unreachable!("Integers are equal to, less than, or greater than 0"),
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
