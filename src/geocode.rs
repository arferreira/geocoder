use reqwest::Client;

use crate::{errors::GeocoderError, structs::{Address, GeocodeResponse}};

pub struct Geocoder {
    api_key: String,
    client: Client,
}


impl Geocoder {
    pub fn new(api_key: &str) -> Geocoder {
        Self {
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    /// Geocode a location string to an address
    pub async fn geocode(&self, location: &str) -> Result<Address, GeocoderError> {
        let url = format!("https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}", location, self.api_key);
        let response = self.client.get(&url).send().await?;

        let body: serde_json::Value = response.json().await?;


        // Parse response to extract relevant data
        let geocode_response: GeocodeResponse = match serde_json::from_value(body) {
            Ok(response) => response,
            Err(e) => return Err(GeocoderError::ParseError(e)),
        };

        // Check if the response contains any results
        if geocode_response.results.is_empty() {
            return Err(GeocoderError::AddressNotFound);
        }

        // Return the first result
        Ok(geocode_response.results[0].address_components[0].clone())
    }
}
