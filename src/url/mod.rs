use crate::tcg::sets;

pub fn builder(pokemon: String, set: Option<String>, card_number: Option<u32>) -> String {
    let mut url = format!("https://api.pokemontcg.io/v2/cards?q=name:{}", pokemon);
    if let Some(set) = set {
        let set_query = format!(" set.id:{}", sets()[&set as &str]);
        url.push_str(&set_query);
    }
    if let Some(card_number) = card_number {
        let card_number_query = format!(" number:{}", card_number);
        url.push_str(&card_number_query);
    }
    url
}
