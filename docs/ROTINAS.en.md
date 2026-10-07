*English version of [ROTINAS.md](ROTINAS.md). Both are kept in sync; the Portuguese file is the original.*

# Routines migrated to Rust

Each routine enters through a hook (`rust/src/desvios.rs`) and only becomes official after:
1. self-test (`CM_AUTOTESTE=1`): synthetic inputs as soon as the game opens the video, original × Rust, comparing the result and the memory touched;
2. control mutation: a deliberate defect in the Rust version must be caught;
3. real game in comparison mode (`CM_COMPARAR=1`): automatic on-holiday career (`docs/VERSAO_SEM_TELA.md`) or a match played by Lucas;
4. full career: `simular.py` with `--modo rust` and `--modo original` (same DLL, same seed) must give the same fingerprint.

## Modules

### Random draw — `rust/src/aleatorio.rs` (official on 2026-10-06)

| Routine | Original | Convention |
|---|---|---|
| `aleatorio(n)` | 0x90cfc0 | cdecl |

Self-test: 5,000,000 cases, 0 divergences. Mutations caught: +4 in the large-n branch; table wrap-around. Real game: >100 million calls, 0 divergences.

### Dates — `si/code/Date.cpp` → `rust/src/data.rs` (official on 2026-10-06)

| Routine | Original | Convention |
|---|---|---|
| `Data()` | 0x54a110 | thiscall |
| `Data(dia, ano)` | 0x549ef0 | thiscall |
| `Data(dia_do_mes, mes, ano, dia_da_semana)` | 0x549f70 | thiscall, ret 0x10 |
| `dia_da_semana()` | 0x54a3f0 | thiscall |
| `ajustar_semana(alvo, dest)` | 0x54a2d0 | thiscall |
| `operator=(src)` → copy | 0x417c10 | thiscall, return by value |
| `operator+(n dias)` | 0x54c5b0 | thiscall, return by value, local static |
| `operator-(n dias)` | 0x54c8e0 | thiscall, return by value, local static |
| `mais_semanas(n)` | 0x54c770 | thiscall, return by value, local static |

Self-test: 9,446,717 cases (every 16-bit year in the constructors and in the day of the week), 0 divergences. Mutations caught: leap year `% 200`; previous leap year adding 365. Real game: >10 million calls, 0 divergences.

