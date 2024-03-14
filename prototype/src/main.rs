use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Cue {
    name: String,
}

impl Cue {
    fn new(name: String) -> Self {
        Cue { name: name }
    }
}

#[derive(Debug, Clone)]
struct NineBall {
    balls: Vec<u8>,
}

impl NineBall {
    fn new(balls: Vec<u8>) -> Self {
        NineBall { balls: balls }
    }
}

fn main() {
    let snooker_cue = Cue::new(String::from("小头杆"));

    let mut chinese_ball_cue = snooker_cue.clone();

    chinese_ball_cue.name = String::from("大头杆");

    println!("Clone前: {:?}", snooker_cue);
    println!("Clone后: {:?}", chinese_ball_cue);

    let nine_balls = NineBall::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut chinese_balls = nine_balls.clone();
    let mut add_balls: Vec<u8> = vec![10, 11, 12, 13, 14, 15];
    chinese_balls.balls.append(&mut add_balls);

    println!("九球: {:?}", nine_balls);
    println!("中八: {:?}", chinese_balls);

    let original_vec = Rc::new(RefCell::new(vec![1, 2, 3]));
    let shallow_vec = Rc::clone(&original_vec);
    println!("修改前: ");
    println!("Original: {:?}", original_vec);
    println!("Shallow: {:?}", shallow_vec);

    original_vec.borrow_mut().push(4);

    println!("修改后: ");
    println!("Original: {:?}", original_vec);
    println!("Shallow: {:?}", shallow_vec);
}
