trait Cue {
    fn shoot(&self);
}

struct SnookerCue {
    name: String,
}
struct ChinesePoolCue {
    name: String,
}

impl SnookerCue {
    fn new(name: String) -> Self {
        SnookerCue { name: name }
    }
}

impl ChinesePoolCue {
    fn new(name: String) -> Self {
        ChinesePoolCue { name: name }
    }
}

impl Cue for SnookerCue {
    fn shoot(&self) {
        println!("Snooker cue({}), 打点精准", self.name)
    }
}

impl Cue for ChinesePoolCue {
    fn shoot(&self) {
        println!("Chinese pool cue({}), 旋转加塞", self.name)
    }
}

trait CueFactory {
    fn create_cue(&self, name: String) -> Box<dyn Cue>;
}

struct SnookerCueFactory;
struct ChinesePoolCueFactory;

impl CueFactory for SnookerCueFactory {
    fn create_cue(&self, name: String) -> Box<dyn Cue> {
        Box::new(SnookerCue::new(name))
    }
}

impl CueFactory for ChinesePoolCueFactory {
    fn create_cue(&self, name: String) -> Box<dyn Cue> {
        Box::new(ChinesePoolCue::new(name))
    }
}

fn main() {
    let snooker_cue_factory = SnookerCueFactory;
    let snooker_cue = snooker_cue_factory.create_cue(String::from("小头杆"));
    snooker_cue.shoot();

    let chinese_pool_cue_factory = ChinesePoolCueFactory;
    let chinese_pool_cue = chinese_pool_cue_factory.create_cue(String::from("大头杆"));
    chinese_pool_cue.shoot();
}
