use rand::{distributions::Uniform, Rng};
use std::iter;

pub struct Roller<'r, R> {
    rng: &'r mut R,
    distribution: Uniform<usize>,
}

impl<'r, R> Roller<'r, R> {
    pub fn new(rng: &'r mut R, max: usize) -> Self {
        Self {
            rng,
            distribution: Uniform::new_inclusive(1, max),
        }
    }
}

impl<R> Roller<'_, R>
where
    R: Rng,
{
    pub fn explode<'a>(&'a mut self, threshold: usize) -> impl Iterator<Item = usize> + 'a {
        let head = self.sample();
        let tail = self.sample_iter().take_while(move |&x| x > threshold);

        iter::once(head).chain(tail)
    }

    pub fn sample(&mut self) -> usize {
        self.rng.sample(self.distribution)
    }

    pub fn sample_iter<'a>(&'a mut self) -> impl Iterator<Item = usize> + 'a {
        self.rng.sample_iter(self.distribution)
    }
}
