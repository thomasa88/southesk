<!-- To make diffing easier, this file leans towards using semantic linebreaks ( https://sembr.org/ ) -->

![Southesk banner](https://github.com/thomasa88/southesk/raw/main/docs/images/southesk-banner.svg)

# southesk

A library for creating clients for the [Montrose MCP](https://www.montrose.io/mcp) API.
It provides the API functions and handles user authentication.

Although the library uses an MCP, it does not contain any AI functionality.

This project is not affiliated, endorsed by and does not have any connections to Montrose.

## API Stability

The available APIs of the Montrose MCP service has been fetched by asking the service for its available tools.
Since the API has no versioning and AI agents should be able to adopt to changing APIs,
one should not expect the API to be stable.

## API Levels

To handle moving underlying APIs, the idea is to provide three API levels:

* High-level API: Follows Rust type conventions and provides documentation for all types and members.
* Low-level API: Maps closely to the underlying MCP API.
* Raw API: Direct calls to the MCP API using JSON data.

The lower levels can be used before southesk has implemented support for the calls in the high-level API.

## High-level API Functions

The following high-level API functions are provided:

<!-- BUILD: HIGH-LEVEL START -->
* [get_holdings](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.get_holdings)
* [get_user_accounts](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.get_user_accounts)
* [create_trade_ticket](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.create_trade_ticket)
* [search_instruments](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.search_instruments)
* [get_watchlists](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.get_watchlists)
* [get_watchlist](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.get_watchlist)
* [create_watchlist](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.create_watchlist)
* [add_to_watchlist](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.add_to_watchlist)
* [remove_from_watchlist](https://docs.rs/southesk/latest/southesk/struct.Client.html#method.remove_from_watchlist)
<!-- BUILD: HIGH-LEVEL END -->

## Getting Started

<!--
This is the same example as in crates/southesk/src/lib.rs.
Therefore, it is built when running doc tests.
-->

To use `southesk`, add it as a dependency along with an async runtime:

```bash
> cargo init my_client
> cargo add southesk
> cargo add tokio -F rt-multi-thread
```

Then you can create a client and make API calls:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let montrose = southesk::ClientBuilder::new("My Montrose Client")
        .build()
        .await?;
    let montrose = montrose.connect().await?;

    let accounts = montrose.get_user_accounts().await?;
    dbg!(&accounts);

    montrose.disconnect().await;

    Ok(())
}
```

## Examples

A set of examples using `southesk` is available in the [examples](crates/southesk/examples) directory.
They can be run as follows:

```bash
cargo run -p southesk --example=show_data
```

## Feature flags

- `keyring` *(enabled by default)*: Enables support to store OAuth credentials in the OS keyring.
  The keyring will be used by default if the feature is enabled.
- `high-api` *(enabled by default)*: Enables the high-level API.
- `low-api`: Enables the low-level API.
- `raw-api`: Enables the raw API.

## Security Warning

The MCP API does not permit any actions taken on your behalf,
but a malicious actor can extract information about your Montrose account and investments.
**You should never trust third party code, such as the code in this repository,
without examing the source code carefully.**

To re-iterate from the license:

> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## License

This software is licensed under the [MIT license](https://github.com/thomasa88/southesk/blob/main/LICENSE).

## What's in a name?

The river South Esk flows through the town of Montrose, Scotland, out into the North Sea. South Esk gives its name to the title Earl of Southesk, held by the Carnegie family.
