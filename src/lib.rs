pub mod geocode;
pub mod errors;
pub mod structs;


#[cfg(test)]
mod tests {
    use geocode::Geocoder;

    use super::*;

    #[tokio::test]
    async fn test_geocode() {
        let geocoder = Geocoder::new("YOUR_API_KEY");
        let result = geocoder.geocode("1600 Amphitheatre Parkway, Mountain View, CA").await;

        assert!(result.is_ok());
        let address = result.unwrap();

        assert_eq!(address.long_name, "1600");

    }
}
