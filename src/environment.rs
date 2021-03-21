pub fn run(agent: &mut dyn crate::agents::Agent, n_steps: u32) {
    for _ in 0..n_steps {
        let action: usize = agent.get_action();

        let reward: f32 = environment_step(action);

        agent.record_state_transition(action, reward);
    }
}

fn environment_step(action: usize) -> f32 {
    let rewards = get_rewards();

    return generate_random_normal(rewards[action]);
}

pub fn get_rewards() -> [f32; 10] {[
    0.3,
    -0.8,
    1.5,
    0.5,
    1.2,
    -1.5,
    -0.2,
    -1.0,
    0.8,
    0.6,
]}

use rand_distr::{Normal, Distribution};

fn generate_random_normal(mean: f32) -> f32 {
    let deviation = 1.0;
    Normal::new(mean, deviation).unwrap().sample(&mut rand::thread_rng())
}
