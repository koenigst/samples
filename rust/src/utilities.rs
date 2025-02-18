use std::{
    cell::LazyCell, collections::HashMap, fs::read_to_string, str::Lines
};

pub trait IntoLines<'a> : IntoIterator<Item = &'a str> {}
impl<'a, T: IntoIterator<Item = &'a str>> IntoLines<'a> for T {}

pub struct ReadLines {
    content: String,
}

impl<'a> IntoIterator for &'a ReadLines {
    type Item = &'a str;
    type IntoIter = Lines<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.lines()
    }
}

pub fn read_lines(path: &str) -> ReadLines {
    ReadLines { content: read_to_string(path).unwrap(), }
}

pub fn group_join<'a, OS, IS, K, OKS, IKS, R, RS>(outer_source: OS, inner_source: IS, outer_key_selector: OKS, inner_key_selector: IKS, result_selector: RS) -> impl 'a + Iterator<Item = R>
where
    OS: 'a + IntoIterator,
    IS: 'a + IntoIterator,
    K: 'a + Eq + std::hash::Hash,
    OKS: 'a + Fn(&OS::Item) -> K,
    IKS: 'a + Fn(&IS::Item) -> K,
    RS: 'a + Fn(&OS::Item, &[IS::Item]) -> R,
{
    let inner_map_cache = LazyCell::new(move || {
        let mut inner_map : HashMap<K, Vec<IS::Item>> = HashMap::new();

        for inner in inner_source {
            let key = inner_key_selector(&inner);
            let values = inner_map.entry(key).or_insert_with(Vec::new);
            values.push(inner);
        }

        inner_map
    });
    
    outer_source.into_iter().map(move |o| {
        let key = outer_key_selector(&o);
        let inner_values = inner_map_cache.get(&key).map_or_else(|| [].as_slice(), Vec::as_slice);
        result_selector(&o, inner_values)
    })
}
