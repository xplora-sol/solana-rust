# 🏗️ Xplora Architecture Diagram

## 📱 Complete System Flow

```
┌──────────────────────────────────────────────────────────────────────┐
│                          MOBILE APP (React Native)                    │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  1. User Profile Creation          2. Quest Discovery                │
│  ┌──────────────┐                  ┌──────────────┐                 │
│  │ New User     │                  │ Browse       │                 │
│  │ Enters Name  │                  │ Quests       │                 │
│  └──────┬───────┘                  └──────┬───────┘                 │
│         │                                  │                         │
│         ▼                                  ▼                         │
│  3. Photo Capture                  4. IPFS Upload                   │
│  ┌──────────────┐                  ┌──────────────┐                 │
│  │ Take Photo   │────────────────▶ │ Upload to    │                 │
│  │ of Landmark  │                  │ IPFS         │                 │
│  └──────────────┘                  └──────┬───────┘                 │
│                                            │                         │
│                                            ▼                         │
│                                     Get IPFS Hash                    │
│                                     "QmXxx..."                       │
└───────────────────────────────────────────┬──────────────────────────┘
                                            │
                                            ▼
┌──────────────────────────────────────────────────────────────────────┐
│                        SOLANA BLOCKCHAIN                              │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  5. Submit Transaction                                                │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ submit_quest_completion(location, quest_index, ipfs_hash)    │    │
│  └────────────────────────┬────────────────────────────────────┘    │
│                           │                                           │
│                           ▼                                           │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │          PROGRAM ACCOUNTS (PDAs)                             │    │
│  │                                                               │    │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │    │
│  │  │ UserProfile  │  │ LocationQuests│  │ QuestSub-    │       │    │
│  │  │              │  │              │  │ mission      │       │    │
│  │  │ - XP         │  │ - Quests[]   │  │              │       │    │
│  │  │ - Level      │  │ - Location   │  │ - Status:    │       │    │
│  │  │ - Streaks    │  │              │  │   Pending    │       │    │
│  │  └──────────────┘  └──────────────┘  └──────────────┘       │    │
│  │                                                               │    │
│  └───────────────────────────┬───────────────────────────────────┘    │
│                              │                                        │
│                              ▼                                        │
│  6. Emit Event                                                        │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ QuestSubmissionEvent {                                       │    │
│  │   user, submission_pda, ipfs_hash, location, quest_index    │    │
│  │ }                                                            │    │
│  └────────────────────────┬────────────────────────────────────┘    │
│                           │                                           │
└───────────────────────────┼───────────────────────────────────────────┘
                            │
                            │ WebSocket / Event Listener
                            │
                            ▼
┌──────────────────────────────────────────────────────────────────────┐
│                    BACKEND SERVICE (FastAPI + AI)                     │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  7. Event Received                                                    │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ Listen to QuestSubmissionEvent                               │    │
│  └────────────────────────┬────────────────────────────────────┘    │
│                           │                                           │
│                           ▼                                           │
│  8. Download from IPFS                                                │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ ipfs.get(event.ipfs_hash) → photo_file                       │    │
│  └────────────────────────┬────────────────────────────────────┘    │
│                           │                                           │
│                           ▼                                           │
│  9. AI Validation                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ • Check GPS metadata                                         │    │
│  │ • Vision AI: Match landmark                                 │    │
│  │ • Verify against quest.verifiable_landmark                  │    │
│  └────────────────────────┬────────────────────────────────────┘    │
│                           │                                           │
│                  ┌────────┴────────┐                                 │
│                  │                 │                                 │
│            ✅ Valid           ❌ Invalid                              │
│                  │                 │                                 │
│                  ▼                 ▼                                 │
│  10. Send Transaction                                                 │
│  ┌──────────────────┐  ┌──────────────────┐                         │
│  │ approve_         │  │ reject_          │                         │
│  │ submission()     │  │ submission()     │                         │
│  └─────────┬────────┘  └─────────┬────────┘                         │
│            │                     │                                   │
└────────────┼─────────────────────┼───────────────────────────────────┘
             │                     │
             ▼                     ▼
┌──────────────────────────────────────────────────────────────────────┐
│                        SOLANA BLOCKCHAIN                              │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  11. Update Accounts                                                  │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ QuestSubmission.status = Approved ✅                         │    │
│  │ UserProfile.quests_completed += 1                            │    │
│  │ UserProfile.experience_points += xp_reward                   │    │
│  │ UserProfile.level = calculate_level()                        │    │
│  │ UserProfile.current_streak = update_streak()                 │    │
│  └────────────────────────┬────────────────────────────────────┘    │
│                           │                                           │
│                           ▼                                           │
│  12. Emit Reward Event                                                │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ QuestRewardEvent {                                           │    │
│  │   user, xp_reward: 150, token_reward: 200000000,            │    │
│  │   new_level: 3                                               │    │
│  │ }                                                            │    │
│  └────────────────────────┬────────────────────────────────────┘    │
│                           │                                           │
└───────────────────────────┼───────────────────────────────────────────┘
                            │
                            │ WebSocket Subscription
                            │
                            ▼
┌──────────────────────────────────────────────────────────────────────┐
│                          MOBILE APP                                   │
├──────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  13. UI Update                                                        │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ 🎉 Quest Complete!                                           │    │
│  │                                                               │    │
│  │ ✨ +150 XP                                                   │    │
│  │ 🪙 +0.2 XPLORA tokens                                        │    │
│  │ 📈 Level 3 → Level 4                                         │    │
│  │                                                               │    │
│  │ [View Profile] [Next Quest]                                  │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 🗺️ Data Model Relationships

```
┌─────────────────────────────────────────────────────────────────┐
│                     QuestRegistry (Global PDA)                   │
│  Seeds: ["quest_registry"]                                       │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │ authority: Pubkey (admin who can create quests)         │    │
│  │ total_locations: u64                                     │    │
│  │ version: u8                                              │    │
│  └─────────────────────────────────────────────────────────┘    │
└───────────────────────────┬─────────────────────────────────────┘
                            │ has many
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│             LocationQuests (Per Location PDA)                    │
│  Seeds: ["location_quests", location_name]                       │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │ location: "Kathmandu"                                    │    │
│  │ quests: Vec<Quest> (max 10)                              │    │
│  │   ├─ Quest 0: "Hidden Temple"                            │    │
│  │   ├─ Quest 1: "Guardian Owl"                             │    │
│  │   └─ Quest 2: "Lotus Pillar"                             │    │
│  └─────────────────────────────────────────────────────────┘    │
└───────────────────────────┬─────────────────────────────────────┘
                            │ referenced by
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│            QuestSubmission (Per User-Quest PDA)                  │
│  Seeds: ["submission", user, location, quest_index]              │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │ user: User1's Pubkey                                     │    │
│  │ location: "Kathmandu"                                    │    │
│  │ quest_index: 0                                           │    │
│  │ ipfs_hash: "QmXxx..."                                    │    │
│  │ status: Pending → Approved ✅                            │    │
│  │ reward_amount: 200000000                                 │    │
│  └─────────────────────────────────────────────────────────┘    │
└───────────────────────────┬─────────────────────────────────────┘
                            │ updates
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│               UserProfile (Per User PDA)                         │
│  Seeds: ["user_profile", user_pubkey]                            │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │ user: User1's Pubkey                                     │    │
│  │ username: "Explorer123"                                  │    │
│  │ quests_completed: 15                                     │    │
│  │ experience_points: 3,500                                 │    │
│  │ level: 7                                                 │    │
│  │ rank_tier: Silver 🥈                                     │    │
│  │ current_streak: 5 days 🔥                                │    │
│  │ unique_locations: 3                                      │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔄 Instruction Flow Diagram

