# Reorganisation (2026-11-07/08): proofs and inputs

The reorganisation Lucas approved on 2026-11-07 stopped the batches and tidied the repository so anyone can read it:
(1) tool fixes; (2) the `cm0102-base` crate; (3) `motor.rs` split into one file per source file of the game;
(4) readability in rounds, each fixing what an outside reader (an agent that does not know the project nor read
Portuguese) pointed out. No round changes behaviour: it changes names, comments, where a routine lives.

## The proof of each step

Each step was proved before its commit, on a bench (`engenharia-reversa/bancada.py`), with the DLL built from that step:

- `prova_*.txt`: two Rust careers to 2002-01-01 (`simular.py --repetir 2 --detector`): both equal and the garbage
  detector equal. The fingerprint alternates between two known forms (7cc445c2 and b38ccaa6) depending on how the
  compiler lays out the code (stack leftovers); under the 2026-10-31 criterion that is accepted.
- `comparar_*.txt`: comparison mode to 2002-01-01 (each rewritten routine runs next to the original); the last line is
  the number of divergences and exceptions in the log, which must be 0.

## The inputs (what the agents read and the tools applied)

| Files | What they are | Tool |
|---|---|---|
| `leitor_de_fora_rodada1..6.txt` | the outside reader's reports, round by round | — |
| `tipos_*.tsv`, `tipos2_*.tsv`, `tipos3_*.tsv` | the record or object each variable holds in each function (`file\tfunction\tvariable\tobject`) | `aplicar_tipos_variaveis.py` |
| `campos_motor/` | the fields of the match engine and of the in-match player, and the functions where `m`/`j` are something else | `aplicar_campos_motor.py`, `aplicar_tipos_variaveis.py` |
| `cabecalhos_en/` | the complete English of the headers whose detail was only in `[pt]` | `cabecalhos_completos.py` |
| `docs_en/` | the English of the other docs (`///`) that were only in Portuguese or short (key: the block's hash) | `docs_em_ingles.py` |
| `rodada5/` | the 5th typing round: variables (`tipos5_*`) and 835 proposed fields (`campos5_*`, 113 new objects) | `juntar_campos.py`, `aplicar_tipos_variaveis.py` |
| `objetos/` | the fields of 5 objects that had no module (the scheduled match, the match-day manager, Finance, Stadium, FIXTURE_MANAGER) and who holds them | `aplicar_tipos_variaveis.py` |

The tools are in `engenharia-reversa/`, and the readability steps after integrating a batch are in
[`docs/COMO_CONTINUAR.en.md`](../../docs/COMO_CONTINUAR.en.md). The scoreboard is in [`docs/LEGIBILIDADE.en.md`](../../docs/LEGIBILIDADE.en.md).
