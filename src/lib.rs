pub mod geocode;
pub mod errors;
pub mod structs;


#[cfg(test)]
mod tests {
    use geocode::Geocoder;
    use std::env;
    use super::*;

    #[tokio::test]
    async fn test_geocode() {
        let api_key = env::var("GEOCODER_API_KEY").expect("GEOCODER_API_KEY must be set");
        let geocoder = Geocoder::new(&api_key);
        let result = geocoder.geocode("1600 Amphitheatre Parkway, Mountain View, CA").await;

        assert!(result.is_ok());
        let geocode_response = result.unwrap();

        assert_eq!(geocode_response.status, "OK");
        assert_eq!(geocode_response.results.len(), 1);
    }
}
