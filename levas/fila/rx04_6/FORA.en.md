*Portuguese version: [FORA.md](FORA.md).*

# rx04_6: langlib_apply_grammar_modifier (0x6701f0, Langlib.cpp): left out, for now

Reader rx04, round 6, 25/11/2026. The routine was **not** ported and is not queued: it is too big for one round. The
rest of Langlib.cpp went in ([rx04_1](../rx04_1/rotinas.py) to [rx04_5](../rx04_5/rotinas.py)); the copy of
`langlib_sprintf` (rx04_2) calls this original through `exe::langlib_apply_grammar_modifier`.

## The size

- The map entry runs from 0x6701f0 to 0x682060 (73,328 bytes). By the disassembly, the code runs to 0x67fdc4 (~64 KB,
  14 `ret`; the body goes on past the last `ret` at 0x678466, through jumps) and 0x67fdc4 to 0x682060 holds ~170 jump
  and index tables (8.8 KB) of the `switch`es on the first letter (`movsx eax, byte [esp+0x1018]; add eax, -0x41; cmp
  eax, 0x34`) and on the gender.
- A 0x2808-byte frame (`call 0x945610`, the `_chkstk`), four 0x800 buffers (the text at 0x1018, the outputs at 0x18
  and 0x818) and a zeroed 0x800 one at 0x1818.

## What is known (for whoever goes on)

- **Convention:** cdecl, 5 dwords: the text, the mark before ("upper", "lower"...), the mark after ("s", "S",
  articles), and the two gender bytes (read only as bytes: `movsx eax, byte [esp+0x2828]` 82 times, `[esp+0x282c]` 59
  times). `langlib_sprintf` passes the caller's genders (0xff 0xff = none, and then it does not call) or 0x64 0x64 in
  the English second pass.
- **Output:** returns 0xb64804 (the only global it writes, cleared on entry), 0xad9c64 (the empty text, for a null
  text) or 0 (7 `xor eax, eax` exits: `langlib_sprintf` reads the pointer's byte and faults; check when that happens).
- **Branches by language** (`[0x9eb2dc]` at 0x670271): 0 English (0x670276: "upper" upper-cases the first letter with
  `toupper` 0x9471cd; "s"/"S" and the articles "a "/"an "/"A "/"An " by the first letter, tables 0x67fdc4/0x67fe04), 7
  Dutch (0x6704bd), 10 Swedish (0x6705a8), 8 Norwegian (0x670e2f; calls rx04_5's 0x6820a0), 6 Danish (0x6718c2), 2
  French (0x671de8), then Spanish (5; the verbs "seguía", "ganará", "comenzó" of the globals 0x9ed9cc..), Italian,
  German (calls rx04_5's 0x682150) and Portuguese. 23 calls to `sprintf` 0x944c9f, ~30 fixed texts.
- **The data:** language.ldb has 9,807 texts and 10 languages (0, 2..10); `scan_ldb.py` in the `rx04` scratch folder
  reads the translations and the marks they use, which are the list of cases to prove per language.

## How to go on

1. Split by language branch (one named piece per language, as gra_1), starting with Portuguese (9), the test game's.
2. Prove each piece by emulation (`gra_2b/emu.py` or `sbB\emu.py`): run the original in unicorn with each (mark
   before, mark after, gender) Portuguese uses in language.ldb, and each common piece text, and compare with the copy.
3. A simulation item, the whole routine hooked in one item; the comparer side by side on 0xb64804 (the copy into a
   buffer of its own, the original into the global).
