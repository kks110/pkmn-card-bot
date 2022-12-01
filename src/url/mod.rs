use crate::tcg::sets;

pub fn builder(pokemon: String, set: Option<String>, card_number: Option<u32>) -> String {
    let mut url = format!("https://api.pokemontcg.io/v2/cards?q=name:{}", pokemon);
    if set.is_some() {
        let set_query = format!(" set.id:{}", sets()[&set.unwrap() as &str]);
        url.push_str(&set_query);
    }
    if card_number.is_some() {
        let card_number_query = format!(" number:{}", card_number.unwrap());
        url.push_str(&card_number_query);
    }
    url
}
