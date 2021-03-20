fn main() {
    agent1();
}

fn agent1() {
    let mut rewards_sums = [0.0; 10];
    let mut times_chosen: [i32; 10] = [0; 10];
    let mut expected_rewards = [0.0; 10];

    let n_steps = 1000;
    let e = 0.1;

    use rand::Rng;

    for _ in 0..n_steps {
        let should_choose_random_action = rand::thread_rng().gen_bool(e);

        let action: usize = if should_choose_random_action {
            rand::thread_rng().gen_range(0..10)
        } else {
            get_optimal_action(&expected_rewards)
        };

        let reward: f32 = environment_step(action);

        rewards_sums[action] += reward;
        times_chosen[action] += 1;
        expected_rewards[action] = rewards_sums[action] / times_chosen[action] as f32;
    }

    println!("Agent 1 (sample-average value estimate):");
    log_error(&expected_rewards);
}

fn get_optimal_action(expected_rewards: &[f32; 10]) -> usize {
    let mut best_action = 0;

    for current_action in 1..10 {
        if expected_rewards[current_action] > expected_rewards[best_action] {
            best_action = current_action;
        }
    }

    return best_action;
}

fn environment_step(action: usize) -> f32 {
    let rewards = get_rewards();

    return generate_random_normal(rewards[action]);
}

fn get_rewards() -> [f32; 10] {[
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

fn log_error(expected_rewards: &[f32; 10]) {
    let mut total_error = 0.0;
    let actual_rewards: [f32; 10] = get_rewards();
    println!("{:>7} {:>15} {:>15} {:>15}", "Action", "Actual reward", "Expected reward", "Error");
    for i in 0..10 {
        let expected_reward = expected_rewards[i];
        let actual_reward = actual_rewards[i];
        let error = (expected_reward - actual_reward).powf(2.0);

        println!("{:>7} {:>15} {:>15} {:>15}", i, actual_reward, expected_reward, error);
        total_error += error;
    }

    println!("Total error: {}", total_error);
}
