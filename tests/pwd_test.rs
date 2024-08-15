use md5::{Digest, Md5};

#[test]
fn pwd() {
    //admin 21232f297a57a5a743894a0e4a801fc3
    let pwd = format!("{:x}", Md5::digest("admin".as_bytes()));
    println!("{}", pwd);
}
