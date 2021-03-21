mod agents;
mod environment;

mod util;

fn main() {
    use agents::agent_sample_average::AgentSampleAverage;
    use agents::agent_recency_weighted::AgentRecencyWeighted;
    use agents::agent_greedy_optimistic::AgentGreedyOptimistic;

    test_agent(&mut AgentSampleAverage::new(), "Agent 1 (sample-average value estimate):");
    test_agent(&mut AgentRecencyWeighted::new(), "Agent 2 (recency-weighted value estimate):");
    test_agent(&mut AgentGreedyOptimistic::new(), "Agent 3 (greedy optimistic):");
}

fn test_agent(agent: &mut dyn crate::agents::Agent, description: &str) {
    let n_steps = 5000;
    environment::run(agent, n_steps);
    println!("{}", description);
    log_error(&agent.get_expected_rewards());
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
