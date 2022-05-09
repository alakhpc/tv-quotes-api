use crate::db::Quote;
use crate::tests::{self, QUOTES};
use rand::Rng;
use std::collections::HashSet;

#[actix_web::test]
async fn test_get_quote() {
    let app = tests::get_service().await;

    let path = "/quotes";
    let res: (_, Quote) = tests::test_get(&app, path).await;

    println!("Response status code should be 200");
    assert_eq!(res.0, 200);

    println!("Response should have a valid quote");
    assert!(QUOTES.contains(&res.1));
}

#[actix_web::test]
async fn test_get_quote_from_show() {
    let app = tests::get_service().await;

    for show in &QUOTES
        .iter()
        .map(|q| q.show.clone())
        .collect::<HashSet<_>>()
    {
        println!("Testing quote from show {}", show);

        let params = serde_urlencoded::to_string(&[("show", show)]).unwrap();
        let path = format!("/quotes?{params}");
        let res: (_, Quote) = tests::test_get(&app, &path).await;

        println!("Response status code should be 200");
        assert_eq!(res.0, 200);

        println!("Response should be from same show");
        assert_eq!(&res.1.show, show);

        println!("Response should have a quote from show");
        assert!(QUOTES.contains(&res.1));
    }
}

#[actix_web::test]
async fn test_case_insensitive_get_quote() {
    let show = QUOTES.iter().next().unwrap().show.clone();
    let app = tests::get_service().await;

    let params = serde_urlencoded::to_string(&[("show", show.to_uppercase())]).unwrap();
    let path = format!("/quotes?{params}");
    let res: (_, Quote) = tests::test_get(&app, &path).await;

    println!("Response status code should be 200");
    assert_eq!(res.0, 200);

    println!("Uppercase request should have correct show");
    assert_eq!(res.1.show, show);

    let params = serde_urlencoded::to_string(&[("show", show.to_lowercase())]).unwrap();
    let path = format!("/quotes?{params}");
    let res: (_, Quote) = tests::test_get(&app, &path).await;

    println!("Response status code should be 200");
    assert_eq!(res.0, 200);

    println!("Lowercase request should have correct show");
    assert_eq!(res.1.show, show);
}

#[actix_web::test]
async fn test_get_multiple_quotes() {
    let app = tests::get_service().await;

    let number_of_quotes: usize = rand::thread_rng().gen_range(1..=10);

    println!("Testing {} quotes", number_of_quotes);

    let path = format!("/quotes/{}", number_of_quotes);
    let res: (_, Vec<Quote>) = tests::test_get(&app, &path).await;

    println!("Response status code should be 200");
    assert_eq!(res.0, 200);

    println!("Response should have {} quotes", number_of_quotes);
    assert_eq!(res.1.len(), number_of_quotes);

    for quote in res.1 {
        println!("Repsonse should have valid quote");
        assert!(QUOTES.contains(&quote));
    }
}

#[actix_web::test]
async fn test_get_multiple_quotes_from_show() {
    let app = tests::get_service().await;

    for show in &QUOTES
        .iter()
        .map(|q| q.show.clone())
        .collect::<HashSet<_>>()
    {
        let number_of_quotes: usize = rand::thread_rng().gen_range(1..=10);

        println!("Testing {} quotes from show {}", number_of_quotes, show);

        let params = serde_urlencoded::to_string(&[("show", show)]).unwrap();
        let path = format!("/quotes/{number_of_quotes}?{params}");
        let res: (_, Vec<Quote>) = tests::test_get(&app, &path).await;

        println!("Response status code should be 200");
        assert_eq!(res.0, 200);

        println!("Response should have {number_of_quotes} quotes");
        assert_eq!(res.1.len(), number_of_quotes);

        println!("Response should have quotes from the show");

        for quote in res.1 {
            println!("Repsonse should be from same show");
            assert_eq!(&quote.show, show);

            println!("Repsonse should have valid quote");
            assert!(QUOTES.contains(&quote));
        }
    }
}

#[actix_web::test]
async fn test_max_10_multiple_quotes() {
    let app = tests::get_service().await;

    let number_of_quotes: usize = 100;

    println!("Testing {}", number_of_quotes);

    let path = format!("/quotes/{}", number_of_quotes);
    let res: (_, Vec<Quote>) = tests::test_get(&app, &path).await;

    println!("Response status code should be 200");
    assert_eq!(res.0, 200);

    println!("Response should have 10 quotes");
    assert_eq!(res.1.len(), 10);

    for quote in res.1 {
        println!("Repsonse should have valid quote");
        assert!(QUOTES.contains(&quote));
    }
}
