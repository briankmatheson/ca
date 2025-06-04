use std::fs::File;
use std::io::Write;
use rcgen::{generate_simple_self_signed, CertifiedKey};

fn new() -> std::io::Result<()> {
    // Generate a certificate that's valid for "localhost" and "hello.world.example"
    let subject_alt_names = vec!["CA".to_string()];
    
    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();
    
    let mut c = File::create("ca.pem")?;
    let mut k = File::create("ca.key")?;

    let result = c.write_all(&cert.pem().into_bytes())?;
    k.write_all(&key_pair.serialize_pem().into_bytes())?;
    Ok(result)
}


fn main() -> std::io::Result<()> {
    new()
}
