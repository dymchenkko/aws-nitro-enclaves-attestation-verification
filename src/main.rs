use aws_nitro_enclaves_attestation::{
    NitroAdError,
    NitroAdDoc};
    use std::env;
    use std::fs::File;
    use std::io::Read;
fn main() {

    let args: Vec<String> = env::args().collect();
    let cert_file_path = &args[1];
    let der_file_path = &args[2];
    let timestamp = &args[3].parse::<u64>().unwrap();

    if test_payload_to_valid_json(cert_file_path, der_file_path, timestamp).is_ok(){
        println!("0");
    }
    else {
        println!("1");
    }
}

fn test_payload_to_valid_json(cert_file_path: &String, der_file_path: &String, timestamp: &u64) -> Result<(), NitroAdError> {

    let mut cert_file = File::open(cert_file_path).unwrap();
    let mut ad_blob = Vec::new();
    cert_file.read_to_end(&mut ad_blob).unwrap();

    let mut der_file = File::open(der_file_path).unwrap();
    let mut root_cert = Vec::new();
    der_file.read_to_end(&mut root_cert).unwrap();

    let nitro_addoc = NitroAdDoc::from_bytes(&ad_blob, &root_cert, *timestamp)?;
    let js = nitro_addoc.to_json().unwrap();
    
    let _: serde::de::IgnoredAny = serde_json::from_str(&js)?;

    Ok(())
}