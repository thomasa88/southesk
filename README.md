<!-- To make diffing easier, this file leans towards using semantic linebreaks ( https://sembr.org/ ) -->

# southesk

A library for creating clients for the [Montrose MCP](https://www.montrose.io/mcp) API. It provides the API calls and handles user authentication.

Although the library uses an MCP, it does not contain any AI functionality.

This project is not affiliated, endorsed by and does not have any connections to Montrose.

## API Stability

The available APIs of the Montrose MCP service has been fetched by asking the service for its available tools.
Since the API has no versioning and AI agents should be able to adopt to changing APIs,
one should not expect the API to be stable.

### The Plan

To handle moving underlying APIs, the plan is to implement 3 API levels:

* High level API: A stable API, with core functionality for transactions and trading. It is likely to be more limited than the low level API.
* Low level API: Maps closely to the underlying MCP API.
* Raw API: Direct calls to the MCP API. Can be used before southesk has implemented support for the API call.

## Getting Started

<!-- This is the same exmple as in src/lib.rs. Therefore, it is built when running doc tests. -->

To use `southesk`, add it as a dependency along with an async runtime:

```bash
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

    Ok(())
}
```

## Examples

A set of examples using `southesk` is available in the [examples](examples) directory.
They can be run as follows:

```bash
cargo run --example=show-data
```

## Features

Default features:

- `keyring`: Enables support to store OAuth credentials in the OS keyring.
  The keyring will be used by default if the feature is enabled.


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

This software is licensed under the [MIT license](LICENSE).

## Known Issues

OAuth refresh works and can be performed successfully multiple times. However,
the MCP resource seems to only accept the initial access token. The result is
that a new authorization flow is triggered after 24 hours and the user needs to
authenticate again.

## The river South Esk

The river South Esk flows through the town of Montrose, Scotland, out into the North Sea. South Esk gives its name to the title Earl of Southesk, held by the Carnegie family.
