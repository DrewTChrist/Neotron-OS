//! Screen-related commands for Neotron OS

//use neotron_common_bios::video::{Attr, TextBackgroundColour, TextForegroundColour};

use crate::{print, println, Ctx};

pub static CHANGE_LAYOUT: menu::Item<Ctx> = menu::Item {
    item_type: menu::ItemType::Callback {
        function: layout,
        parameters: &[menu::Parameter::Optional {
            parameter_name: "layout",
            help: Some("Keyboard layout"),
        }],
    },
    command: "layout",
    help: Some("Change keyboard layouts"),
};

fn layout(_menu: &menu::Menu<Ctx>, item: &menu::Item<Ctx>, args: &[&str], ctx: &mut Ctx) {
    if let Ok(Some(layout)) = menu::argument_finder(item, args, "layout") {
        match layout {
            "azerty" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::Azerty(pc_keyboard::layouts::Azerty)),
            "colemak" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::Colemak(pc_keyboard::layouts::Colemak)),
            "dvp104key" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::DVP104Key(pc_keyboard::layouts::DVP104Key)),
            "de105key" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::De105Key(pc_keyboard::layouts::De105Key)),
            "dvorak104key" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::Dvorak104Key(pc_keyboard::layouts::Dvorak104Key)),
            "jis109key" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::Jis109Key(pc_keyboard::layouts::Jis109Key)),
            "uk105key" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::Uk105Key(pc_keyboard::layouts::Uk105Key)),
            "us104key" => ctx.keyboard.change_layout(pc_keyboard::layouts::AnyLayout::Us104Key(pc_keyboard::layouts::Us104Key)),
            _ => {}
        }
    }
}
