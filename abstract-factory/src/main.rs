trait ShootCue {
    // 普通击球
    fn shoot(&self);
}

trait BreakCue {
    // 暴力开球
    fn power_shoot(&self);
}

struct SnookerShootCue;
impl ShootCue for SnookerShootCue {
    // 斯诺克球杆普通击球
    fn shoot(&self) {
        println!("每一杆都是用的打杆")
    }
}

struct SnookerBreakCue;
impl BreakCue for SnookerBreakCue {
    // 斯诺克球杆开球
    fn power_shoot(&self) {
        println!("虽然用的打杆, 但是也能暴力开球")
    }
}

struct ChinesePoolShootCue;
impl ShootCue for ChinesePoolShootCue {
    // 中八专门的打杆,普通击球
    fn shoot(&self) {
        println!("使用专门的打杆击球")
    }
}

struct ChinesePoolBreakCue;
impl BreakCue for ChinesePoolBreakCue {
    // 中八专门的冲杆, 开球
    fn power_shoot(&self) {
        println!("使用专门的冲杆,在开球是会很暴力,把球打得很散")
    }
}

trait CueFactory {
    fn create_shoot_cue(&self) -> Box<dyn ShootCue>;
    fn create_break_cue(&self) -> Box<dyn BreakCue>;
}

// 斯诺克球杆工厂
struct SnookerCueFactory;
impl CueFactory for SnookerCueFactory {
    fn create_shoot_cue(&self) -> Box<dyn ShootCue> {
        Box::new(SnookerShootCue)
    }

    fn create_break_cue(&self) -> Box<dyn BreakCue> {
        Box::new(SnookerBreakCue)
    }
}

// 中八球杆工厂
struct ChinesePoolCueFactory;
impl CueFactory for ChinesePoolCueFactory {
    fn create_shoot_cue(&self) -> Box<dyn ShootCue> {
        Box::new(ChinesePoolShootCue)
    }

    fn create_break_cue(&self) -> Box<dyn BreakCue> {
        Box::new(ChinesePoolBreakCue)
    }
}

fn main() {
    println!("在玩斯诺克时");
    let snooker_cue_factory = SnookerCueFactory;
    let snooker_cue = snooker_cue_factory.create_shoot_cue();
    snooker_cue.shoot();
    let snooker_cue = snooker_cue_factory.create_break_cue();
    snooker_cue.power_shoot();

    println!("在玩中八时");
    let chinese_pool_cue_factory = ChinesePoolCueFactory;
    let chinese_pool_cue = chinese_pool_cue_factory.create_shoot_cue();
    chinese_pool_cue.shoot();
    let chinese_pool_cue = chinese_pool_cue_factory.create_break_cue();
    chinese_pool_cue.power_shoot();
}
