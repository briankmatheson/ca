use rcgen::{ExtendedKeyUsagePurpose, DnType, DnValue, DistinguishedName, CertificateParams, KeyPair};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn new() -> std::io::Result<()> {
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

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, DnValue::PrintableString(subject_alt_names[0].clone().try_into().unwrap()));

    let server_key = KeyPair::generate().unwrap();
    
    let mut cert = CertificateParams::new(subject_alt_names.clone()).unwrap();
    cert.distinguished_name = dn.clone();
    
    let file_name = subject_alt_names[0].clone();
    let mut s = File::create(format!("{}.pem", &file_name)).unwrap();
    let mut sk = File::create(format!("{}.key", &file_name)).unwrap();
    s.write_all(&cert.signed_by(&server_key, &ca, &ca_key).unwrap().pem().into_bytes())?;
    sk.write_all(&server_key.serialize_pem().into_bytes())?;
    Ok(())
}
fn client(subject_alt_names: &Vec<String>) -> std::io::Result<()> {
    let ca_key_pem = fs::read_to_string("ca.key")?;
    let ca_pem = fs::read_to_string("ca.pem")?;

    let ca_key = KeyPair::from_pem(&ca_key_pem).unwrap();
    let ca = CertificateParams::from_ca_cert_pem(&ca_pem).unwrap().
	self_signed(&ca_key).unwrap();

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, DnValue::PrintableString(subject_alt_names[0].clone().try_into().unwrap()));

    let server_key = KeyPair::generate().unwrap();
    
    let mut cert = CertificateParams::new(subject_alt_names.clone()).unwrap();
    cert.distinguished_name = dn.clone();
    cert.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth];
    
    
    
    let file_name = subject_alt_names[0].clone();
    let mut s = File::create(format!("{}.pem", &file_name)).unwrap();
    let mut sk = File::create(format!("{}.key", &file_name)).unwrap();
    s.write_all(&cert.signed_by(&server_key, &ca, &ca_key).unwrap().pem().into_bytes())?;
    sk.write_all(&server_key.serialize_pem().into_bytes())?;
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
	"client" => client(&args[2..].to_vec()),
	"sign" => sign(args),
	&_ => usage()
    }
}

// Test
//
// openssl s_server -Verify 2 -verify_hostname bmath.bmath.nyc -cert foo.bmath.nyc.pem -key foo.bmath.nyc.key -CAfile ca.pem -verifyCAfile ca.pem
// curl foo.bmath.nyc --key bmath.bmath.nyc.key -E bmath.bmath.nyc.pem -v --cacert ./ca.pem  https://foo.bmath.nyc:4433
//

