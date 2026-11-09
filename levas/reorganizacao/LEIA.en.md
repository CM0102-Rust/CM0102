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

## Round 29: typed records pilot

`engenharia-reversa/registros.py` generates `rust/src/motor/records.rs`: for each object in `fields.rs` (162), a
`#[repr(transparent)]` pointer type (`Club(usize)`, the same ABI as the raw address) with one getter and one setter per
field (1,576 typed), in the type the code already uses at that field (what round 28 checked against the original).
Fields pointing to another record return its type (`club.stadium()` is a `Stadium`; the `POINTERS` table). Pilot on
one function, `season_end_club_update` (fin_prm.rs, the stadium works): raw reads and writes 62 → 20, `+ 0x` offsets
11 → 1, pointer casts 10 → 2, right names (`cash` instead of `goal_difference`; `'works`); the friction: 39 `.addr()`
to pass the club to helpers that still take `usize` (they go when those take `Club`). Proof: 7cc445c2 ×2, detector
equal, comparison 0.

## Round 28: the automatic checker of signatures and field widths

Round 27 only caught the functions declared in different ways; one declared wrong in the same way everywhere slipped
through. Two tools now check everything against the original machine code:

- `engenharia-reversa/conferir_assinaturas.py`: every exe function the Rust code calls (1,435, counting the wrappers,
  the local declarations and the fixed addresses passed to helpers) against the routine and its callers in the
  original: the stack (`ret N` or the caller's `add esp`), ecx read before written (`thiscall`) and the return width
  (what the routine writes last before each `ret` and what the callers read: `al`, `ax`, `eax`, `edx`, `st0`).
  Validated against the 53 of round 27: it flags none of the true ones and catches every wrong one that was a real bug.
  It found 31 functions returning only `al`/`ax` declared `u32`: narrowed by `estreitar_retorno.py` (signed where the
  original callers use `movsx`), each existing call keeping the old type with an `as`. 24 ambiguous suspects were
  checked by agents: all harmless (return ignored or truncated), recorded as `accepted_sites` in
  `assinaturas_conferidas.json`. Result: 0 bugs. It runs on every batch in `publicar_leva.py` and stops the publication
  on a bug (exit code 1); the full list goes to `conferencia_assinaturas.tsv`.
- `engenharia-reversa/larguras_campos.py --original`: every access to a named field inside a ported routine against
  the widths the original routine uses at that displacement. Of the 8,964 checkable accesses, 8,886 match the original
  instruction; the other 78 were checked by agents (55 collisions with another object at the same displacement, 14
  fields merged into a wider or narrower access, 9 address only), none wrong (`larguras_conferidas.json`). Without
  `--original`, the list of fields read with different widths across uses (`larguras_campos.tsv`), for review. Known
  limit: some routine sizes in the community map are short (0x6c6a30, 0x6e2f20, 0x6ec010, 0x736720) and the matching
  access lies past the stated end.

Calls through pointers that only exist at run time (the game's tables and vtables) stay out: they cannot be checked
statically.

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
| `conferencia_assinaturas.tsv`, `larguras_no_original.tsv`, `larguras_conferidas.json`, `larguras_campos.tsv` | the output of the signature checker and of the width checker, and the verdicts on the 78 widths | `conferir_assinaturas.py`, `larguras_campos.py` |
| `assinaturas_conferidas.json` | the true signature of 53 exe functions, checked against the disassembly (with the evidence) | `involucros_exe.py` |
| `objetos/` | the fields of 5 objects that had no module (the scheduled match, the match-day manager, Finance, Stadium, FIXTURE_MANAGER) and who holds them | `aplicar_tipos_variaveis.py` |

The tools are in `engenharia-reversa/`, and the readability steps after integrating a batch are in
[`docs/COMO_CONTINUAR.en.md`](../../docs/COMO_CONTINUAR.en.md). The scoreboard is in [`docs/LEGIBILIDADE.en.md`](../../docs/LEGIBILIDADE.en.md).
