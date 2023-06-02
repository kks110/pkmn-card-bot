use crate::tcg::sets;

pub fn find_card(pokemon: &String, set: &Option<String>, card_number: &Option<String>) -> String {
    let mut url = format!("https://api.pokemontcg.io/v2/cards?q=name:{}", pokemon);
    if let Some(set) = set {
        let set_query = format!(" set.id:{}", sets()[set as &str]);
        url.push_str(&set_query);
    }
    if let Some(card_number) = card_number {
        let card_number_query = format!(" number:{}", card_number);
        url.push_str(&card_number_query);
    }
    url
}

pub fn find_top_five(pokemon: &Option<String>, set: &Option<String>) -> String {
    let mut addition_query: String = "".to_string();
    if let Some(pokemon) = pokemon {
        addition_query.push_str(&format!("name:{}", pokemon));
    }

    if let Some(set) = set {
        addition_query.push_str(&format!(" set.id:{}", sets()[set as &str]));
    }
    format!("https://api.pokemontcg.io/v2/cards?q={}&orderBy=-cardmarket.prices.avg30&page=1&pageSize=5", addition_query)
}
