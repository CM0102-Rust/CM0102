*Portuguese version: [FORA.md](FORA.md).*

# r7_01_1: what of the grammar modifier (0x6701f0) still runs in the original

Reader r7_01, round 7, 28/11/2026. The item ports the entry, English (0), Portuguese (9) and the languages without rules.
The other languages still run the original: the copy calls 0x6701f0 through the trampoline (`original_fn`), as
`area_layout` does.

## Why it did not become several items

The routine has no helpers of its own: everything is inline code (about 540 inline `strcmp` and 100 `strcpy` and ~170 jump
tables). The only callees are the CRT (`toupper` 0x9471cd, `tolower` 0x9465fc, `sprintf` 0x944c9f, `strstr` 0x945a60,
`_chkstk` 0x945610) and the two player type forms of rx04_5 (0x6820a0, 0x682150). A detour only fits at the entry, and a
queue item does not change another item's code. So the routine is a single item, and each missing language comes later as
a new arm of the `match` in `grammar_modifier_into`, in an ordinary batch that edits the integrated code.

## The missing branches

By the `cmp eax, N` ladder at 0x670271 (`[0x9eb2dc]`); lines of the collapsed listing (the `colapsa.py` of the r7_01
scratch folder: each inline `strcmp` becomes one line, the jump tables resolved):

| Language | Start | Collapsed lines | Notes |
|---|---|---|---|
| Dutch (7) | 0x6704bd | ~45 | |
| Swedish (10) | 0x6705a8 | ~540 | |
| Norwegian (8) | 0x670e2f | ~780 | calls 0x6820a0 (rx04_5) |
| Danish (6) | 0x6718c2 | ~270 | |
| French (2) | 0x671de8 | ~900 | and the final fix "de le " → "du " (0x67099b), which only runs in French |
| Italian (4) | 0x673777 | ~730 | |
| German (3) | 0x674ba9 | ~4,000 | the largest; calls 0x682150 (rx04_5) |
| Spanish (5) | 0x67c2e5 | ~2,000 | the verbs of the globals 0x9ed9cc.. |

All end in the common tail 0x67fd61: when the output already has text, it becomes `prefix + output + suffix`; otherwise
(0x670974), `prefix + text + suffix`. Then, only in French, the fix at 0x67099b.

## How to continue

1. One language at a time, starting with the small ones (Dutch, Danish). The model is `grammar_portuguese`: the marks and
   the forms read from the exe by address, in a table checked against the jump tables.
2. Prove it in the emulator: `check_gram.py` in the r7_01 scratch folder runs the original in unicorn against a model that
   reads the tables of `codigo.rs`; just add the language to the model and to the `for lang in ...` list.
3. In the chain, a language other than the test game's (Portuguese) is only proven by the emulator or by a comparing
   session with `game.cfg` in that language.