Quirks preserved:
- `dia_da_semana` (day of the week) returns −1 for 1600, day 0, leap year (negative remainder).
- `operator+`/`operator-` with n = −32768: the 16-bit negation returns −32768 and the two call each other until the stack overflows (same in the original; the game never asks for that). Left out of the self-test.
- Each operator initializes its static `Data` on the first call and registers an empty destructor with `atexit`; the Rust version does the same (that is why the self-tests only run after the game's CRT is ready).

Pending in `Date.cpp`: the larger routines (0x54a130, 0x54a470, 0x54a650, 0x54a950, 0x54ae10, 0x54b2f0, 0x54b550, 0x54ba20, 0x54bef0, 0x54c3b0), which format dates as text.

## Choosing the next targets

`engenharia-reversa/alvos.py` (small, heavily called leaf functions) and `docs/MAPA_MODULOS.md`.

### Match engine, first batch — `match_eng.cpp`/`match_pl.cpp` → `rust/src/motor.rs` (official on 2026-10-06)

Chosen by coverage (`cobertura.py` + `simular.py --cobertura`, career with the engine on until 2001-10-01): the most-called leaves. They only read engine tables.

| Routine | Original | Convention | Calls in 2 months |
|---|---|---|---|
| `tabela_a(jogador)`: word from the side's table, by position 0..=10 | 0x6c38c0 | thiscall, ret 4, returns `ax` | 870 million |
| `tabela_b(jogador)`: same, another table | 0x6c38f0 | thiscall, ret 4, `ax` | 18 million |
| `diferenca_lado(lado, v)` | 0x6c4180 | thiscall, ret 8, `al` | 9 million |
| `faixa(lado, b, c)` | 0x6cb960 | stdcall, ret 0xc, `al` | 8 million |
| `por_diferenca(a, b, c, d)`: global table 0xb67c30 | 0x6cb6c0 | thiscall, ret 0x10, `ax` | 6 million |
| `limite(lado, v)` | 0x6cb930 | stdcall, ret 8, `al` | 4 million |
| `condicao_jogador()` | 0x70dfa0 (match_pl.cpp) | thiscall, `al` | 21 million |

Partial return: the originals only set `al`/`ax`; the high bits of `eax` are garbage (in 0x6c4180, `sub eax, ecx` mixes in the `this`). The Rust version reproduces the part the type defines; the career fingerprint would catch a caller that used the rest.

Self-test: 14,000,000 cases (fake engine object and player, positions and sides out of range), 0 divergences. Mutation caught: `v > 8` → `v >= 8` in `limite` (divergence at `limite(0, 8)`). Real game, engine on, comparison: ~950 million calls, 0 divergences. Career until 2001-10-01: Rust = original (597,180,435 random draws, same fingerprint).

### Match engine, second and third batches — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `tabela_do_jogador()` | 0x717410 | thiscall | `tabela_a` of the player's engine (0x19e), called through the game's address |
| `diferenca_jogador(j)` | 0x6c4130 | thiscall, ret 4, `al` | 0 if `j` is the one stored at 0xf582 |
| `byte_do_lado(j)` | 0x6cfda0 | thiscall, ret 4, `al` | |
| `limite_jogador()` | 0x70df40 | thiscall, `al` | |
| `faixa_jogador(lado)` | 0x6f5b20 | thiscall, ret 4, full `eax` | |
| `byte_da_posicao(i)` | 0x5b57c0 | thiscall, ret 4, `al` | `[this + 0x5b4 + i]` |
| `campo_pela_posicao()` | 0x6f4600 | thiscall, `al` | the side's block in the engine: 0x91e2 + 6371 × side |
| `pode_escolher(j, b, c)` | 0x6bce20 | thiscall, ret 0xc, full `eax` | |
| `apontar(a, b, c, d)` | 0x6f4920 | thiscall, ret 0x10 | direction in degrees (0x19a, 0..=359); **draws random numbers** (360, 120, 30, 30) and stores |

0x714d30 (`mov al, [ecx+0x27]; ret`, 4 bytes) has no room for a hook; the Rust versions read the field directly.

Routines that draw random numbers: in the comparison, original and Rust start from the same generator state (`aleatorio::REGIOES_DO_ESTADO` in `lado_a_lado`), the replica's draws are not counted (`aleatorio::sem_contar`) and the game continues with the original's state.

Self-tests: 37 and 51 million cases, 0 divergences. Mutations caught (11): return without engine; swapped field; swapped side bytes; `<`/`<=` thresholds and ranges; table index; position-mark bit; `0x2b >= 2`; fixed angle 180 → 181; `<` → `<=` in the draw threshold; order of the two ±30 draws. Real game in comparison: >1 billion calls, 0 divergences. Career with the engine on until 2002-06-01: Rust = original (2,340,778,192 random draws), two identical Rust runs.

### Match engine, fourth batch: floating point — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `desgastar(x, codigo)` | 0x6f56c0 | thiscall, ret 8 | player's float at 0x4d, never below zero. Code −1: `x` × (12, 10, 8 or 6 by the value's range: above 9500, 8750, 7750) × 1/11. Others: (200 − 8.5 × float at 0xd5) × code factor (19 and 33: 0.3818; 21, 36 and 46: 0.5454; others: 0.1636) |
| `virado_para(a, b)` | 0x6fa140 | thiscall, ret 8, `al` | direction (0x19a) against the angle from table 0xb67c30, tolerance `_ftol`(0x10c × 2.25 + 45) |

Floating point: the game's x87 runs in double precision (Windows default), so each operation rounds like `f64` in SSE2. What is reproduced is where the original rounds to `float`: `fstp`+`fld` compares the already-rounded value, `fst` followed by a comparison compares the 53-bit one. The constants are read from the game's `.rdata`. `_ftol` (0x945640) = truncation to `i64`.

Self-test: 55,479,770 cases, 0 divergences. Mutations caught: range `>` → `>=`; factor for code 46; **product computed in `float` instead of `double` (one-bit difference)**; tolerance `<` → `<=`; wrap-around `360 − d` → `359 − d`. Two uncaught mutations were equivalent (comparing with zero before or after rounding only differs below 10⁻⁴⁵; `> 180` and `>= 180` give the same at 180) and were replaced. Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust = original (2,340,778,192 random draws).

### Match engine, fifth batch — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `procurar_no_lado(lado, grupo, a3..a6, modo)` | 0x6cd330 | thiscall, ret 0x1c, `al` | walks the side's players (count at grupo[0x58 + side], pointers at grupo + 44 × side); mode 2 only counts; otherwise asks 0x6f5c30 (through the game's address, arguments in the order a4, a5, a3, a6) and on the first yes zeroes 0x1a5/0x1a6 and returns 1 |
| `alcanca(j, b, c)` | 0x6cb9b0 | stdcall, ret 0xc, `al` | coordinate ranges; at the end, distance table (0xa15e44, 12 × 9, ≈ √(r² + c²), symmetric) against 1.5 |
| `classificar_lance(codigo, a, b, c, d, forca, saida)` | 0x713e90 | thiscall, ret 0x1c, `al` | ignores 11 codes (returns 2); angle against the tolerance `_ftol`(90 − (18 × forca − float at 0xe5)); outside it stores (20 − attribute) × 12 in `saida` |

**Record and replay** (routines that call another one with side effects): 0x6cd330 calls 0x6f5c30 (3.9 KB, draws random numbers and alters players). In the comparison, a gate at 0x6f5c30 records each call made by the original (arguments and answer); the Rust copy receives the recording in place of the routine, and the call sequence is compared too. The copy must see the state the original saw: the bytes the loop reads (count, 0x19 and 0x2b of each player) are included in the regions restored by `lado_a_lado`; the compared memory is only what the routine writes. In the self-test the gate answers synthetically (fake players would bring down the real routine).

Self-test: 63,974,778 cases, 0 divergences. Mutations caught (7): active player `<= 1` → `< 1`; order of the forwarded arguments (the recording catches it); write to 0x1a5; `x == 4` → `x == 5`; attribute 0x3b → 0x30; sign in the tolerance; one code from the list. Equivalent (2): threshold `>` → `>=` 1.5 and table row/column index (symmetric, and at reachable positions the column is ≥ 2, so the value is ≥ 2: the original's final check is always true). Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws).

### Match engine, sixth batch: distances — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `distancia_ao_ponto(a, b)` | 0x6f09b0 | thiscall, ret 8, `float` in st0 | distance table: row \|y − b\|, column \|x − a\| |
| `distancia_ao_gol()` | 0x6fa1e0 | thiscall, `float` in st0 | to (4, 11) for side 0, (4, 0) for the other |
| `mais_perto_do_gol(j)` | 0x6cb780 | thiscall, ret 4, `al` | 0: blocked (0x70dfa0, square marked in the grid engine + 0x21b6 + opposite side + 90 × (y + 12x)) or active opponent at distance ≤ limit; 2: active teammate at distance < limit; otherwise 1. Limit: his distance to goal + 0.5 (+ 1.0 outside the box), rounded to `float` at each step. Blocks of 20 players per side at engine + 0x4796 + 8920 × side, every 0x1be |

0x6f4650 (in the box) calls 0x6f9f90, which draws random numbers and calls 0x6cbc00 (1.1 KB): left for the 0x6f9f90 batch. In the comparison of 0x6cb780, calls to it go through record and replay (gate at 0x6f4650; the recording now stores which routine was called).

Self-test: 97,418,758 cases, 0 divergences. Mutations caught (6): swapped axes in the distance to a point; goal line 11 → 10; margin 0.5 → 1.0; square without the side offset; swapped opponent block; return 2 → 1. Equivalent (2): `>` ↔ `>=` against the limit (at reachable positions no pair in the table differs by exactly 0.5 or 1.5). Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws).

### Match engine, seventh batch: passing line — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `jogadores_na_linha(x1, y1, x2, y2)` | 0x6cbc00 | thiscall, ret 0x10, `al` | counts the players in the squares of the line between the two points (9 × 12 grid, groups at engine + 0x215e + 90 × (y + 12x)), walked Bresenham-style: vertical and horizontal from smallest to largest without the last square; diagonals with the endpoints, starting from the one with smaller x (shallow) or smaller y (steep), error subtracted before the square and added after. Each square via `procurar_no_lado` in mode 2 (only counts; the forwarded float is not used). The original reuses its own arguments on the stack as variables |
| `consegue_jogar(a, b, calcular, forca, ajuste, saida)` | 0x6f9f90 | thiscall, ret 0x18, `al` | 1 if already facing the target; tolerance 75° or ftol(0x10b × 4 or 5 + 10 × forca + 60 or 40) − ftol((distance − ajuste) × 20 − 50), clamped to 30..135°; with `saida`, stores the players on the line (0x6cbc00) and **draws a random number**: 1 if 0x10b ≥ draw(c²) |
| `na_area()` | 0x6f4650 | thiscall, `al` | goal line and columns 3..5; otherwise 0x6f9f90 facing the middle of the goal, with no calculation and no output (does not draw) |

With this, 0x6f4650 no longer needs the recording gate (it was pure on the path in use).

Self-test: 130,887,068 cases, 0 divergences. Mutations caught (8): last square included in the vertical; `>=` → `>` in the error; reversed step; limit 135 → 134; tolerance 75 → 74; `>=` → `>` in the draw; sign of the adjustment; columns 3..5 → 3..4. Equivalent (1): step sign with `>= 0` → `> 0` (only differs with zero vertical difference, which never reaches the diagonal). Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws).

