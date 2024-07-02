use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

enum TamoInteractions {
    Feed,   // 'F'
    Quench, // 'Q'
    Play,   // 'P'
}

fn send_tamo_request(interaction: TamoInteractions, stream: &mut TcpStream) -> io::Result<()> {
    let request_contents = match interaction {
        TamoInteractions::Feed => 'F',
        TamoInteractions::Quench => 'Q',
        TamoInteractions::Play => 'P',
    };
    let mut buffer = [0u8; 4];
    let encoded_bytes = request_contents.encode_utf8(&mut buffer).as_bytes();
    stream.write_all(encoded_bytes)?;
    stream.write_all(b"\n")?;  // Ensure newline at the end
    stream.flush()?;
    println!("Sent request: {}", request_contents);
    Ok(())
}

fn read_response(stream: &mut TcpStream) -> io::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    Ok(response)
}

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;

    // Example of sending a single request
    send_tamo_request(TamoInteractions::Quench, &mut stream)?;
    send_tamo_request(TamoInteractions::Feed, &mut stream)?;
    send_tamo_request(TamoInteractions::Play, &mut stream)?;
    let response = read_response(&mut stream)?;
    println!("Received Response: {}", response.trim());

    // Client automatically closes after running
    Ok(())
}
