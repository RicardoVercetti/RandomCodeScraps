### PPIMS in Rust

this is just a toy project to rebuild the PPIMS dummy server that I built in python for the DCMS project.
Using this I wanna learn handling http requests, basic serialization and deserialization, file handling, etc in rust.

#### Items

- [x] basic get & post server
- [x] define default endpoints
- [x] requst json pretty printer
- [x] deserialize with custom field names
- [] error logs in server side console
- [x] store customer data in file
- [x] load data on startup
- [] have ppims headers
- [x] have mobile number for customer dedupe check
- [] have the KYC restrictions enabled
- [] no std logs or err logs, full async
- [] optional json params

#### Things to learn in the process

- Building a basic http server
- have endponts to respond in json
- file handling
- async in rust
- error handling
- code separation