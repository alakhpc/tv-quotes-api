use crate::routes::quotes::shows::ShowsResponse;
use crate::tests::{self, QUOTES};
use std::collections::HashSet;

#[actix_web::test]
async fn test_get_shows() {
    let app = tests::get_service().await;
    let res: (_, ShowsResponse) = tests::test_get(&app, "/quotes/shows").await;

    println!("Response status should be 200");
    assert_eq!(res.0, 200);

    let expected_shows = QUOTES
        .iter()
        .map(|q| q.show.clone())
        .collect::<HashSet<_>>();

    println!("Response shows should be 2");
    assert_eq!(res.1.shows.len(), expected_shows.len());

    for show in &expected_shows {
        println!("Response shows should contain {}", show);
        assert!(expected_shows.contains(show));
    }
}
