use crate::battle_ground::BattleGround;
use crate::field::Field;
use crate::ship::Ship;

fn print_field(field: &Field, ships: Vec<Ship>) {
    print!("| {} ", field.has_ship(ships) as u8);
}

pub fn battle_ground(battle_ground: BattleGround) {
    print!("      ");

    for (idx, field) in battle_ground.fields.iter().enumerate() {
        if field.asics_x == 1 {
            print!("| {} ", (idx + 1));
        }
    }

    println!();

    print!("      ");

    for _x in 1..11 {
        print!("----");
    }

    println!();

    for (idx, field) in battle_ground.fields.iter().enumerate() {
        if idx != 0 {
            if battle_ground.fields.get(idx - 1).unwrap().asics_x != field.asics_x {
                if field.asics_y == 1 {
                    println!();
                    print!(" А |  ");

                    print_field(field, battle_ground.ships.clone());
                }
            } else {
                print_field(field, battle_ground.ships.clone());
            }
        } else {
            print!("      ");
            print_field(field, battle_ground.ships.clone());
        }
    }
}
