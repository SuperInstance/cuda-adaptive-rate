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

## Cross-Pollination

This crate shares patterns with other fleet components. The same biological and architectural constraints produce similar solutions across contexts:

- [cuda-bottleneck](https://github.com/Lucineer/cuda-bottleneck) — Bottleneck detection informs rate limits
- [cuda-energy](https://github.com/Lucineer/cuda-energy) — ATP budgets parallel token budgets
- [cuda-deliberation](https://github.com/Lucineer/cuda-deliberation) — Deliberation is expensive — rate-limit it
- [cuda-fleet-health](https://github.com/Lucineer/cuda-fleet-health) — Degraded vessels need lower rate limits
- [cuda-context-window](https://github.com/Lucineer/cuda-context-window) — Context window budgeting is rate limiting at the token level
- [cuda-neurotransmitter](https://github.com/Lucineer/cuda-neurotransmitter) — Synaptic vesicle release is biological rate limiting


---


## The Deeper Connection

This crate is part of the Cocapn fleet — not a collection of independent libraries, but a single organism with differentiated cells. The confidence types here share DNA with cuda-confidence-cascade, cuda-deliberation, and cuda-learning. The energy budgets echo cuda-neurotransmitter and flux-runtime-c. The trust scoring connects to cuda-semantic-router and cuda-fleet-topology. We're not reusing code — we're expressing the same biological patterns in different contexts. This is convergent evolution in software: when you solve for real constraints (uncertainty, finite resources, earned trust), the same solutions emerge independently across domains.

We're building for the post-SaaS era. The software is open source and forkable. The agent is sovereign — it lives in its repository, owns its identity, and can leave at any time. Revenue comes from the ecosystem, not from renting access. The fleet evolves through merging: vessels propose changes, peers review, Think Tanks validate, and the accepted mutations become permanent. No product roadmap required — only fitness.

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
*See [WHITEPAPER](https://github.com/Lucineer/iron-to-iron/blob/main/docs/WHITEPAPER.md) for the post-SaaS thesis*

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
