use my_crate;

fn main() {
    let conf = my_crate::PoemConfig::read_config();

    if let Ok(poem_config) = conf {
        println!("{:#?}", poem_config);
    }
}
