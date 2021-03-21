pub struct AgentRecencyWeighted {
    pub expected_rewards: [f32; 10],
    e: f64,
    a: f64,
}

impl AgentRecencyWeighted {
    pub fn new() -> AgentRecencyWeighted {
        AgentRecencyWeighted {
            expected_rewards: [0.0; 10],
            e: 0.1,
            a: 0.95,
        }
    }
}

impl super::Agent for AgentRecencyWeighted {
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
        let error = reward - self.expected_rewards[chosen_action];
        self.expected_rewards[chosen_action] += self.a as f32 * error;
    }
}
