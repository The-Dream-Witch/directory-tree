fn main() {
    use rand::{distributions::Alphanumeric, Rng};
    for _ in 0..10 {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        println!("{}", s);
    }
    let mut stringz: Vec<String> = Vec::new();

    for _ in 0..8 {
        stringz.push(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect::<String>());
    }

    let mut path: String = "/".to_string();
    for x in stringz {
        path = path + &x.to_string() + &"/".to_string(); 
    }

    println!("{:?}", path);
}
