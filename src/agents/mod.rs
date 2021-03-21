pub mod agent_sample_average;
pub mod agent_recency_weighted;

pub trait Agent {
    fn get_action(&self) -> usize;
    fn record_state_transition(&mut self, chosen_action: usize, reward: f32);
}
