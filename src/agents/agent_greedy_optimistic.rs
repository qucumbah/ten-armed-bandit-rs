pub struct AgentGreedyOptimistic {
    pub expected_rewards: [f32; 10],
    n_rewards_taken: [u32; 10],
    e: f64,
}

impl AgentGreedyOptimistic {
    pub fn new() -> AgentGreedyOptimistic {
        AgentGreedyOptimistic {
            expected_rewards: [5.0; 10],
            n_rewards_taken: [0; 10],
            e: 0.0,
        }
    }
}

impl super::Agent for AgentGreedyOptimistic {
    fn get_action(&self) -> usize {
        use rand::Rng;

        let should_choose_random_action = rand::thread_rng().gen_bool(self.e);

        if should_choose_random_action {
            rand::thread_rng().gen_range(0..10)
        } else {
            crate::util::get_optimal_action(&self.expected_rewards)
        }
    }

    fn record_state_transition(&mut self, chosen_action: usize, reward: f32) {
        self.n_rewards_taken[chosen_action] += 1;
        let coeff = 1.0 / self.n_rewards_taken[chosen_action] as f32;
        let error = reward - self.expected_rewards[chosen_action];
        self.expected_rewards[chosen_action] += coeff * error;
    }

    fn get_expected_rewards(&self) -> &[f32; 10] {
        &self.expected_rewards
    }
}
