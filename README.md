# GreenFlow Protocol

[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
[![Stellar Testnet](https://img.shields.io/badge/Network-Stellar%20Testnet-blue)](https://www.stellar.org)

## 🌿 About GreenFlow Protocol

GreenFlow Protocol is a renewable-energy verification platform powered by Stellar. It provides cooperative operators with an admin interface for asset tracking and publishes proof-of-generation certificate tokens on-chain (1 token = 1 kWh).

### 🚀 Core capabilities

1. **Cooperative operations console**
   - Cooperative managers can onboard members, register meters, record readings, and monitor generation KPIs.
2. **On-chain certificate minting**
   - Generates SEP-41 renewable certificates on Stellar and supports purchase + retirement workflows.

### 🔁 Certificate flow

1. Smart meter posts readings to `POST /api/meters/readings`
2. GreenFlow Protocol mints proto-certificates on-chain (1 token = 1 kWh)
3. External buyer purchases certificates
4. Buyer retires certificate (burn on-chain)

> Each token is a verifiable on-chain claim of renewable generation. It is not electricity, not a financial instrument, and not designed for speculative trading.

### 🏆 Recognition

- Featured Project at Stellar Buenos Aires Hackathon 2025
- Innovation Certificate awarded by Stellar jury

### 🌐 Live Demo

- Frontend: https://greenflow-protocol.vercel.app
- Network: Stellar Testnet

## 🧱 Monorepo structure

```
greenflow-protocol/
├── apps/
│   ├── contracts/           # Soroban smart contracts (Rust)
│   │   ├── energy_token/
│   │   ├── energy_distribution/
│   │   └── community_governance/
│   └── web/                 # Next.js application
├── packages/
│   └── stellar/             # Shared Stellar utilities
└── tooling/
    └── issues/              # GitHub issue templates
```

Powered by Turborepo + pnpm workspaces.

## ⚙️ Quick Start

### Prerequisites
- Node.js v22+
- pnpm v10+ (`corepack enable`)
- Rust + Cargo (for contracts)

### Install
```bash
git clone https://github.com/Mozez155/GreenFlow-Protocol.git
cd GreenFlow-Protocol
pnpm install
```

### Run development
```bash
pnpm dev
```

Frontend local: http://localhost:3000

### Build and test contracts
```bash
cd apps/contracts
stellar contract build
cargo test
```

## 🧪 Contracts deployed on Testnet

| Contract | Address | Purpose |
| --- | --- | --- |
| energy_token | CCYOVOFD...MRPBA6 | SEP-41 token for renewable certificates |
| energy_distribution | CBTDPLFN...NX2UDZ | Certificate allocation by participation |
| community_governance | CCH2EXXN...BJD6YI | Cooperative governance proposals |

Built with OpenZeppelin Stellar v0.5.1 (Pausable + Upgradeable) and Soroban SDK 23.1.0.

65 tests passing. See `docs/5-CONTRACTS.md` for details.

## 🧠 Tech Stack

- Blockchain: Stellar (Soroban smart contracts)
- Smart Contracts: Rust + OpenZeppelin Stellar v0.5.1
- Frontend: Next.js 16 + React 19 + TypeScript
- Styling: Tailwind CSS v4 + shadcn/ui
- Wallets: Freighter + Stellar Wallets Kit
- Backend: Next.js API Routes + Supabase
- Deployment: Vercel
- Monorepo: Turborepo + pnpm

## 🤝 Contributing

1. Browse open issues
2. Comment to claim issue
3. Fork and implement feature
4. Submit PR to `develop`

Branch workflow:
```bash
git checkout develop
git checkout -b feat/your-feature
pnpm install && pnpm dev
# submit PR to develop
```

## 📈 Product roadmap levels

| Level | Description | Status |
| --- | --- | --- |
| 1 | Internal registry (token = production record) | Current |
| 2 | Verifiable certification (meters + oracles + verification) | Next |
| 3 | Recognized standard (I-REC, Energy Web, TIGR) | Future |

## 🧾 License

Apache-2.0 — See `LICENSE`.

Built on Stellar | GreenFlow Protocol 2026
