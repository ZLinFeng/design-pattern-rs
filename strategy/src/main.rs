trait CueingStrategy {
    fn cueing(&self);
}

struct HighCueingStrategy;

impl CueingStrategy for HighCueingStrategy {
    fn cueing(&self) {
        println!("击打后向前滚动")
    }
}

struct LowCueingStrategy;

impl CueingStrategy for LowCueingStrategy {
    fn cueing(&self) {
        println!("击打后向后滚动")
    }
}

struct StraightCueContext<'a> {
    cue_strategy: &'a dyn CueingStrategy,
}

impl<'a> StraightCueContext<'a> {
    fn new(strategy: &'a dyn CueingStrategy) -> Self {
        Self {
            cue_strategy: strategy,
        }
    }

    fn cueing(&self) {
        self.cue_strategy.cueing();
    }
}

fn main() {
    let high_cueing = HighCueingStrategy;
    let straight = StraightCueContext::new(&high_cueing);
    straight.cueing();

    let low_cueing = LowCueingStrategy;
    let straight = StraightCueContext::new(&low_cueing);
    straight.cueing();
}
