# Geocoder

[![Build Status](https://github.com/arferreira/geocoder/actions/workflows/ci.yml/badge.svg)](https://github.com/arferreira/geocoder/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/arferreira/geocoder/badge.svg?branch=main)](https://coveralls.io/github/arferreira/geocoder?branch=main)

Rust crate that provides an easy way to use the Google Geocoding API.

See more information about the **Google Geocoding API** at the following link: [Google Geocoding API](https://developers.google.com/maps/documentation/geocoding/start)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
geocoder = "0.1.2"
```

## Usage

Example of usage:

```rust
use geocoder::Geocoder;
use std::env;

#[tokio::main]
async fn main() {
    // Set your API key as an environment variable.
    let api_key = env::var("GEOCODER_API_KEY").expect("GEOCODER_API_KEY must be set");

    // Create a new Geocoder instance.
    let geocoder = Geocoder::new(&api_key);

    // Geocode an address to a location (latitude, longitude).
    match geocoder.geocode("1600 Amphitheatre Parkway, Mountain View, CA").await {
        Ok(address) => {
            println!("Formatted address: {}", address.formatted_address);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Examples

Convert an address to a latitude and longitude:

```rust
use geocoder::Geocoder;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("GEOCODER_API_KEY").expect("GEOCODER_API_KEY must be set");
    let geocoder = Geocoder::new(&api_key);
    match geocoder.geocode("1600 Amphitheatre Parkway, Mountain View, CA").await {
        Ok(address) => {
            println!("Formatted address: {}", address.formatted_address);
            println!("Latitude: {}", address.geometry.location.lat);
            println!("Longitude: {}", address.geometry.location.lng);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Reverse Geocoding
Convert a latitude and longitude to an address:

```rust
use geocoder::{Geocoder, structs::LatLng};
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("GEOCODER_API_KEY").expect("GEOCODER_API_KEY must be set");
    let geocoder = Geocoder::new(&api_key);
    let location = LatLng { lat: 37.4224764, lng: -122.0842499 };
    match geocoder.reverse_geocode(location).await {
        Ok(addresses) => {
            if let Some(address) = addresses.first() {
                println!("Formatted address: {}", address.formatted_address);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Create an issue (optional)
2. Fork the repo
3. Make your changes
4. Commit your changes (`git commit -am 'Some cool feature'`)
5. Push to the branch (`git push origin master`)
6. Create a new Pull Request

## Using async functions

With a little more effort you can use **async** functions with **tokio**:

```rust
use geocoder::Geocoder;
use std::env;
use tokio::task;

async fn geocode_address(geocoder: Geocoder, address: &str) {
    match geocoder.geocode(address).await {
        Ok(result) => {
            println!("Formatted address: {}", result.formatted_address);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[tokio::main]
async fn main() {
    let api_key = env::var("GEOCODER_API_KEY").expect("GEOCODER_API_KEY must be set");
    let geocoder = Geocoder::new(&api_key);
    let address = "1600 Amphitheatre Parkway, Mountain View, CA";

    let handle = task::spawn(async move {
        geocode_address(geocoder, address).await;
    });

    handle.await.unwrap();
}
```
