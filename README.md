# vuldb-api-rust-examples
VulDB Rust code to fetch data via API

<a href="https://vuldb.com/">VulDB</a> is the number 1 vulnerability database worldwide. Our specialists work with the crowd-based community to document the latest vulnerabilities on a daily basis since 1970. Besides technical details there are additional threat intelligence information like current risk levels and exploit price forecasts provided.

VulDB provides a simple, reliable and efficient <a href="https://vuldb.com/?kb.api">API</a>. This interface allows to initiate queries for single entries or collection of items. It does also support transactional bots which implement robotic business process automation (BPA). For example collecting data in Splunk and other correlation tools. You may find code examples and various tools in our <a href="https://github.com/vuldb">official GitHub repository</a>.

---
To execute the rust API demo, you will have to setup a rust project first: 
```bash
   cargo new vuldb_api_demo
   cd vuldb_api_demo
```
Add the dependencies to the `Cargo.toml`:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```
Then update the `src/main.rs` using the provided program.
Now build and run the project: 
```bash
cargo build
cargo run
```
