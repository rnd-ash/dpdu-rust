# D-PDU-RS

An implementation of the core D-PDU (ISO 22900-2) library in Rust

## Why ISO 22900-2

ISO 22900-2 is a protocol standard for VCI (Vehicle communication interfaces) that allows for communicating with ECUs in a vehicle from an application using this API as an abstraction layer.

## What does this crate do?

Similarly to [J2534-rust](https://github.com/rnd-ash/j2534-rust), this crate contains just the core definitions of the API (Core functions, error types, structures and enum values),
therefore, it has to be incorporated in a wrapper library in order to get the API to actually work. For an example of such library, check the rust [ecu_diagnostics](https://github.com/rnd-ash/ecu_diagnostics/) crate (**TBA**)

Note that the function and other definition names have been renamed slightly to match the Rust naming
convention.
