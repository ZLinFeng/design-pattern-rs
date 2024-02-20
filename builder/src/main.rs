// 定义一个新的产品
#[derive(Debug)]
struct Cue {
    wood: String,
    cue_tip: String,
    wrap: String,
}

impl Default for Cue {
    fn default() -> Self {
        Self {
            wood: String::from("普通木料"),
            cue_tip: String::from("普通皮头"),
            wrap: String::from("普通握把"),
        }
    }
}

// 定义一个抽象生成器builder
trait CueBuilder {
    fn new() -> Self;
    fn build_wood(&mut self, wood: String);
    fn build_cue_tip(&mut self, cue_tip: String);
    fn build_wrap(&mut self, wrap: String);
    fn build(&mut self) -> Cue;
}

// 定义一个具体的生成器
struct SnookerCueBuilder {
    cue: Cue,
}

impl CueBuilder for SnookerCueBuilder {
    fn new() -> Self {
        SnookerCueBuilder {
            cue: Cue::default(),
        }
    }

    fn build_wood(&mut self, wood: String) {
        self.cue.wood = wood
    }

    fn build_cue_tip(&mut self, cue_tip: String) {
        self.cue.cue_tip = cue_tip
    }

    fn build_wrap(&mut self, wrap: String) {
        self.cue.wrap = wrap
    }

    fn build(&mut self) -> Cue {
        std::mem::take(&mut self.cue)
    }
}

fn main() {
    let mut snooker_builder = SnookerCueBuilder::new();

    // 只选择橡木的木料
    snooker_builder.build_wood(String::from("橡木"));
    let cue = snooker_builder.build();
    println!("第一次构建: {:?}", cue);

    // 只选择握把的材质
    snooker_builder.build_wrap(String::from("皮革"));
    let cue = snooker_builder.build();
    println!("第二次构建: {:?}", cue);
}
