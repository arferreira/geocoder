/// LatLng is a struct that represents a latitude and longitude pair.
#[derive(Debug, Clone, Copy)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

/// Bounds is a struct that represents NorthEast and SouthWest LatLng pairs.
#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub northeast: LatLng,
    pub southwest: LatLng,
}


/// Geometry is a struct that represents a location and a viewport.
#[derive(Debug, Clone)]
pub struct Geometry {
    pub bounds: Bounds,
    pub location: LatLng,
    pub location_type: String,
    pub viewport: Bounds,
}

/// Address is a struct that represents a formatted address.
#[derive(Debug, Clone)]
pub struct Address {
    pub long_name: String,
    pub short_name: String,
    pub types: Vec<String>,
}

/// Result is a struct that represents a geocoding result.
#[derive(Debug, Clone)]
pub struct Result {
    pub address_components: Vec<Address>,
    pub formatted_address: String,
    pub geometry: Geometry,
    pub place_id: String,
    pub types: Vec<String>,
}

/// Results is a struct that represents a vector of geocoding results.
#[derive(Debug, Clone)]
pub struct Results {
    pub results: Vec<Result>,
    pub status: String,
    pub error_message: Option<String>,
}