```
ADMIN OPERATIONS:
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  initialize  │────▶│ create_      │────▶│ add_quest_   │
│  (once)      │     │ location_    │     │ to_location  │
│              │     │ quests       │     │              │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                            ┌─────────────────────┴─────────────────┐
                            │                                        │
                            ▼                                        ▼
                     ┌──────────────┐                       ┌──────────────┐
                     │ update_quest │                       │ delete_quest │
                     └──────────────┘                       └──────────────┘

USER OPERATIONS:
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ initialize_  │────▶│ submit_quest_│────▶│ (wait for    │
│ user_profile │     │ completion   │     │ validation)  │
│ (once)       │     │              │     │              │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                                    ┌─────────────┴──────────────┐
                                    │                            │
                                    ▼                            ▼
                            ┌──────────────┐           ┌──────────────┐
                            │ approve_     │           │ reject_      │
                            │ submission   │           │ submission   │
                            │ (backend)    │           │ (backend)    │
                            └──────────────┘           └──────────────┘
                                    │
                                    ▼
                            ┌──────────────┐
                            │ Rewards      │
                            │ Distributed! │
                            │ 🎉           │
                            └──────────────┘
```

---

## 🎯 PDA Derivation Map

```
Program ID: 3rD6xKajAwvt8xbN5tkSSM8CvftGDs5x9jinkCK4BCCj

1. QuestRegistry
   ├─ Seeds: ["quest_registry"]
   └─ Address: findProgramAddress(["quest_registry"], programId)

2. LocationQuests (Example: Kathmandu)
   ├─ Seeds: ["location_quests", "Kathmandu"]
   └─ Address: findProgramAddress(["location_quests", "Kathmandu"], programId)

3. UserProfile (Example: User ABC123...)
   ├─ Seeds: ["user_profile", userPubkey]
   └─ Address: findProgramAddress(["user_profile", ABC123...], programId)

4. QuestSubmission (Example: User ABC... for Kathmandu Quest 0)
   ├─ Seeds: ["submission", userPubkey, "Kathmandu", [0]]
   └─ Address: findProgramAddress(["submission", ABC..., "Kathmandu", [0]], programId)
```

