//* Checking out maze generation.
use rand::Rng;

fn main() {
        let mut rng = rand::thread_rng();
        let side_len = rng.gen_range(1..=300);
        let chance_of_obstacle: f64 = rng.gen_range(0.01..=0.3);
        let is_obstacle_gen = std::iter::from_fn(move || Some(rng.gen_bool(chance_of_obstacle)));
        let row_iter = is_obstacle_gen
                .take(side_len)
                .map(|is_obstacle| if is_obstacle { '#' } else { '.' })
                .chain(std::iter::once('\n'));
        let maze_string: String = row_iter.cycle().take(side_len * (side_len + 1)).collect();

        println!("{}", maze_string);
}
