# Header/body contradictions: brief for the judges (11/11/2026)

The header readers of decision (b) (brief: `BRIEF.en.md` here) reported, in `contradicoes.txt`, places where a routine's
English header and its Rust body disagree: the header calls an offset one thing, the code names it another (a field of
`rust/src/motor/fields.rs`, a method of `rust/src/motor/records.rs`), or the body reads a record through the wrong type.
The machine code is right (every routine is compared against the original, 0 divergences); only words can be wrong.
Your job: for each item of your sections, decide which side is wrong, with evidence.

Evidence, best first: the original's code (`python engenharia-reversa/desmonta.py 0xADDR` prints the disassembly of a
routine; the `Original:` line of each header gives the address; `pacotes/` may already hold a reading package),
`docs/FORMATOS.en.md`, `docs/CAMPOS.md`, `docs/MAPA_*.md`, the community map `docs/mapa_funcoes.csv`, the other uses of
the same field in the code (`grep`), and the field's own comment and confidence mark in `fields.rs`.

Verdicts, one per item:
- `header`: the header's word is wrong; give the corrected English sentence(s) for that header (the whole new header
  text, as in the readers' TSV: `<file>\t<fn>\t<english>`, paragraphs separated by ` \n `, the current header is the text
  above `Original:` in the source).
- `field_name`: the field's name is wrong; give `{module, old, new, evidence, confidence: verified|read|inferred}`. A new
  name must say what the field is, in the style of the module. Check every use of the old name (`grep -rn
  'module::OLD\|\.old_method()'`) still reads right with the new one.
- `wrong_type`: the body reads the record through the wrong type (e.g. a League wrapped as Competition); say the file,
  function, line, the right type and the right name, and the minimal code change.
- `stale`: the header describes something no longer true (e.g. "DESLIGADAS", "the copy does not run"); give the new text.
- `unclear`: you could not decide; say what is missing.

Do not edit any project file. Write your output as JSON (a list of objects: `item` (the line of contradicoes.txt),
`verdict`, `evidence`, and the fields of that verdict) to the path given, plus, for `header` and `stale` verdicts, a
TSV of the new headers next to it. Finish with a short summary in Portuguese.
