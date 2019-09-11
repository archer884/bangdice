use rand::{distributions::Uniform, Rng};

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
    pub fn explode<'a, 'r: 'a>(&'r mut self, max: usize) -> impl Iterator<Item = usize> + 'a {
        let mut take = true;
        self.rng.sample_iter(self.dst).take_while(move |&x| {
            let tmp = take;
            take = x == max;
            tmp
        })
    }

    pub fn sample_iter<'a, 'r: 'a>(&'r mut self) -> impl Iterator<Item = usize> + 'a {
        self.rng.sample_iter(self.dst)
    }
}
