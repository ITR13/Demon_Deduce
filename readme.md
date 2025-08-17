# Demon Deduce

This project is an automatic solver for the social deduction game [**Demon Bluff**](https://store.steampowered.com/app/3522600/Demon_Bluff/).

## Command Line Usage

```bash
cargo run -- <deck> <villagers> <outcasts> <minions> <demons> <card1_info> <card2_info> ...
```

### Arguments

- `<deck>`: Comma-separated list of roles in the deck
- `<villagers>`: Number of villagers in play
- `<outcasts>`: Number of outcasts in play
- `<minions>`: Number of minions in play
- `<demons>`: Number of demons in play
- `<cardN_info>`: Information for each card in format `visible:confirmed:statement`:
  - `visible`: The role shown face-up (or "?" if unknown)
  - `confirmed`: The confirmed true role (or "?" if unknown)
  - `statement`: The statement made by the card (or blank if unknown/no statement)

**Note:** All card positions are 0-indexed (one less than in-game position numbers)

### Alternative Usage

If you have `-c` or `-l` anywhere in the arguments, it will try to parse a different format from the clipboard instead. -c does it once, -l does it in loop every time it changes.
The format for this is:
- One line with a comma-separated list of the roles in the deck
- One line with 4 ints representing the villager, outcast, minion, and demon counts
- N lines in the following format: `[index]|[visible-role]|[confirmed-role]|[statement]`. Where N <= the amount of cards in play

## Implemented Roles

### Villagers:
- ✅ Alchemist
- ✅ Bard
- ✅ Confessor
- ✅ Empress
- ✅ Enlightened
- ✅ Fortune Teller
- ✅ Gemcrafter
- ✅ Hunter
- ✅ Jester
- ✅ Judge
- ✅ Knight
- ✅ Knitter
- ✅ Lover
- ✅ Medium
- ✅ Scout
- ✅ Slayer
- ❌ Architect
- ❌ Baker
- ❌ Bishop
- ❌ Dreamer
- ❌ Druid
- ❌ Oracle
- ❌ Poet
- ❌ Witness

### Outcasts:
- ✅ Bombardier
- ✅ Doppelganger
- ✅ PlagueDoctor
- ✅ Wretch
- ❌ Drunk

### Minions:
- ✅ Minion
- ✅ Poisoner
- ✅ TwinMinion
- ✅ Witch
- ✅ Counsellor
- ❌ Puppeteer/Puppet
- ❌ Shaman

### Demons:
- ✅ Baa
- ❌ Pooka
- ❌ Lilis

## How It Works

It brute forces every possible starting combination and random effects, then matches that against the visible cards & statements to find out which ones are possible.

## Output

Solutions are shown with color-coded roles:
- Green: Villagers / Good
- Yellow: Outcasts
- Red: Minions / Evil
- Bright Red: Demons

