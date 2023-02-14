use crate::battle_ground::BattleGround;

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

                    print!("| {} ", false as u8);
                }
            } else {
                print!("| {} ", false as u8);
            }
        } else {
            print!("      ");
            print!("| {} ", false as u8);
        }
    }
}