### Match engine, eighth batch: tactical positioning — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `marcas_da_posicao(posicao)` | 0x5b5270 | thiscall, ret 4, `u64` in edx:eax | the position's tactical instructions: `[this + 0x188b + 8 × position]` |
| `papel_no_ataque()` | 0x716270 | thiscall, `al` | the player's marks (block 0x91e2 + 6371 × side): 1 (0x20000); 0 if `tabela_a` = 1; 2 (0x40000000); 0 |
| `papel_na_defesa()` | 0x716310 | thiscall, `al` | 0 if `tabela_a` = 1; 3 (0x80000000); 4 (0x80000); 0 |
| `posicionar(j, saida_x, saida_y, _)` | 0x6c2be0 | thiscall, ret 0x10 | where the player should be. "Attacking": his team has possession, the player stored at 0xf57a is on his side, or ball at the edge of the opponent's box with 0x8ea9 ≠ −1. Square from the side's formation table (`[motor + 0xf56e + 4 × lado]` + 12 × (9 × (attacking + 2 × position) + ball_x) + ball_y; x in the low nibble, y in the high one), ball clamped to 9 × 12. With 0x8eb1 > 0: current position if the role says to stay, everything clamped to the grid. Otherwise (unless `tabela_do_jogador` = 1): the side's marks at 0x9766 (0x1000/0x10000/0x8000: one step, with limits) and 0x4000 (follow the ball's line: ±2 when stationary, or chained ranges with 9/2 and ±2/±4) |

Self-test: 172,911,600 cases, 0 divergences (formation tables with a margin, because the index becomes negative with position < 0). Mutations caught (10): offset of the marks; attacking-role bit; defending-role bit; 0x8ea9 ≠ −1 → ≠ 0; ball x limit; step direction; +2 → +1 on the line; −2 → −1; "stationary" on the wrong side; output x copying y. Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws).

### Match engine, ninth batch: defensive lines — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `linhas_defensivas()` | 0x6c3aa0 | thiscall | offside: side 1, largest y (0x103) among the active ones starting from 6, plus 1, at 0x8eaf; side 0, smallest y starting from 5, minus 1, at 0x8eb0 (the fields read by `diferenca_lado` and `diferenca_jogador`). Active: position >= 0 and `tabela_do_jogador` != 1. The original has both 20-player loops unrolled (1.7 KB) |

Self-test: 581,108,689 comparisons (each call checks 40 players, also compared), 0 divergences. Mutations caught (4): start 6 → 7; +1 removed; start 5 → 4; active filter. Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws).

### Match engine, tenth batch: player update on every move — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `atualizar_jogador()` | 0x6f4aa0 | thiscall | 2.8 KB, 908 instructions. Counter 0x2b (decreases down to 0); float 0x1a2 (× 0.85, floor 2 × (25 − the person's attribute 0x5d), ceiling 100); word 0x35 (disposition, ~4,000 to ~8,750): ranges by the attributes and, by the tactical role (bits 1, 2, 4, 8, 0x10, 0x20, 0x40 of `tabela_a`; "strong" = bits 0x880 of the whole word), draws from 1/75 to 1/1000, floating-point ratios between attributes and the side's state (0xf5bd/0xf5f3). Transcribed block by block, with the original's addresses in the comments; the shared endings became local functions |

Self-test: 588,775,364 cases, 0 divergences. Mutations caught (12): floor, ceiling, threshold 7350, `[0x1f] > 30`, range (b + 14) × 500, division by 11, ratio `a4 > 4`, "strong" factor, base (29 − v) × 250, `v == 1`, `−5v`, `+5c`. Four of them were only caught after improving the case generator (fields zeroed with probability 1/2, full range of values, word 0x35 exactly at the thresholds): they were coverage gaps, not equivalent mutations. Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws).

### Match engine, 11th and 12th batches (bottom-up) — `rust/src/motor.rs` (official on 2026-10-06)

