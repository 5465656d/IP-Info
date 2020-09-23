// File - src/lib.rs
use std::net::IpAddr;
use maxminddb::geoip2;

pub fn ip_lookup(ip: &IpAddr) -> Result<IpMetadata, String> {
    // read db in from file
    let ipdb = match maxminddb::Reader::open_readfile("test-data/GeoLite2-City.mmdb") {
        Ok(info) => info,
        Err(e) => return match e {
            // clean up error messages from maxminddb::MaxMindDBError enum
            // TODO: better implicit handling for scalability
            maxminddb::MaxMindDBError::AddressNotFoundError(msg) => Err(msg),
            maxminddb::MaxMindDBError::InvalidDatabaseError(msg) => Err(msg),
            maxminddb::MaxMindDBError::IoError(msg) => Err(msg),
            maxminddb::MaxMindDBError::MapError(msg) => Err(msg),
            maxminddb::MaxMindDBError::DecodingError(msg) => Err(msg),
        },
    };

    // search db for ip data
    let location_info: geoip2::City = match ipdb.lookup(*ip) {
        // maxminddb library consumes ip, we do not so must de-ref here
        Ok(info) => info,
        Err(e) => return match e {
            // clean up error messages from maxminddb::MaxMindDBError enum
            // TODO: better implicit handling for scalability
            maxminddb::MaxMindDBError::AddressNotFoundError(msg) => Err(msg),
            maxminddb::MaxMindDBError::InvalidDatabaseError(msg) => Err(msg),
            maxminddb::MaxMindDBError::IoError(msg) => Err(msg),
            maxminddb::MaxMindDBError::MapError(msg) => Err(msg),
            maxminddb::MaxMindDBError::DecodingError(msg) => Err(msg),
        },
    };

    let country: String = match location_info.country {
        None => "Unknown".to_string(),
        Some(city) => match &city.names {
            None => "Unknown".to_string(),
            Some(name) => name["en"].to_string(),
        },
    };

    let subdivision: String = match location_info.subdivisions {
        None => "Unknown".to_string(),
        Some(sub) => match &sub[0].names {
            None => String::from("Unknown"),
            Some(name) => name["en"].to_string(),
        },
    };

    let city: String = match location_info.city {
        None => "Unknown".to_string(),
        Some(city) => match &city.names {
            None => "Unknown".to_string(),
            Some(name) => name["en"].to_string(),
        },
    };

    let postcode: String = match location_info.postal {
        None => "Unknown".to_string(),
        Some(postal) => match &postal.code {
            None => "Unknown".to_string(),
            Some(code) => code.to_string(),
        },
    };

    let location = Location {
        country: country,
        subdivision: subdivision,
        city: city,
        postcode: postcode,
    };

    let network = Network {
        ip: *ip,
        hostname: "unimplemented data".to_string(),
        asn: 1234, // might need to be an optional if not in db?
        isp: "unimplemented data".to_string(),
    };

    // return Ok result
    Ok(IpMetadata {network, location})
}

// Struct definitions
#[derive(Debug)]
pub struct Network {
    ip: IpAddr,
    hostname: String,
    asn: u32, // ASNs are 4 bytes
    isp: String,
}

#[derive(Debug)]
pub struct Location {
    country: String,
    subdivision: String,
    city: String,
    postcode: String,
}

#[derive(Debug)]
pub struct IpMetadata {
    network: Network,
    location: Location,
}

impl IpMetadata {
    pub fn new (network: Network, location: Location) -> IpMetadata {
        IpMetadata { network, location}
    }
}