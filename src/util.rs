pub fn get_optimal_action(expected_rewards: &[f32; 10]) -> usize {
    let mut best_action = 0;

    for current_action in 1..10 {
        if expected_rewards[current_action] > expected_rewards[best_action] {
            best_action = current_action;
        }
    }

    return best_action;
}
