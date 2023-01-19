use std::collections::VecDeque;

const QUEUE_SIZE: u32 = 624;

#[derive(Debug)]
pub struct Generator {
    queue: VecDeque<u32>,
}

impl Generator {
    pub fn new(seed: u32) -> Self {
        let (queue, _): (VecDeque<u32>, u32) = (1..=QUEUE_SIZE)
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

    pub fn generate_u32(&mut self) -> u32 {
        let a:   u32 = 0x9908b0df;
        let b:   u32 = 0x9d2c5680;
        let c:   u32 = 0xefc60000;
        let l:   u32 = 0x7fffffff;
        let m:   usize = 387;
        let sr0: u32 = 11;
        let sl1: u32 = 7;
        let sl2: u32 = 15;
        let sr3: u32 = 18;
        let u:   u32 = 0x80000000;
        let x:   u32 = (self.queue[0] & u) | (self.queue[1] & l);
        let y:   u32 = self.queue[m] ^ (x >> 1) ^ (a * (x & 1));
        self.queue.pop_front();
        self.queue.push_back(y);
        let y: u32 = y ^ y >> sr0;
        let y: u32 = y ^ y << sl1 & b;
        let y: u32 = y ^ y << sl2 & c;
        let y: u32 = y ^ y >> sr3;
        y
    }
}