From here on the order is bottom-up: first the routines that only call routines already ported (the large integrating routines, such as 0x7109f0 with 13 KB and 46 calls, wait until what is underneath is ready, so that the comparison is safe).

| Routine | Original | Convention | Note |
|---|---|---|---|
| `sortear_momento()` | 0x6f44a0 | thiscall | the moment's attributes (0x104 to 0x10d), several "best of two draws"; 15 draws in the original's order |
| `manter_posicao_um(lado)` | 0x6ce110 | thiscall, ret 4, pointer | the position-1 player of each side (0xf59e, 0xf5a2): zeroes it if lost, looks for whoever has `tabela` 1, otherwise promotes the first active one by swapping with the substitute (0x1a, position −2, 0x20 = −1) |
| `caminho_livre(j, ignorar_casa)` | 0x6bd5b0 | thiscall, ret 8, `al` | 0 if his square is marked or if some active opponent ahead is in the same column |
| `espaco_a_frente(j)` | 0x6cbad0 | thiscall, ret 4, `al` | free square ahead (diagonals first) in the opposite side's grid |
| `fora_do_meio(v)` | 0x6cb910 | stdcall, ret 4, `al` | 1 outside 2..=6 |
| `alem_da_linha(y, lado, sem_guardado)` | 0x6cb700 | thiscall, ret 0xc, `al` | how many active players of the side are beyond line y |
| `distancia_entre(a, b, c, d)` | 0x6cc4e0 | stdcall, ret 0x10, `float` | distance table, 1000 outside |
| match state readers and writers | 0x700320..0x700410 | thiscall | ball (0x8ea7/0x8ea8), 0x8ea9, 0x8eaa, 0x8eb2, 0x8ed8, 0x8f7a |
| `registrar_posse(lado)` | 0x7003b0 | thiscall, ret 4 | 0x8eab = side; except −1, +1 on the side's counter (0x8d46 + side) and −10 on the other's, if positive |
| `na_lista(alvo)` | 0x714c60 | thiscall, ret 4, `al` | pointer in list 0x1ae (count at 0x101) |
| `guardar_extremos()` | 0x6cd660 | thiscall | players with the smallest and largest y on each side (0xf58e, 0xf596, 0xf592, 0xf59a); with no candidate the previous value stays |

11th batch (7 routines): self-test 667,013,079 cases, 10 mutations caught, >1 billion comparisons, career Rust = original. 12th batch (14 routines): self-test 695,082,635 cases, 11 mutations caught, career Rust = original.

Self-test fixes in these batches: the fake player (0x1a4 bytes) and the fake person record (0x46 bytes) were smaller than what the routines read and write (0x1ce and 0x5d); the write outside the buffer corrupted the process heap and brought down the self-test now and then (crash inside `ntdll`, logged by the exception handler). Now 0x200 and 0x80.

### Match engine, 13th batch — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `contar_ativos(lado)` | 0x6cbbb0 | thiscall, ret 4, `al` | the side's active players, stored at motor[4 + lado] |
| `codigo_especial(codigo)` | 0x6cb650 | stdcall, ret 4, `al` | 1 for codes 37, 42..=50, 96 and 97 (switch tables read from the exe) |
| `atras_do_extremo(j)` | 0x6cd5f0 | thiscall, ret 4, `al` | side 0: y <= y of the smallest (0xf58e, without it 12); otherwise y >= y of the largest (0xf592, without it −1) |
| `sem_partida()` | 0x6d4280 | thiscall, `al` | `*[0x4792] == −2` |
| `pressao_na_area(lado)` | 0x6bd6b0 | thiscall, ret 4 | sum of opponents on the lines near the goal (weights 6..1, 16 bits) > draw(15) + 30 → motor[6 + lado] = 1 |
| `mais_proximo_do_ponto(j, a, b)` | 0x6c3010 | thiscall, ret 0xc, `al` | the closest among the available teammates (a tie counts against whoever has `tabela` 1) |
| `pressao_no_ponto(lado, a, b, todos)` | 0x6c3a00 | thiscall, ret 0x10, `al` | +3 on the square itself, +1 at distance <= 1.5 |
| `menor_distancia(lado, a, b, todos)` | 0x6c3950 | thiscall, ret 0x10, `float` | starting from 5000; 0 if someone is on the square; filter by bit 0 of `tabela` |

Self-test: 1,073,998,479 cases, 0 divergences. Mutations caught (10), two of them only after leaving the count grid almost empty (with high counts the pressure sum always exceeded the threshold).

**Important self-test fix:** the fake engine had 0xf590 bytes, but since the tenth batch the test writes fields up to 0xf5f5 (side state, position 1). The writes past the end corrupted the heap and brought down the self-test now and then, depending on how the game was launched (the environment changes the heap layout). Now 0x10000. In addition, dummy pointers (0xdead…) in the extremes were read by a new routine; replaced with valid fake players. Rule for the next batches: when using a new field in the fake objects, check the buffer size.

Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws).

### Match engine, 14th batch — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `repartir_a(lado, delta)` | 0x6c06b0 | thiscall, ret 8 | adds to the disposition (word 0x35) of each active player `ftol(delta × fator)` by the first bit of the role; role 1 is left out |
| `repartir_b(lado, delta)` | 0x6c07a0 | thiscall, ret 8 | same, another factor table; role 4 receives the whole delta |
| `repartir_c(lado, delta)` | 0x6c0a70 | thiscall, ret 8 | same, third factor table |
| `escolher_por_papel(lado)` | 0x6c2750 | thiscall, ret 4, `al` | largest draw(25) × role factor (0.1/0.2/0.5/0.75/1.0 or 1.05 with bits 0x700); −1 with no candidate |

Original's curiosity: `escolher_por_papel` stores the factor in the argument's slot on the stack. A player with no role bit inherits the previous factor, and the first one inherits the bits of the whole word the caller pushed (that is why the argument is treated as u32).

