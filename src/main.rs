use rcgen::{generate_simple_self_signed, CertifiedKey, CertificateParams, KeyPair};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write; 
use std::process;

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

fn server(subject_alt_names: Vec<String>) -> std::io::Result<()> {
    let c = fs::read_to_string("ca.pem")?;
    let k = fs::read_to_string("ca.key")?;

    let ca_key = KeyPair::from_pem(&k).unwrap();
    let ca = CertificateParams::from_ca_cert_pem(&c).unwrap().self_signed(&ca_key).unwrap();

    let server_key = KeyPair::generate().unwrap();
    let cert = CertificateParams::new(subject_alt_names.clone()).unwrap().
	signed_by(&server_key, &ca, &ca_key);
    let file_name = subject_alt_names[2].clone();
    let s = File::create(format!("{}.pem", &file_name));
    let sk = File::create(format!("{}.key", &file_name));
    s.unwrap().write_all(&cert.unwrap().pem().into_bytes())?;
    sk.unwrap().write_all(&server_key.serialize_pem().into_bytes())?;
    Ok(())
}

fn client(_args: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
fn sign(_args: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
fn usage() -> std::io::Result<()> {
    println!("Usage: ca new|server NAME|client NAME|sign FILENAME");
    std::process::exit(0);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    
    match command.as_str() {
	"new" => new(),
	"server" => server(args),
	"client" => client(args),
	"sign" => sign(args),
	&_ => usage()
    }
}
