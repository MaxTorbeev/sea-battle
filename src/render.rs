use crate::battle_ground::BattleGround;
use crate::field::Field;
use crate::ship::Ship;

fn print_field(field: &Field, ships: Vec<Ship>) {
    print!("| {} ", if !field.has_ship(ships) { "  " } else {"[]"});
}

fn print_legend(field: &Field, is_horizontal: bool) {
    if is_horizontal {
        print!("| {}  ", field.asics_y);
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

pub fn battle_ground(battle_ground: BattleGround) {
    print_space();
    for field in battle_ground.fields.iter() {
        if field.asics_x == 1 {
            print_legend(field, true)
        }
    }

    println!();

    print_space();

    for _x in 1..11 {
        print!("-----");
    }

    println!();

    for (idx, field) in battle_ground.fields.iter().enumerate() {
        if idx != 0 {
            if battle_ground.fields.get(idx - 1).unwrap().asics_x != field.asics_x {
                if field.asics_y == 1 {
                    println!();
                    print_legend(field, false);

                    print_field(field, battle_ground.ships.clone());
                }
            } else {
                print_field(field, battle_ground.ships.clone());
            }
        } else {
            print_legend(field, false);
            print_field(field, battle_ground.ships.clone());
        }
    }
}
