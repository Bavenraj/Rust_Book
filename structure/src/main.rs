struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}
fn main() {

    let user1 = User{
        active: true,
        username: String::from("somomss") ,
        email: String::from("somsm@email.com"), 
        sign_in_count: 1,
    };
    let mut user2 = User{
        active: true,
        username: String::from("somomss") ,
        email: String::from("somsm@email.com"), 
        sign_in_count: 1,
    };

    user2.email = String::from("hellow wordls");
    let user3 = build_user(String::from("hellow "), String::from(" wordls"));

    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@exammple.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user5 = User{
        email: String::from("tetstststs"),
        ..user2
    };
    
    println!("{}", user1.email);
    println!("{}", user2.email);
    println!("{} {}", user3.email, user3.username);
    println!("{} {}", user4.email, user4.username);
    println!("{} {}", user5.email, user5.username);
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count:1,
    }
}
