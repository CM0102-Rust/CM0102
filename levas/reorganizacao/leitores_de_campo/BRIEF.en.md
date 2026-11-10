# Field reader: brief

Project: Championship Manager 01/02 (3.9.68) rewritten in Rust, routine by routine, as a DLL loaded by the original exe.
Repo: D:\Sistemas\CM0102. READ ONLY: never edit a file of the repo; the only file you write is your output JSON.

## Task
Your census (TSV: line, offset, line text) lists each raw `+ 0x..` of one Rust file. For each line, decide which
record the base points to and name the field, or say it is not a field. Also name the **bits** of flag fields you meet
in the same functions (see below).

## Sources
- the Rust file: read the whole function around each line (where the base comes from, how the value is used);
- rust/src/motor/fields.rs: the named fields, one `pub(crate) mod <record>` per record, with comments and confidence
  marks. An offset that already has a name in the right module is `existing`;
- rust/src/motor/records.rs: the typed records (Club, Staff, ...) and their module;
- docs/CAMPOS.en.md, docs/CONFIANCA_DOS_CAMPOS.en.md, docs/FORMATOS.en.md;
- pacotes/0x<address>/: the original routine's disassembly and Ghidra pseudocode (the Rust header gives the address);
  referencias/cm0102-function-map for community names. Earlier verdicts, for style: levas/reorganizacao/leitores_de_campo/.

## Rules
- `+ 0x..` that is arithmetic, an index, a constant, a vtable slot or a date: `not_field`.
- An array element of a known field: `existing` with `NAME + k`.
- Confidence: `verified` only with concrete evidence (another routine or the save format gives a clear meaning, a
  community name, an exe string); `read` when this code's use makes it clear; `inferred` for a guess. A wrong name is
  worse than none: if you cannot tell, `unknown`.
- Names: English UPPER_SNAKE_CASE for the meaning (not the type), consistent with the module's style. The same
  (module, offset) gets the same name everywhere. Check that the width you give matches the reads/writes.
- A record that has no module yet: propose one (`new_module: true`, and `record`: one line describing it).

## Bits
When a function tests or sets bits of a flag field (`read_u8(x + board_record::FLAGS) & 0x20`, `| 0x40`, `& !0x8`,
`^ 0x80`), give one entry per bit value and line, only when the meaning is clear from the code or the evidence:
{"kind": "bit", "line": N, "value": "0x40", "field": "board_record::EXPECTATION_FLAGS", "name": "AVOID_RELEGATION",
 "confidence": "verified|read|inferred", "evidence": "..."}
A byte access to a wider field (`EXPECTATION_FLAGS + 1` with `& 4`) names the bit of the byte (value as written).

## Lessons from the earlier files
- `name` is only the field name (`HOME_CLUB` or `NAME + 4`); never a variable term (`NAME + 4 * k`): if the line adds a
  variable index, name only the constant part.
- When the base is inside the record (`x = row + 0x1c; x + 0x1a`), give the field's own offset in `field_offset`.
- A bit entry's `value` is one number and `name` one UPPER_SNAKE name; a byte access keeps `+ k` in `field`
  (`board_record::EXPECTATION_FLAGS + 1`). Masks of several bits get their own name only when the code uses them so.
- Offsets written in decimal in the code (`ln() + 6`) go with `literal` (the exact text) - only if the census missed them.

## Output
A JSON array, one object per census line (plus the bit entries):
{"line": 148, "offset": "0x38", "base": "p", "module": "stadium", "verdict": "new|existing|not_field|unknown",
 "name": "CITY", "width": 4, "rust_type": "usize", "description": "short English description",
 "confidence": "verified|read|inferred", "evidence": "one line: why"}
Plain ASCII. Finish with a short summary in Portuguese: counts per verdict and confidence, new modules, doubts.
