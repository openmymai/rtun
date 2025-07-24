# rtun ⚡

**rtun** (Rust Tunnel) is a fast, simple, and reliable command-line tool that exposes your local web server to the internet. It's like ngrok, but written in Rust for performance and safety.

`rtun` acts as a client for the open-source [localtunnel.me](https://localtunnel.me) service, creating a secure tunnel from a public URL to your `localhost`.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![GitHub Actions](https://img.shields.io/github/actions/workflow/status/your-username/rtun/rust.yml?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/rtun?style=for-the-badge)

---

## Features

- **Expose Local Servers**: Make your local development server accessible from anywhere.
- **Custom Subdomains**: Request a specific subdomain for your tunnel.
- **Real-time Logging**: See incoming requests logged directly in your terminal.
- **Cross-Platform**: Works on Linux, macOS, and Windows.
- **Lightweight & Fast**: Built with Rust and Tokio for minimal resource usage.

---

## Why `rtun`?

- **Webhook Testing**: Easily test webhooks from services like Stripe, GitHub, or Twilio that need to send requests to your machine.
- **Live Demos**: Show a work-in-progress to a client or colleague without deploying.
- **Mobile Testing**: Test your local website on a real mobile device by accessing the public URL.

---

## Installation

1.  **Prerequisites**:

    - [Rust Toolchain](https://www.rust-lang.org/tools/install)

2.  **From Crates.io (Recommended)**:
    Once published, you can install `rtun` with a single command:

    ```bash
    cargo install rtun
    ```

3.  **From Source**:
    ```bash
    git clone https://github.com/openmymai/rtun.git
    cd rtun
    cargo install --path .
    ```
    This will compile and install the `rtun` binary into your Cargo bin path (`~/.cargo/bin`), making it available system-wide.

---

## Usage

The basic usage is to specify the local port you want to expose.

1.  **Start your local web server.** For example, a Python server on port 8000:

    ```bash
    python3 -m http.server 8000
    ```

2.  **In a new terminal, start `rtun`:**

    ```bash
    rtun --port 8000
    ```

3.  `rtun` will connect to the server and give you a public URL.

    ```
     Tunnel Details
    Public URL: https://heavy-badger-42.loca.lt
    Local Port: 8000
    Status: Live

    INFO rtun::tunnel: Waiting for incoming requests...
    ```

### Options

- **Request a custom subdomain:**

  ```bash
  rtun -p 3000 -s my-cool-project
  ```

  This will attempt to create a tunnel at `https://my-cool-project.loca.lt`.

- **Specify a different tunneling server:**
  ```bash
  rtun -p 8080 --host http://my-localtunnel-server.com
  ```

### Logging

When a request is made to your public URL, you will see a log in the terminal where `rtun` is running:

```
INFO rtun::tunnel: Incoming request from 123.45.67.89:54321
```

---

## How It Works

`rtun` communicates with a `localtunnel-server` instance.

1.  It makes an initial HTTP request to get a subdomain and connection details.
2.  It then establishes a pool of persistent TCP connections to the server's proxy port.
3.  When the server receives a public request, it routes the raw HTTP traffic through one of these TCP connections to your `rtun` client.
4.  `rtun` forwards this traffic to your specified local port (`localhost:<port>`).
5.  The response from your local server is sent back up the same pipeline.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/openmymai/rtun/issues).

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
