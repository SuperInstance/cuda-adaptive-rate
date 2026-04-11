# cuda-adaptive-rate

Adaptive rate limiting for agent workloads — token-aware, model-specific backpressure

Part of the Cocapn fleet layer — how vessels coordinate, route, and scale.

## What It Does

### Key Types

- `ModelLimit` — core data structure
- `RateDecision` — core data structure
- `AdaptiveRateLimiter` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-adaptive-rate.git
cd cuda-adaptive-rate

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_adaptive_rate::*;

// See src/lib.rs for full API
// 5 unit tests included
```

### Available Implementations

- `TokenBucket` — see source for methods
- `AdaptiveRateLimiter` — see source for methods

## Testing

```bash
cargo test
```

5 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: fleet
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-semantic-router](https://github.com/Lucineer/cuda-semantic-router)
- [cuda-fleet-topology](https://github.com/Lucineer/cuda-fleet-topology)
- [cuda-bottleneck](https://github.com/Lucineer/cuda-bottleneck)
- [cuda-fleet-health](https://github.com/Lucineer/cuda-fleet-health)
- [cuda-swarm-agent](https://github.com/Lucineer/cuda-swarm-agent)
- [cuda-trust](https://github.com/Lucineer/cuda-trust)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
