# ReputeChain

**Blockchain Reputation System for Gig Economy Riders**

## The Problem
Bad passengers falsely report drivers. Drivers get kicked off platforms. Riders face no consequences.

## The Solution
ReputeChain tracks RIDER reputation across platforms using blockchain technology. Problem passengers can't escape their history.

## Status
- ✅ Frontend: **WORKING** (http://localhost:3000)
- ⚠️ Blockchain: Code complete, compilation blocked by Substrate version issues

## Quick Start

### Frontend (Working Now)
```bash
cd frontend
npm install
npm run dev
```

Open http://localhost:3000

### Blockchain (Compilation Issues)
The Substrate polkadot-v1.0.0 branch has Rust compiler incompatibilities. See DEVPOST_SUBMISSION.md for details.

## Features

### Dashboard
- View your reputation score as a rider
- See recent trips and ratings
- Track rating history

### Leaderboard
- Compare rider ratings across platforms
- Filter by time period
- See top and bottom performers

### Analytics
- Platform-wide statistics
- Recent reviews
- Trending metrics

## Architecture

### Frontend
- React + Vite
- Tailwind CSS
- Polkadot.js integration
- Responsive design

### Blockchain
- Substrate framework
- Custom reputation pallet
- Immutable reputation storage
- Cross-platform tracking

## Market

- 5M+ Uber/Lyft drivers in US
- $400B+ gig economy globally
- No existing rider accountability solution
- Drivers need protection from false reports

## Future Applications

Same architecture works for:
- Police body cam footage (immutable evidence)
- Cross-platform identity verification
- Any reputation system requiring transparency

## Built For
Polkadot Cloud Hack - November 2025

## License
MIT
