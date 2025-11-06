# ReputeChain - Quick Start Guide

## ⚡ 5-Minute Setup

### Step 1: Clone & Navigate
```bash
cd /home/bill/pallet-reputation
```

### Step 2: Install Dependencies
```bash
make init-dev
```

This will:
- Install Rust toolchain
- Add WebAssembly target
- Install Node.js dependencies

### Step 3: Start the Blockchain
```bash
make run
```

Wait for the node to start. You should see:
```
🔨 Initializing Genesis block/state
⏱  Block time: 6000ms
🏁 Substrate node started
```

### Step 4: Start Frontend (New Terminal)
```bash
make frontend-dev
```

Frontend will be available at: **http://localhost:3000**

### Step 5: Connect Wallet
1. Install Polkadot.js extension (Chrome/Firefox)
2. Click "Connect Wallet" on the app
3. Approve the connection

### Step 6: Create Profile & Submit Rating
1. Your profile is automatically created
2. Enter a test address in "Rate User Address"
3. Set ratings and click "Submit Rating"
4. View your stats on the dashboard

---

## 🐳 Docker Quick Start

```bash
# Start full stack (node + frontend + IPFS)
make docker-up

# Access:
# Frontend: http://localhost:3000
# Node RPC: ws://localhost:9944
# IPFS: http://localhost:5001

# Stop everything
make docker-down
```

---

## 📊 Explore the App

### Dashboard
- View your reputation score
- See category breakdowns
- Submit new ratings
- Track staked amount

### Leaderboard
- View top-rated users
- Filter by category
- See global rankings

### Analytics
- Score trends over time
- Category performance
- Platform distribution
- Recent reviews

---

## 🧪 Testing

```bash
# Run all tests
make test

# Run specific pallet tests
cargo test --package pallet-reputation

# Run frontend tests
cd frontend && npm test
```

---

## 🔧 Development Commands

```bash
# Format code
make fmt

# Run linter
make lint

# Clean build artifacts
make clean

# Build for production
make build

# Deploy
make deploy
```

---

## 📝 Key Files

| File | Purpose |
|------|---------|
| `reputation_pallet.rs` | Core blockchain logic |
| `runtime/lib.rs` | Runtime configuration |
| `frontend/src/App.tsx` | Main React component |
| `frontend/src/api.ts` | Polkadot.js integration |
| `Makefile` | Development commands |
| `docker-compose.yml` | Container orchestration |

---

## 🚀 Next Steps

1. **Customize:** Modify the pallet or frontend for your use case
2. **Deploy:** Use `make docker-up` for local deployment
3. **Test:** Run `make test` to verify everything works
4. **Demo:** Follow `DEMO_SCRIPT.md` to record a video
5. **Submit:** Use `PITCH.md` for hackathon submission

---

## 🆘 Troubleshooting

### Node won't start
```bash
# Clean and rebuild
make clean
make build
make run
```

### Frontend won't load
```bash
# Reinstall dependencies
cd frontend
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Wallet connection fails
- Ensure Polkadot.js extension is installed
- Refresh the page
- Check browser console for errors

### Docker issues
```bash
# Remove all containers
docker-compose down -v

# Rebuild images
make docker-build
make docker-up
```

---

## 📚 Documentation

- **README.md** - Full project documentation
- **DEMO_SCRIPT.md** - Video demo script
- **PITCH.md** - Investor pitch deck
- **PROJECT_SUMMARY.md** - Complete overview
- **CONTRIBUTING.md** - Development guidelines

---

## 💡 Tips

- Use `make help` to see all available commands
- Frontend hot-reloads during development
- Check `docker-compose logs` for container issues
- Use browser DevTools to debug frontend
- Check terminal output for blockchain errors

---

## 🎯 Common Tasks

### Add a new feature
```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes
# Test
make test

# Format and lint
make fmt && make lint

# Commit
git commit -m "feat: add my feature"
```

### Deploy to production
```bash
# Build
make build
make frontend-build

# Create Docker images
make docker-build

# Push to registry
docker push reputechain:latest

# Deploy to cloud
# (Use your cloud provider's deployment method)
```

### Run tests
```bash
# All tests
make test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

---

## 📞 Need Help?

- Check the documentation files
- Review the code comments
- Check browser console for errors
- Look at terminal output for logs
- Join our Discord community

---

**You're all set! 🚀 Start building with ReputeChain!**
