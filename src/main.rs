mod agents;
mod environments;

mod util;

fn main() {
    use agents::agent_sample_average::AgentSampleAverage;
    use agents::agent_recency_weighted::AgentRecencyWeighted;
    use agents::agent_greedy_optimistic::AgentGreedyOptimistic;

    use environments::environment_static::EnvironmentStatic;
    use environments::environment_dynamic::EnvironmentDynamic;

    test_agent(&mut AgentSampleAverage::new(), &mut EnvironmentStatic::new(), "1.txt");
    test_agent(&mut AgentRecencyWeighted::new(), &mut EnvironmentStatic::new(), "2.txt");
    test_agent(&mut AgentGreedyOptimistic::new(), &mut EnvironmentStatic::new(), "3.txt");

    test_agent(&mut AgentSampleAverage::new(), &mut EnvironmentDynamic::new(), "4.txt");
    test_agent(&mut AgentRecencyWeighted::new(), &mut EnvironmentDynamic::new(), "5.txt");
    test_agent(&mut AgentGreedyOptimistic::new(), &mut EnvironmentDynamic::new(), "6.txt");
}

fn test_agent(
    agent: &mut dyn crate::agents::Agent,
    environment: &mut dyn crate::environments::Environment,
    output_data_file_name: &str,
) {
    let n_steps = 5000;
    environment.run(agent, n_steps);

    let rewards_history = environment.get_history();

    use std::fs::File;
    use std::io::Write;

    let file_path = format!("{}{}", "output/", output_data_file_name);

    let mut output_file = File::create(&file_path).unwrap();

    for reward in rewards_history {
        output_file.write(reward.to_string().as_bytes()).expect("Should have written the output");
        output_file.write(b"\n").expect("Should have written the output");
    }
}
