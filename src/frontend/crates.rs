use crate::app::App;
use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use semver::Version;
use sqlx::FromRow;

#[derive(Template)]
#[template(path = "crates/index.html")]
pub(crate) struct IndexTemplate {
    /// the name of the crate
    pub name: String,
    pub version: String,
    pub versions: Vec<CrateVersion>,
    pub downloads: u64,
    pub description: String,
    pub documentation: String,
    pub repository: String,
    pub tags: Vec<String>,
    pub owners: Vec<String>,
    pub readme: String,
}

#[derive(Debug, FromRow)]
pub struct CrateVersion {
    pub name: String,
    pub version: String,
    pub downloads: u64,
    pub created_at: u64,
}

#[derive(Debug, FromRow)]
pub struct Crate {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub documentation: String,
    pub repository: String,
}

pub async fn handler(
    State(state): State<App>,
    Path(name): Path<String>,
) -> Result<IndexTemplate, StatusCode> {
    let krate: Crate = sqlx::query_as("SELECT * FROM crates WHERE name = $1")
        .bind(&name)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let versions: Vec<CrateVersion> =
        sqlx::query_as("SELECT * FROM crate_versions WHERE name = $1")
            .bind(&name)
            .fetch_all(&state.db.pool)
            .await
            .unwrap(); // FIXME

    let owners: Vec<String> = sqlx::query_scalar(
        "SELECT name FROM users WHERE id IN (SELECT user_id FROM crate_owners WHERE crate_id = $1)",
    )
    .bind(&krate.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap();

    let downloads = versions.iter().map(|v| v.downloads).sum::<u64>();

    let latest = versions
        .iter()
        .filter_map(|v| Version::parse(&v.version).ok())
        .max_by(|a, b| a.cmp(b))
        .unwrap();

    let parser = pulldown_cmark::Parser::new(README);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Ok(IndexTemplate {
        name,
        version: latest.to_string(),
        versions,
        downloads,
        documentation: krate.documentation,
        repository: krate.repository,
        description: krate.description,
        tags: vec!["test".into(), "crate".into(), "not-real".into()],
        owners,
        readme: html_output,
    })
}

#[derive(Template)]
#[template(path = "crates/versions.html")]
pub struct CrateVersionTemplate {
    pub name: String,
    pub versions: Vec<CrateVersion>,
    pub latest_version: String,
    pub description: String,
}

pub async fn versions_handler(
    Path(name): Path<String>,
    State(state): State<App>,
) -> Result<CrateVersionTemplate, StatusCode> {
    let krate: Crate = sqlx::query_as("SELECT * FROM crates WHERE name = $1")
        .bind(&name)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let versions: Vec<CrateVersion> =
        sqlx::query_as("SELECT * FROM crate_versions WHERE name = $1")
            .bind(&name)
            .fetch_all(&state.db.pool)
            .await
            .unwrap();

    let latest = versions
        .iter()
        .filter_map(|v| Version::parse(&v.version).ok())
        .max_by(|a, b| a.cmp(b))
        .unwrap();

    Ok(CrateVersionTemplate {
        name,
        versions,
        latest_version: latest.to_string(),
        description: krate.description,
    })
}

const README: &str = r##"# Serde &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![serde msrv]][Rust 1.31] [![serde_derive msrv]][Rust 1.56]

[Build Status]: https://img.shields.io/github/actions/workflow/status/serde-rs/serde/ci.yml?branch=master
[actions]: https://github.com/serde-rs/serde/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/serde.svg
[crates.io]: https://crates.io/crates/serde
[serde msrv]: https://img.shields.io/crates/msrv/serde.svg?label=serde%20msrv&color=lightgray
[serde_derive msrv]: https://img.shields.io/crates/msrv/serde_derive.svg?label=serde_derive%20msrv&color=lightgray
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[Rust 1.56]: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html

**Serde is a framework for *ser*ializing and *de*serializing Rust data structures efficiently and generically.**

---

You may be looking for:

- [An overview of Serde](https://serde.rs/)
- [Data formats supported by Serde](https://serde.rs/#data-formats)
- [Setting up `#[derive(Serialize, Deserialize)]`](https://serde.rs/derive.html)
- [Examples](https://serde.rs/examples.html)
- [API documentation](https://docs.rs/serde)
- [Release notes](https://github.com/serde-rs/serde/releases)

## Serde in action

<details>
<summary>
Click to show Cargo.toml.
<a href="https://play.rust-lang.org/?edition=2018&gist=72755f28f99afc95e01d63174b28c1f5" target="_blank">Run this code in the playground.</a>
</summary>

```toml
[dependencies]

# The core APIs, including the Serialize and Deserialize traits. Always
# required when using Serde. The "derive" feature is only required when
# using #[derive(Serialize, Deserialize)] to make Serde work with structs
# and enums defined in your crate.
serde = { version = "1.0", features = ["derive"] }

# Each data format lives in its own crate; the sample code below uses JSON
# but you may be using a different one.
serde_json = "1.0"
```

</details>
<p></p>

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
```

## Getting help

Serde is one of the most widely used Rust libraries so any place that Rustaceans
congregate will be able to help you out. For chat, consider trying the
[#rust-questions] or [#rust-beginners] channels of the unofficial community
Discord (invite: <https://discord.gg/rust-lang-community>), the [#rust-usage] or
[#beginners] channels of the official Rust Project Discord (invite:
<https://discord.gg/rust-lang>), or the [#general][zulip] stream in Zulip. For
asynchronous, consider the [\[rust\] tag on StackOverflow][stackoverflow], the
[/r/rust] subreddit which has a pinned weekly easy questions post, or the Rust
[Discourse forum][discourse]. It's acceptable to file a support issue in this
repo but they tend not to get as many eyes as any of the above and may get
closed without a response after some time.

[#rust-questions]: https://discord.com/channels/273534239310479360/274215136414400513
[#rust-beginners]: https://discord.com/channels/273534239310479360/273541522815713281
[#rust-usage]: https://discord.com/channels/442252698964721669/443150878111694848
[#beginners]: https://discord.com/channels/442252698964721669/448238009733742612
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/122651-general
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust
[/r/rust]: https://www.reddit.com/r/rust
[discourse]: https://users.rust-lang.org

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>"##;
