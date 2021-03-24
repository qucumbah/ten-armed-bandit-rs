pub struct EnvironmentDynamic {
    rewards: [f32; 10],
    history: Vec<f32>,
}

impl super::Environment for EnvironmentDynamic {
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
        "Dynamic environment (rewards may shift on every step)"
    }
}

impl EnvironmentDynamic {
    pub fn new() -> EnvironmentDynamic {
        EnvironmentDynamic {
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

    fn environment_step(&mut self, action: usize) -> f32 {
        use rand::Rng;

        for i in 0..10 {
            self.rewards[i] += rand::thread_rng().gen_range(-0.1..0.1);
        }

        return crate::util::generate_random_normal(self.rewards[action]);
    }
}
