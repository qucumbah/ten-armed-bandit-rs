pub fn get_optimal_action(expected_rewards: &[f32; 10]) -> usize {
    let mut best_action = 0;

    for current_action in 1..10 {
        if expected_rewards[current_action] > expected_rewards[best_action] {
            best_action = current_action;
        }
    }

    return best_action;
}

use rand::prelude::*;

pub fn get_seeded_rng(seed: u64) -> StdRng {
    SeedableRng::seed_from_u64(seed)
}

use rand_distr::{Normal, Distribution};

pub fn generate_random_normal(mean: f32, rng: &mut StdRng) -> f32 {
    let deviation = 1.0;
    Normal::new(mean, deviation).unwrap().sample(rng)
}
