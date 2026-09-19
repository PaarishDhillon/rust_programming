struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
    age:u16,
}

fn main(){
    let user1 = User{
    active :true,
    email: String::from( "paarishdhillon@gmail.com"),
    username:String::from( "Paarish Dhillon"),
    sign_in_count: 1,
    age:18,
};
println!("{} is {} years old.",user1.username, user1.age);
}