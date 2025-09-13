use futures_util::SinkExt;
use std::env::var;
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::TlsConnector;
use tokio_native_tls::{TlsConnector as TokioTlsConnector, TlsStream}; // Needed to bypass ssl cert
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{WebSocketStream, client_async};
use url::Url;

/// API to run a websocket command on the heatmiser neohub.
///
/// Function takes a command and value and an API key from a .env file, it needs to be created from the mobile app.
///
/// Then the command message is created by formatting a string following the neohub conventions and we can then send
/// the request and await the result.
///
/// The socket future is then returned either as an `Err` or `Ok`.
///
/// ### Request structure
///
/// A websocket request needs to follow this structure.
/// ```plaintext
/// "message_type":"hm_get_command_queue",
/// "message":"{\"token\":\"{{token}}\",\"COMMANDS\":[
///     {\"COMMAND\":\"{{command}}\",\"COMMANDID\":1}
/// ]}"}
/// ```
///
/// ## Errors
///
/// - Returns [`Result<String, VarError>`](core::result::Result<Error>) - if the environment variable isn't present.
/// - Returns [`SinkExt<Item>`](core::result::Result<Error>) - if the future is unable to process all items into the sink.
/// - Returns [`StreamExt`](core::result::Result<Error>) - if the future is unable to resolve the next item in stream.
///
/// ## Examples
///
/// Get live data
/// ```rust
/// websoc::run("GET_LIVE_DATA", "0")
///     .await
///     .expect("websocket connection failed");
/// }
/// ```
///
/// Reboot system
/// ```rust
/// websoc::run("RESET", "0")
///     .await
///     .expect("websocket connection failed");
/// }
/// ```
///
/// Get recipes
/// ```rust
/// websoc::run("GET_RECIPES", "1")
///     .await
///     .expect("websocket connection failed");
/// }
/// ```
///
/// Run a recipe
/// ```rust
/// websoc::run("RUN_RECIPE", "['recipeName']")
///     .await
///     .expect("websocket connection failed");
/// }
/// ```
///
/// Get system files
/// ```rust
/// websoc::run("GET_SYSTEM", "0")
///     .await
///     .expect("websocket connection failed");
/// }
/// ```
///
pub async fn run(command: &str, value: &str) -> WebSocketStream<TlsStream<TcpStream>> {
    let token: String = var("API_KEY").expect("can't retrieve .env value");

    let mut socket: WebSocketStream<TlsStream<TcpStream>> = neo_connect().await;

    let message: String = format!(
        "{{\"token\":\"{}\",\"COMMANDS\":[{{\"COMMAND\":\"{{'{}':{}}}\",\"COMMANDID\":1}}]}}",
        token, command, value
    );
    let headers: serde_json::Value = serde_json::json!({
        "message_type": "hm_get_command_queue",
        "message": message
    });

    socket
        .send(Message::Text(headers.to_string()))
        .await
        .expect("unable to complete future");

    socket
}

/// Creates a websocket connection to heatmiser neohub.
///
/// Function works by first grabbing the endpoint and separating into host and port, then creating the new raw TCP connection
/// bypassing the usual required SSL cert for heatmiser neohub connections.
///
/// We use `TlsConnector` to do this by running `accept_invalid_certs` thus ensuring we can create a connection, as these are localy
/// hosted this is not a huge security issue, but be advised the connection is not secure.
///
/// Then to send a async request, we need to wrap the current TlsConnector instance as a declared `TokioTlsConnector` type then we are able
/// to create a tls stream, build and then attempt a handshake. This method of wrapping the types is essential due to both
/// `tokio_naitive_tls::Tlsconnector` and `tokio_native_tls::native_tls::TlsConnector` sharing the same function name which causes conflicts.
///
/// Then we are able to return a `WebSocketStream` object that is connected to the neohub.
///
/// ## Errors
///
/// - Returns [`Result<Url, crate::ParseError>`](core::result::Result<Error>) - if the url parsing fails.
/// - Returns [`Result<TlsConnector, Error>`](core::result::Result<Error>) - if the tcp stream is unable to establish a connection.
/// - Returns [`TlsConnectorBuilder`](core::result::Result<Error>) - if the tls connector build fails.
/// - Returns [`Result<TlsStream<S>, Error>`](core::result::Result<Error>) - if the tls stream is unable to establish a connection.
///
/// # Example
///
/// ```rust
/// let mut socket: WebSocketStream<TlsStream<TcpStream>> = neo_connect().await;
/// ```
///
async fn neo_connect() -> WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>> {
    let neohub_url: String = var("NEOHUB_URL").expect("can't retrieve env value");

    let url: Url = Url::parse(&neohub_url).expect("unable to parse url");
    let host: &str = url.host_str().unwrap();
    let port: u16 = url.port_or_known_default().unwrap();

    let tcp: TcpStream = TcpStream::connect((host, port))
        .await
        .expect("unable to open a TCP connection to the neohub.");

    let native_connector: TlsConnector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .expect("unable to build TlsConnecter instance");

    let connector: TokioTlsConnector = TokioTlsConnector::from(native_connector);

    let tls_stream: tokio_native_tls::TlsStream<TcpStream> = connector
        .connect(host, tcp)
        .await
        .expect("unable to open a TCP connection to the neohub.");

    let request: tokio_tungstenite::tungstenite::http::Request<()> =
        url.into_client_request().expect("unable to build request");

    let (socket, _) = client_async(request, tls_stream)
        .await
        .expect("unable to handshake with neohub");

    println!("Connected to neohub");

    socket
}
