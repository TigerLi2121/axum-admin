pub mod date_format;
pub mod db;
pub mod jwt;
pub mod req;
pub mod res;
pub mod macros;

#[cfg(test)]
mod date_test {
    use chrono::{Local, NaiveDateTime};
    use crate::{date_su, date_s};
    use crate::module::sys::user::model::User;

    #[derive(Debug)]
    struct Dog {
        pub created_at: Option<NaiveDateTime>,
    }

    #[test]
    pub fn date_def() {
        let mut user = User {
            id: Some(1),
            username: Option::from(String::from("bbb4")),
            password: Option::from(String::from("bbb")),
            email: Option::from(String::from("bbb")),
            mobile: Option::from(String::from("bbb")),
            status: Option::from(1),
            created_at: None,
            updated_at: None,
        };
        date_su!(&mut user);
        println!("{:?}", user);

        let mut dog = Dog {
            created_at: None,
        };
        date_s!(dog);
        println!("{:?}", dog);
    }
}
