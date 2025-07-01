use snmp::{SyncSession, Value, ObjectIdentifier};
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <IP:port> <OID>", args[0]);
        return;
    }

    let target = &args[1];
    let oid_str = &args[2];


}

fn format_oid(oid: &ObjectIdentifier) -> String {
    oid.to_string()
}
