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
        let t_name = format!("{}({})", name, String::from("小头杆"));
        SnookerCue { name: t_name }
    }
}

impl Cue for SnookerCue {
    fn shoot(&self) {
        println!("{}, 精准打击", self.name);
    }
}

impl Cue for ChinesePoolCue {
    fn shoot(&self) {
        println!("{}, 旋转加塞", self.name);
    }
}

impl ChinesePoolCue {
    fn new(name: String) -> Self {
        let t_name = format!("{}({})", name, String::from("大头杆"));
        ChinesePoolCue { name: t_name }
    }
}

enum CueType {
    S,
    C,
}

struct CueSimpleFactory;

impl CueSimpleFactory {
    // 定义一个创建对象的类, 由这个类来封装实例化对象的行为
    fn create_cue(cue_type: CueType) -> Box<dyn Cue> {
        match cue_type {
            CueType::S => Box::new(SnookerCue::new(String::from("snooker cue"))),
            CueType::C => Box::new(ChinesePoolCue::new(String::from("chinese pool cue"))),
        }
    }
}

fn main() {
    let cue_s = CueSimpleFactory::create_cue(CueType::S);
    cue_s.shoot();

    let cue_c = CueSimpleFactory::create_cue(CueType::C);
    cue_c.shoot();
}
