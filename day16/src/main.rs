use std::collections::HashSet;

fn main1() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Step {
        from: usize,
        to: usize,
    }
    let validate_step = |step: Step| {
        (0..input.len()).contains(&step.to) && (step.from % width).abs_diff(step.to % width) <= 1
    };
    let down = width as isize;
    let up = -down;
    let mut steps_taken = HashSet::new();
    let mut next_steps = HashSet::new();
    next_steps.insert(Step { from: 0, to: 0 });
    while let Some(step) = next_steps.iter().copied().next() {
        next_steps.remove(&step);
        steps_taken.insert(step);
        let velocity = step.to.wrapping_sub(step.from).max(1) as isize;
        let cell = input[step.to];
        let new_velocities = match (velocity, cell) {
            (-1, b'|') | (1, b'|') => [down, up],
            (-1, b'\\') | (1, b'/') => [up, up],
            (-1, b'/') | (1, b'\\') => [down, down],
            (..=-2, b'-') | (2.., b'-') => [-1, 1],
            (..=-2, b'\\') | (2.., b'/') => [-1, -1],
            (..=-2, b'/') | (2.., b'\\') => [1, 1],
            _ => [velocity, velocity],
        };
        for new_vel in new_velocities {
            let new_step = Step {
                from: step.to,
                to: step.to.wrapping_add_signed(new_vel),
            };
            if validate_step(new_step) && !steps_taken.contains(&new_step) {
                next_steps.insert(new_step);
            }
        }
    }
    let unique_tos = steps_taken
        .into_iter()
        .map(|step| step.to)
        .collect::<HashSet<_>>()
        .len();
    dbg!(unique_tos);
}

fn main() {
    let mut input = std::fs::read("input").unwrap();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    input.retain(|&ch| ch != b'\n');
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct Step {
        from: usize,
        to: usize,
    }
    let validate_step = |step: Step| {
        (0..input.len()).contains(&step.to) && (step.from % width).abs_diff(step.to % width) <= 1
    };
    let down = width as isize;
    let up = -down;
    let height = input.len() / width;
    let top = (0..width).map(|x| (down, x));
    let left = (0..height).map(|y| (1, y * width));
    let right = (0..height).map(|y| (-1, y * width + width - 1));
    let bottom = (0..width).map(|x| (up, x + (height - 1) * width));
    let mut most = 0;
    for (init_vel, init_pos) in top.chain(left).chain(right).chain(bottom) {
        let mut steps_taken = HashSet::new();
        let mut next_steps = HashSet::new();
        next_steps.insert(Step {
            from: init_pos,
            to: init_pos,
        });
        while let Some(step) = next_steps.iter().copied().next() {
            next_steps.remove(&step);
            steps_taken.insert(step);
            let velocity = match step.to.wrapping_sub(step.from) {
                0 => init_vel,
                v => v as isize,
            };
            let cell = input[step.to];
            let new_velocities = match (velocity, cell) {
                (-1, b'|') | (1, b'|') => [down, up],
                (-1, b'\\') | (1, b'/') => [up, up],
                (-1, b'/') | (1, b'\\') => [down, down],
                (..=-2, b'-') | (2.., b'-') => [-1, 1],
                (..=-2, b'\\') | (2.., b'/') => [-1, -1],
                (..=-2, b'/') | (2.., b'\\') => [1, 1],
                _ => [velocity, velocity],
            };
            for new_vel in new_velocities {
                let new_step = Step {
                    from: step.to,
                    to: step.to.wrapping_add_signed(new_vel),
                };
                if validate_step(new_step) && !steps_taken.contains(&new_step) {
                    next_steps.insert(new_step);
                }
            }
        }
        let unique_tos = steps_taken
            .into_iter()
            .map(|step| step.to)
            .collect::<HashSet<_>>()
            .len();
        if unique_tos > most {
            most = unique_tos;
        }
    }
    dbg!(most);
}
