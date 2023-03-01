use crate::battle_ground::BattleGround;
use crate::field::Field;

fn print_field(field: &Field, battle_ground: BattleGround) {
    let ship = field.get_ship(&battle_ground.ships);

    if ship.is_some() {
        print!("| [{}] ", ship.unwrap().desks);
    } else {
        print!("|     ");
    }
}

fn print_legend(field: &Field, is_horizontal: bool) {
    if is_horizontal {
        if field.asics_y.to_string().len() == 2 {
            print!("|  {} ", field.asics_y);
        } else {
            print!("|  {}  ", field.asics_y);
        }

    } else {
        if field.asics_x.to_string().len() == 2 {
            print!(" {}   ", field.asics_x);
        } else {
            print!(" {}    ", field.asics_x);
        }
    }
}

fn print_space() {
    print!("      ");
}

pub fn battle_ground(battle_ground: &BattleGround, size: u8) {
    println!();
    println!();
    print_space();

    for field in battle_ground.fields.iter() {
        if field.asics_x == 1 {
            print_legend(field, true)
        }
    }

    println!();

    print_space();

    for _x in 1..(size + 1) {
        print!("------");
    }

    println!();

    for (idx, field) in battle_ground.fields.iter().enumerate() {
        if idx != 0 {
            if battle_ground.fields.get(idx - 1).unwrap().asics_x != field.asics_x {
                if field.asics_y == 1 {
                    println!();
                    print_legend(field, false);

                    print_field(field, battle_ground.clone());
                }
            } else {
                print_field(field, battle_ground.clone());
            }
        } else {
            print_legend(field, false);
            print_field(field, battle_ground.clone());
        }
    }
}
