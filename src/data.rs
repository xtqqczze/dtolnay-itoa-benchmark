use crate::Unsigned;
use rand::SeedableRng as _;
use rand::distr::{Distribution as _, Uniform};
use rand::rngs::SmallRng;

pub(crate) struct Data {
    pub u32: [Vec<u32>; 10],
    pub u64: [Vec<u64>; 20],
    pub u128: [Vec<u128>; 39],
}

impl Data {
    pub(crate) fn random(count: usize) -> Self {
        let mut rng = SmallRng::seed_from_u64(1);
        let mut data = Data {
            u32: [const { Vec::new() }; 10],
            u64: [const { Vec::new() }; 20],
            u128: [const { Vec::new() }; 39],
        };
        fill(&mut rng, &mut data.u32, count);
        fill(&mut rng, &mut data.u64, count);
        fill(&mut rng, &mut data.u128, count);
        data
    }
}

fn fill<T, const N: usize>(rng: &mut SmallRng, data: &mut [Vec<T>; N], count: usize)
where
    T: Unsigned,
{
    for (i, vec) in data.iter_mut().enumerate() {
        let lo = T::TEN.pow(i as u32) - T::from(i == 0);
        let hi = T::TEN
            .saturating_pow(i as u32 + 1)
            .wrapping_add(T::ONE)
            .saturating_sub(T::ONE)
            .wrapping_sub(T::ONE);
        let distr = Uniform::new_inclusive(lo, hi).unwrap();
        vec.reserve_exact(count);
        for _ in 0..count {
            vec.push(distr.sample(rng));
        }
    }
}
