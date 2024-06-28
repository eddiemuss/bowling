use bowling::round::Round;

fn main() {
    let round = Round::new().role(1).unwrap().role(9);

    let last_throw = round.as_ref().unwrap().last_throw();

    println!("number {} have bin rolled.", round.unwrap());
    println!("number {} have bin rolled.", Round::new().role(2).unwrap());
    if let Some(last_throw) = last_throw {
        println!("number {last_throw} have bin rolled.");
    }
}
