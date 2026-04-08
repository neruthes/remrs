// Warning: Implementation happened with significant LLM assistance


use axum::{
    extract::{Query, State},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use serde::Deserialize;
use std::error::Error;
use std::{
    env, fs,
    net::{IpAddr, SocketAddr, UdpSocket},
    path::PathBuf,
    sync::Arc,
};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    file_path: PathBuf,
    token: String,
}

#[derive(Deserialize)]
struct AuthQuery {
    token: String,
}

#[derive(Deserialize)]
struct UpdateRequest {
    content: String,
}

fn get_ips() -> Vec<IpAddr> {
    let mut ips = Vec::new();
    let _ = UdpSocket::bind("0.0.0.0:0").and_then(|s| {
        s.connect("8.8.8.8:80")
            .map(|_| s.local_addr().map(|a| ips.push(a.ip())))
    });
    let _ = UdpSocket::bind("[::]:0").and_then(|s| {
        s.connect("[2001:4860:4860::8888]:53")
            .map(|_| s.local_addr().map(|a| ips.push(a.ip())))
    });
    if ips.is_empty() {
        ips.push(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    }
    ips
}

async fn show_editor(
    Query(params): Query<AuthQuery>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    if params.token != state.token {
        return Html("<h1>Unauthorized</h1>".to_string());
    }
    let content = fs::read_to_string(&state.file_path).unwrap_or_default();
    let rendered = include_str!("editor.html")
        .replace("[[filename]]", &state.file_path.to_string_lossy())
        .replace("[[content]]", &content)
        .replace("[[token]]", &state.token);
    Html(rendered)
}

async fn update_file(
    Query(params): Query<AuthQuery>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateRequest>,
) -> String {
    if params.token != state.token {
        return "Unauthorized".into();
    }
    match fs::write(&state.file_path, &payload.content) {
        Ok(_) => "Saved successfully".into(),
        Err(e) => format!("Error: {}", e),
    }
}

// Add this handler function
async fn get_file_content(
    Query(params): Query<AuthQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<String, (axum::http::StatusCode, String)> {
    if params.token != state.token {
        return Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }
    fs::read_to_string(&state.file_path)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rem <file>");
        return Ok(());
    }

    let file_path = fs::canonicalize(&args[1])?;
    let initial_content = fs::read_to_string(&file_path).unwrap_or_default();
    let token = Uuid::new_v4().to_string();

    let lock_path = file_path.with_extension("rem.lock");
    if let Ok(old_pid) = fs::read_to_string(&lock_path) {
        if let Ok(pid) = old_pid.trim().parse::<i32>() {
            unsafe {
                libc::kill(pid, libc::SIGTERM);
            }
        }
    }
    fs::write(&lock_path, std::process::id().to_string())?;

    let cert =
        rcgen::generate_simple_self_signed(vec!["localhost".to_string(), "127.0.0.1".to_string()])?;
    let tls_config = RustlsConfig::from_der(
        vec![cert.cert.der().to_vec()],
        cert.key_pair.serialize_der(),
    )
    .await?;

    let state = Arc::new(AppState {
        file_path: file_path.clone(),
        token: token.clone(),
    });

    let app = Router::new()
        .route("/", get(show_editor))
        .route("/", post(update_file))
        .route("/content", get(get_file_content)) // <--- Add this line
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 0));
    let listener = std::net::TcpListener::bind(addr)?;
    let port = listener.local_addr()?.port();

    println!("\n🔐 HTTPS Editor active for: {}", file_path.display());
    for ip in get_ips() {
        let host = if ip.is_ipv6() {
            format!("[{}]", ip)
        } else {
            ip.to_string()
        };
        println!("👉 https://{}:{}/?token={}", host, port, token);
    }

    // axum-server graceful shutdown uses a Handle
    let handle = axum_server::Handle::new();
    let shutdown_handle = handle.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        shutdown_handle.graceful_shutdown(Some(std::time::Duration::from_secs(1)));
    });

    axum_server::from_tcp_rustls(listener, tls_config)
        .handle(handle)
        .serve(app.into_make_service())
        .await?;
    
    

    let _ = fs::remove_file(&lock_path);

    println!("\n--- Session Closed. Generating Diff ---");

    // We use a Pipe to send the 'initial_content' to the diff command
    use std::process::{Command, Stdio};
    use std::io::Write;

    // We invoke: diff -u /dev/stdin <actual_file>
    // This compares the piped-in 'initial_content' with the current file on disk
    let mut diff_child = Command::new("diff")
        .arg("-u")
        .arg("--color=always")
        .arg("/dev/stdin")          // Descriptor for initial_content
        .arg(&file_path)            // The actual file on disk
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Write the initial content to the child's stdin
    if let Some(mut stdin) = diff_child.stdin.take() {
        stdin.write_all(initial_content.as_bytes())?;
    }

    diff_child.wait()?;

    Ok(())
}
