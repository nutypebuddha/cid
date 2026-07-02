# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| 0.0.x   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability within CID, please send an email to the maintainer. All security vulnerabilities will be promptly addressed.

### What to include

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Suggested fix (if any)

### Response timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix or mitigation**: Within 2 weeks (depending on severity)

## Security Considerations

### Input Validation

CID validates LLM output, but users should be aware:

- **Math validation**: Checks arithmetic correctness, not mathematical truth
- **Fact validation**: Checks against known knowledge base, not absolute truth
- **Logic validation**: Checks logical consistency, not semantic meaning

### Proxy Mode

When using CID as an HTTP proxy:

- API keys are passed in environment variables, not stored
- No data is logged or persisted by default
- HTTPS is recommended for production use

### MCP Server

When running as an MCP server:

- No authentication is built-in (relies on transport security)
- Tools are rate-limited by default
- Input sanitization is applied to all tool arguments

### WASM Binary

The WASM binary:

- Runs in a sandboxed environment
- No filesystem access
- No network access (unless explicitly configured)
- Memory-safe by construction

## Best Practices

1. **Use environment variables** for API keys, not command-line arguments
2. **Run with minimal privileges** in production
3. **Enable HTTPS** for proxy mode
4. **Rate-limit MCP access** in production
5. **Monitor logs** for suspicious activity
6. **Keep CID updated** to the latest version

## Dependencies

CID has minimal dependencies:

- **Default**: Zero external dependencies
- **With `proxy` feature**: `ureq` HTTP client

All dependencies are audited regularly via `cargo audit`.

## Cryptography

CID does not implement cryptographic functions. All security relies on:

- Transport security (TLS/HTTPS)
- Operating system security
- Network security measures

## Contact

For security inquiries, please open an issue on Codeberg or contact the maintainer directly.