Self-test: 1,524,875,300 cases, 0 divergences; 9 mutations caught. Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (2,356,606,166 random draws, fingerprint 7866d02de7578a90).

### Match engine, 15th to 17th batches — `rust/src/motor.rs` (official on 2026-10-06)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `direcao()` | 0x6f4a30 | thiscall, `ax` | the player's word 0x19a |
| `zerar_guardado(lado)` | 0x6ce0f0 | thiscall, ret 4 | zeroes 0xf59e (side 0) or 0xf5a2 |
| `x_fora_do_meio()` | 0x70df20 | thiscall, `al` | x (0x102) outside 2..=6 |
| `e_o_guardado()` | 0x7103c0 | thiscall, `al` | player == motor[0xf582] |
| `lado_do_lance()` | 0x6bba50 | thiscall, `al` | 0xff with no move; otherwise the move's player is on side 0 |
| `jogador_no_bloco(lado, k)` | 0x6c2fe0 | thiscall, ret 8, pointer | 0x4796 + 0x1be × (20 × side + k) |
| `tabela_a_1a(j)` | 0x6c3920 | thiscall, ret 4, `ax` | `tabela_a` by the position at 0x1a |
| `momento_marcado()` | 0x6d4250 | thiscall, `eax` | 0x8ed4 outside 0x1ef/0x3de and bit 0x80 of the match's byte 0x38 |
| `estado_do_lado()` / `estado_do_adversario()` | 0x6f4a40 / 0x6f4a70 | thiscall, `al` | 0xf5bd / 0xf5f3 according to the side |
| `gravar_19c(v)` | 0x70a890 | thiscall, ret 4 | clamped to −200..=200 |
| `codigo_de_bola_parada(c)` | 0x70a8c0 | ret 4, `al` | 12, 13, 21, 37, 46, 59 |
| `perto_do_gol()` | 0x70df70 | thiscall, `al` | side 1: y < 4; side 0: y > 7 |
| `limpar_lance()` | 0x6bba10 | thiscall | 0x475a..=0x4769 |
| `somar_disposicao(lado, delta)` | 0x6cf710 | thiscall, ret 8 | whole delta into 0x35 of all active players |
| `repartir_d/e/f(lado, delta)` | 0x6c0890 / 0x6c0980 / 0x6c0b70 | thiscall, ret 8 | three more factor tables; in `f`, role 1 receives double (`fadd st0, st0`) |
| `registrar_lance(...)` | 0x6bb9a0 | thiscall, ret 0x24 | 9 arguments, only if 0x4761 is empty |
| `remover_da_lista(p)` | 0x714ca0 | thiscall, ret 4, `al` | swaps with the last one; returns the new count |
| `entrar_em_campo(j)` | 0x6cdd00 | thiscall, ret 4, `al` | puts at (8, 6) and in the side's list (0x453a, 11 per side) |
| `em_perigo()` | 0x70d630 | thiscall, `al` | 12 × 9 float table (0xa15e44) ≤ 1.0, otherwise `atras_do_extremo` |
| `conferir_lance()` | 0x6bba80 | thiscall, `al` | 1/2 according to the move's side and area; draw(200) decides whether to clear |
| `caminho_livre_ate(lado, x, y)` | 0x6c41b0 | thiscall, ret 0xc, `al` | opponent on the square or in the column between the point and the goal |
| `alcanca_o_ponto(x, y)` | 0x713d90 | thiscall, ret 8, `al` | `por_diferenca` against 0x10c + 10, and `distancia_entre` against the 12 × 9 table |

Finding from the 16th: in `entrar_em_campo`, with side outside 0/1 the stored slot lands on the count itself (0x4592), and the original re-reads the count after writing; the first Rust version kept the value read before and diverged in the self-test. Fixed by re-reading like the original.

Self-tests: 1.56 / 2.23 / 2.44 billion cases, 0 divergences. Mutations: 12/12, 11/13 and 12/13 caught; the ones that passed are equivalent (an empty list returns 0 by both paths; the table has no NaN; with equal y, the column only coincides on the square itself, already handled before). One gap (exact equality at the `alcanca_o_ponto` threshold) was closed in the generator. Real game: >1 billion comparisons, 0 divergences. Career with the engine on until 2002-06-01: Rust ×2 = original (fingerprint 7866d02de7578a90).

### Match engine, 18th to 20th batches — `rust/src/motor.rs` (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `escolher_destaque()` | 0x6d0f80 | thiscall | among the (match byte 0x4d & 0xf) + 11 of each side: 16-bit score (disposition + 125 × (0x10 + 2 × 0x0c) + 50 × 0x03 + 35 × 0x06) and ftol(disposition × c); stores the record's id at 0x477a |
| `escolher_por_papel_b(lado)` | 0x6c2880 | thiscall, ret 4, `al` | factors 1.2/1.2/0.65/0.5/0.35/0.25 or 0.15, adjusted by three of the person's attributes; the factor lives in the argument's slot and carries over |
| `casa_da_formacao(j, &x, &y)` | 0x6c2a40 | thiscall, ret 0xc | nibbles of a formation byte (216 bytes per position); field according to 0x8eae and the side |
| `calcular_percentuais(i)` | 0x6ce9c0 | thiscall, ret 4 | indices i, 3 and 4; ftol(a / (a + b) × 100) and the complement; 50/50 with no positive total |
| `escolher_por_papel_c(lado, modo)` | 0x6c2050 | thiscall, ret 8, `al` | draw over the player's float attributes (four formulas, two without the factor), person (word 5, draw against 0x21), deduction of draw(5) above a limit |
| `aplicar_fator(f)` | 0x716450 | thiscall, ret 4 | condition 0x1a2 × f, with minimum 2 × (25 − 0x5d of the record) and ceiling 100 |
| `valor_pela_condicao(alvo)` | 0x7165b0 | thiscall, ret 4, `eax` | two formulas according to `alvo` == 0x1a6 |
| `aplicar_ao_time(j, f)` / `aplicar_ao_lado(lado, f)` | 0x6bb8d0 / 0x6bb950 | thiscall, ret 8 | factor to the player and derived values to the teammates with a record |
| `escolher_pela_condicao(lado, modo)` | 0x6c2310 | thiscall, ret 8, `al` | draw(value + 5 or + 25) × (0.2 with role 1, otherwise 1) |

