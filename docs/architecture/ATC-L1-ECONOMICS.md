# A-TownChain L1 Economics

## Canonical native coin

- Symbol: **ATC**
- Layer: **A-TownChain L1**
- Maximum lifetime supply: **360,000,000 ATC**
- Emission duration: **360 protocol months / 30 years**
- Halving interval: **60 protocol months / 5 years**
- Halving events: **5**
- Discretionary minting: **disabled**
- Scheduled emission: **enabled only through the canonical consensus schedule**
- Supply accounting: balances + staked balances
- Supply and emission state are committed into the deterministic L1 state root.

## 30-year halving schedule

The L1 uses six 60-month emission epochs. The mathematical monthly rate is halved at each boundary:

| Protocol months | Years | Rate multiplier |
|---|---:|---:|
| 1–60 | 1–5 | 1 |
| 61–120 | 6–10 | 1/2 |
| 121–180 | 11–15 | 1/4 |
| 181–240 | 16–20 | 1/8 |
| 241–300 | 21–25 | 1/16 |
| 301–360 | 26–30 | 1/32 |

The initial mathematical rate is 64,000,000 / 21 ATC per month. Because ATC is represented as whole units, the implementation uses a fixed-point denominator of 672 and carries the remainder from month to month. This makes the complete 360-month schedule deterministic and produces **exactly 360,000,000 ATC**, with zero remainder at month 360.

A protocol month is a sequential consensus emission index. This avoids depending on local wall-clock/calendar interpretation. The transaction/block layer is responsible for enforcing when the next protocol month may be released.

## Consensus invariants

The canonical Rust L1 state machine enforces:

1. the maximum lifetime supply is 360,000,000 ATC;
2. no discretionary post-genesis mint operation exists;
3. scheduled issuance can advance only one month at a time;
4. month 361 and every later month are rejected;
5. the halving epoch is derived deterministically from the month index;
6. the fixed-point remainder is persisted in state;
7. emission counters and released supply are included in the state root;
8. the complete schedule cannot exceed the supply cap.

### Genesis

Genesis allocations remain subject to the same 360,000,000 ATC cap. For the canonical 30-year emission tokenomics, mainnet genesis should use **zero premine** unless an explicit allocation is separately approved, because any genesis allocation consumes the lifetime supply cap.

After genesis is sealed, scheduled emission can credit a consensus-selected recipient account. The recipient is not hard-coded into the economics module; allocation policy must be specified by the corresponding consensus/reward/governance layer.

## L2 relationship

ATC is the native L1 settlement asset. The ATC-L2 architecture may use ATC as its settlement/gas asset, but L2 token issuance remains governed by the L2 protocol and does not create additional L1 ATC.

## Deterministic state

The emission state contains:

- months_released
- released_supply
- fixed-point remainder

All three fields participate in the L1 state-root calculation, so restart/replay must reconstruct the same monetary state.
