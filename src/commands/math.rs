//! Math commands for Neotron OS

use crate::{println, Ctx};

pub static MATH_ITEM: menu::Item<Ctx> = menu::Item {
    item_type: menu::ItemType::Callback {
        function: math,
        parameters: &[
            menu::Parameter::Optional {
                parameter_name: "operator",
                help: Some("The math operator to use"),
            },
            menu::Parameter::Optional {
                parameter_name: "left_operand",
                help: Some("The left operand"),
            },
            menu::Parameter::Optional {
                parameter_name: "right_operand",
                help: Some("The right operand"),
            },
        ],
    },
    command: "math",
    help: Some("Simple two operand math functions"),
};

/// Called when the "math" command is executed.
fn math(_menu: &menu::Menu<Ctx>, item: &menu::Item<Ctx>, args: &[&str], _ctx: &mut Ctx) {
    if let Ok(Some(operator)) = menu::argument_finder(item, args, "operator") {
        if let Ok(Some(left)) = menu::argument_finder(item, args, "left_operand") {
            if let Ok(Some(right)) = menu::argument_finder(item, args, "right_operand") {
                let l = match left.parse::<i32>() {
                    Ok(val) => val,
                    Err(e) => {
                        println!("left_operand: {}", e);
                        return;
                    }
                };
                let r = match right.parse::<i32>() {
                    Ok(val) => val,
                    Err(e) => {
                        println!("right_operand: {}", e);
                        return;
                    }
                };
                match operator {
                    "add" => {
                        println!("{}", l + r);
                    }
                    "sub" => {
                        println!("{}", l - r);
                    }
                    "mult" => {
                        println!("{}", l * r);
                    }
                    "div" => {
                        if r == 0 {
                            println!("Cannot divide by zero");
                            return;
                        }
                        println!("{}", l / r);
                    }
                    _ => {}
                }
            }
        }
    }
}