Mutations: 14/14, 12/12 and 10/12 caught (the two remaining ones are equivalent: the ceiling is 100.0, and rounding to f32 would only change a value that is rewritten identically). The mutation script now stops at the first divergence. Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 21st to 23rd batches — `rust/src/motor.rs` (official on 2026-10-07)

Routines with no file in the map (`docs/mapa_funcoes.csv` with "-"), but called by the engine; the coverage list only saw those in `match_*.cpp`. Found through the calls of the covered routines.

| Routine | Original | Convention | Note |
|---|---|---|---|
| `ler_1bc()` | 0x717400 | thiscall, `ax` | word 0x1bc |
| `somar_2b(v)` / `gravar_2b(v)` | 0x7160d0 / 0x7160e0 | thiscall, ret 4 | byte 0x2b |
| `tabela_a_1a_do_jogador()` / `tabela_b_do_jogador()` / `diferenca_do_jogador()` | 0x717430 / 0x717450 / 0x715f30 | thiscall | through the player's engine; without an engine 0 (or 0xff) |
| `marcas_do_jogador_rot()` | 0x7163c0 | thiscall, `edx:eax` | 0x5b5270 of the position |
| `marca_800000()` | 0x7163f0 | thiscall, `al` | bit 0x1000 of the block: byte 6 + side; otherwise bit 0x800000 of the marks |
| `perto_do_proprio_gol()` | 0x70def0 | thiscall, `al` | side 0: y < 3; side 1: y > 8 |
| `papel_880()` | 0x717c70 | thiscall, `al` | 0x880 in `tabela_a` and `tabela_b` |
| `soltar_bola()` | 0x6f0970 | thiscall | zeroes the owner and the move state |
| `jogador_pela_ficha(f)` | 0x6e1cd0 | thiscall, ret 4 | searches the 20 at [this + 0x23] |
| `byte_do_codigo(c)` | 0x6d5cd0 | ret 4, `eax` | 18-byte-per-code table at 0xb68788 (uninitialized area: the self-test writes temporary values) |
| `nota_ajustada(base)` | 0x715f50 | thiscall, ret 4, `eax` | three parts (base or per-side value), draws against the person's attributes, / 3 |
| `ajustar_17(delta)` | 0x7172d0 | thiscall, ret 4, `al` | byte 0x17 ± delta with draws near 10, floor by the disposition, range 1..=20 |
| `pegar_bola()` | 0x6f0630 | thiscall | the player takes the ball: releases the previous owner's, credits the pass (counter and disposition by zone), records possession and the ball on his square |

**Hook limit:** 0x714d00 stayed as Original. The short prologue (4 bytes) is followed by a relative `jne`, and the trampoline would copy the jump out of place (in the self-test, it jumped into garbage). `prologo_ok.py` (session draft) checks all prologues before each batch: whole instructions and no relative jump or call.

