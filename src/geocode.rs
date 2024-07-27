use reqwest::Client;

use crate::{errors::GeocoderError, structs::{GeocodeResponse}};

pub struct Geocoder {
    api_key: String,
    client: Client,
}


impl Geocoder {
    /// Creates a new instance of Geocoder.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key.

    pub fn new(api_key: &str) -> Geocoder {
        Self {
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    /// Geocodes a location string to an address.
    ///
    /// # Arguments
    ///
    /// * `location` - A string slice that holds the location to geocode.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok` containing an `Address` if the geocoding is successful.
    /// - `Err` containing a `GeocoderError` if there is an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails, the response cannot be parsed,
    /// or if the address is not found.
    pub async fn geocode(&self, location: &str) -> Result<GeocodeResponse, GeocoderError> {
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
        Ok(geocode_response)
    }
}
