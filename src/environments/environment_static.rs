pub struct EnvironmentStatic {
    rewards: [f32; 10],
    history: Vec<f32>,
}

impl super::Environment for EnvironmentStatic {
    fn run(&mut self, agent: &mut dyn crate::agents::Agent, n_steps: u32) {
        for _ in 0..n_steps {
            let action: usize = agent.get_action();

            let reward: f32 = self.environment_step(action);

            self.history.push(reward);

            agent.record_state_transition(action, reward);
        }
    }

    fn get_rewards(&self) -> &[f32; 10] {
        &self.rewards
    }

    fn get_history(&self) -> &Vec<f32> {
        &self.history
    }

    fn description(&self) -> &str {
        "Static environment (rewards are always the same)"
    }
}

impl EnvironmentStatic {
    pub fn new() -> EnvironmentStatic {
        EnvironmentStatic {
            rewards: [
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
            ],
            history: Vec::new(),
        }
    }

    fn environment_step(&self, action: usize) -> f32 {
        crate::util::generate_random_normal(self.rewards[action])
    }
}
