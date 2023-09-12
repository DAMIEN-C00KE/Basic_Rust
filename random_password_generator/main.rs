use rand::Rng;

fn main() {

const CHARSET: &str = // English, Greek, Russian, Numerals
                        "qwertyuiopasdfghjklzxcvbnm\
                        QWERTYUIOPASDFGHJKLZXCVBNM\
                        ςερττυθιοπασδφγηξκλζχψωβνμ\
                        ΕΡΤΥΘΙΟΠΑΣΔΦΓΗΞΚΛΖΧΨΩΒΝΜ\
                        ёйцукенгшщзхъфывапролдячсмитьбю\
                        ЁЙЦУКЕНГШЩЗХЪФЫВАПРОЛДЯЧСМИТЬБЮ\
                        0123456789";
const PASSWORD_LEN: usize = 50; // Adjustable password length
let mut rng = rand::thread_rng();

let password: String = (0..PASSWORD_LEN)
.map(|_| {
    let index = rng.gen_range(0..CHARSET.chars().count()); // Generate password from the given range of characters
    CHARSET.chars().nth(index).unwrap()
})
.collect();

  println!("{:?}", password);
}
