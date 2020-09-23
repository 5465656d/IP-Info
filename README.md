## IP Info
IP Info is a **prototype** command line tool that can take in IPv4 or IPv6 addresses and return some known information from public sources. Right now it's a simple cli-based wrapper for the GeoLite2 or GeoIP2 database. Feed IP Info a valid IP address and it will query the database for information and return it to you in a standard format to be consumed however the user needs. 

### API documentation
The API is not considered stable at this time. 

### Example usage
```bash
    ipinfo -i "8.8.8.8"
```

#### This sounds a lot like whois with JSON formatting... why not just use whois <ip> ?
You probably should, at least while this is still a prototype. Currently there are no gaurantees *at all* for what this tool is or is not, nor are there plans to replace `whois` tools in functionality.

#### Closing Info 
This tool derives some information using the GeoLite2 database created by MaxMind. Usage of GeoLite2 is bound by the [GeoLite2 EULA](https://www.maxmind.com/en/geolite2/eula). 
See [https://www.maxmind.com](https://www.maxmind.com) for additional information.

##### Obtaining the database used by IP Info
IP Info is compatible with both the GeoLite2 and GeoIP2 databases from MaxMind. Signup is required for a license to download the databases. GeoLite2 is freely available upon signup.
The .mmdb files are *not* provided in this repo and must be added by the end user. 