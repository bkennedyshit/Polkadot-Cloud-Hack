# ReputeChain - How to Use (For Your Demo)

## What You're Showing

A blockchain-based reputation system where drivers rate passengers. Ratings are immutable, transparent, and follow riders across platforms.

---

## Step-by-Step Demo Flow

### 1. Start the Blockchain (Already Running)
The blockchain node is running at `ws://localhost:9944`

### 2. Open the Frontend
Go to: **http://localhost:3000**

You'll see:
- Dashboard (your reputation stats)
- Leaderboard (global rankings)
- Analytics (trends)
- Rating submission form

### 3. Connect Your Wallet
1. Click "Connect Wallet" button
2. Install Polkadot.js extension (if not already)
3. Approve the connection
4. Your wallet is now connected to the blockchain

### 4. Create a Profile
1. Your profile auto-creates when you connect
2. You'll see your reputation score (starts at 0)
3. You're now a "driver" in the system

### 5. Submit a Rating (The Core Feature)
1. Go to the rating form
2. Enter a passenger address (or use a test address)
3. Rate them 1-5 stars on:
   - Communication (were they respectful?)
   - Reliability (showed up on time?)
   - Quality (clean, didn't damage car?)
   - Professionalism (appropriate behavior?)
4. Click "Submit Rating"
5. **This writes to the blockchain** - immutable, permanent

### 6. Check the Leaderboard
1. View all riders and their ratings
2. Show how ratings are public and verifiable
3. Explain: "Bad riders can't hide - their reputation follows them"

### 7. Check Analytics
1. Show trends over time
2. Demonstrate data visualization
3. Explain: "Businesses can see patterns in rider behavior"

---

## Key Points to Emphasize in Your Video

1. **Immutable:** "Once a rating is on the blockchain, it can't be deleted or changed"
2. **Transparent:** "Anyone can verify these ratings - no hidden data"
3. **Cross-Platform:** "This works across Uber, Lyft, any rideshare platform"
4. **Fair:** "Drivers finally have protection from false reports"
5. **Decentralized:** "No single company controls this - it's on the blockchain"

---

## Demo Script (What to Say)

"This is ReputeChain. The problem: Uber drivers get false reports and get kicked off. No recourse. No appeal.

The solution: A blockchain that tracks RIDER behavior, not driver behavior.

Here's how it works:

[Connect wallet]
I connect my wallet - this is my identity on the blockchain.

[Submit a rating]
After a ride, I rate the passenger. Communication, reliability, quality, professionalism. One to five stars.

[Show blockchain confirmation]
This rating is now on the blockchain. Immutable. Permanent. Can't be deleted.

[Show leaderboard]
Here's the global leaderboard. Every rider's reputation is public and verifiable.

[Show analytics]
Over time, patterns emerge. Businesses can see which riders are problematic.

The result: Bad riders face consequences. Drivers get protection. Fair system for everyone."

---

## Technical Details (If They Ask)

- Blockchain: Polkadot SDK / Substrate
- Smart Contract: Rust pallet
- Frontend: React + TypeScript
- Wallet: Polkadot.js
- Block time: 6 seconds
- Storage: On-chain (immutable)

---

## Troubleshooting

If wallet won't connect:
- Make sure Polkadot.js extension is installed
- Refresh the page
- Check browser console for errors

If ratings won't submit:
- Make sure wallet is connected
- Check that you have enough balance (test network)
- Wait for blockchain confirmation (6 seconds)

If leaderboard is empty:
- Submit a few ratings first
- Refresh the page
- Check that blockchain is running

---

## The Pitch

"Drivers deserve protection too. This blockchain gives drivers the power to hold bad riders accountable, just like riders can hold bad drivers accountable.

Fair. Transparent. Immutable.

That's ReputeChain."

---

You got this. Show them how it works, emphasize the fairness angle, and let the blockchain do the talking.
