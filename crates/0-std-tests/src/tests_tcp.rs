#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        time::Duration,
    };
    use std::{
        net::{Shutdown, TcpListener, TcpStream},
        sync::atomic::{AtomicBool, Ordering},
    };
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    fn handle_client(mut stream: TcpStream, counter: Arc<Mutex<i32>>, should_end: Arc<AtomicBool>) {
        let mut buffer = [0; 3];

        while match stream.read(&mut buffer) {
            Ok(size) => match &buffer[..size] {
                b"ADD" => {
                    let mut counter = counter.lock().unwrap();
                    *counter += 1;
                    true
                }
                b"END" => {
                    stream.shutdown(Shutdown::Both).unwrap();
                    should_end.store(true, Ordering::SeqCst);
                    false
                }
                _ => true,
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(1000));
                true
            }
            Err(e) => {
                println!(
                    "An error occurred, terminating connection with {}. {:?}",
                    stream.peer_addr().unwrap(),
                    e
                );
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }

    #[test]
    pub fn test_tcp() {
        let counter = Arc::new(Mutex::new(0));
        let should_end = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel();

        let server_handle = {
            let counter = Arc::clone(&counter);
            let tx = tx.clone();

            thread::spawn(move || {
                let listener =
                    TcpListener::bind("localhost:12345").expect("Unable to bind to port");
                listener
                    .set_nonblocking(true)
                    .expect("unable to set non-blocking mode");

                tx.send(()).unwrap();

                loop {
                    if should_end.load(Ordering::SeqCst) {
                        break;
                    }

                    match listener.accept() {
                        Ok((stream, _)) => {
                            let counter = Arc::clone(&counter);
                            let should_end = Arc::clone(&should_end);
                            thread::spawn(move || {
                                handle_client(stream, counter, should_end);
                            });
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            thread::sleep(Duration::from_millis(100));
                            continue;
                        }
                        Err(e) => {
                            println!("Failed: {}", e);
                        }
                    }
                }
            })
        };

        // Wait until server starts
        rx.recv().unwrap();

        {
            let mut stream =
                TcpStream::connect("localhost:12345").expect("unable to connect to server");
            for _ in 0..4 {
                let _ = stream.write(b"ADD");
                let _ = stream.flush();
                thread::sleep(Duration::from_millis(100));
            }

            let _ = stream.write(b"END");
            let _ = stream.flush();
        }

        server_handle.join().expect("unable to join server thread");

        let counter = counter.lock().unwrap();

        assert!(*counter == 4, "counter should be 4");
    }
}
