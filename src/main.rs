mod agents;
mod environment;

mod util;

fn main() {
    let mut agent = agents::AgentSampleAverage::new();
    let n_steps = 1000;
    environment::run(&mut agent, n_steps);
    println!("Agent 1 (sample-average value estimate):");
    log_error(&agent.expected_rewards);
}

fn log_error(expected_rewards: &[f32; 10]) {
    let mut total_error = 0.0;
    let actual_rewards: [f32; 10] = environment::get_rewards();
    println!("{:>7} {:>20} {:>20} {:>20}", "Action", "Actual reward", "Expected reward", "Error");
    for i in 0..10 {
        let expected_reward = expected_rewards[i];
        let actual_reward = actual_rewards[i];
        let error = (expected_reward - actual_reward).powf(2.0);

        println!("{:>7} {:>20} {:>20} {:>20}", i, actual_reward, expected_reward, error);
        total_error += error;
    }

    println!("Total error: {}", total_error);
}
