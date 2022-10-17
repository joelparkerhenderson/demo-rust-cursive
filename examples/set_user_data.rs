//! Set user data via a custom struct with fields we want.

struct UserData {
    foo: usize,
    goo: usize,
}

pub fn main() {
    let mut c = cursive::default();
    let user_data = UserData {
        foo: 1,
        goo: 2,
    };
    c.set_user_data(user_data);
    c.run();
}
