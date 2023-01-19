use std::collections::VecDeque;

const QUEUE_SIZE: u32 = 624;

#[derive(Debug)]
pub struct Generator {
    queue: VecDeque<u32>,
}

impl Generator {
    pub fn new(seed: u32) -> Self {
        let (queue, _): (VecDeque<u32>, u32) = (0..QUEUE_SIZE)
            .fold((VecDeque::<u32>::new(), seed), |(queue, next), i| {
                let mut queue: VecDeque<u32> = queue;
                queue.push_back(next);
                let next: u32 = 1812433253 * (next ^ (next >> 30)) + i;
                (queue, next)
            });
        Self {
            queue,
        }
    }
}

