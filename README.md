# rust_wireshark_dissector

Simple example using [Wirdigen](https://crates.io/crates/wirdigen) library to generate LUA dissector for Wireshark.

## Usage

`./rust_wireshark_dissector <path_to_json>`

### Note

On linux, this sample generate the plugin automatically into wirehsark plugin directory. You might need to manually create the folder before executing. 

For non-linux OS, output directory is the default directory configured by `Wirdigen`: OS temporary folder.