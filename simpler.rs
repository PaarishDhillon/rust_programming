fn main(){
    move_around("East".to_string());
}

fn move_around(Direction: String){
    if Direction == "North"{
        println!("Moving North");
    }
    if Direction == "West"{
        println!("Moving West");
    }
    if Direction == "East"{
        println!("Moving East");
    }
    if Direction == "South"{
        println!("Moving South");
    }
}