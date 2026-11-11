# Converting the queue to the current style: brief for the converters (11/11/2026)

The CM 01/02 Rust port (`D:\Sistemas\CM0102`) keeps a queue of items written before the 07/11 reorganization
(`levas/fila/<item>/`: `codigo.rs`, `rotinas.py`, `mutacoes.py`; the format is in the header of
`engenharia-reversa/fila.py`). Since then the engine code (`rust/src/motor/`) became readable: English names and
headers, named fields and typed records, `exe::` wrappers, `covered!` test markers. The queue items still use the old
style (Portuguese names, comments and headers, item prefixes like `cqu1_`, `transmute` of raw addresses, raw offsets,
`crate::comparison::branch`). Your job: rewrite your items in the current style **without changing what they do**.

Read first: `docs/COMO_ESCREVER_UM_ITEM.en.md` (the rules, especially "Names and header" and the typed-records style),
the header of `rust/src/motor.rs` (how to read the code), and one or two current files of the same kind as your items
for the style (e.g. `rust/src/motor/comp/leagues/bra_reg_rio.rs`, `rust/src/motor/comp/cup.rs`). The names live in
`rust/src/motor/fields.rs` (fields, by record module), `records.rs` (typed records and their accessor methods),
`exe_calls.rs` (`exe::` wrappers of exe functions), `exe_fns.rs` (`FN_*`), `globals.rs` (`G_*`), `keys.rs` (`KEY_*`),
`rdata_consts.rs` (`rdata::F64_*`), `bits.rs`, `enums.rs`, `vtables.rs`, `match_codes.rs`, `events.rs`, `mem.rs`.

For each item:
- `codigo.rs`: English function, helper, local and constant names (the routine named after the original method, see
  `docs/NOMES.csv` and `docs/mapa_funcoes.csv`; no item prefix; the comparer `compare_<name>`); an English header on
  every routine (what it does in full, fields and functions by name, then `/// Original: \`Class::method\` (0xADDR,
  file.cpp, convention).`; no `[pt]`, no Portuguese); typed records and accessors where they exist, `exe::name(..)`
  instead of a `transmute` of an address when the wrapper exists, `read_T`/`write_T` for what stays raw, named fields
  instead of raw offsets (only names that exist; for a field that has no name yet, keep the raw offset and list it in
  `campos_novos.json` next to the item, objects `{module, offset, name, width, description, confidence:
  verified|read|inferred, evidence}`), `covered!("<item> <english label>")` instead of `crate::comparison::branch`;
  English log and comparer messages. Keep `%%I0%%`-style index placeholders and `%%LEVA%%` exactly as they are.
- `rotinas.py`: the first field of each `ROTINAS` entry becomes the new Rust function name (addresses and prologue
  bytes unchanged); the `DOCS` stay in Portuguese and English, with the new names; comments may stay as they are.
- `mutacoes.py`: each mutation's anchor and replacement rewritten to the new text, so the anchor appears exactly once
  in the new `codigo.rs` and the replacement makes the same change as before; the fourth field (branch label), if
  present, matches the `covered!` label.
- **Same behaviour, byte for byte where it matters:** the same offsets, widths and signedness of every read and write
  (an accessor counts only if its width and type match: check its definition in `records.rs`), the same calling
  conventions and argument order, the same order of effects, the same constants. When unsure, keep the raw form.
- Add one line to the item's `PRONTO.txt`: `convertido para o estilo atual em 11/11/2026`.

You cannot build here; read carefully instead, and check each name you use exists (grep `fields.rs`, `records.rs`,
`exe_calls.rs`). Edit only your items' files under `levas/fila/` (and create their `campos_novos.json`); touch nothing
else. Finish with a short report in Portuguese: per item, the new routine names, the fields left raw, and anything you
were unsure about.
