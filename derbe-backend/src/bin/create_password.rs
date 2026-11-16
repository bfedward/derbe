fn main() {
    let password = "TestPassword123!";
    let hash = pwhash::bcrypt::hash(password).unwrap();

    println!("Password: {password}");
    println!("Hash: {hash}");
}
