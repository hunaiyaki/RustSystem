fn main() {
    
    let mut user1 = build_user(String::from("hello")
        , String::from("world"));
    
    let user2 = User {
        email: String::from(".com"),
        username: String::from(".jp"),
        ..user1//残りをuser1のインスタンスで埋める
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

}

//関数
fn build_user(email: String, username: String) -> User {

    User {
      email,
      username,
      active: true,
      sign_in_count: 1
    }
}

//構造体
struct User {
    username: String,    
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);