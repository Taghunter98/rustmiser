use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::TlsConnector;
use tokio_native_tls::{TlsConnector as TokioTlsConnector, TlsStream}; // Needed to bypass ssl cert
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{WebSocketStream, client_async};
use url::Url;

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
/// - Returns [`except`](core::result::Result<Error>) - if the native tls connector build fails.
/// - Returns [`Result<TlsConnector, Error>`](core::result::Result<Error>) - if the tls stream is unable to establish a connection.
/// 
async fn neo_connect() -> WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>> {
    let url: Url = Url::parse("wss://192.168.4.174:4243").expect("unable to parse url");
    let host: &str = url.host_str().unwrap();
    let port: u16 = url.port_or_known_default().unwrap();

    let tcp: TcpStream = TcpStream::connect((host, port))
        .await
        .expect("unable to establish tcp stream");

    let native_connector: TlsConnector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .expect("unable to build TlsConnecter instance");

    let connector: TokioTlsConnector = TokioTlsConnector::from(native_connector);

    let tls_stream: tokio_native_tls::TlsStream<TcpStream> = connector
        .connect(host, tcp)
        .await
        .expect("unable to establish tcp stream");

    let request: tokio_tungstenite::tungstenite::http::Request<()> =
        url.into_client_request().expect("unable to build request");

    let (socket, _) = client_async(request, tls_stream)
        .await
        .expect("unable to handshake with neohub");
    println!("Connected to neohub");

    socket
}

pub async fn run_recipe() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket: WebSocketStream<TlsStream<TcpStream>> = neo_connect().await;

    // Create JSON object with command
    let token: &'static str = "0e0df290-8821-4de8-b14a-45cd3b83c33f";
    let inner: String = format!(
        "{{\"token\":\"{}\",\"COMMANDS\":[{{\"COMMAND\":\"{{'GET_RECIPES':0}}\",\"COMMANDID\":1}}]}}",
        token
    );
    let outer = serde_json::json!({
        "message_type": "hm_get_command_queue",
        "message": inner
    });

    socket.send(Message::Text(outer.to_string())).await?;

    // Await a response
    if let Some(msg) = socket.next().await {
        let response = msg?;
        println!("Response: {}", response);
    }

    Ok(())
}
