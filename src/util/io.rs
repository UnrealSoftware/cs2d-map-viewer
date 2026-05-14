use std::io;
use std::io::Read;

pub fn read_string(reader: &mut impl Read) -> io::Result<String> {
    let mut string = String::new();
    let mut buf = [0u8; 1];

    loop {
        match reader.read_exact(&mut buf) {
            Ok(_) => {
                let byte = buf[0];
                if byte == b'\r' {
                    // Check for \n
                    match reader.read_exact(&mut buf) {
                        Ok(_) => {
                            if buf[0] == b'\n' {
                                break;
                            } else {
                                string.push('\r');
                                string.push(buf[0] as char);
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            string.push('\r');
                            break;
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    string.push(byte as char);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(string)
}