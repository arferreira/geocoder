use serde::Deserialize;

/// LatLng is a struct that represents a latitude and longitude pair.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

/// Bounds is a struct that represents NorthEast and SouthWest LatLng pairs.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Bounds {
    pub northeast: LatLng,
    pub southwest: LatLng,
}

/// Geometry is a struct that represents a location and a viewport.
#[derive(Debug, Clone, Deserialize)]
pub struct Geometry {
    pub bounds: Option<Bounds>,
    pub location: LatLng,
    #[serde(rename = "location_type")]
    pub location_type: String,
    pub viewport: Bounds,
}

/// AddressComponent is a struct that represents a component of an address.
#[derive(Debug, Clone, Deserialize)]
pub struct Address {
    #[serde(rename = "long_name")]
    pub long_name: String,
    #[serde(rename = "short_name")]
    pub short_name: String,
    pub types: Vec<String>,
}

/// Result is a struct that represents a geocoding result.
#[derive(Debug, Clone, Deserialize)]
pub struct Result {
    #[serde(rename = "address_components")]
    pub address_components: Vec<Address>,
    #[serde(rename = "formatted_address")]
    pub formatted_address: String,
    pub geometry: Geometry,
    #[serde(rename = "place_id")]
    pub place_id: String,
    pub types: Vec<String>,
}

/// GeocodeResponse is a struct that represents a response from the geocoding API.
#[derive(Debug, Clone, Deserialize)]
pub struct GeocodeResponse {
    pub results: Vec<Result>,
    pub status: String,
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
}
