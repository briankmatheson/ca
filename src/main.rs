use rcgen::{generate_simple_self_signed, CertifiedKey};
use std::env;
use std::fs::File;
use std::io::Write;

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

fn server(args: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
fn client(args: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
fn sign(args: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
fn usage() -> std::io::Result<()> {
    println!("Usage: ca new|server NAME|client NAME|sign FILENAME");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    println!("{command}");
    
    match command.as_str() {
	"new" => new(),
	"server" => server(args),
	"client" => client(args),
	"sign" => sign(args),
	&_ => usage()
    }
}
