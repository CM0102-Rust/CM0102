# Reorganisation (2026-11-07/08): proofs and inputs

The reorganisation Lucas approved on 2026-11-07 stopped the batches and tidied the repository so anyone can read it:
(1) tool fixes; (2) the `cm0102-base` crate; (3) `motor.rs` split into one file per source file of the game;
(4) readability in rounds, each fixing what an outside reader (an agent that does not know the project nor read
Portuguese) pointed out. No round changes behaviour: it changes names, comments, where a routine lives. The exception is
round 27 (09/11/2026, checked signatures, below), which fixes wrong reads of the return value of exe functions.

## Round 27: signatures checked against the disassembly

53 exe functions were declared with different signatures in different places (calling convention, argument count,
float vs integer, return width). Three agents disassembled each one and its callers in the original exe (`ret N`, use
of ecx, return register) and wrote the true one in `assinaturas_conferidas.json` (with the evidence);
`involucros_exe.py` uses that signature for the `exe::` wrapper and converts the call sites to it.

- **Fixed (real bugs):** call sites that read the whole of eax from functions that only return `al` or `ax` (the upper
  bits are garbage): `transfer_manager_collect_*_news_recipients`, `match_player_decide_ball_action`,
  `injury_get_condition`, `human_manager_get_commentary_speed`, `database_get_staff_age`,
  `database_get_contract_rating_modifier`, `hostcountry_find_hosts`, `comp_util_get_fixture_venue_side` and others; each
  site now reads the right width and extends it.
- **Converted for clarity, same result:** `finance_get_transfer_budget` and `match_player_get_instructions` return 64 bits
  (edx:eax; the sites only read the low half); `formation_set_player_instruction` takes a `u64` (it used to be split into
  two `u32`).
- **Left as they are, harmless:** the `stdcall` sites of routines the original calls as `thiscall` but that never read
  ecx (the stack is the same); the extra arguments to `utils_get_version_string` (cdecl, the caller cleans up); `get_stat`
  read as `f32` in match_man's `instructions` (the original stores it as a dword between the calls; going through f64
  would round twice).
- **Mutations:** round 25 had left 48 mutations naming, in their replacement text, a local alias that no longer exists
  (`let _ = alias;`, which would not compile); `mutacoes_sem_apelidos.py` drops those names, and each changed mutation
  was compiled one by one.

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
| `leitor_de_fora_rodada1..7.txt` | the outside reader's reports, round by round | — |
| `tipos_*.tsv`, `tipos2_*.tsv`, `tipos3_*.tsv` | the record or object each variable holds in each function (`file\tfunction\tvariable\tobject`) | `aplicar_tipos_variaveis.py` |
| `campos_motor/` | the fields of the match engine and of the in-match player, and the functions where `m`/`j` are something else | `aplicar_campos_motor.py`, `aplicar_tipos_variaveis.py` |
| `cabecalhos_en/` | the complete English of the headers whose detail was only in `[pt]` | `cabecalhos_completos.py` |
| `docs_en/` | the English of the other docs (`///`) that were only in Portuguese or short (key: the block's hash) | `docs_em_ingles.py` |
| `rodada5/` | the 5th typing round: variables (`tipos5_*`) and 835 proposed fields (`campos5_*`, 113 new objects) | `juntar_campos.py`, `aplicar_tipos_variaveis.py` |
| `rodada6/` | the 6th round (motor.rs, managers, match, comparers): `tipos6_*`, `campos6_*` | `juntar_campos.py`, `aplicar_tipos_variaveis.py` |
| `comentarios5/` | the English of 873 body comments | `aplicar_comentarios.py` |
| `assinaturas_conferidas.json` | the true signature of 53 exe functions, checked against the disassembly (with the evidence) | `involucros_exe.py` |
| `objetos/` | the fields of 5 objects that had no module (the scheduled match, the match-day manager, Finance, Stadium, FIXTURE_MANAGER) and who holds them | `aplicar_tipos_variaveis.py` |

The tools are in `engenharia-reversa/`, and the readability steps after integrating a batch are in
[`docs/COMO_CONTINUAR.en.md`](../../docs/COMO_CONTINUAR.en.md). The scoreboard is in [`docs/LEGIBILIDADE.en.md`](../../docs/LEGIBILIDADE.en.md).
