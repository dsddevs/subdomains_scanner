## 📄 Subdomains Scanner
- Simple. Secure. Insightful.
- Fetch and store SSL certificate transparency logs directly from crt.sh — all with a single command.

## 🚀 Overview
- SSL Certificate Fetcher is a lightweight command-line tool designed to retrieve SSL certificate records for any domain using crt.sh's public API. Whether you're a security analyst, penetration tester, or DevOps engineer, this tool gives you instant visibility into issued certificates, subdomains, and expiration dates.

## 🔧 Features
- ✅ Fetch public SSL certificate data for any domain
- 📥 Save results to a local .json file
- 🧾 View readable certificate metadata (issuer, CN, validity period)
- ⚡ Fast, simple, and reliable — built in Rust for performance and safety

## 📦 Installation
- Requirements
- Rust
- Internet connection
- Build from source

## 🖥️ Usage
```dg
git clone https://github.com/dsddevs/subdomains_scanner.git
cd subdomains_scanner
cargo build --release
cargo run <domain>
```
## ✅ This will:
- Fetch all matching certificates for example: dsd.com and its subdomains
- Save them in a file called subdomains.json
- Print formatted certificate metadata to the console

## 📂 Output
subdomains.json:
[
{
"common_name": "www.dsd.com",
"name_value": "www.dsd.com",
"issuer_name": "C=US, O=Let's Encrypt, CN=R10",
"not_before": "2025-03-28T04:33:30",
"not_after": "2025-06-26T04:33:29"
},
...
]

## 🛡 Use Cases
- ✅ Security monitoring: Detect unauthorized or unexpected certificates
- ✅ Subdomain enumeration: Discover hidden or legacy infrastructure
- ✅ Compliance auditing: Track certificate validity and expiration
- ✅ Pentesting: Gain insight into target infrastructure

## 📖 License
This project is licensed under the Apache-2.0

## 📫 Contact
For business inquiries or support, please email: dsddevs@gmail.com

