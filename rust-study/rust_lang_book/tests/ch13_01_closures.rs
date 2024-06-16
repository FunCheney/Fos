/// rust 的闭包可以保存在变量中或作为参数传递给其他函数的匿名函数
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor{
        let mut mut_red = 0;
        let mut  mut_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => mut_red += 1,
                ShirtColor::Blue => mut_blue += 1,
            }
        }

        if mut_red > mut_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


#[test]
fn test_01() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
