use askama::Template;

#[derive(Template)]
#[template(path = "search.html")]
pub struct Search {
    pub query: String,
    pub results: String,
}

#[derive(Template)]
#[template(path = "grid.html")]
pub struct GridItem {
    pub id: String,
    pub poster: String,
}

#[derive(Template)]
#[template(path = "movie.html")]
pub struct MovieDetails {
    pub id: String,
    pub title: String,
    pub poster: String,
    pub plot: String,
}
