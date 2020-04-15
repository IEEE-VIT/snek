use stdweb::traits::*;
use stdweb::web::document;

pub fn update_score(score: u16) {
    let score_dom = document().query_selector("#score").unwrap().unwrap();
    js! {
        @{score_dom}.innerHTML = @{score};
    }
}
