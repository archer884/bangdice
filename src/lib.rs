mod error;
mod output;
mod roller;

use error::ParseExpressionError;
use output::ExpressionResult;
use rand::Rng;
use roller::Roller;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Expression {
    /// The number of dice to be rolled.
    num: usize,

    /// The maximum value of the dice being rolled.
    max: usize,

    /// Advantage or disadvantage modifier.
    ///
    /// 2d6+2 represents a roll of 2 six-sided dice with an advantage of two, meaning that four
    /// dice will be rolled and the two best rolls kept.
    modifier: i32,

    /// Explosion threshold.
    ///
    /// 2d6+2!5 represents a roll of 2 six-sided dice with an advantage of two and an explosion
    /// threshold of five, meaning that rolls of 5 or 6 will explode. This can be used to make
    /// rolls compatible with, for example, destructive trance. If not provided, this threshold
    /// will be equal to max.
    threshold: Option<usize>,
}

impl Expression {
    pub fn execute(&self, rng: &mut impl Rng) -> ExpressionResult {
        let mut roller = Roller::new(rng, self.max);

        // Roll the vanilla result of the expression with extra dice per advantage/disadvantage.
        let mut values: Vec<_> = roller.sample_iter().take(self.total_dice()).collect();
        values.sort_unstable();
        let values = self.apply_modifier_window(&values);

        // Explode.
        let explosions = match self.threshold {
            None => 0,
            Some(threshold) => values.iter().filter(|&&x| x >= threshold).count(),
        };

        let mut values: Vec<_> = values.iter().cloned().collect();
        for _ in 0..explosions {
            values.extend(roller.explode(self.threshold.unwrap_or(self.max)));
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
        let (threshold, s) = read_threshold(s)?;
        let (modifier, s) = read_modifier(s)?;
        let (num, max) = read_segments(s)?;

        Ok(Expression {
            num,
            max,
            modifier: modifier.unwrap_or_default(),
            threshold: threshold.map(|x| if x == 0 { max } else { x }),
        })
    }
}

fn read_threshold(s: &str) -> Result<(Option<usize>, &str), ParseExpressionError> {
    match s.rfind('!') {
        Some(idx) if idx + 1 < s.len() => {
            let value = s[(idx + 1)..].parse()?;
            Ok((Some(value), &s[..idx]))
        }

        // In the event a BANG is provided without a threshold, return zero to signify that the
        // max threshold should be applied.
        Some(idx) => Ok((Some(0), &s[..idx])),
        _ => Ok((None, s)),
    }
}

fn read_modifier(s: &str) -> Result<(Option<i32>, &str), ParseExpressionError> {
    match s.rfind(|c| c == '+' || c == '-') {
        Some(idx) => {
            let value = s[idx..].parse()?;
            Ok((Some(value), &s[..idx]))
        }
        _ => Ok((None, s)),
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
    fn can_parse() {
        assert_eq!(
            Ok(Expression {
                num: 1,
                max: 6,
                modifier: 2,
                threshold: None,
            }),
            "6+2".parse()
        );

        assert_eq!(
            Ok(Expression {
                num: 2,
                max: 6,
                modifier: -2,
                threshold: None,
            }),
            "2d6-2".parse()
        );

        assert_eq!(
            Ok(Expression {
                num: 2,
                max: 6,
                modifier: 0,
                threshold: None,
            }),
            "2d6".parse()
        );
    }

    #[test]
    fn can_parse_with_threshold() {
        assert_eq!(
            Ok(Expression {
                num: 2,
                max: 6,
                modifier: 0,
                threshold: Some(5)
            }),
            "2d6!5".parse()
        );

        assert_eq!(
            Ok(Expression {
                num: 2,
                max: 6,
                modifier: 2,
                threshold: Some(5)
            }),
            "2d6+2!5".parse()
        );

        assert_eq!(
            Ok(Expression {
                num: 2,
                max: 6,
                modifier: 2,
                threshold: Some(6)
            }),
            "2d6+2!".parse()
        );
    }
}
