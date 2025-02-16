use serial2_tokio::SerialPort;
// use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let serial = SerialPort::open("COM4", 9600)?;
    let mut buffer = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        match serial.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                data.extend_from_slice(&buffer[..n]);

                // Look for complete packets starting with '~' (0x7E)
                while let Some(start) = data.iter().position(|&b| b == 0x7E) {
                    if data.len() > start + 2 {
                        let length = ((data[start + 1] as usize) << 8) | (data[start + 2] as usize);
                        if data.len() >= start + 3 + length {
                            let packet = &data[start..start + 3 + length];
                            println!("{:?}", String::from_utf8_lossy(&packet[15..]));
                            data.drain(..start + 3 + length);
                            continue;
                        }
                    }
                    break;
                }
            }
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                break;
            }
        }
    }

    Ok(())
}
