# Routine headers in names: brief for the readers (decision (b), 11/11/2026)

The Rust port of Championship Manager 01/02 (`D:\Sistemas\CM0102\rust\src\motor`) has an English doc header above every
ported routine. Many were written when the body was still raw memory access, so they narrate the code in hex
("with the person's entry (`0x7e82e0` without creating) and a ban (`+6`)... by matches (`+0` ≠ `0xff`)"). The body
now uses names (`ban_record::MATCHES`, `exe::find_ban_entry`, `.days()`), so the header repeats in hex what the code
says by name. Your job: rewrite those headers with the names the body uses.

Input: a JSONL file, one header per line: `{"file": "discipline.rs", "fn": "apply_appeal_result", "english": "..."}`
(`english` is the text above the `Original:` line; paragraphs are separated by blank lines). For each item open
`rust/src/motor/<file>`, find `fn <fn>` and read its body (and the helpers it calls, if needed).

Rules:
- An offset (`+0x1c`, `+6`) the body reads or writes by name becomes that name, as the code writes it
  (`ban_record::MATCHES`, or the method, `.reputation()`). An exe address the body calls by name becomes that name
  (`exe::sort_entries`, or the Rust function). A global by name (`G_STAFF_TABLE`). A constant the code names
  (`bits::...`, `enums::...`, `rdata::F64_0_8`) by its name.
- Parameter names: use the Rust signature's names (`minus_games`, not the old Portuguese `menos_jogos`).
- What the body does not name stays in hex. Addresses of the original's callers ("called by `0x6abf99`") stay.
- Keep the meaning exactly: every condition, value, order and noted defect of the original. Do not add claims, do not
  summarise, do not drop details. Plain English, no hype.
- Leave the `Original:` line and everything else in the file alone. Do not edit any file except your output.

Output: a TSV at the path given, one line per header you changed: `<file>\t<fn>\t<english>`, the English on one line
with paragraphs separated by ` \n ` (space, backslash, n, space). Skip headers with nothing to rename. Finish with a
short summary in Portuguese: how many rewritten, how many left as they were, anything odd (a header that contradicts
the body is worth reporting, not fixing).
