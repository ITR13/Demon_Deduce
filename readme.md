# Demon Deduce

This project is an automatic solver for the social deduction game [**Demon Bluff**](https://store.steampowered.com/app/3522600/Demon_Bluff/).

## Command Line Usage

```bash
cargo run -- <deck> <villagers> <minions> <demons> <outcasts> <card1_info> <card2_info> ...
```

### Arguments

- `<deck>`: Comma-separated list of roles in the game
- `<villagers>`: Number of villagers in play
- `<minions>`: Number of minions in play
- `<demons>`: Number of demons in play
- `<outcasts>`: Number of outcasts in play
- `<cardN_info>`: Information for each card in format `visible:confirmed:statement`:
  - `visible`: The role shown face-up (or "?" if unknown)
  - `confirmed`: The confirmed true role (or "?" if unknown)
  - `statement`: The statement made by the card (or "unrevealed"/"?")

**Note:** All card positions are 0-indexed (one less than in-game position numbers)

### Statement Syntax

Statements can be any of:
- `unrevealed` or `?` - No statement made
- `clockwise`/`counterclockwise`/`equidistant` - Enlightened statements
- `iamgood`/`iamdizzy` - Confessor statements
- `claim[target;type]` - For Gemcrafter/Judge/Slayer (type: good/evil/truthy/lying)
- `evilcount[targets;count;minimum;none_closer]` - For Empress/Hunter/Jester/Lover
- `roleclaim[target;role]` - For Medium
- `roledistance[role;distance]` - For Scout

## Implemented Roles

### Villagers:
- ✅ Bard
- ✅ Confessor
- ✅ Empress
- ✅ Enlightened
- ✅ Gemcrafter
- ✅ Hunter
- ✅ Jester
- ✅ Judge
- ✅ Knight
- ✅ Lover
- ✅ Medium
- ✅ Scout
- ✅ Slayer
- ❌ Alchemist
- ❌ Architect
- ❌ Baker
- ❌ Bishop
- ❌ Dreamer
- ❌ Druid
- ❌ Fortune Teller
- ❌ Knitter
- ❌ Oracle
- ❌ Poet
- ❌ Witness

### Outcasts:
- ✅ Bombardier
- 🟡 PlagueDoctor
- ✅ Wretch
- ❌ Drunk

### Minions:
- ✅ Minion
- ✅ Poisoner
- ✅ TwinMinion
- ✅ Witch
- ❌ Counsellor
- ❌ Doppelganger
- ❌ Puppeteer/Puppet
- ❌ Shaman

### Demons:
- ✅ Baa
- ❌ Pooka
- ❌ Lilis

## How It Works

The solver:
1. Travels through each valid combination of starting cards that produce the visible cards
2. For each combination, check if the visible statements are valid
3. Creates a list of valid combinations and what roles each position can have

## Output

Solutions are shown with color-coded roles:
- Green: Villagers / Good
- Yellow: Outcasts
- Red: Minions / Evil
- Bright Red: Demons

