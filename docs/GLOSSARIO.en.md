*English version of [GLOSSARIO.md](GLOSSARIO.md). Both are kept in sync; the Portuguese file is the original.*

# Glossary: Portuguese names in the code

Since 7 Nov 2026 the hooked routines have English names in the code (the original method name from the maxevans60 map); the helpers and the tools still have Portuguese names, being renamed in rounds. This glossary links each code name to the original name and to the old Portuguese name (`docs/NOMES.csv`), which the batch documentation still uses. Addresses are VAs in `cm0102.exe` 3.9.68.

## Common words

| Português | English |
|---|---|
| motor | engine (the match engine object) |
| jogador | player |
| pessoa | person (the player's database record) |
| ficha | record / sheet |
| lado | side (0 or 1) |
| bloco | block (20 players of a side) |
| casa | square (grid cell) |
| grade | grid (pitch, 9 × 12) |
| papel | role (tactical) |
| lance | move / play |
| bola | ball |
| chute | shot |
| passe | pass |
| falta | foul |
| cartão | card |
| impedimento | offside |
| goleiro | goalkeeper |
| dono | owner (of the ball) |
| posse | possession |
| desgaste | fatigue |
| condição | condition |
| formação | formation |
| narração / comentário | commentary |
| sorteio | random draw |
| leva | batch |
| rotina | routine |
| desvio | hook / detour |
| autoteste | self-test |
| mutação | mutation |
| divergência | divergence |
| impressão digital | fingerprint |
| carreira | career |
| ler / gravar | read / write |
| escolher | choose |
| somar | add |
| zerar | clear (set to zero) |
| registrar | record |
| guardado | stored |
| tabela | table |
| faixa | band / range |
| limite / limiar | limit / threshold |

## Match engine routines (`rust/src/motor.rs`)

| Address | Name in the code | Original / meaning | Old Portuguese name |
|---|---|---|---|

## Random generator and dates

| Address | Português | English |
|---|---|---|
| 0x90cfc0 | `aleatorio(n)` | random(n) |
| 0x54a110 | `Data()` | Date() |
| 0x549ef0 | `Data(dia, ano)` | Date(day, year) |
| 0x549f70 | `Data(dia_do_mes, mes, ano, dia_da_semana)` | Date(day_of_month, month, year, weekday) |
| 0x54a3f0 | `dia_da_semana()` | weekday() |
| 0x54a2d0 | `ajustar_semana(alvo, dest)` | adjust_week(target, dest) |
| 0x54c770 | `mais_semanas(n)` | plus_weeks(n) |

## Helpers in `motor.rs`

| Português | English | What it is |
|---|---|---|
| `jogador_do_bloco` | block_player | player k of the side's 20-player block |
| `casa_da_grade` | grid_square | pitch grid square (x, y) |
| `sortear` | draw | calls the game's random draw (0x90cfc0) |
| `ftol` | ftol | float → integer truncation (_ftol, 0x945640) |
| `f32_em / f64_em` | f32_at / f64_at | float/double constant read from .rdata |
| `byte` | byte | signed byte at address + offset |
| `lado` | side | player's side (byte 0x27) |
| `degrau` | step | ladder of multipliers by attribute |
| `pow_do_jogo` | game_pow | power via the game's CRT (0x945ff0) |
| `comparar_*` | compare_* | runs original and Rust side by side and records divergences |
| `comentar / porta_6d6f70` | comment / gate_6d6f70 | commentary and the gate that records the calls |
| `PLACAR` | SCOREBOARD | counter of comparisons and divergences |

## Modules and tools

| File | English | What it is |
|---|---|---|
| `video.rs` | video | own DirectDraw |
| `apresentacao.rs` | presentation | frame → window, mouse |
| `iat_hooks.rs` | hooks (IAT) | import table hooks |
| `detours.rs` | detours | detours in the exe, private stack |
| `simulation.rs` | simulation | reproducible simulation |
| `allocator.rs` | allocator | fixed allocator |
| `rng.rs` | random | the game's random generator |
| `date.rs` | date | dates (Date.cpp) |
| `motor.rs` | engine | match engine |
| `spy.rs` | spy | diagnostics (CM_ESPIAO=1) |
| `comparison.rs` | comparison | side-by-side comparison |
| `registro.rs` | log | logging to ddraw.log |
| `simular.py` | simulate | headless automatic career |
| `laboratorio.py` | laboratory | cross-session lock and memory watchdog |
| `desmonta.py` | disassemble | disassembly |
| `desvios.py` | detours | where a hook can go |
| `escuta_tela.py` | screen listener | listens to what the screen draws |
| `patch_portatil.py` | portable patch | generates the no-CD exe |
| `resolucao.py` | resolution | generates the 1920×1080 exe |
| `cobertura.py` | coverage | per-function call counter |
| `candidatos.py` | candidates | next routines |
| `mutacoes.py` | mutations | self-test with mutations |
| `prologos.py` | prologues | checks the hooked bytes |
| `textos.py / refs.py` | strings / refs | exe strings and who uses them |
