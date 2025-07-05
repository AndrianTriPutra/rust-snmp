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

    let oid: Vec<u32> = match oid_str
        .trim_start_matches('.')
        .split('.')
        .map(|s| s.parse())
        .collect::<Result<_, _>>() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Invalid OID: {:?}", e);
                return;
            }
        };

    let community = b"public";
    let timeout = Some(Duration::from_secs(2));
    let mut sess = match SyncSession::new(target, community, timeout, 0) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("SNMP session error: {:?}", e);
            return;
        }
    };

    let pdu = match sess.get(&oid) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("SNMP get error: {:?}", e);
            return;
        }
    };

    if let Some((oid_vec, value)) = pdu.varbinds.into_iter().next() {
        println!("OID: {} => {:?}", format_oid(&oid_vec), value);

        match value {
            Value::Integer(i) => println!("Integer: {}", i),
            Value::OctetString(s) => println!("String: {}", String::from_utf8_lossy(&s)),
            Value::Counter32(c) => println!("Counter32: {}", c),
            Value::Unsigned32(u) => println!("Unsigned32: {}", u),
            Value::Timeticks(t) => println!("Timeticks: {}", t),
            _ => println!("Other SNMP type: {:?}", value),
        }
    } else {
        eprintln!("No varbinds returned.");
    }
}

fn format_oid(oid: &ObjectIdentifier) -> String {
    oid.to_string()
}
