use fake::{
    faker::{
        internet::raw::{Password, Username},
        lorem::raw::{Paragraph, Sentence, Sentences, Word, Words},
        name::raw::Name,
    },
    locales::{self, Data, DE_DE, EN, FR_FR, PT_BR},
    Dummy, Fake, Faker,
};
use rand::{
    seq::{IndexedRandom, SliceRandom},
    Rng,
};
use regex::Regex;

use std::{
    any::{self, Any},
    ops::Range,
    path::Display,
};

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
}

#[derive(Debug, Dummy)]
struct Bar<T> {
    field: Vec<T>,
}

#[derive(Debug)]
enum Langenum {
    EN,
    FR,
    PT,
    DE,
}

#[derive(Copy, Clone)]
enum LocaleWrapper {
    EN(EN),
    FR(FR_FR),
    PT(PT_BR),
    DE(DE_DE),
}

#[derive(Debug)]
struct Asu;

impl Data for Asu {}

#[derive(Clone, Copy)]
enum TestAjah {
    Jancuk(DE_DE),
}

impl Data for TestAjah {}

impl Data for LocaleWrapper {}

impl Langenum {
    fn to_locale(&self) -> LocaleWrapper {
        match self {
            Langenum::EN => LocaleWrapper::EN(EN),
            Langenum::FR => LocaleWrapper::FR(FR_FR),
            Langenum::PT => LocaleWrapper::PT(PT_BR),
            Langenum::DE => LocaleWrapper::DE(DE_DE),
        }
    }
}

pub fn generate_password(
    over_12: bool,
    use_nonenglish: bool,
    use_specialchar: bool,
    use_uppercase: bool,
    use_number: bool,
) -> String {
    // let f: Foo = Faker.fake();
    // println!("{:?}", f);

    // let b: Bar<Foo> = Faker.fake();
    // println!("{:?}", b);

    // let name: String = Name(EN).fake();
    // println!("name {:?}", name);

    let name: String = Name(EN).fake();
    // println!("name {:?}", name);

    let length = if over_12 { 4..6 } else { 4..8 };

    let mut available_lang: Vec<Langenum> = if use_nonenglish {
        vec![Langenum::PT, Langenum::FR, Langenum::DE]
    } else {
        vec![Langenum::EN]
    };

    let mut rng = rand::rng();
    let random_index = rng.random_range(0..available_lang.len());

    // println!("👉 random index {:?}", random_index);

    // println!("🚀 {:?}", LocaleWrapper::EN(EN));

    let mut selected_lang = available_lang.get(random_index).unwrap().to_locale();
    let base_password: String = Password(selected_lang.clone(), length.clone()).fake();
    let base_words: String = Sentence(selected_lang, length).fake();
    // let mut output = format!("{} {}", base_password, base_words);

    let mut output = generator(if over_12 { rng.random_range(14..=20) } else { rng.random_range(8..=12) }, use_specialchar);

    if !use_number {
        let re = Regex::new(r"[\d\s]+").unwrap();
        output = re.replace_all(&output, "").to_string();
    }

    if !use_uppercase {
        output = output.to_lowercase();
    }

    output = output.replace(" ", "");

    let test = TestAjah::Jancuk(DE_DE);

    println!(
        "😭 {use_number} {use_uppercase} {} 👻 \n\n{:#?}\n\n",
        output, base_words
    );

    output
} // end func

fn generator(max_length: usize, use_specialchar: bool) -> String {
    let mut generated_password = String::new();

    loop {
        let kata = decider();
        generated_password.push_str(&kata);

        if !use_specialchar {
            let re = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
            generated_password = re.replace_all(&generated_password, "").to_string();
        }

        if generated_password.len() >= max_length {
            break;
        }
    }

    generated_password
}

fn decider() -> String {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..=7);
    let charuniq = ["!", "@", "#", "$", "%", "&", "?"];
    let uppercase = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    match random_index {
        1 => Word(EN).fake(),
        2 | 3 => rng.random_range(0..1000).to_string(),
        4 | 5 => charuniq[rng.random_range(0..charuniq.len())].to_string(),
        _ => uppercase[rng.random_range(0..uppercase.len())].to_string(),
    }
}