---

## 📊 Event Monitoring Flow

```
┌─────────────────────────────────────────────────────────────┐
│                  SOLANA PROGRAM EVENTS                       │
└───────────────┬─────────────────────────────────────────────┘
                │
                │ Emits
                │
    ┌───────────┼───────────┬──────────────┬──────────────┐
    │           │           │              │              │
    ▼           ▼           ▼              ▼              ▼
┌────────┐ ┌────────┐ ┌─────────┐ ┌──────────┐ ┌──────────┐
│Profile │ │Quest   │ │Quest    │ │Quest     │ │(Future   │
│Created │ │Submit  │ │Reward   │ │Rejection │ │Events)   │
└────┬───┘ └────┬───┘ └─────┬───┘ └────┬─────┘ └──────────┘
     │          │           │           │
     │          │           │           │
     └──────────┴───────────┴───────────┴──────────┐
                                                    │
                                                    ▼
                                    ┌───────────────────────────┐
                                    │  Backend Event Listeners  │
                                    │  • FastAPI WebSocket      │
                                    │  • Helius Webhooks        │
                                    │  • Direct RPC Subscribe   │
                                    └─────────┬─────────────────┘
                                              │
                ┌─────────────────────────────┼──────────────────┐
                │                             │                  │
                ▼                             ▼                  ▼
    ┌───────────────────┐       ┌───────────────────┐  ┌──────────────┐
    │ AI Validator      │       │ Analytics DB      │  │ Push Notifs  │
    │ (Photo Check)     │       │ (Track Stats)     │  │ (User Alerts)│
    └───────────────────┘       └───────────────────┘  └──────────────┘
```

---

## 🏆 Leaderboard Architecture

```
┌────────────────────────────────────────────────────────────┐
│            SOLANA (On-Chain - Source of Truth)             │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  UserProfile 1       UserProfile 2       UserProfile N     │
│  ├─ XP: 10,500      ├─ XP: 8,200       ├─ XP: 1,000       │
│  ├─ Quests: 45      ├─ Quests: 32      ├─ Quests: 5       │
│  └─ Streak: 12      └─ Streak: 5       └─ Streak: 1       │
│                                                             │
└──────────────────────────┬─────────────────────────────────┘
                           │
                           │ Query every 5 minutes
                           │
                           ▼
┌────────────────────────────────────────────────────────────┐
│           BACKEND AGGREGATOR (Off-Chain Service)           │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  1. getProgramAccounts(UserProfile)                        │
│  2. Sort by XP descending                                  │
│  3. Rank users 1, 2, 3...                                  │
│  4. Cache in Redis (TTL: 5 min)                            │
│                                                             │
└──────────────────────────┬─────────────────────────────────┘
                           │
                           │ Fast API Response
                           │
                           ▼
┌────────────────────────────────────────────────────────────┐
│                  MOBILE APP (Client)                       │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  🏆 LEADERBOARD                                            │
│  ┌────────────────────────────────────────────────────┐   │
│  │ #1  🥇 CryptoExplorer    10,500 XP   Level 21      │   │
│  │ #2  🥈 AdventureKing      8,200 XP   Level 16      │   │
│  │ #3  🥉 QuestMaster        6,900 XP   Level 13      │   │
│  │ ...                                                 │   │
│  │ #47 👤 You                1,500 XP   Level 3       │   │
│  └────────────────────────────────────────────────────┘   │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

---

## 🔐 Security Layers

```
┌─────────────────────────────────────────────────────────┐
│                    INPUT VALIDATION                      │
├─────────────────────────────────────────────────────────┤
│ ✓ IPFS hash format (Qm + min 46 chars)                 │
│ ✓ Coordinate bounds (Nepal: 26-31°N, 80-89°E)          │
│ ✓ String length limits (title, description, etc.)      │
│ ✓ Quest index in bounds                                 │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  ACCOUNT VALIDATION                      │
├─────────────────────────────────────────────────────────┤
│ ✓ PDA derivation verification                           │
│ ✓ Signer authorization checks                           │
│ ✓ Account ownership validation                          │
│ ✓ Account initialized status                            │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                   STATE VALIDATION                       │
├─────────────────────────────────────────────────────────┤
│ ✓ Status transition rules (Pending → Approved)          │
│ ✓ Max attempts enforcement                              │
│ ✓ Duplicate submission prevention                       │
│ ✓ Overflow protection (checked_add)                     │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                 AUTHORITY CONTROL                        │
├─────────────────────────────────────────────────────────┤
│ ✓ Admin-only quest management                           │
│ ✓ Validator-only approval/rejection                     │
│ ✓ Registry authority checks                             │
│ ✓ User-specific profile/submission access               │
└─────────────────────────────────────────────────────────┘
```

---

_This architecture ensures a fully decentralized, scalable, and secure quest game on Solana! 🚀_
