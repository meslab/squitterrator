# SQUITTERATOR: ADS-B Mode S Decoder

Welcome to Squitterator, an ADS-B Mode S decoder application! This project is a modernization of my original Perl-based ADS-B Mode S decoder study from 2013, now rewritten in Rust.

## Installation

To get started with Squitterator, you'll need the Rust toolchain installed on your system. If you haven't already set it up, you can find detailed instructions on how to do so [here](https://www.rust-lang.org/tools/install).

```
git clone https://github.com/meslab/squitterator.git
cd squitterator/
make install
```

## Usage

```
squitterator -t <hostname>:30002
```

make sure to check help section of the command
```
squitterator -h
```

## References

- [The 1090 Megahertz Riddle: A Guide to Decoding Mode S and ADS-B Signals](https://mode-s.org/decode/index.html)
- [FAA ADS-B Resources](https://www.faa.gov/air_traffic/technology/adsb/documents)
- DOI: [10.1109/TIT.1964.1053665](https://ieeexplore.ieee.org/document/1053665)
- MODE S DOWNLINK AIRCRAFT PARAMETERS IMPLEMENTATION AND OPERATIONS GUIDANCE DOCUMENT - Edition 1.0 - March 2019
- Working Paper 1090-WP30-18 as DRAFT Version 4.2 of DO-282B
- Guidelines for ICAO 24 Bits Adresses Assignment.pdf

