use crate::Options;
use rand::{distributions::Uniform, Rng};

pub struct Roller<'r, R> {
    rng: &'r mut R,
    dst: Uniform<usize>,
    options: Options,
}

impl<'r, R> Roller<'r, R> {
    pub fn new(rng: &'r mut R, max: usize, options: Options) -> Self {
        Self {
            rng,
            dst: Uniform::new_inclusive(1, max),
            options,
        }
    }
}

impl<R> Roller<'_, R>
where
    R: Rng,
{
    pub fn explode<'a, 'r: 'a>(&'r mut self, max: usize) -> impl Iterator<Item = usize> + 'a {
        let destructive_trance = self.options.destructive_trance;

        let mut take = true;
        self.rng.sample_iter(self.dst).take_while(move |&x| {
            let tmp = take;
            take = if destructive_trance {
                x >= max - 1
            } else {
                x == max
            };
            tmp
        })
    }

    pub fn sample_iter<'a, 'r: 'a>(&'r mut self) -> impl Iterator<Item = usize> + 'a {
        self.rng.sample_iter(self.dst)
    }
}
