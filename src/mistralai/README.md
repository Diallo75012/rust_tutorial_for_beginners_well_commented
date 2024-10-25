### Installation

- Mistral Client
```bash
cargo add mistralai-client
```

- Package Config installation for Rust to be able to use Openssl or install it if not found
```bash
sudo apt install pkg-config
```
OR set environment variable: `OPENSSL_DIR`
```bash
# see if openssl is installed somewhere
whereis openssl
# outputs: 
# openssl: /usr/bin/openssl /usr/include/openssl /usr/share/man/man1/openssl.1ssl.gz

export OPENSSL_DIR="/usr/bin/openssl"
```

- Export API key:
```bash
export MISTRAL_API_KEY=<PUT API KEY HERE GET IT AT CONSOLE.MISTRAL.AI>
``` 
