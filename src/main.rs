use rcgen::{DnType, DnValue, DistinguishedName, CertificateParams, KeyPair};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn new() -> std::io::Result<()> {
    // Generate a certificate that's valid for "localhost" and "hello.world.example"
    
    let subject_alt_names = vec!["CA".to_string()];
    let key = KeyPair::generate().unwrap();
    let mut cert = CertificateParams::new(subject_alt_names.clone()).unwrap();
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, DnValue::PrintableString("CA".try_into().unwrap()));
    cert.distinguished_name = dn;
    cert.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    cert.key_usages = vec![
        rcgen::KeyUsagePurpose::KeyCertSign,
        rcgen::KeyUsagePurpose::DigitalSignature,
    ]; 
    let mut k = File::create("ca.key")?;
    let mut c = File::create("ca.pem")?;

    k.write_all(&key.serialize_pem().into_bytes())?;
    c.write_all(&cert.self_signed(&key).unwrap().pem().into_bytes())?;
    Ok(())
}

fn server(subject_alt_names: &Vec<String>) -> std::io::Result<()> {
    let ca_key_pem = fs::read_to_string("ca.key")?;
    let ca_pem = fs::read_to_string("ca.pem")?;

    let ca_key = KeyPair::from_pem(&ca_key_pem).unwrap();
    let ca = CertificateParams::from_ca_cert_pem(&ca_pem).unwrap().
	self_signed(&ca_key).unwrap();

    let server_key = KeyPair::generate().unwrap();
    let cert = CertificateParams::new(subject_alt_names.clone()).unwrap().
	signed_by(&server_key, &ca, &ca_key).unwrap();
    let file_name = subject_alt_names[0].clone();
    let mut s = File::create(format!("{}.pem", &file_name)).unwrap();
    let mut sk = File::create(format!("{}.key", &file_name)).unwrap();
    s.write_all(&cert.pem().into_bytes())?;
    sk.write_all(&server_key.serialize_pem().into_bytes())?;
    Ok(())
}

fn client(_args: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
fn sign(_args: Vec<String>) -> std::io::Result<()> {
    Ok(())
}
fn usage() -> std::io::Result<()> {
    println!("Usage: ca new | server NAME");
    std::process::exit(0);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
	let _ = usage();
    }
    let command = &args[1];
    
    match command.as_str() {
	"new" => new(),
	"server" => server(&args[2..].to_vec()),
	"client" => client(args),
	"sign" => sign(args),
	&_ => usage()
    }
}