Mutations: 12/12 (after closing two gaps: the zeroed table and byte 0x1a7 always zero), 13/13 and 11/12 (the remaining one, releasing the player's own ball before he takes it, is equivalent). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 24th and 25th batches — refereeing and cards (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `iniciar_rigor(motor, pessoa)` | 0x6e9470 | thiscall, ret 8 | the referee object (engine, person, value 5000 with no person or 7750..5250 by draws against word 0x1e) |
| `lance_valendo()` | 0x6bab90 | thiscall, `al` | does the recorded move continue? the ball crossing the move's line ends it (clears) |
| `decidir_falta(j, codigo, &tipo, &valor)` | 0x6e9540 | thiscall, ret 0x10, `al` | foul given? role 1 in the box, the referee's person, draws by code; stores type 4 and draw(0x14) + 1 |
| `punir_falta(j, &tipo, x, sev, flag, &peso)` | 0x6e9630 | thiscall, ret 0x18 | straight red, yellow (0x24/0x25/0x26) or −25 on the disposition; with flag, draw(3000) |
| `punir_lance(j, outro, &tipo, x, sev, flag, &peso, expulsar)` | 0x6ea4e0 | thiscall, ret 0x20 | variant: inactive (weight 10), type 7, red type 5 or 4, counters 0x14 and 0x15 |

The yellow-card deduction is ftol(c × −225) with c = 1, or 45 / (0x8ed0 / 11) above 45: continuous at the threshold (45/45 = 1), which is why the threshold mutation is equivalent. Other equivalents: with sides 0 and 1, "owner on the other side" written in two ways.

Mutations: 13/14 and 16/17 (the remaining ones, equivalent). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

New tool: `engenharia-reversa/prologos.py` checks the prologues in `motor.rs` (whole instructions, no relative jump or call).

### Match engine, 26th to 28th batches — offside and card by severity (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `motor_decidir_falta` / `motor_punir_lance` / `motor_punir_falta` | 0x6cfca0 / 0x6cfcd0 / 0x6cfd10 | thiscall | wrappers: the routine of the engine's referee (this + 0xf5b2) |
| `procurar_impedimento(lado, linha, y, passador)` | 0x6ea760 | thiscall, ret 0x10, `eax` | match_official.cpp: attacker offside by distance to the last line (0x8eaf/0x8eb0) and draws against byte 0x107 (one of them with 0x107³ + 25); adjusts the referee's value |
| `motor_procurar_impedimento` | 0x6cfd70 | thiscall, ret 0x10 | wrapper |
| `cartao_pela_gravidade(j, &tipo, gravidade, flag)` | 0x6eab60 | thiscall, ret 0x10 | draw(0x14) adjusted by the referee and by the severity (1..6); second yellow, red or yellow; the record's code stays in the slot of argument `j` (with no known severity, the low byte of the pointer) |
| `motor_cartao_pela_gravidade` | 0x6cfd40 | thiscall, ret 0x10 | wrapper |

The wrappers are compared over the same regions as the inner routine (the inner one, hooked, is compared inside them).

Mutations: 3/3, 13/13 and 13/13. Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 29th to 31st batches (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `escolher_por_papel_e(lado, modo)` | 0x6c23f0 | thiscall, ret 8, `al` | fifth weighted choice (855 bytes): factors by role with 0x880 variants, adjustments by five of the person's attributes (one of them with a double constant), three draw formulas; 1 in 5 discards the choice |
| `estatistica_de_posse(lado, janela)` | 0x6cf750 | thiscall, ret 8 | possession by ball zone and period; with `janela` (window), totals of the last 55 instants (circular window at 0x8e9b) |
| `resumo_da_partida(momento)` | 0x6ce460 | thiscall, ret 4 | drawn tempo (0xf628/0xf62c/0xf630), each player's rating ((disposition + 500) / 1000 in 1..=10, at 0x1b) and sums of attributes per side; always recomputes the percentages |

Equivalent by construction: multiplying by 0.75 in f32 or in double (0.75 is exact); rating 10 clamped to 10 by `>` or `>=`. Two mutations of the 30th only passed because the test had neither side −2 nor 0xff bytes in the window: inputs widened, both caught.

Mutations: 10/11, 10/10 and 10/11. Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 32nd and 33rd batches (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `mais_perto_no_lado(lado, a, b, todos)` | 0x6c30e0 | thiscall, ret 0x10, `eax` | 20-iteration loop unrolled in the original: the closest to (a, b) by the table, zero distance returns immediately |
| `defesa_do_goleiro(a, b, a3, codigo, &saida, &rebote)` | 0x700420 | thiscall, ret 0x18, `ax` | value of the save: base from word 0x168/0x16a + 220, draw(0x3b) against draw(285) chooses the formula (0.7/0.3), adjustments by code (3/5, 26, 27, 0x33), by area and by distance to the ball; rebound when the final draw loses |

Equivalent over the possible inputs: the distance table has neither the threshold value 2.1 (`>` and `>=` the same) nor negative values (the first zero distance wins either way).

Mutations: 14/16 (the two remaining ones, equivalent). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 34th batch — occupancy grid (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `mover_para(x, y)` | 0x6f46b0 | thiscall, ret 8 | moves the player on the grid (90 bytes per square starting at 0x215e; per side, a list of 11 pointers and the count at 0x58 + side): the last one in the list takes his slot (0x28), he enters at the end of the new square's list; updates the ball if he is the owner |

Impossible states of the original: empty list when removing (the original shows an error box and returns; in Rust, it just returns) and more than 11 when adding (the original shows the error and terminates the game; in Rust, it terminates the same way). The self-test builds consistent grids so it never falls into them.

Mutations: 6/6, and the seventh (grid row size) brought down the game through the original's exit. Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 35th and 36th batches — square choice and control of the move (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `escolher_casa(j, &x, &y, flags)` | 0x6bce80 | thiscall, ret 0x10, `al` | the "brain" of movement: scores the 9 squares around the player (opponents on the square or the closest to it, dropping back, offside line with flag 4, central strip, previous target with flag 2) starting from 7.5; a tie goes to the last one |
| `dominio_do_lance(&codigo, &a, &b)` | 0x70d690 | thiscall, ret 0xc, `ax` | control value by code (two jump tables): the player's word minus draws by physical condition, by the direction of an ownerless ball, by the owner's value and 20 × (0x8eaa − 1)³; outside the box it can lose immediately (−20); the goalkeeper in the box changes the code |

Equivalents: with flag 2, the player's own square is already excluded by `virado_para` (which returns 0 for (0, 0)); a 180-degree difference gives 180 by `>` or `>=`. Gaps closed: the role tables did not have bits 0x10/0x20 with 0x80/0x800, and the other owner always had 0x19c zeroed. The full self-test now exceeds 600 s (4.7 billion cases); the mutation script's limit went up to 1500 s.

Mutations: 21/22 and 14/15 relevant ones (the remaining ones, equivalent). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 37th batch — tracing the move, with recorded internal calls (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `tracar_lance(x0, y0, x1, y1, codigo, a6, a7)` | 0x6ccb60 | thiscall, ret 0x1c, `al` | walks the squares of the line (major axis with accumulated error, four directions) and at each one asks `procurar_no_lado` (0x6cd330) for a player from a randomly drawn side and from the other, with the distance from the origin; move type by code (switch) |

**Recording and replaying internal calls.** 0x6cd330 has effects on the players (through 0x6f5c30). In the comparison, the original `tracar_lance` runs with the 0x6cd330 hook recording each call (arguments, answer and the random-generator state afterwards); the Rust copy receives the call as a closure (`tracar_com`) and, instead of running 0x6cd330, consumes the recording: it checks the arguments (type, a6 and a7 only by the low byte or word, because the rest is stack garbage in the original), returns the answer and sets the generator to the recorded state. The original's effects stay in memory (outside the compared regions), and the call sequence is compared too. Outside the comparison, the Rust version really calls 0x6cd330.

Equivalent by the geometry of the trace: on a one-point line, y is already shifted before the first square (the "non-degenerate line" condition never decides); the accumulated error always lands exactly on the destination (ending one square earlier gives the same result, because the last one is the target, which is skipped).

Mutations: 7/9 (the two remaining ones, equivalent). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 38th and 39th batches — the day's matches and commentary (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `tem_jogo(clube, so_marcados)` | 0x6b59a0 | thiscall, ret 8, `eax` | match_day.cpp: is the club in the list of the day's matches (home or away)? |
| `venceu_o_jogo(jogo, clube)` | 0x6b7cf0 | cdecl, `al` | match_day.cpp: decides by penalties, extra time or normal time (score −1 does not count) |
| `comentario_a(...)` / `comentario_b(...)` | 0x7002f0 / 0x70a860 | thiscall, ret 0x14 / 0x10 | wrappers of 0x6d6f70 (enqueue a commentary event) with fields −1 |

The original's error cases (null pointers, club not in the match) show a message box and return 0; in Rust, they just return 0 (they never occur in play; the self-test does not trigger them).

**Gate at 0x6d6f70 (commentary).** In the comparison, a hook at 0x6d6f70 records the calls made by the original of a ported routine (with the random-generator state afterwards); the Rust copy receives the call as a closure, checks the sequence and does not run the commentary again (no duplicated events). 0x6d6f70 can call itself (one comment triggers another): only the top-level call is recorded (depth counter) — the first version recorded the inner ones and reported a false divergence. No synthetic self-test (the commentary object belongs to the game); discrimination was proven in the real game with a mutation in each wrapper (342 thousand and 3.6 thousand divergences).

Mutations: 6/6 (38th, self-test). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 40th batch — ball out of play (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `bola_fora(lado)` | 0x6cd490 | thiscall, ret 4, `al` | releases the ball from its owner and, with the ball out of play, sets the restart (0x8eb2: 2 throw-in, 3/4 over the byline and by the side that touched it; 0x8eae: who restarts) and narrates (0x1f76/0x1f75/0x1f77) |

The check of the calls to 0x6d6f70 now uses masks: the routine reads the 1st argument as a word and the 3rd to 6th as bytes, and the original pushes garbage in the high bytes (for example, `sete al` over an old eax). No synthetic self-test (narration); discrimination proven in the real game with a mutation in the side that restarts (638 divergences). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 41st batch — shot value, proof with a large routine (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `valor_do_chute(&tipo, &som)` | 0x700bd0 | thiscall, ret 8, `ax` | `MATCH_PLAYER::get_shot_score` (5.2 KB, about 1,600 instructions): chooses the shot type when it is not given (chip over a goalkeeper off his line, according to the move's situation at 0x8ea9 and attributes 0x146..0x154), computes the value (word per type at 0x11a + 2 × type, physical condition, goalkeeper, angle, distance to goal with powers 8 and 7, wind on long shots), stores the quality part at 0x39 and the commentary in `som` |

**Feasibility proof (requested by Lucas).** Reading and translation in one session. The problems found were all infrastructure, none in the logic:
1. The self-test generator overwrote the engine pointer (0x19e) when filling the per-type words; loop limited to 0x18e. The crash was located through the linker map (`-C link-arg=/MAP`).
2. The power function calls the one in the game's CRT (0x945ff0, `_CIpow`) via `asm!`, for a bit-for-bit identical result; the buffer pointer has to stay in a register preserved across the call (`edi`), not in `eax`.

The logic matched on the first attempt: 4.77 billion cases and 0 divergences. Mutations: 26/30 (the 4 remaining ones are equivalent given the possible values: the distance table has neither exactly 1.5 nor 2.2, and the angles come from fractions with denominators from 1 to 4, which never fall into the altered ranges). Real game: 1 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 42nd batch — team strength, large integrating routine (official on 2026-10-07)

| Routine | Original | Convention | Note |
|---|---|---|---|
| `forca_do_time(lado)` | 0x6c0c60 | thiscall, ret 4 | match_eng.cpp (5.1 KB, about 1,250 instructions): zeroes and sums the side's seven strength words (0x1da … 0x1f2, in steps of 4, + 2 × side) over the 20 players of the block. Per active player: wear (0x6f56c0), seven components (combinations of the player's floats; steps of multipliers by the person's attributes; draw of the day, attribute 0x21 against 20; value by condition from 0x7165b0) and, by the tactical role (bits 1..0x40), adjustment of word 0x35 (deduction and ±5 against a threshold) and the truncated parts. Then, adjustments by competition (ids at 0x9cf2c0..0x9cf4a8), by 0x5c6710, by the clubs' reputation (+0x80) and by the side |

Second step of the feasibility proof: a large integrating routine, with internal calls that have effects (wear, random draws, condition), compared by the side-by-side execution itself (the calls land in the hooks already proven). x87 details reproduced: one component stays on the x87 stack without going through float (only the others are stored in 32 bits); one of the steps multiplies directly in st0, without rounding at each step; the competition correction reads side 0's word and writes to the word of the calling side. Player without a record (0x69): the original shows an error box and zeroes 0xb67a34; in Rust, it only zeroes (never occurs in play; the self-test does not trigger it).

The logic matched on the first attempt: 5.1 billion cases and 0 divergences. The self-test writes distinct ids into the game's competitions during the test (at the start of the game they may be empty) and restores them. Mutations: 22/22.
Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 43rd batch — kick-off (official on 2026-10-07)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `pontape_inicial()` | 0x6c51e0 | thiscall | match_eng.cpp: who kicks off (0x8eae; outside 0..=1, side 0), ball at the centre (4, 5 or 6); every active player goes to their formation square (table 0xf56e + 0x9e for side 0 or 0xf572 + 0xa6 for side 1, 216 bytes per position; low nibble x, high nibble y, clamped to their own half) through 0x6f46b0 and faces forward (0x6f4920). On the kicking side, the player closest to the ball (a position > 10 becomes −1) takes the ball (0x6f0630) and faces the square computed from the last formation square used (same point: the low byte of 0x8ed8, sign-extended). With nobody available, it sets byte 0 of the engine |

In the original, x and y start as stack garbage, but they are only read after some active player (with no active player, the routine exits first). The comparison checks the whole engine object (players, grid and ball), plus the random draw.

Self-test: 5.5 billion cases, 0 divergences, with a consistent pitch grid (each player in the list of their square) and formations with distinct squares. Mutations: 11/13. The two that pass are equivalent: the clamp on y (at most 5 on side 0, at least 6 on side 1) makes the direction tests `y < 9` and `y > 3` always true. One gap found and fixed: the generator never left the kicking side without active players, and the "nobody available" mutation passed. Real game: 1 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.
