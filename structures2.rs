struct user{
    active:bool,
    name:String,
    id:String,
    age:u64,
    sign_in_count:u64,
}

fn main(){
    let mut user1 = user{
    id: String::from("paarishdhillon@gmail.com"),
    name: String::from("Paarish Dhillon"),
    age: 21,
    sign_in_count: 1,
    active: true
};

    let mut user2 = user{
        id: String::from("viraajdhillon@gmail.com"),
        name: String::from("Viraaj Dhillon"),
        ..user1,
    };
    // user2.name = String::from("Viraaj Dhillon");
    user1.id = String::from("dhillonpaarish@gmail.com");
    println!("id = {}, name = {}, age =  {}", user1.id, user1.name, user1.age);
    println!("id = {}, name = {}, age =  {}", user2.id, user2.name, user2.age);
}