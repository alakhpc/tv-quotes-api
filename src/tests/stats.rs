use crate::routes::quotes::stats::StatsResponse;
use crate::tests::{self, QUOTES};

#[actix_web::test]
async fn test_get_stats() {
    let app = tests::get_service().await;
    let res: (_, StatsResponse) = tests::test_get(&app, "/quotes/stats").await;

    println!("Response status should be 200");
    assert_eq!(res.0, 200);

    println!("Response total should be > 0");
    assert!(res.1.total > 0);

    let expected_total = QUOTES.len();
    println!("Response total should be {}", expected_total);
    assert_eq!(res.1.total, expected_total);
}
