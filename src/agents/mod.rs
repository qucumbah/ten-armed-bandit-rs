pub mod agent_sample_average;
pub mod agent_recency_weighted;
pub mod agent_greedy_optimistic;

pub trait Agent {
    fn get_action(&self) -> usize;
    fn record_state_transition(&mut self, chosen_action: usize, reward: f32);
    fn get_expected_rewards(&self) -> &[f32; 10];
}
