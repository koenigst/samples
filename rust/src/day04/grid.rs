use super::location::Location;

pub struct Grid {
    content: Vec<Vec<char>>
}

impl Grid {    
    pub fn try_get(&self, location: Location) -> Option<&char> {
        self.content.get(location.0).and_then(|l| l.get(location.1))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Grid, Location)> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(li, l)| l.iter().enumerate().map(move |(ci, _)| Location(li, ci)))
            .map(move |l| (self, l))
    }
}

impl<'a> FromIterator<&'a str> for Grid
{
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Grid {
            content: iter
                .into_iter()
                .map(|l| l.chars().collect())
                .collect(),
        }
    }
}
