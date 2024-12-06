mod tamo;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::process;
use std::time::Duration;
use tamo::load_tamo;

fn process_request_string(request_contents: &[u8]) -> Option<fn(&mut tamo::Tamo)> {
    match request_contents {
        [b'F'] => Some(tamo::Tamo::feed),
        [b'P'] => Some(tamo::Tamo::play),
        [b'Q'] => Some(tamo::Tamo::quench),
        _ => None,
    }
}

fn handle_client(mut stream: TcpStream, tamo: Arc<Mutex<tamo::Tamo>>) {
    
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    let mut request = String::new();

    loop {
        request.clear();
        match buf_reader.read_line(&mut request) {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                println!("Received: {}", request.trim());

                let response_char = request.trim().as_bytes();
                let mut tamo_status = String::new();
                if let Some(tamo_fn) = process_request_string(response_char) {

                    let mut tamo = tamo.lock().unwrap();
                    tamo_fn(&mut tamo); // calls function according to request (fp!)
                    tamo.time_pass();
                    tamo.save_tamo().unwrap();
                    tamo_status = format!("{}", tamo);
                    println!("Updated Tamo: {}", tamo);  // Print update immediately
                }

                let response = format!("{:?}\n", tamo_status); // RESPONSE
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to write to client: {}", e);
                    break;
                }
                println!("Sent response to client");
                if let Err(e) = stream.flush() {
                    eprintln!("Failed to flush stream: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    println!("Server listening on 127.0.0.1:3333");

    let my_tamo = Arc::new(Mutex::new(load_tamo("tamo.txt").unwrap()));
    // let my_tamo_clone = Arc::clone(&my_tamo);
    
    let my_tamo_ctrlc = Arc::clone(&my_tamo);
    ctrlc::set_handler(move || {
        let tamo = my_tamo_ctrlc.lock().unwrap();
        tamo.save_tamo().unwrap();
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // thread for periodic updates of tamogachi
    let my_tamo_update = Arc::clone(&my_tamo);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let mut tamo = my_tamo_update.lock().unwrap();
            tamo.time_pass();
            if let false = tamo.test_dead() {
                break;
            }
            tamo.save_tamo().unwrap();
            println!("time passed AUTOMATICALLY for {}", tamo);

        }
    });

    // request handler
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut my_tamo_client = Arc::clone(&my_tamo);
                thread::spawn(move || {
                    handle_client(stream, my_tamo_client);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }

    Ok(())
}
