pub mod environment_static;
pub mod environment_dynamic;

pub trait Environment {
    fn run(&mut self, agent: &mut dyn crate::agents::Agent, n_steps: u32);
    fn get_rewards(&self) -> &[f32; 10];
    fn get_history(&self) -> &Vec<f32>;
    fn description(&self) -> &str;
}
