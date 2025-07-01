# SNMP Client in Rust

A minimal SNMP client implementation using Rust, useful for querying SNMP-enabled devices.

---

## 🔪 Getting Started

### 1. Clone the Repository

```bash
$ git clone https://github.com/AndrianTriPutra/rust-snmp.git snmp
$ cd snmp
```

### 2. Install SNMP

```bash
$ sudo apt install snmp snmpd -y

check version:
$ snmpwalk --version

test:
$ snmpwalk -v1 -c public 127.0.0.1
$ snmpget -v1 -c public 127.0.0.1 .1.3.6.1.2.1.1.5.0
```

### 3. Run with Cargo

You can query an SNMP OID using:

```bash
$ cargo run -- <target-ip>:<port> <oid>
```

#### Example:

```bash
check hostname:
$ cargo run -- 127.0.0.1:161 .1.3.6.1.2.1.1.5.0
```

> This example queries the system uptime from an SNMP agent.

---

## 🔍 Features

* Simple SNMP GET support
* Synchronous query using community string (default: `public`)
* Based on SNMP v1 (compatible with some SNMPv2c targets)
* Command line interface
* Timeout customization

---

## ⚙️ Configuration

Currently hardcoded/default:

* **Community**: `public`
* **Timeout**: 2 seconds
* **SNMP Version**: Implicitly v1 (depends on target device compatibility)

If you need SNMP v2c or v3 support, this example may need to be extended manually.

---

## 👷️ Project Structure

```
src/
  main.rs      # main logic for command-line SNMP query
Cargo.toml     # dependencies and metadata
```

---

## 🚧 Limitations

* Only supports SNMP `GET` operation
* No support for SNMP `SET`, `GETNEXT`, `WALK`, or traps
* SNMP version is not configurable from CLI
* No support for SNMPv3

---

## 🧠 Notes

This project is for educational purposes and minimal integrations. If you require full SNMP functionality (v2c or v3), consider using a Go-based tool like [`gosnmp`](https://github.com/gosnmp/gosnmp) or using external tools (`snmpget`, `snmpwalk`) from Rust.

---


## ✍️ Author

Created by [Andrian Tri Putra](https://medium.com/@andriantriputra) for exploration of SNMP from a systems programming perspective in Rust.
