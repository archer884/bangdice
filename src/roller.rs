use rand::{distributions::Uniform, Rng};
use std::iter;

pub struct Roller<'r, R> {
    rng: &'r mut R,
    dst: Uniform<usize>,
}

impl<'r, R> Roller<'r, R> {
    pub fn new(rng: &'r mut R, max: usize) -> Self {
        Self {
            rng,
            dst: Uniform::new_inclusive(1, max),
        }
    }
}

impl<R> Roller<'_, R>
where
    R: Rng,
{
    pub fn explode<'a, 'r: 'a>(&'r mut self, threshold: usize) -> impl Iterator<Item = usize> + 'a {
        let head = self.rng.sample(self.dst);
        let tail = self
            .rng
            .sample_iter(self.dst)
            .take_while(move |&x| x > threshold);

        iter::once(head).chain(tail)
    }

    pub fn sample_iter<'a, 'r: 'a>(&'r mut self) -> impl Iterator<Item = usize> + 'a {
        self.rng.sample_iter(self.dst)
    }
}
