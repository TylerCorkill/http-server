# Tust
![example workflow](https://github.com/TylerCorkill/tust-server/actions/workflows/rust.yml/badge.svg)

Tust is an HTTP server modeled after Express.js

## Tenants
### 1. Stability
After initialization, the server should produce error messages instead of crashing at all times. A request should never be able to crash the server.

### 2. Performance
Performance is paramount and new features should not impact stability or performance. That said, bug fixes may have impact on performance occasionally.

### 3. Abstraction
Complexity should be abstracted away as long as it has no impact on stability or performance. This is in line with the [KISS Principle](https://en.wikipedia.org/wiki/KISS_principle).
