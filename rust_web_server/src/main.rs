use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on port 8080..");
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 2048];
            let bytes_read = match stream.read(&mut buf).await {
                Ok(0) => return,
                Ok(n) => n,
                Err(_) => return,
            };
            let http_request = String::from_utf8_lossy(&buf[0..bytes_read]);
            let request_line = http_request.lines().next().unwrap_or("");
            let path = request_line.split_whitespace().nth(1).unwrap_or("/");
            let (status_line, filename) = match path {
                "/" => ("HTTP/1.1 200 OK", "index.html"),
                "/index.html" => ("HTTP/1.1 200 OK", "index.html"),
                "/image.jpg" => ("HTTP/1.1 200 OK", "image.jpg"),
                _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
            };
            let contents = match tokio::fs::read(filename).await {
                Ok(contents) => contents,
                Err(_) => {
                    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\nFiler Not Found";
                    stream.write_all(response.as_bytes()).await.unwrap_or(());
                    return;
                }
            };

            let content_type = if filename.ends_with(".jpg") { "image/jpeg" } else { "text/html" };
            let response = format!(
                "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                status_line,
                content_type,
                contents.len()
                );

            stream.write_all(response.as_bytes()).await.unwrap_or(());
            stream.write_all(&contents).await.unwrap_or(());
            //echo server
           // loop {
           //     match stream.read(&mut buf).await {
           //         Ok(0) =>  {
           //             println!("client disconnected");
           //             return;
           //         }
           //         Ok(n)=> {
           //             if stream.write_all(&buf[0..n]).await.is_err() {
           //                 return;
           //             }
           //         }
           //         Err(e) => {
           //             println!("failed to read from socket; err = {:?}", e);
           //             return;
           //         }
           //     }
                //println!("Handling connection in a background task!");
                //let n = stream.read(buf).await;
                //stream.write_all(&buf[0..n]);
           // }
        });
    }
}
