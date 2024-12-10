use super::Error;
use reqwest::{blocking::ClientBuilder, cookie::Jar, StatusCode, Url};

pub trait InputReciever {
    fn recieve_input(day: u32, year: u32) -> Result<String, Error> {
        let aoc_session = std::env::var("AOC_SESSION").map_err(|_| Error::AocSession)?;
        let cookie = format!("session={}", aoc_session);
        let url = format!("https://adventofcode.com/{year}/day/{day}/input")
            .parse::<Url>()
            .unwrap();

        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &url);

        let client = ClientBuilder::new()
            .cookie_provider(std::sync::Arc::new(jar))
            .build()
            .map_err(|_| Error::ClientBuild)?;

        let response = client.get(url).send().map_err(|_| Error::SendRequest)?;

        if response.status() == StatusCode::BAD_REQUEST {
            return Err(Error::BadRequest);
        } else if response.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound);
        }

        response.text().map_err(|_| Error::SendRequest)
    }
}
