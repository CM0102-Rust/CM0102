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
| `forca_do_time(lado)` | 0x6c0c60 | thiscall, ret 4 | match_eng.cpp (5.1 KB, about 1,250 instructions): zeroes and sums the side's seven strength words (0x1da … 0x1f2, in steps of 4, + 2 × side) over the 20 players of the block. Per active player: wear (0x6f56c0), seven components (combinations of the player's floats; steps of multipliers by the person's attributes; draw of the day, attribute 0x21 against 20; value by condition from 0x7165b0) and, by the tactical role (bits 1..0x40), adjustment of word 0x35 (deduction and ±5 against a threshold) and the truncated parts. Then, adjustments by the match country (national league style: nations at 0x9cf2c0..0x9cf4a8, see `CAMPOS.en.md`), by 0x5c6710, by the clubs' reputation (+0x80) and by the side |

Second step of the feasibility proof: a large integrating routine, with internal calls that have effects (wear, random draws, condition), compared by the side-by-side execution itself (the calls land in the hooks already proven). x87 details reproduced: one component stays on the x87 stack without going through float (only the others are stored in 32 bits); one of the steps multiplies directly in st0, without rounding at each step; the competition correction reads side 0's word and writes to the word of the calling side. Player without a record (0x69): the original shows an error box and zeroes 0xb67a34; in Rust, it only zeroes (never occurs in play; the self-test does not trigger it).

The logic matched on the first attempt: 5.1 billion cases and 0 divergences. The self-test writes distinct ids into the game's competitions during the test (at the start of the game they may be empty) and restores them. Mutations: 22/22.
Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 43rd batch — kick-off (official on 2026-10-07)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `pontape_inicial()` | 0x6c51e0 | thiscall | match_eng.cpp: who kicks off (0x8eae; outside 0..=1, side 0), ball at the centre (4, 5 or 6); every active player goes to their formation square (table 0xf56e + 0x9e for side 0 or 0xf572 + 0xa6 for side 1, 216 bytes per position; low nibble x, high nibble y, clamped to their own half) through 0x6f46b0 and faces forward (0x6f4920). On the kicking side, the player closest to the ball (a position > 10 becomes −1) takes the ball (0x6f0630) and faces the square computed from the last formation square used (same point: the low byte of 0x8ed8, sign-extended). With nobody available, it sets byte 0 of the engine |

In the original, x and y start as stack garbage, but they are only read after some active player (with no active player, the routine exits first). The comparison checks the whole engine object (players, grid and ball), plus the random draw.

Self-test: 5.5 billion cases, 0 divergences, with a consistent pitch grid (each player in the list of their square) and formations with distinct squares. Mutations: 11/13. The two that pass are equivalent: the clamp on y (at most 5 on side 0, at least 6 on side 1) makes the direction tests `y < 9` and `y > 3` always true. One gap found and fixed: the generator never left the kicking side without active players, and the "nobody available" mutation passed. Real game: 1 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 44th batch — rolling ball (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `rolar_bola()` | 0x6bbe40 | thiscall | match_eng.cpp: loose ball (no owner at 0xf582, no 0xf57a) with momentum (0x8eaa). Play stopped: clears the momentum and 0x8ea9. Momentum 1 only moves if draw(1) ≥ draw(2); momentum ≤ 0 does not move. When it moves, it spends one momentum (and one of 0x8ea9) and takes a step along direction 0x8ed8: (220, 320) x − 1, (40, 140) x + 1, (130, 230) y − 1, below 50 or above 310 y + 1. Out of the pitch: ball out (0x6cd490) for the side in 0x8eab |

**Commentary gate in the self-test.** In the self-test, the 0x6d6f70 gate only records the calls (the commentator is fake) instead of running the commentary: routines that narrate now have a synthetic self-test (before, only the real game). **Nested comparisons:** when a compared routine calls another one that also narrates (the rolling ball calls ball out), the inner comparison just passes the call on, and its commentary goes into the outer recording (the first version compared the inner original against an empty value).

Self-test: 5.5 billion cases, 0 divergences (directions at the band edges, exits through all four sides, with commentary). Mutations: 9/12; the 3 that pass are equivalent: `v <= 0` × `v < 0` (momentum 0 already returned earlier), the lower limit 130 and the 50 (the earlier bands, 40 to 140, already catch those values). Real game: 1 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 45th batch — condition test (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `teste_de_condicao(codigo, alvo)` | 0x7164b0 | thiscall, ret 8, `al` | match_pl.cpp: v = condition² (0x1a2, power through the game's CRT); limit 8 if `alvo` is the one stored at 0x1a6, else 1. Code 1: draw(110 − condition) < limit; 2 to 5: draw(K / v) < limit, K = 400 thousand, 2, 4 and 8 million; others: 0 |

**`_ftol` out of range.** With condition below 1, v = 0 and K / v is infinite: the x87 `fistp` stores the indefinite integer (low part 0), while Rust's `as` saturates (−1). The Rust `ftol` now returns 0 out of range (it only changes values the original already handled that way). Mutations: 9/10 (the signed-byte conversion of the condition is equivalent: the condition goes up to 100).

### Match engine, 46th batch — person preferences and mark by position (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `gosta_de(alvo, pessoa)` | 0x547810 | cdecl, `eax` | `alvo` is among the people `pessoa` likes (preferences at +0x65; slots +0x1c, +0x20, +0x24) |
| `nao_gosta_de(alvo, pessoa)` | 0x547840 | cdecl, `eax` | same, among those disliked (+0x28, +0x2c, +0x30) |
| `marcar_posicao(posicao, ficha)` | 0x5b5910 | thiscall, ret 8 | writes into the per-position table (+0x5bf + 4 × position) the record's value (−1 without a record), clears +0x187f, returns 1 |

Mutations: 8/8, after closing two generator gaps (empty preference slots, and +0x187f already zero in the fake engine).

### Match engine, 47th batch — fine positions to draw the match (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `posicoes_finas(saida)` | 0x6d1770 | thiscall, ret 4 | match_eng.cpp: fine position (27 × 36 grid) of each player to draw the match. Each square becomes 3 × 3 sub-squares; the ball holder takes the centre; the others draw a sub-square (never the centre of the ball holder's square), accepting an occupied one only after 50 attempts per occupant; the side that starts in each square is drawn. Output: ball (x, y) and, per side and tactical position, 6 bytes (fine x and y, player byte 0, role, mark 1) |

It is the basis of a future 2D view of the match. The "stack aliasing" that had postponed this routine did not exist: it was a `push` offset not counted while reading. **Test generator fixed:** the 43rd batch formations piled more than 11 players into one square (out-of-half y clamped to the same row), corrupting the grid lists; the kick-off passed its test (original and Rust corrupt alike), but the new routine reads the lists and the original crashed. Formations now have distinct squares per position, and a diagnostic checks the grid before and after the kick-off. Mutations: 8/9 (marking the centre as taken by the ball holder is equivalent: the centre of his square is already barred to the others).

### Match engine, 48th batch — leaves from other modules (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `unidades(n)` | 0x4b0850 | cdecl, `al` | comp_util.cpp: units digit of n (below 1000; else 0) |
| `e_amistoso(comp)` | 0x5c6710 | thiscall, ret 4 | friendly.cpp: the competition is the one stored at +4 or among the last 127 club competitions (0xae2378) |
| `contrato_de(pessoa)` | 0x4cf5a0 | thiscall, ret 4 | contract_manager.cpp: contract (80 bytes) by the index in table 0xae2d0c (79 bytes per person; count at 0xae2364) |
| `e_selecao(clube)` | 0x53b900 | cdecl | Database.cpp: the id comes after the domestic clubs (0xae235c − 2 × 0xae2350) |

First routines outside `match_*.cpp`: they unlock engine integrators that depended on them. The self-test swaps the game's globals during the test and restores them. Mutations: 9/10 (`n <= 1000` is equivalent: for 1000 both give 0). One mutation run crashed inside the DLL; repeated with the linker map, it did not reproduce (6 billion cases, 0 divergences).

### Match engine, 49th batch — human manager and formation masks (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `tecnico_9a01(pessoa)` | 0x5fd210 | thiscall, ret 4 | human_manager.cpp: field +0x9a01 of the person's human-manager record (the last 16 people, 42,735-byte records; without a person, the current manager from table 0xb74c26) |
| `tecnico_9a05(pessoa)` | 0x5fd480 | thiscall, ret 4 | same, is byte +0x9a05 non-zero? |
| `bit_da_casa(codigo)` | 0x5b5a60 | stdcall, ret 4 | formation.cpp: formation-mask bit by tactical square code (31 codes, taken from the game itself; 0x802 and 0x804 give the same bit in the original) |
| `separar_mascara(m, &setor, &lado)` | 0x5b73a0 | cdecl | formation.cpp: the first sector bit (1..0x40) and side bit (0x80, 0x100, 0x200 also with bit 1, 0x400, 0x800) |

Mutations: 11/11. Real game (batches 45 to 49 together): 1 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

### Match engine, 50th batch — second national team and aggregate score (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `e_segunda_selecao(clube)` | 0x5409b0 | cdecl | Database.cpp: the id is at or after total clubs − nations (0xae235c − 0xae2350) |
| `placar_agregado(lado, &casa, &fora, &classificado, &ida, &fase3, bonus)` | 0x6ce310 | thiscall, ret 0x1c, `al` | match_eng.cpp: goals of this match (0xf5bd/0xf5f3, swapped for side 1) plus the first leg's stored in the match object (+0x4b/+0x4c; 0xff = no first leg); `fase3` from the units digit of +0x30; with the aggregate level (plus the bonus), without bit 4 of +0x38 and, with bit 0x4000, only after time 0x3de, it decides on away goals. Returns 1 if through, else home − away |

Mutations: 11/11.

### Match engine, 51st batch — direct marker (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `marcador_direto(lado, k)` | 0x6d4330 | thiscall, ret 8, `al` | match_eng.cpp: the opponent with the mirrored role (sector: 2/4 ↔ 0x40, 8 ↔ 0x20, 0x10 ↔ 0x10; side: 0x80 ↔ 0x800, 0x100 ↔ 0x400, 0x200 ↔ 0x200); with nobody, the nearest by the distance table with the mirrored formation (0x5b73a0). −1 for a goalkeeper or a player off |

**Correlated generator.** The test's role tables came from rotations of the same number (`rotate_left` in steps of 3 bits): the remainder by 9 was so correlated that some sectors almost never appeared, and two mutations passed. Each entry now has its own draw. Mutations: 10/10.

### Match engine, 52nd batch — take off the pitch (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `tirar_de_campo(jogador, modo)` | 0x6cdd50 | thiscall, ret 8 | match_eng.cpp: takes the player out of his square's list (the last one fills the gap) and undoes the markings (0x1aa and list 0x1ae). With mode −3, or being the stored goalkeeper (0xf59e/0xf5a2), and not a forward: picks who takes over the position, by the highest role score (forward 70 thousand, attacking midfielder 50 thousand, 0x10 15 thousand, others −5 thousand; +5 thousand for the flanks); the first candidate enters unfiltered, the next ones cannot be a goalkeeper or role 2 or 4. A human manager's team only picks on its own with mark 0x904b, not being a national team and without 0x5fd480 |

**Intermittent crash explained.** The rare crashes inside the DLL (in `memcpy`, copying 1 byte from an invalid address) came from the 0x6f46b0 comparator: it saves byte +0x28 of the "moved" player, read from the square's list; with more than 11 players in a square (43rd batch generator), that player was garbage. The generator now puts each position in a distinct square (at most 4 per square with the y clamp). 43rd batch mutations redone with the new generator: the same. 52nd batch mutations: 13/14 (15 mil × 20 mil para o papel 0x10 é equivalente: só a ordem das pontuações importa). Real game (batches 50 to 52): 1 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original.

**The reference fingerprint changed (in batches 50 to 52).** The Rust career gave c76ad0ff6ce3daa6 (2,357,335,216 draws), different from the earlier 7866d02de7578a90; bisection (`SEM_ROTINAS` in `motor.rs`) pointed to `e_segunda_selecao`, which always returns the same value as the original (checked over the whole season). The cause is the stack: in the simulation every hook runs on the private stack, including in "original" mode (the original goes through it in isolation), and the original 0x5409b0 leaves the caller's `esi` 0x204 bytes below the game's stack, garbage that some later routine reads. The "original" mode of the same build also gives c76ad0ff6ce3daa6: **the career is always compared with the original mode of the same build**, never with a remembered number. With a null club, the Rust version calls the original itself (the error path has effects).

### Match engine, 53rd batch — individual marking (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `marcacao_individual()` | 0x6bbf60 | thiscall | match_eng.cpp: for each side and active player whose position has marks 0x10 and 4 in the side's table (0x5b57c0), finds the direct marker (0x6d4330) and stores his record (+0x69) in the per-position table (0x5b5910). Marker −1 reads the "player" before the block, like the original |

Mutations: 5/5.

### Match engine, 54th batch — new reputation (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `nova_reputacao(motor, lado, valor, alt, bonus)` | 0x6d3de0 | cdecl | match_eng.cpp, called at the end of the match for each side (except national teams): stores at +0x1d6 + 2 × side the club's new reputation (1..=10000), a moving average with weight `n` between the current reputation (club +0x80) and a target. Level (up to 20): the competition's reputation (+0x69), or its average with the opponent's reputation / 500 when the match has +0x40; 5 in match −2. Weight: 202 − 10 × level; national team, and competition type 8 with certain codes (+0x32 in 0x41f..0x426, with +1; +0x30 in 0x78..0x7c, 0x82..0x87, 0x96..0x9b, with the level uncapped), reputation/1000 + 16 (minimum 21) − level. Target: (level × 0.5 + 5) × 50 × (value − ratio), with a negative difference squared (keeping the sign); the ratio comes from the global statistics (0xb74350: +0x20/+0x28 or +0x10/+0x18, 6.8 if the divisor is zero). With `bonus`, the target is at least 250 × level × value / ratio. The result does not exceed max(300 × level + 4000, reputation) nor move away from the reputation by more than 5 × level × (2 − reputation/10000) |

Mutations: 13/13 (`n_21` flagged by a crash: the mutated version divides by zero). Level 21..=28 with codes 0x78.. would give weight zero in the original too (division by zero); the generator does not produce that case.

**Faster tests.** `CM_AUTOTESTE=motor:N` runs only the match-engine self-test, with N cases. `mutacoes.py` runs the full base (about 30 minutes) and then each mutation with `motor:200000` (`--casos` changes it): in the 54th batch, each mutation was flagged about 1 second after the game opened, against a few minutes before. Real game (batches 53 and 54): over 1 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original (c76ad0ff6ce3daa6, 2,357,335,216 draws).

### Match engine, 55th batch — player reputation (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `reputacao_do_jogador(reg, meu, outro, partida, valor, alt, bonus)` | 0x6d3610 | cdecl | match_eng.cpp, at the end of the match for each player (record at motor + 0x9032, player at +0x1b): the same computation as the club's reputation (54th) for the three reputations of the record (+0x69 of the player): current (+8), home (+0xa) and world (+0xc); stores (home, current, world) in the record. Weights: home = (reputation/1000 + 16, minimum 22) − level; current = the same if the club is a national team or the player's club (+0x1a) is the club's (+0x53) or the competition's (+0x5d), else the average of that with 202 − 10 × level (minimum 4); world like the club's (minimum 21). Target: (level × 0.5 + 5) × 50 × 1.3 × (value − ratio), without the club's square. Range per match (× 2 − reputation/10000): home −250/+300, current −250/+375, world −500/+100 × level. Cap: 400 × level (÷ 2 without the special case, for current and world) between 5000 (world 3000) and 10000, never below the current reputation |

Mutations: 13/13.

### Match engine, 56th batch — choose the taker (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `escolher_cobrador(lado, tipo, excluidos)` | 0x6ceb90 | thiscall, ret 0xc | match_eng.cpp: the side's taker for set piece `tipo` (1..=8), by the highest score among the 20 of the block (the first tie stays). Out: position < 0 or marked in `excluidos` (type 5); position > 10 or no person (+0x6d): the position becomes −1. The stored goalkeeper does not take 1, 2, 3, 8, nor 6/7 far from the box. 1 and 2: attribute (0xdd, 0xe9) − 2 × distance to the ball; 3 (corner): the one designated in the club's tactics (0x897990 on 0xde1d08; 10000/9000, with a draw on the opposite side) or 0x89 − 0xad/2, × 0.75 (1.25 if negative) with mark 0x40; 4: 100 with the table = 1; 5 (penalty): the human manager's list (10000 − order, 0x601070) or 0xc9 by age, a draw on +0x5a and the person's +0xb; 6/7 (free kick): near the goal, the designated one or set piece (person +0x29, 0x109); far, 0xa1 − distances; 8: 100 − distance. Nobody: 0 with `excluidos`, else a fatal error |

**Mutations in the real game.** The club's tactics (0xde1d08) only exist with the database loaded; in the synthetic self-test the corner and the free kick near the goal do not run (the generator switches to type 8). `mutacoes.py --jogo YYYY-MM-DD` runs each mutation in the real game in comparison mode up to the date (about 95 s until 2001-08-15). The computer's clubs almost never have a designated taker: mutations only in the designated path pass (equivalent in practice); those in the corner and free-kick scores are flagged. Mutations: 18/18 in the self-test (one gap closed: the human manager's penalty list now holds the players' records), 4/4 in the real game. Real game (batches 55 and 56): 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original (c76ad0ff6ce3daa6).

**Shorter base.** From the 57th batch on, the mutation base runs with 200 thousand cases (about 3 minutes); the full base (2 million, about 35 minutes, `--base-completa`) is kept for every 5 batches.

### Match engine, 57th batch — throw-in, corner and penalty (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `lateral()` | 0x6c4290 | thiscall | match_eng.cpp: throw-in for side 0x8eae. Counts (0xf5c8 + 54 × side), puts the ball (0x8ea7, 0x8ea8) inside the pitch, picks the taker (type 2), arranges the 40 players by the formation table (0xf56e/0xf572 by the player's side; index: of the side taking?, position, ball column and row; byte = column + 16 × row) and puts the taker on the ball, facing inwards (90 or 270). With no taker the original shows an error box (no box here) and marks the engine (+0 = 1) |
| `escanteio()` | 0x6c4470 | thiscall | match_eng.cpp: counts (0xf5c7 + 54 × side), marks the play (0x8eb1 = 3, 0x8eae = −1) and arranges the 40: by the attacking role (0x716270, side taking) or defending role (0x716310), some go to drawn squares in the box (column 4 ± 1; row 5/6, or drawn 0..1/10..11), the others by the formation table (+0x3b or +0x30). Off the pitch: fatal error. Taker (type 3) at the flag (column 0 or 8 by a draw against 0x8ea7; row 0 or 11) |
| `penalti()` | 0x6c50d0 | thiscall | match_eng.cpp: taker (type 5) with the ball on the spot (column 4; row 1 or 10, also in 0x8ea7/0x8ea8); the stored goalkeepers go to goal, those on rows 0..1 go to row 2 and those on 10..11 to row 9 |

No synthetic self-test: the three arrange the 40 players on the grid (a synthetic generator would have to keep the grid consistent) and the corner looks up the club's tactics. Proof by the real game: mutations 8/8 with `--jogo 2001-08-15`; real game 10 billion comparisons, 0 divergences; career until 2002-06-01: Rust ×2 = original (c76ad0ff6ce3daa6).

### Match engine, 58th batch — formation squares and stars (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `gravar_casa(posicao, mascara)` | 0x5b5290 | thiscall, ret 0xc | formation.cpp: sets bit `mascara` (64-bit, a single one) in the position's square mask (+0x188b + 8 × position), first clearing its group (13 groups of mutually exclusive bits; two in the high dword), and zeroes +0x187f; returns 1. Position outside 0..=10 or unknown bit: the original shows an error box (no box here), zeroes 0xb67a34 and returns 0 |
| `marcar_craques(lado)` | 0x6d10a0 | thiscall, ret 4 | match_eng.cpp: sets bit 0x800000 in the side's formation for active players rated above 8500: 10 × (person +0x43 + +0x33 + +0x27) + form (+0x4d), by role (0x717410): goalkeeper out; 2 +500; 4 −1000 (−500 with 0x800 or 0x80); 8 −300 (without 0x800 and 0x80); 0x10 +1000; 0x20 +2000 |

Mutations: 12/12 (the error case of `gravar_casa` opens the game's box and stays out of the self-test). First batch with the 200-thousand-case base (194 s). Real game: 10 billion comparisons, 0 divergences. Career until 2002-06-01: Rust ×2 = original (c76ad0ff6ce3daa6).

### Match engine, 59th batch — designated marking (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `designado_da_posicao(posicao)` | 0x5b58e0 | thiscall, ret 4 | formation.cpp: the person designated for the position (index at +0x5bf + 4 × position, in the person array at 0xae23bc, 110 bytes each); negative index: 0 |
| `marcado_designado()` | 0x7160f0 | thiscall | match_pl.cpp: the opponent the player man-marks, or 0. Needs mark 4 on the position's square; looks among the active opponents for the record designated for the position. With mark 0x10: if the opposing formation has a centre forward (bit 0x10000000 of +0x187b) or the other side's byte +0xaf is above 1, a player with 0x106 does not mark; and, if the marked player has tables A and B different (arrow to another square) and B without the attack mark 0x40, a player with 0x106 does not mark either |

Mutations: 9/9. **Two traps.** (1) A hook on 0x714d30 (the player's side, 4 bytes: reads +0x27 and returns) froze the game at startup; such a getter is not worth a hook and was left out. (2) The test changed tables A and B, which are shared by the whole case, and a later section of the self-test stopped without using the CPU (probably an original routine's error box, invisible with the game hidden); the test now saves and restores the tables.

### Match engine, 60th batch — national team club (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `selecao_principal(nacao)` | 0x53b3d0 | cdecl | Database.cpp: the nation's main national team club. National teams sit at the end of the club array (0xae23b4, 581 bytes each): index = nation id − 2 × nations (0xae2350) + clubs (0xae235c). Without a nation, clubs or 0xae23a8: left to the original (error box) |
| `selecao_b(nacao)` | 0x53b4f0 | cdecl | Database.cpp: the B team club; index = id − nations + clubs |

Mutations: 5/5, with the full base (2 million cases, 0 divergences). Heavily called (over 600 call sites); 0 divergences in the real game in comparison mode.

### Match engine, 61st batch — who to mark (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `quem_marcar(jogador)` | 0x6bc030 | thiscall, ret 4 | match_eng.cpp, 3 KB (about 920 instructions): the opponent the player will mark, or 0. No marking with mark 1 on his square; an active designated marked player (0x7160f0) comes first. From the square (x, y) where he should be (0x6c2be0), evaluates the ball carrier (0xf582, if close: 0x968950) and the opponents in squares y−1..y+1 × x−1..x+1 (lists at engine + 0x215e, 90 bytes per square). Score: draws, surrounding opponents, distance to goal, attributes 0x0a/0x0b/0x10/0x35/0x101, role in the formation (0x8ebc/0x8ec4), reputation (home, current or world). The highest 16-bit score wins; the one kept at 0x1aa may be picked at once |

Mutations in the real game (`--jogo`): 11/11. The original `q_reput` added 1 to every score and does not change the pick (equivalent mutation); its replacement (`leva61b.py`) was caught. **Two traps.** (1) The comparison did not restore the RNG state between the original and Rust runs (`REGIOES_DO_ESTADO` missing from `lado_a_lado`): 50 thousand false divergences. (2) In the ball-carrier part, the original reads `[esp+0x1c]` with the draw's argument still on the stack: it is the distance to goal, not the distance to the player. Found by logging each candidate's score on divergences.

### Match engine, 62nd batch — human manager (0x9ace, 0x9a06) (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `tecnico_9ace(pessoa)` | 0x5fe040 | thiscall, ret 4 | human_manager.cpp: dword +0x9ace of the person's human manager record (or the current manager's, 0xb74c26 + 768 × [0xb77c7a]). No person or out of range: the original (error box) |
| `tecnico_9a06(pessoa, saida)` | 0x5fd6b0 | thiscall, ret 8 | human_manager.cpp: byte +0x9a06 ≠ 0? With a club (+0x24) whose nation has +0x88 < 400: false and `saida` = 1; `saida` starts at 0 |

First batch read from Ghidra's pseudocode (`descompilar.py`). Mutations: 6/6. `t_atual` (taking slot 0 instead of the current manager's) passed in the first round: when the wrong person does not resolve, the routine hands the case to the original, which gets it right, hiding the error. The self-test now puts another valid person in slot 0, and the mutation is caught.

### Match engine, 63rd batch — club controlled by a human (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_humano(clube, sem_tatica, qualquer, pessoa, forcar)` | 0x600a50 | cdecl | human_manager.cpp: is the club run by a human manager who decides for it? The manager (`pessoa`, or the club's at +0xcf) must be the club's and one of the 16 humans. Without `sem_tatica`, those with 0x9ace (0x5fe040) or +0x9a01 are excluded. With `forcar` or `qualquer`: yes. Otherwise, national team: 0x5fd6b0; club: 0x5fd480. If not, and the club is in the second set of national teams (0x5409b0), repeat with the nation's main national team (0x53b3d0), without `qualquer`. Null club: the original (error box) |

Mutations: 8/8 in the self-test (club, national team and second national team going back to the main one; the club's manager, another or none; with and without tactics). In the real game only 2/8 showed: the automatic career uses this routine little. **Trap:** editing `motor.rs` while `mutacoes.py` runs loses the edit, because at the end it rewrites the whole file as it was at the start.

### Match engine, 64th batch — side under control (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `lado_sob_controle(lado)` | 0x6d42a0 | thiscall, ret 4 | match_eng.cpp: not in matches of type +0x3a 1 or 9. The side's club run by a human (0x600a50, `qualquer` = byte +0x40 of the match): by mark 0x4000 of the side's block (0x9766 + 6371 × side). Otherwise: the side's mode at +0x1ab is 3 or 4 |

Mutations: 5/5 in the self-test (built on the 63rd batch's). Also: in the tests (hidden game), the DLL refuses `DirectSoundCreate` (DSOUND, imported by ordinal 1) with DSERR_NODRIVER; the game runs without sound and the career gives the same fingerprint.

### Match engine, 65th batch — club tactics table (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `linha_tatica(clube, humano)` | 0x8982b0 | stdcall, ret 8 (`this` unused) | tactics.cpp: the club's row in the tactics table. Club run by a human (0x600a50 with 1, 1) and `humano` set: second national team, −1; otherwise the manager's row after the clubs (0xae235c): people (0xae2364) − manager id (+0xcf) − 1, or + 15 for a national team. Out of range or no manager: the original (error box). Otherwise, the club id |
| `byte_tatico(clube, pos, humano)` | 0x898040 | thiscall, ret 0xc | tactics.cpp: byte `pos` (0..=10) of the tactics row in the table at +0x601 (0x91 bytes per row, from +0x86); row −1: 0 |

Mutations: 6/6 in the self-test. Also: `simular.py --som` keeps the game's sound (`CM_OCULTO=2`; one more environment variable would shift the heap and the career).

### Match engine, 66th batch — penalty taker order (official on 2026-10-08)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ordem_dos_penaltis()` | 0x6be520 | thiscall | match_eng.cpp: the order of the 11 takers of each side at +0x1af + 11 × side (−1 first), marking those chosen in each side's 20 slots at +0x8eda (zeroed first). Side's manager (+0x904d + 208 × side) human, with +0x5f, without 0x9a01 and (with +0x904b) national team or with 0x9a05: the order comes from the tactics table (0x898040 on [0xde1d08]). Otherwise, `escolher_cobrador` type 5; with no one, clears the side's marks and repeats the turn |

Mutations: in the real game `p_marca`, `p_tipo5` and `p_11`; in the self-test (table path) `p_marca` and `p_11`. `p_limpa` is equivalent (the loop rewrites all 22 slots). **Limitation:** `p_5f` (the manager's +0x5f mark check) is not caught: the automatic career does not take the human path, and the self-test does not send a side to `escolher_cobrador` with synthetic blocks. **Traps:** (1) the routine marks `+0x8eda + 20 × side + table byte` (signed); with random bytes in the synthetic table the mark fell up to 128 bytes around (formation pointers at 0x8ebc), and a later test opened the formation.cpp 3133/3174 error box, visible on screen because the self-test runs before the game's interface; the test now saves and restores that range. (2) Interrupting `mutacoes.py` in the middle of a mutation leaves `motor.rs` mutated: check the mutation lists before going on.

### Match engine, 67th batch — player order in the play (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ordem_dos_jogadores()` | 0x6bd760 | thiscall | match_eng.cpp: the list of active players (+0x19 ≥ 0) at +0x8f7e, count at +0x8eb8. For each slot 0..20 draws (0x90cfc0(2)) which side goes first. Sorts with the CRT `qsort` (0x945333) and the game's own compare (0x6bd840, which reads the engine at 0xb67a64) |

Mutations: 4/4 in the self-test (the one that does not write 0xb67a64 crashes the original compare: an exception, also detected).

### Match engine, 68th batch — tactics table slots (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `vaga_marcada(clube, humano, menos_um)` | 0x897590 | thiscall, ret 0xc | tactics.cpp: the slot (0..20) of the tactics row (0x8982b0) whose person (index at +2 + 4 × slot, in the 0xae23bc array) has the id kept at +0x52; none: −1 with `menos_um`, otherwise 0. Row −1: 0 |
| `marcar_vaga(clube, vaga, humano)` | 0x897760 | thiscall, ret 0xc | tactics.cpp: stores at +0x52 the index of the slot's person (0..20); 1 if there was someone |
| `pessoa_da_linha(clube, qual, humano)` | 0x897dc0 | thiscall, ret 0xc | tactics.cpp: the person of the index at +0x56 + 4 × `qual` (0 or 1); negative: 0 |

Errors (null club, slot out of range): the original (error box). Mutations: 7/7 in the self-test (synthetic people; indices always 0 to 29, so as not to read outside the synthetic array).

### Match engine, 69th batch — club in the competition (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_na_competicao(clube)` | 0x48db80 | thiscall, ret 4 | comp.cpp: is the club in the competition? First by nation: national team (id from clubs − nations) of another continent than the competition's (+4 → +0x59): no. Club of the competition's nation (+4 → +0x5d) and of its own division (+0x57): yes; of a nation of another continent: no. Then walks the competition (−1) and the stages (+0xc, up to +0x30): the last of type 2 (+0x43; table at +0xb1, rows of 0x3b) or, with none, the current stage (participants at +0xa7, 6 bytes); count at +0x3e |
| `clube_especial(clube)` | 0x76d940 | stdcall, ret 4 | is the club one of the nine kept at 0x9cf78c, 0x9cf790, 0x9cf79c..0x9cf7ac, 0x9cf8a0, 0x9cf964 (by id)? |

Mutations: 8/8 in the self-test. `e_lista` was only caught after the self-test put distinct values in the nine addresses: before the database loads they are all 0, and removing one from the list changes nothing.

### Match engine, 70th batch — referee's foul decision (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `decisao_da_falta(j, outro, tipo, quem, cartao, grav, vantagem)` | 0x6e9820 | thiscall, ret 0x1c | match_man.cpp, 3 KB: the referee decides foul `tipo` by player `j` on `outro`; returns 1 or 7 (card given) and writes the card (4, 5, 7, 10, 11), the severity and the advantage flag. `this`: +0 engine, +4 referee, +8 patience. Strictness factor from severity (+0x25), the side's mode (+0x1ad) and style (+0x27), the advantage and the score (0xf5c9 + 54 × side), clamped to 0.1..10; friendlies (0x5c6710) soften it. 19 foul types, each with its own draw against the factor; severity rises with hardness (+0x14) and falls with the minute (0x8ed0 / 11). Advantage: 20 (or 5) × card+0x5c^1.5 / (1 or 3), with the CRT `pow` |

Mutations in the real game: 8/10. **Limitation:** `f_vantagem` (divisor 3 when `j` himself last brought `outro` down) and `f_teto` (factor above 10) do not happen up to 2002-01-01 in the automatic career; there is no synthetic self-test for this routine.

### Match engine, 71st batch — choosing the teammate for the play (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `escolher_companheiro(bx, by, tipo, moveu, x, y, melhor, pedido)` | 0x6c6a30 | thiscall, ret 0x20 | match_eng.cpp, 7.4 KB (the largest so far, about 2,100 instructions and 24 `ftol`s): chooses the teammate and the kind of play (pass, cross, through ball, one-two; codes 7, 8, 0xb, 0x1e, 0x1f, 0x3c, 0x3d) for the ball carrier. For each teammate: free space (nearest opponent to him and to the point he can move to, 0x6bce80), role in the formation, position relative to the offside line (0x8eaf/0x8eb0), surrounding opponents, tactical marks (0x7163c0, 64-bit return), the manager's preference (0x897dc0), personal likes (0x547840/0x547810), a nation factor; then the kind of play by distance to the target (or the `pedido`), keeping the best score above `*melhor` |

Read through Ghidra's pseudocode and checked instruction by instruction. Points the pseudocode hid: `pow` with base (3 − surrounding count) and exponent 3; comparisons on the not-yet-rounded value after `fst dword` (the float is stored, but the double is compared); tests on bit 32 of 0x7163c0's return (`edx`). Mutations in the real game: 10/12 (the two that pass are nearly equivalent: comparing the rounded instead of the unrounded value, and +1 on a score that rarely ties). Zero divergences on the very first comparison.

### Match engine, 72nd batch — foul through the engine, human manager (0x9b23), favourite clubs (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `falta_no_motor(...)` | 0x6cfc60 | thiscall, ret 0x1c | match_eng.cpp: forwards to the foul decision (0x6e9820) with the engine's referee (+0xf5b2) |
| `tecnico_9b23(i, pessoa)` | 0x5fea50 | thiscall, ret 8 | human_manager.cpp: byte +0x9b23 + `i` of the human manager record; errors: the original |
| `gosta_do_clube(pessoa, clube)` | 0x547d60 | cdecl | preferences +0x04, +0x08, +0x0c |
| `nao_gosta_do_clube(pessoa, clube)` | 0x547d30 | cdecl | preferences +0x10, +0x14, +0x18 |

Mutations: 4/4 (the 0x9b23 one only after the self-test filled the bytes around +0x9b23 with varied values; with zeros, reading the neighbouring byte gave the same value).

### Match engine, 73rd batch — partner club (B team) (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_parceiro(clube, eh_a, ativo)` | 0x540a50 | cdecl | Database.cpp: A team ↔ B team; only clubs of nations whose id is in 0x9cf488/0x9cf314/0x9cf434; 82 pairs of globals (A→B accepts partner ≠ −1, B→A accepts ≥ 0); a refused partner zeroes +0x244; the partner record lives in the 0xae23b4 array (581 bytes) |

Mutations: 8/8 (the 82 pairs were extracted from the disassembly; the self-test gives distinct values to all 167 globals, with partners −1 and −5 to tell the two acceptance tests apart).

### Match engine, 74th batch — a person's contract with a club (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `contrato_com_clube(pessoa, clube)` | 0x4cf5f0 | thiscall, ret 8 | 80-byte records in [this]; a person has up to two indices (table 0xae2d0c, 79 bytes per person); the record whose +4 is the club id wins; with none, tries the partner club (0x540a50) when it is the A team |
| `nivel_do_contrato(pessoa, clube)` | 0x53ce90 | cdecl | (byte +0x4f of the contract via 0xae1050 & 0xf) − 10; national side or invalid person: the club pointer itself with the low byte cleared |

Mutations: 7/7 (the national-side limit one only after the self-test gave values to 0xae235c and 0xae2350, which are 0 before the database loads).

### Match engine, 75th batch — fog of war (what the human club knows about players) (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `chance_de_conhecer(pessoa, nivel)` | 0x544410 | cdecl | x = (word +4 + 2 × level) × 0.05 + 10; (x + 0.1 x²) × ⅓ + 0.5, clamped to 1..20 and truncated (`_ftol`) |
| `conhecer_pessoa(pessoa, fonte)` | 0x5b09d0 | thiscall, ret 8 | fog_of_war.cpp: a 9-byte record per id (level 0..3 and today's date, 0xae2c90, via 0x417c10); without an observer the level goes to 3; with one it draws by the chance (0x544410) |
| `nevoa_apos_partida(partida, dados)` | 0x5afe00 | stdcall | fog_of_war.cpp: for the human sides (0x600a50) with a knowledge table (0xae2a60), the 20 line-up slots become known |

Mutations: 13/15. Two equivalent: multiplying by ⅓ (constant 0x968a80) instead of dividing by 3 almost never changes the truncation; and the `id < 0xae2364` limit in 0x5afe00, which 0x5b09d0 checks again. Swapping the sides only showed with different clubs on the two sides.

### Match engine, 76th batch — shortlist entry, contract flags (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `entrada_da_lista(pessoa, alvo, nao_11, tambem_13)` | 0x849ce0 | thiscall, ret 0x10 | shortlist_manager.cpp: the entry (40 of 37 bytes, pointer at [this+0xdc] + id × 9) whose +0xf (or +0x13) is the target id, with type (+0) 11 or other than 11 |
| `bandeiras_do_contrato()` | 0x8676f0 | thiscall | staff_contracts.cpp: +0x40 (a 4-byte routine; the hook covers the first alignment nop) |

Mutations: 6/6.

### Match engine, 77th batch — next list item, word +0xc, rounding, group nations, markers (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `proximo_da_lista()` | 0x68d1b0 | thiscall | cursor at +0xa (unaligned): returns the node's +0 and advances to its +6 |
| `palavra_0c(id)` | 0x62c3d0 | thiscall, ret 4 | injury.cpp: signed word +0xc of the 31-byte record; id from 0xae2364 on: 0 |
| `arredondar_abaixo_de_1000(v)` | 0x4b0870 | cdecl | comp_util.cpp: `v` minus the (signed) remainder by 10 of the low word, when it is below 1000 |
| `nacao_do_grupo(nacao)` | 0x7e6f20 | stdcall | is the nation one of the six at 0x9cf794, 0x9cf684, 0x9cf798, 0x9cf6b0, 0x9cf5d8, 0x9cf548? |
| `quantos_marcam(alvo)` | 0x6d59b0 | thiscall, ret 4 | how many of the 20 on the other side (active) man-mark the target (0x7160f0) |
| `quem_marca(alvo)` | 0x6d5a10 | thiscall, ret 4 | the first of them, or 0 |

Mutations: 12/13 (one equivalent: `< 1000` becoming `<= 1000`, since 1000 has remainder 0). The markers join the 59th batch self-test, with valid engine and side on the 20 of the other side during the calls.

### Match engine, 78th batch — competition that counts, sort comparisons (comp_util.cpp) (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `competicao_conta(id)` | 0x4b0700 | cdecl | comp_util.cpp: the three at 0x9cf6b0/0x9cf5d8/0x9cf548 yes; without +0x5d of the record (0xae23d0, 107 bytes) no; otherwise only the object (0xadadfc) in phase 2 (+0x43), with rounds (+0x3e) and the first round (+0xb1) holding club +4 at +0x57 or +0x60 |
| `ordem_por_5f(a, b)` | 0x4b0780 | cdecl (qsort) | byte +0x5f of b − that of a (null counts as 0) |
| `ordem_por_80(a, b)` | 0x4b07b0 | cdecl (qsort) | word +0x80 of b − that of a |
| `ordem_por_80_desc(a, b)` | 0x4b07d0 | cdecl (qsort) | same, with nulls last |
| `ordem_por_80_cresc(a, b)` | 0x4b0810 | cdecl (qsort) | ascending, with nulls last |

Mutations: 11/11.

### Match engine, 79th batch — player rating (contract, form and age) (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `nota_do_jogador()` | 0x6ebd70 | thiscall | match_pl.cpp: n = contract level (0x53ce90); form = (word +0xd of the record − club word +0x80) × 0.002 + 1.5, clamped to −4..4; c = contract byte +0x44 (0x4cf5f0) / 75 + 0.5, adjusted by type (+0x4f >> 4: −10, +2.5, +2, −3, −15, −20, −50); x = byte +0x22 × 75 for a national side, otherwise days alive (0x54cdb0); y = 1 + (x − 1000) × 0.000125 above 1000. Rating = byte +0x2f × y − (n² + 5) × 0.1 + form + c + 0.5, truncated |

Mutations: 10/10 (the one on the sign of contract byte +0x44 only after the 74th batch self-test filled that byte; it used to be always 0).

### Match engine, 80th batch — forget a person and the manager's table (fog of war), formation slots (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `esquecer_pessoa(pessoa)` | 0x5afd00 | thiscall, ret 4 | fog_of_war.cpp: with fog of war on (0x9cc270) and a valid person, zeroes their level in the 16 active tables (+4) |
| `quadro_do_tecnico(tecnico)` | 0x5afde0 | thiscall, ret 4 | fog_of_war.cpp: [this + (0xae2364 − id) × 4 − 4]; no manager, 0 |
| `casa_do_setor_ocupada(mascara, jogadores)` | 0x5b2e80 | thiscall, ret 8 | formation.cpp: the first of the 11 slots (+0x12d) with the mask sector (0x5b73a0) and bit 0x200; without players, 1; otherwise whether any of the 20 (+0x19, or +0x1a with 0xfe) is in it |
| `vaga_da_casa(codigo)` | 0x5b2f10 | thiscall, ret 4 | formation.cpp: slot 0..10 of the code, or 0xff |
| `limpar_designados()` | 0x5b5960 | thiscall | formation.cpp: marks (+0x5b4) with 4 and 8 become 2; assigned records (+0x5bf) go back to −1; returns 1 |
| `mascara_da_formacao()` | 0x5b59a0 | thiscall | formation.cpp: +0x187b = OR of the bits of the 11 slots (0x5b5a60), with 11 masks swapped for 4 reference formations |

Mutations: 13/13.

### Match engine, 81st batch — slot code, competition criterion (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `casa_da_vaga(vaga, segunda)` | 0x5b2ae0 | thiscall, ret 8 | formation.cpp: word +0x12d (or +0x14e) of slot 0..10; out of range, the original (error box and 1) |
| `criterio_da_competicao(registro, a, b, tipo)` | 0x4bff70 | cdecl | comp_util.cpp: with the low word of b (or of a), 0x4ba8f0(word, 0); otherwise byte +0x37 (type 1) or +0x52 of the record; no record, 1. The word path depends on the configuration at 0xae3138 and is covered only by the real game |

Mutations: 5/5 (the one on the word of `a` crashes 0x4ba8f0 without configuration, which counts as caught).

### Match engine, 82nd batch — person types, national side class, per-person marks, free record (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `tipo_nao_11(tipo)` | 0x53b610 | cdecl | Database.cpp: type ≠ 11 |
| `tipo_11_a_15(tipo)` | 0x53b620 | cdecl | Database.cpp: type 11, 12, 13 or 15 |
| `classe_da_selecao(clube, flag)` | 0x76d780 | stdcall | national_teams.cpp: with `flag`, 1; without it, 2 for the second national side (0x5409b0) and 0 for the others; returns only `al` |
| `ler_marca_da_pessoa(pessoa, k)` | 0x76d7b0 | thiscall, ret 8 | national_teams.cpp: byte [this+4] + id × 110 + k × 20 + 0x45; returns only `al` |
| `zerar_marca_da_pessoa(pessoa, k)` | 0x76d7e0 | thiscall, ret 8 | national_teams.cpp: zeroes that byte |
| `registro_livre()` | 0x53a5b0 | cdecl | Database.cpp: the first record with +0 zero in table 0xae23f8 (60 bytes, 0xae23a0 records), cached at 0x9b59b4; with none, a random one |

Mutations: 9/9.

### Match engine, 83rd batch — more per-person fields (national_teams.cpp), global 0x9b5984 (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ligar_44_da_pessoa(pessoa, k)` | 0x76c0f0 | thiscall, ret 8 | writes 1 to byte +0x44 of the person's block k ([this+4] + id × 110 + k × 20) |
| `byte_44_da_pessoa(pessoa, k)` | 0x76e010 | thiscall, ret 8 | the same byte, signed |
| `campo_3a_da_pessoa(pessoa, k)` | 0x76e040 | thiscall, ret 8 | the address of field +0x3a of the block |
| `classe_pela_selecao(pessoa)` | 0x76e070 | stdcall | no national side (+0x24), 0xff; second national side (0x5409b0), 2; otherwise bit 3 of byte +0x5e; returns only `al` |
| `global_9b5984()` | 0x535920 | cdecl | Database.cpp: reads global 0x9b5984 |

Mutations: 8/8.

### Match engine, 84th batch — person's competition, national side block bytes, special national sides, date with mark (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `competicao_da_pessoa(pessoa)` | 0x7751e0 | thiscall, ret 4 | national_teams.cpp: the competition (0xae23d0, 107 bytes) of index +4 of the person's record ([this+4], 110 bytes); no person or a negative index, 0 |
| `byte_42_da_selecao(clube, flag)` | 0x7754c0 | thiscall, ret 8 | byte +0x42 of the class block (as in 0x76d780) of the club's nation (+0x53); the original calls 0x5409b0 twice |
| `byte_43_da_selecao(clube, flag)` | 0x775520 | thiscall, ret 8 | same, byte +0x43 |
| `selecao_ou_especial(clube)` | 0x775790 | stdcall | national side (0x53b900) or one of the clubs at 0x9d00a4/0x9d00a8 |
| `nao_gosta_da_selecao(pessoa)` | 0x776380 | stdcall | does the person dislike (0x547d30) the main national side (0x53b3d0) of nation +0x1a? |
| `copiar_data_e_marca(origem)` | 0x777590 | thiscall, ret 4 | copies the date (0x417c10) and byte +8; returns `this` |

Mutations: 8/8 (the 0x9d00a8 one only after the self-test put a non-national-side id there; with 4850, the national side test answered first).

### Match engine, 85th batch — player attributes in the match (0x6ec010, ~11 KB) (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `atualizar_atributos()` | 0x6ec010 | thiscall | match_pl.cpp: recomputes word +0x3b and the 34 f32 fields from +0x71 to +0xf5 of the player: the designated player's rating (0x6ebd70, slot 0x897590), age, days alive, contract (0x4cf5f0, +0x45 bits, +0x10/+0x14 history), morale, tactical tables A/B (0x6c38c0/0x6c38f0), formation (0x6d5b20/0x6d5af0), managers and preferences (0x547810/0x547840/0x547d30/0x547d60), powers (0x945ff0) and random draws. Only the player record changes |

Mutations: 10/12 in the real game (9 up to 2001-08-20; the contract bit 0x40000 one only by 2002-01-01). Two not reached even by 2002-01-01: tactic position 5 (bit 0x40 only) and the byte +0x1b cap for match types 1/8/32; left for Lucas's real career.

### Match engine, 86th batch — player performance in the match (0x6eecb0, ~6.4 KB) (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `desempenho_na_partida(marcar)` | 0x6eecb0 | thiscall, ret 4 | match_pl.cpp: calls 0x6ec010; word +0x37 from tactical tables A and B against record bytes +0x10..+0x1a (with `marcar`, adds 1 or 2 to the side byte +0xaf in the engine); bytes +0x10e..+0x117 (at least 2); ~60 derived words from +0x11a to +0x196 (T = x·10 + 0.5, Q = (x² + 1)/2, integer averages); 63 of them adjusted by +0x37 through a table of 8 formulas; finally 0x6f44a0 |

Mutations: 11/12 in the real game up to 2001-08-20 (the missing one needs record byte +0xf to be exactly 15).

### Match engine, 87th batch — side strength (0x6bbb60) (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `forca_do_lado(lado)` | 0x6bbb60 | thiscall, ret 4 | match_eng.cpp: clears word +0x1ce + 2 × side and the side byte +0xaf; for each of the 20 active players, −500 when the slot bit (0x717410, 0x5b5a60) already appeared (the mask adds, not ORs), adjustments by rating (0x6ebd70) per contract (0x4cf5a0 into 0xae1080, 0x4cf5f0), three draws and the tactic's designated player (0x897590); finally 0x6eecb0 with mark for each active one |

Mutations: 5/8 in the real game. The other three (mask added vs ORed, the −750 and the designated player's −3×) depend on the contract branch with three draws and bits 0x17a0b, which the automatic career does not reach even by 2002-01-01; left for Lucas's real career.

### Match engine, 88th batch — ball in attack, moment evolution, match of interest; prologues with a relative instruction (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `bola_no_ataque()` | 0x714d00 | thiscall | match_pl.cpp: already in Rust, not hooked; now hooked (prologue with a relative `jne` rebuilt in the trampoline) |
| `evoluir_momento()` | 0x6f55c0 | thiscall | match_pl.cpp: 0x6f44a0; 1 in 11: +0xff drops 1 above (+0x100/2 + record +0x25 + 10)/4, +0x4d adds +0x59 (which clears if 1 or halves), above 50 in +1 subtracts (+1 − 50)^1.25/4 (up to 25); +0x4d within 1..10000 (prologue with a relative `call`) |
| `partida_de_interesse(partida)` | 0x6b61e0 | cdecl | match_day.cpp: does the match interest any of the 16 human managers? By competition (+0x14): national sides, friendly (0x5c6710), round 150 (0x4b0870), nation (+0x5d) or continental competition (0xae23a4) |

Mutations: 5/8 in the real game up to 2001-08-20; `i_amistoso`, `i_principal` and `i_16` survived: the only human in the automatic career (test manager, on holiday) has no club, so the club/friendly branches never fire and there is no 16th manager. Left for a real career.

### Match engine, 89th batch — group decided, qualification commentary (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `grupo_decidido(clube, partida)` | 0x6854a0 | thiscall, ret 8 | league.cpp: with round +0xa9 ≥ 24 in a counting competition, the club leads a sorted copy of the table (0x687710/0x685f60/0x6827d0/0x687970) and nobody can catch it with the remaining games (+0xa7 − played) × points per win (+0xc2) |
| `narrar_classificacao(lado)` | 0x6be120 | thiscall, ret 4 | match_eng.cpp: comments 0x2171/0x2170 (qualification settled: group or knockout via 0x6ce310), 0x2120 (ahead on aggregate), 0x2121 (starters' average +0x1b below 7.0 or 6.67 and the opponent's strength +0x80 weighing; 0.8 for the away side), 0x2001 (side 1, game still open) |

Mutations: 1/8 in the real game up to 2001-08-20 (`n_2001`). The other seven sit in branches that do not occur before August: group decided only from round 24, and the +0x8ebb section (knockout aggregate). They go to a longer mutation run.

### Match engine, 90th batch — who tries the play, players on the path; recording of 0x70dfd0 (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `procurar_jogada(lado, grupo, ..., modo, ...)` | 0x6cd3d0 | thiscall, ret 0x1c | match_eng.cpp: in the side's list in the cell (+0x58 players), who can act (+0x2b up to 1, +0x19 ≥ 0); mode 2 only counts; otherwise each one tries the play (0x70dfd0, arguments 4, 3, 6, 7): 1 = succeeded (+0x12 goes up), 2 = other outcome; the player goes into +0xf5ae. Compared by recording and replaying 0x70dfd0 (a port that also records the regions the comparator asks for) |
| `jogadores_no_trajeto(x1, y1, x2, y2)` | 0x6cc070 | thiscall, ret 0x10 | match_eng.cpp: the 0x6cbc00 walk (now `percorrer_linha`, shared) counting through 0x6cd3d0 in mode 2, with both ends on straight lines and x up to 11 on the horizontal |

Mutations: 7/9 in the real game up to 2001-08-20. `j_ret2` survived (the play's answer 2 did not occur before August) and `t_11` is equivalent in practice (x from 9 to 11 is already off the 9-column grid).

### Match engine, 91st batch — interception on the path to goal; allocation-free comparators (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `interceptar(x, y, a3, gol, a5, a6)` | 0x6cc540 | thiscall, ret 0x18 | match_eng.cpp: cells from (x, y) to the goal centre (x 4; y 11 with `gol` = 1, else 0), Bresenham style in the four quadrants; in each cell it draws (2) which side tries first and searches both through 0x6cd3d0 in mode 1 (table distance from the start, or 1000); returns the first code ≠ 0; +0xf5ae is cleared at the start. Compared with the 0x70dfd0 tape covering the whole grid and the bytes of every player |

Mutations: 7/7 in the real game up to 2001-08-20 (six of them redone after fixing `mutacoes.py`).

### Match engine, 92nd batch — advance one row; generic recording tape (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `avancar_linha(direto)` | 0x6f5b80 | thiscall, ret 4 | match_pl.cpp: the player moves one row towards his side (+0x27 = 1 goes up to 11, else down to 0); with `direto` he goes (0x6f46b0) to the current column on the ball's row (+0x8eb0 + 1 or +0x8eaf − 1 of the engine), otherwise he is repositioned (0x6f09f0) on the new row with −1, 0, 0. Compared through the generic tape: 0x6f09f0 got a recording port and 0x6f46b0 records through its own comparator |

Mutations: 4/6 in the real game up to 2001-08-20. `a_11` and `a_0` (row limits on the 0x6f09f0 path) survived: the path happens (`a_direto` and `a_menos1` were caught), but no player was on the edge row.

### Match engine, 93rd batch — line up for kick-off (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `arrumar_saida()` | 0x6c4ec0 | thiscall | match_eng.cpp: who kicks off (+0xf59e side 0, centre 4, 0; +0xf5a2 side 1, centre 4, 11); releases the ball (0x6f0970); every player on the pitch goes to his formation cell (table +0xf56e of the direction, byte +0xa2 of the position: x in the low nibble, y in the high one), the defensive lines are redone (0x6c3aa0) and, on the kicking-off side, whoever crossed the line steps back one row (0x6f5b80); +0x2b is cleared; finally the kicker goes to the centre and takes the ball (0x6f0630), +0x19c = 200, +0x19a = 180 off row 0. Without him: the original (error box). Compared with `lado_a_lado` over the engine from 0x1a0, the grid, the 40 players and the 0xf56e pointers |

Mutations: 6/6 in the real game up to 2001-08-20.

### Match engine, 94th batch — line up for a set piece (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `arrumar_bola_parada()` | 0x6c4820 | thiscall | match_eng.cpp: counter +0xf5c6 + 54 × side; releases the ball. Ball in the side's own area: formation (as 0x6c4ec0) and whoever of the other side lands on the spot moves one row. Otherwise: spot clamped to the grid, far (+0x8eb1 = 7) or near (−1), base column 4 − draw + draw, base row 3/4 from the spot; near, everyone from the set-piece table (12 rows × 9 columns per position and kicking side); far, by role (0x716270 in attack: 1 deep, 2 middle; 0x716310 in defence: 4 deep, 3 middle). Finally the taker (0x6ceb90, type +0x8eb2) goes to the spot, takes the ball and aims (0x6f4920). Cell off the grid: fatal error, as the original |

Mutations: 8/8 in the real game up to 2001-08-20.

### Match engine, 95th batch — the off-ball player decides where to go (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `decidir_sem_bola(_)` | 0x713fc0 | thiscall, ret 4 (argument unread) | match_pl.cpp, ~3.3 KB: action in +0x198 and the cell. Ball out of play: formation (0x6c2a40, 0x68). Set piece: the taker (+0xf57a) or whoever the side points to goes to the spot (0x100). In play: near the opponent's area, or with position mark 0x200000, tries to move up to a free cell (0x6bce80; 0x6a, +0x35 up by 7) by a draw against (1 − 0x35·c)·0x101·0x107; otherwise starts from 0x6c2be0's cell (x through 0x715f50), wingers go to the corner by the ball (marks 0x80/0x800 of `tabela_a`), the line goes deep by a draw or by the line (0x6cb700) and moves by the side difference (0x6c4180); marking someone (+0x1aa, 0x105) follows his cell, one step ahead when draw(21) beats his 0xad; otherwise advances a row (0x6f5b80, 0x6b) by the player difference; at the cell: stays (0x76) or looks for a free cell (0x6a/0x69); away from it: free cell or goes (0x68). Compared through the tape (0x6f09f0, 0x6f46b0 and 0x6f5b80 itself as a unit) |

Mutations: 8/10 in the real game up to 2001-08-20. `s_formacao` (the 0x7103e0 dispatcher only calls with the ball in play) and `s_fica` (the action written before the call was wiped by the after snapshot) survived; batch 96 brings the before snapshot and redoes both.

### Match engine, 96th batch — general off-ball movement; commentary and ball pick-up on the tape (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `mover_sem_bola()` | 0x714d40 | thiscall | match_pl.cpp, ~4.7 KB (outside the attacking band; inside it, with the ball in play, it is 0x713fc0): set piece (spot, 0x100), formation (0x68), run beyond the offside line (0x7a, by record +0x105/+0x106/+0x10a/+0x10b and the kept goalkeeper), support and pressing (0x100), man-marking (+0x1aa, 0x105, or drop), pressing threshold (1.5 or ~2, reduced without +0x106/+0x10a and by the draw against +0x1a2). Goalkeeper: picks up and comments (0x1f46, 0x103), comes out (0x103) or returns (0x71). The others: deep (0x78), advance (0x6b), go to the ball (0x100; on it picks up and comments 0x1f46/0x2113, or 0x104), stay in the opponent's area (0x104), cell (0x68/0x76) |

Mutations: 9/12 in the real game up to 2001-08-20, including batch 95's `s_fica` and `s_formacao`, now caught by the before-call snapshot. Survivors: `m_goleiro_9` (no kept goalkeeper), `m_2113` (set-piece comment near goal with +0x1cd) and `m_largar` (marked player at position −3): situations that did not occur before August.

### Match engine, 97th batch — each player's decision at every tick (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `decidir_jogador()` | 0x7103e0 | thiscall | match_pl.cpp: off the ball, "switches off" (0x67) when draw(10) + +0x4d·c does not beat draw(65); with the ball out of the side's reach, draw(20) zero and no +0x10d switches off tiring (+0x35 − 15, +0x2b = 1, +0x4d + 5..9); with the side in situation 5 (+0x1ab) and far from goal, switches off without the −15; with +0x2b, idle. Before each outcome the marked players (+0x1ae) move (0x714d40) by draw(900) against +0x107·+0x104. The ball holder dribbles (0x7109f0, new tape port) and may decide again (draw(+0x7d) against draw(5)); the others go to 0x713fc0 (ball with the side, in the band, in play) or 0x714d40 |

Mutations: 8/8 in the real game up to 2001-08-20. Rust ×2 and original careers equal at `cd2c213e0ddda4d1` (2,357,622,872 draws): the reference changed because the language files (`jogo\*.lng`) were rewritten at 13:04 on 2026-10-10, outside this work.

### Match engine, 98th batch — the pass and offside (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `passar(tipo, receptor, saida, a4, a5)` | 0x6fa250 | thiscall, ret 0x14 | match_pl.cpp: the pass (+0x2b goes up). Executes through 0x6fad40 (13.8 KB, new tape port: records x, y, type, receiver and the 4th argument and returns the answer with the byte it writes into the out slot). Outside type 0x36 looks for the offside player (0x6cfd70); draw(2·d1·d2 of his distance to goal) zero: offside, comment 0x1f78, count (+0xf5ca), offside player's +0x13, −1 in the passer's +1, ball to the other side (+0x8eb2 = 7). Otherwise logs the event (0x6bb9a0). With the pass out byte: the receiver becomes the taker (+0xf57a), possession (+0x8d46) and the receiver's immediate decision (0x7103e0) by draw(80) against +0x105·+0x107 |

Mutations: 8/8 in the real game up to 2001-08-20, with nested comparison (before it `p_2b` survived: the pass was never compared).

### Match engine, 99th batch — the long ball (official on 2026-10-09)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `lancar(modo, refinar)` | 0x6fa490 | thiscall, ret 8 | match_pl.cpp: hoof, cross or long ball. The direction (+0x19a, degrees) marks which sides the ball goes to; near goal and off the centre, cross (0x5e) to row 0..1/10..11 and column 3..5; in the own half 0x5c (mode 4) or 0x5d; from midfield forward 0x5b (mode 4) or 9/0x3e (ball in play); the offsets are draws of 2 to 5 around the spot. Without `refinar`, far from the own goal and with +0x10b or draw(3), the target goes halfway to the wing (column 1 or 7) and the box. Executes (0x6fad40), offside as 0x6fa250, ball off the grid: 0x6cd490 (a tape unit) |

Mutations: 9/9 in the real game up to 2001-08-20. Rust ×2 and original careers at `cd2c213e0ddda4d1`.

### Match engine, 100th batch — the deflected ball (official on 2026-10-10)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `desviar_bola(x, y, espalhar, lado, dentro, cansa, chutador)` | 0x6cd770 | thiscall, ret 0x1c | match_eng.cpp (13 callers): releases the ball and it bounces off (x, y): over the sideline (column 9 or −1) off the centre by draw(4), or at x, y ± draw(spread + 1) (with `dentro`, until the row fits). Height +0x8eaa, spin +0x8ed8 from table 0xb67c30 by the offset, +0x8ea9 = draw(4), possession of `lado` (+0x8d46). With `cansa`, draws into the players' +0x2b (0x7160d0). On the goal line next to goal with the `chutador`: 0x70c040 or 0x70a910 (new tape ports); then the ball out (0x6cd490) |

Mutations: 8 of 8 caught (b_lateral, b_espalhar, b_dentro, b_altura, b_efeito, b_estado, b_cansa, b_fundo); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Match engine, 101st batch — the goalkeeper save (official on 2026-10-10)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `defender(forca)` | 0x70d430 | thiscall, ret 4 | match_pl.cpp: above 1000, or without winning draw(record +0x21 + +0xa5) against draw(20), no save (0x1fe2, returns 1). Otherwise parries (+0x106 above draw(8): 0x1fe3, ball deflected 3 cells) or blocks (0x216f, 1 cell; tires +0x35 − 250 and may lose draw(3) in +0x17 with performance redone, 0x7172d0/0x6eecb0); +0x2b adds draw(2 or 3); the ball stays in play with draw(10) and the keeper is the goal-line taker with draw(200) zero. The deflected ball (0x6cd770) is a tape unit when called directly by the compared routine |

Mutations: 6 of 8 caught (g_20, g_espalma, g_2b, g_dentro, g_espalhar, g_cansa); g_1000 (force exactly 1000) and g_17 (block with draw(30 × v + 20) zero) survive, rare paths; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Match engine, 102nd batch — the player stamina (official on 2026-10-10)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `recuperar_folego(teto)` | 0x6f5810 | thiscall, ret 4 | match_pl.cpp, no draws: recovers stamina (+0x4d) and its ceiling (+0x51). Factor 1, or 1.4 − 0.0004 × injury (0x62c3d0) above 1000. Without `teto`, adds 10 × (record pace +0x34 + 15) × factor on the pitch (0x6c38c0 = 1) or off the bench (12 × pace + 325 with +0x1f = −1), up to 10000. With `teto`, +0x51 = stamina + (12 × pace + 85) × factor, times 5.56 × (+0x20 or +0x21) − 255.76 when that value is 46 to 64; out of play, if the injury (0x62c590) exceeds the ceiling it becomes 6500; finally adds +0x59, up to 10000 |

Mutations: 8 of 9 caught (f_campo, f_banco, f_fora, f_k, f_marca, f_copia, f_6500, f_59); f_1000 survives (injury exactly 1000, boundary); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Match engine, 103rd batch — the end of a period (official on 2026-10-10)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `periodo_encerrado()` | 0x6bd930 | thiscall | match_eng.cpp (`period_completed`): closes the period in +0x8ed4 when the clock (+0x8ed0) passes its end plus stoppage (+0x8eb7). 1st half: half-time score into the fixture (+0x4792 +0x45/+0x46), narrates 0x20f1, recovers stamina and moves to the 2nd (1). 2nd half: by the fixture rule (+0x38) and aggregate (+0x4b/+0x4c), ends (4: standings 0x6be120, stamina ceiling, summary 0x6ce460; penalties in +0x8eb3) or goes to extra time (0x20f2, 2). Extra time: 0x20f3 and 2nd half (3), then ends. Unknown period: logs the error (0x944c9f). Standings and summary become tape units when called directly by the compared routine |

Mutations: 9 of 9 caught (p_passou, p_intervalo, p_20f1, p_3de, p_teto0, p_saida, p_placar, p_fim, p_ordem); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Match engine, 104th batch — the pressure on a player (official on 2026-10-10)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `pressao(j, saida, lista)` | 0x6bcbd0 | thiscall, ret 0xc | match_eng.cpp, no draws: adds into `*saida` the pressure of active opponents in `j`'s cell and returns how many count (2 per marker). Whoever marks `j` (0x714c60) is worth (0x6f4600 + 0x10a − +0x19c/20 − +0x107 of `j`) × 1.7; the others standing, outside table 1 (0x717410) and with +0x10a or bit 0x100 of +0x198, are worth (0x10a − …) × 1.7 / (+0x2b + 1). With `lista`, the two heaviest (above 5) go into it as 0x1be-byte copies (0x68d0d0, a tape unit so the Rust copy does not allocate again) |

Mutations: 9 of 9 caught (x_lado, x_base, x_marca2, x_dobro, x_2b, x_bit, x_5, x_segundo, x_ordem); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1). The first attempt read 0x66666667 + sar 3 as /10 (it is /20): the baseline caught it at once.

### Match engine, 105th batch — the opposition challenge (official on 2026-10-10)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `desafio_adversario(atacante, lado, tipo, a, b, nota, lance, arriscado, livre)` | 0x6c87e0 | thiscall, ret 0x24 | match_eng.cpp (`get_opposition_challenge`, ~5.3 KB): among active opponents in the ball cell, who challenges move `tipo`. Distance to goal (table 0xa15e44) gives the difficulty; each one has a weight (tactical marks 0x7163c0 or the side strategy, marking +0x1a6/+0x1aa, draw by reading +0x106 and the attacker +0x19c, form +0x24/+0x75, stamina +0x4d, minimum 0.05) that divides each type's k (0xc–0xe, 0xf/0x10/0x67, 0x11/0x12, 0x15/0x25/0x2e/0x38 with the 5th power of how many are in the cell, 0x3b, 0x6c, 0x6d) in a draw against +0x10d + +0x10a − distance. The winner gets a code and alternative moves; offside (0x6cb780) and position set `*arriscado`; value (0x7165b0, 0x5407e0, 0x6f5b20) picks the move and the domain (0x70d690) gives the score. The best score goes to `*nota`/`*lance` and tires (0x6f56c0). The weight stays in double precision between steps, as in the x87 register |

Mutations: 17 of 18 caught; o_marca survives (the 2.25 weight of whoever marks the attacker, +0x1a6: path not exercised up to 2001-08-20, checked against the disassembly); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Match engine, 106th batch — the player development after a match (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `evoluir_jogador(jm, pessoa, clube, rep, comp, nota, media_b, forte)` | 0x6d26b0 | cdecl | match_eng.cpp (~3.9 KB): the three record ratings (+0x61: +0x9, +0xb, +0xd) move with the match rating. The level comes from the competition (−2: 5; +0x40: mean of reputation/500 and +0x69; national sides 0x76d940 at least 15; up to 20); each rating converges to rating + (level/2 + 5) × deviation × 50 (deviation against the league mean 0xb74350, signed square below it), with divisors from the rating's thousands + 16 minus the level; `forte` sets 250 × level × rating/mean as a floor. Floors rise with reputation and rating in big matches (+0x30/+0x32 of the competition). The +0xb change gives morale by draw up to ±20 (0x7172d0 on the match player or 0x544710 on the person); then ratings are clamped to the potential (+0x5) and age band (250/300/375/500 × (2 − rating/10000)) and +0xb pulls toward the club reputation (0x4cf5f0, CRT pow cubed). With `jm`, writes its +0x47/+0x49/+0x4b |

Mutations: 13 of 15 caught; e_nivel20 (level cap 20, level-20 competitions do not appear up to 2001-08-20) and e_div9 (+9 divisor below 5, rare outside big matches) survive; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Match engine, 107th batch — the kit (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `uniforme(clube, outro, qualquer)` | 0x6d47e0 | cdecl | match_eng.cpp: which of the club's three kits (colour pairs +0x83/+0x87, +0x8b/+0x8f, +0x93/+0x97) to wear against the other club. Without either club, logs the error and returns 0. 0x48ce70 (returns 0) would decide to swap colours (0x48ce90). Without `qualquer`, a pair works if its second colour does not clash (0x540490) with the other's (+0x87, or the table 0xae23ec entry by index 0x9cf17c); returns the pair number, 1 with a zero colour, 2 if the 2nd/3rd kit is missing, 1 if none works |

Mutations: 4 of 5 caught; u_nenhum survives (all three kits clashing, does not happen up to 2001-08-20); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Match engine, 108th batch — the injury leaves (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `gravidade_da_lesao(tipo)` | 0x62ec00 | thiscall, ret 4 | injury.cpp: the severity byte of injury type `tipo` (13-byte-per-type table at 0x9e7ad9) |
| `grau_da_lesao(pessoa)` | 0x62ece0 | thiscall, ret 4 | injury.cpp: the injury grade of the person (31-byte record per id in the injury table, +0x13, at least 1), or 0 with no person, no record (+0x61), id outside the table (0xae2364) or no injury (+0x12 negative) |

Mutations: 2 of 3 caught; l_sem survives (record with +0x12 exactly 0); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1). The 108th and 109th prepared earlier (0x6d3de0, 0x6d3610) were dropped: they were already ported.

### Match engine, 109th batch — the tackle injury (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `lesao_na_dividida(a, tipo_a, b, tipo_b, direto)` | 0x6cfe10 | thiscall, ret 0x14 | match_eng.cpp (~2.6 KB): a tackle between `a` and `b` may injure. Each has a chance (draw(draw(102)), redrawn above 50) and a risk of 4 (6 when already injured, severity 0x62ec00, chance doubled in 24/25) adjusted by stamina (10000/max(1000, +0x4d/2 + 5000)), the record (+0x2d, always `a`'s choosing the branch) and the injury grade (0x62ece0). `a` with no stamina (< 4000) is injured at once. With `direto` only `a` risks; otherwise the ball holder adds risk by the other's move (+0xd9 × factor), move 5 divides by +0x79, and the type comes from severity or a draw. The current injury (0x62ed70, which first updates it via 0x62c680) and the applied injury (0x6d0980, which allocates) are tape units |

Mutations: 6 of 9 caught; d_ramo (record +0x2d exactly 11), d_grau (injury grade above 3) and d_5 (+0x79 up to 2 on move 5) survive, rare values up to 2001-08-20; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (cd2c213e0ddda4d1).

### Dates, 110th batch — the day difference between dates (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `dias_ate(outra)` | 0x54cdb0 | thiscall, ret 8 | Date.cpp (`rust/src/data.rs`): this − other in days, with `outra` by value. Within the same year it is the day difference; across years, the static Date 0xae2470 takes *this and walks year by year to `outra`'s, adding each year's days (backwards those of the reached year; forwards, as in the original, those of the year before each), and keeps `outra`'s year. `mutacoes.py` now accepts `ARQUIVO = 'data.rs'` in the list |

Mutations: 2 of 4 caught; y_frente and y_dia survive (the path with the other date after this one does not run up to 2001-08-20: the game only counts backwards, today minus birth); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal with the new reference db8ee4d7a7e1ef8e (the .lng files were rewritten by the game at 08:14 on 2026-10-11, as on 2026-10-10; the original gave the same fingerprint).

### Match engine, 111th batch — the staff player rating (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `avaliar_jogador(pessoa, jogador, clube)` | 0x6e1cf0 | cdecl | match_man.cpp: the rating (i16) the staff give the person (or `jogador` +0x69's) for the club; −10000 with no person. Adds record +0x2f times the days-alive factor (0x54cdb0 against today 0xae2c90; national sides +0x22 × 75; above 1000 days, 1 + (days − 1000) × 0.000125), subtracts (fit² + 5) × 0.1 (0x53ce90), adds the +0xd gap to the club reputation (× 0.002 + 1.5, between −4 and 4) and the contract (0x4cf5f0: +0x44/75 + 0.5 adjusted by the type in +0x4f >> 4). Then by age (×0.5/0.75/0.9/1.05), ×0.9 with days below (50 − +0x59) × 10 and ×(0.01 × +0x5b + 0.9) with +0x5b below 10 |

Mutations: 7 of 7 caught; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (db8ee4d7a7e1ef8e). First routine from match_man.cpp.

### Match engine, 112th batch — the best-rated player on the side (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `melhor_avaliado()` | 0x6e1a40 | thiscall | match_man.cpp: the index (0..20) of the active player (+0x19 ≥ 0) best rated by the staff (0x6e1cf0 with club +0x2f) among the 20 in the block starting at +0x23; the first active one counts even with a negative rating, then only strictly higher; −1 with none active |

Mutations: 2 of 2 caught; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (db8ee4d7a7e1ef8e).

### Match engine, 113th batch — the best-rated person in a list (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `melhor_da_lista(lista, n)` | 0x6e1940 | thiscall, ret 8 | match_man.cpp: among the `n` people in the list (with record +0x61), the one best rated by the staff (0x6e1cf0 with club +0x2f); 0 with none. With object +0x1f, if its value (0x544410 with byte +0x17) is below the person id mod 20, the rating becomes (1.1 − 0.01 × (object address mod 20)) × rating, as in the original; ties keep the first |

Mutations: 3 of 3 caught; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (db8ee4d7a7e1ef8e).

### Match engine, 114th batch — the player who leaves the pitch (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `jogador_saiu(j, ja_fora)` | 0x6cda20 | thiscall, ret 8 | match_eng.cpp: `j` leaves the pitch (injury or sending-off). Without `ja_fora`, adds +0x23, stores the minute (+0x8ed0/11, capped at the period end) in +0x21 and removes him from the grid (0x6cdd50). Ends with +0x19 = −3. With a human manager on that side (0x5fd210/0x5fd480) it goes on only with +0x904b and outside national sides. Goalkeeper with +0x9049: the staff swap the keeper (0x6e1090 with 1); otherwise they redo the team (0x6e1aa0, 0x6e1090), pick a new captain if it was him (0x897590, 0x6e1a40, 0x897760) and restore the number-1 position player (0x6ce110); stores the substitution minute (+0x8ecc) and redoes side strength (0x6bbb60) and man-marking (0x6bbf60). The staff routines (0x6e1aa0, 0x6e1090, still original) and the stored captain are tape units |

Mutations: 6 of 7 caught; s_zera survives (clearing the other side's number-1 position player: 0x6ce110 redoes both right after); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (db8ee4d7a7e1ef8e). The first baseline diverged: the substitution (a unit) changes the captain in the tactics row (+0x52), outside the regions; both clubs' rows were added to the regions. mutacoes.py retries the build when cargo fails without a message.

### Match engine, 115th batch — the staff redo the team; the tape with vectors (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `refazer_time()` | 0x6e1aa0 | thiscall | match_man.cpp: the staff redo the team. With 11 on the pitch and the line-up rejected (0x6e0230 with 0x8c1, 2) narrates 0x2007. Gathers the outfield players on the pitch (position +0x19, tables 0x717410/0x717450, position marks 0x5b5270 and byte 0x5b57c0, record +0x69), picks the team (0x6e2000 with 0x8800, or 0x408800 when a human manager is to be consulted: 0x600a50, 0x5fd210, 0x5fea50) and gives each picked record the position of the place it won, redoing performance (0x6eecb0). 0x6e0230 and 0x6e2000 (pick, recursive, allocates) are units with vectors: the tape records the bytes of the input and output vectors passed by pointer, checks the Rust copy's inputs and writes its outputs (`chamar_gravando_vetores`, `pedir_vetores`). The provisional 0x6e1aa0 port from batch 114 is gone |

Mutations: 5 of 5 caught; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (db8ee4d7a7e1ef8e).

### Match engine, 116th batch — the substitution (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `substituir(goleiro)` | 0x6e1090 | thiscall, ret 4 | match_man.cpp (~2.2 KB): the staff make up to +0x17 substitutions. Eligible players (on the pitch, or −2 without +0x1e) are sorted by the CRT qsort (0x945333 with the game's comparators 0x6e0df0/0x6e0fe0); the first ones give the places (tables, position, position marks and byte; the first is 0 with `goleiro`) and free reserves from the 12th onward are candidates. The pick mode comes from the club, the strength against the opponent (with draws), the competition (+0x3a; +0x1a = 3 and 0x48f150), +0x16 and the human manager. Whoever wins a place from the bench comes on (0x6cdd00, minute in +0x1f); whoever loses it goes off (0x6cdd50, +0x19 = −4, minute in +0x20). Narrates the substitutions (0x1f59/0x1f5c/0x1f5a), tiredness (0x211d) and injury (0x211e) by draw and 0x20d7/0x1f5b; then the best rated becomes captain (0x897760) and the team is redone (0x6e1aa0). The pick (0x6e2000) is a unit with vectors; the provisional 0x6e1090 port from batch 114 is gone |

Mutations: 4 of 8 caught (joint run with batch 117); survivors v_qsort (differs only with exactly 11 eligible and different orders in the two comparators), v_goleiro (goalkeeper substitution with +0x9049: did not happen before 2001-08-20), v_forca (exact equality on the 3000 draw) and v_cansaco (draw with negative +0x31); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9, new reference: the set of detours changes stack garbage in both modes).

### Match engine, 117th batch — the pick (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `escalar(...)` | 0x6e2000 | cdecl, 15 arguments | match_man.cpp (~3.9 KB, recursive): spreads `n` people over `m` places (+ `x` extras) and writes each place's person. Each (person, place) pair is worth the 0x6e2f20 score (place code, marks, the place's byte and short, mode, club, staff; with bit 0x400000 and a human manager, 0x5fed50). With fewer people than places, each free person looks for the place where it beats the occupant (up to 10000 rounds) and at the end even places with card +0xf = 20 and odd ones with +0xf < 10 are cleared; with more people, each place takes the best and repeats are solved by the smallest loss (up to 10000 rounds). The extras get the leftover people through 0x6e2000 itself (mode | 0x200); finally it swaps pairs of places while the sum improves (up to 100 rounds). Working memory comes from the game's CRT (0x945501/0x9452ca), in the same order. Inside a taped comparison it is a unit with vectors (the provisional port from batch 115 is gone) |

Mutations: 5 of 8 caught (joint run with batch 116); survivors e_impar (+0xf = 9 boundary), e_extras (the extra places' short almost never weighs in the 0x6e2f20 score) and e_tecnico (only with a human manager to consult); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9).

### Match engine, 118th batch — the team's stance (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `postura()` | 0x6df460 | thiscall | match_man.cpp (~3 KB): the stance the staff ask for (1 all-out attack … 5 park the bus). Starts from the sides' strength (with the engine running, +0x8ea5, the sum of the 0x6e86d0 scores of those on the pitch × 50 / 11; otherwise the +0x80 reputation), possession (+0x1fc/+0x206/+0x210), time (+0x8ed0), sendings-off (+0x1ad, 0x6cbbb0) and the aggregate score (0x6ce310); adjusts for the league (table positions through 0x4a4850 and dates through 0x54cdb0), the manager (+0xe) and the end of the match (+0x8ed4 = 0x3de). Stores the factor in engine+0x476a[side] (10 and −1 at the extremes); with the engine running and the factor within 0.5 of the stored one, keeps the current stance (+0x1ab) |

Mutations: 9 of 9 caught (joint run 118–120); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9). The 1st base diverged in side 1's factor: in the league block the exponent is 1.5 (0x968940), not 2.0 (0x968950, the neighbouring constant); found with a temporary trace of the intermediate values.

### Match engine, 119th batch — the formation (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `escolher_formacao(fichas, n, modo, tipo)` | 0x6e0230 | thiscall, ret 0x10 | match_man.cpp (~2.8 KB): the staff choose the club's formation and load the tactic. With the engine's +0 set it draws one; with `tipo` ≠ 2 it only reloads; with a human manager to consult (0x5fd210/0x5fea50) it uses his tactic (0x5fbe20/0x5fbd00). Otherwise it scores (0x6e2f20) each sector (7) and band (5) by the two best of the `n` people (the goalkeeper only in the middle band; with fewer than 11, the last sector counts half and the ends of sectors 1 to 3 take the smaller) and judges every formation in the tactics bank (0xde1d08): time (+2, up to 3000) and stance (0x6df460) weigh the formation's score (up to 3), the manager adjusts (the favourite +0x43 is worth ×1.1 or up to ×3.75; the others ×0.1), each place adds its sector and band score (0x5b2ae0) with the marks (0x5b5270/0x5b57c0), and the sectors' balance comes in through the stance. Stores the best (0x8967d0), copies the tactic (0x896660) to the engine (+0x91e2 of the side) and to +0x2b and sets the instructions (0x6ddab0). The scores go in a list from the game's CRT (0x945501/0x9452ca). Inside a taped comparison it is a unit with vectors (the provisional port from batch 115 is gone) |

Mutations: 7 of 9 caught (joint run 118–120); survivors f_metade (only with fewer than 11 people) and f_escala (the floor of the divisor common to every formation does not change the winner: equivalent); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9).

### Match engine, 120th batch — the individual instructions (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `instrucoes()` | 0x6ddab0 | thiscall | match_man.cpp (~6.4 KB): the staff set the instructions of the tactic copied at +0x2b (0x5b50d0 general, 0x5b2ba0 per position, 0x5b5290 64-bit marks, 0x5b5910 man-marking in the engine's tactic block) and in the tactics bank (0x897ee0). With a human manager to consult it does nothing. At home (or by a draw against the reputation gap in competitions +0x3a = 1 or 9) it asks for 0x20; the manager (+0x13, +0x1f, +0x1c) picks tempo and marking. A factor (reputations / 17000, manager +0xe and the remainder of the ids' sum by 13 − 0x544410/2) is compared with the attributes (0x5431e0) of each of the 20 on the side on the pitch: wing runs, man-marking the opponent from 0x6d4330 (with 0x7c1e30 for midfielders), passing, long shots, dribbling, tackling, offside, set pieces (minus the accumulated value of those with high +0x2e), crosses by table and the set-piece taker (0x897ee0 for whoever is at +6/+0xa/+0xe of the staff). Quirk kept: the marked player's side comes from the player's +0x19 (position 0 becomes the other side) |

Mutations: 9 of 9 caught (joint run 118–120); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9).

### Match engine, 121st batch — the staff's small pieces (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `colocar_na_vaga(pos, pessoa, extra)` | 0x6e0d00 | thiscall, ret 0xc | match_man.cpp: puts the person in place 0..19 of the staff's block (+0x23) through 0x6eaea0; without a person returns 0 |
| `ordem_por_tabela(a, b)` | 0x6e0df0 | cdecl | the substitution qsort comparator: no card goes last, +0x19 = −2 and a goalkeeper with +0xf < 20 come first; otherwise the condition (+0x35 + +0x4d, truncated) weighted by `a`'s table (2 and 4: ×1.35; 8: ×1.2 on `a` and ×1.25 on `b`; 0x10: ×1.1) |
| `ordem_por_condicao(a, b)` | 0x6e0fe0 | cdecl | the other comparator: same ends and only the condition |
| `titulares()` | 0x6dbb80 | thiscall | match_man.cpp: for each place from 0 to 10 + (match +0x4d & 0xf), the person from the club's tactic (0x8968f0) comes in (0x6e0d00) if able to play (0x62c400 ≠ 2, 0x55d270 and 0x7757d0 without impediment); returns how many |
| `guardar_tatica()` | 0x6dbb10 | thiscall, virtual | match_man.cpp: stores the staff's tactic (+0x3f) in the bank (0x8984d0); with a human manager only when 0x600a50 denies or 0x5fd480 confirms |

Mutations: 2 of 8 caught (joint run with batch 122); survivors c_goleiro (goalkeeper with +0xf < 20, rare), c_peso and c_oito (the by-table comparator only comes in with fewer than 11 eligible) and c_total, c_estado and c_extra (the starters only run for a human manager who is not on holiday: the automatic career never goes there; cover with a real comparing career; on 2026-10-12 Lucas's real career, with the line-up changed and two watched matches, passed with 0 divergences). The 1st base flagged colocar_na_vaga: 0x6eaea0 (starts the player, draws and touches things outside the engine) ran twice, in the original and in the copy; it became a tape unit (new port), and colocar_na_vaga and titulares are now compared through the tape. real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9).

### Match engine, 122nd batch — the marked substitutions (official on 2026-10-11)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `aplicar_trocas()` | 0x6d0b60 | thiscall | match_eng.cpp (~1 KB): applies both sides' marked substitutions (+0x1d). With +0x1c the player goes off (on the pitch: narrates 0x1f56, 0x6cdd50 with −1 or −2 by +0x9049, +0x1a = position and +0x19 = −2; with −3: narrates 0x1f57 and 0x6cdd50 with 0) and the minute goes to +0x22; without +0x1c it narrates 0x1f58 by draw(4) or with +0x55; with +0x55 it narrates 0x20f6 with each one's side (0x714d30). With marked players and outside matches with a human manager who doesn't delegate: with +0x9049 the staff substitute (0x6e1090); otherwise they redo the team (0x6e1aa0), give the armband (0x897760, the best rated from 0x6e1a40) if the captain (0x897590) went off, clear +0xf59e/+0xf5a2 and recompose (0x6ce110). Stores the minute (+0x8ecc), redoes the side's strength (0x6bbb60) and the marking (0x6bbf60); at the end clears +0x8eb5. Compared through the tape (narration, staff and captain as units); the 6th argument of narration 0x20f6 goes in byte 1 of the 5th (0x6d6f70 reads both as bytes) |

Mutations: 6 of 7 caught (joint run with batch 121); survivor a_capitao (only matters with the captain in place 0, the goalkeeper's); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9). 0x6db500 (staff at kick-off) and a port on 0x6dbc50 passed the mutations and the real game but changed the careers (see How to continue) and left the queue.

### Match engine, 123rd batch — the pending punishments (official on 2026-10-12)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `punicoes(primeiro)` | 0x6d1210 | thiscall, ret 4 | match_eng.cpp (~1.4 KB): on the 1st call (+0x8eb4 = 0) it prepares `primeiro` (0x716630) and sets +0x8eb4 = 1 if anything is pending. Then it applies each of the +0x1a3 pending incidents (11 bytes from +0xb1: player, other, severity, code, x): the referee's punishment (0x6ea4e0) gives the card; type 2 narrates 0x1ff9, 3 and 4 (sending-off) narrate 0x1ff7/0x1ff8 and mark the side's +0x1cb (goalkeeper: table 1); without a card, severity above draw(7) + 2 with the player on the pitch narrates 0x210a; when narrating, 0x20f5 (draw(severity) > draw(20)) and 0x2109 come first. Outside codes 0x25, 0x2a–0x32, 0x60 and 0x61: by draw(9) it narrates 0x20df (0x20e0 by draw(3); types 3 to 6 set +0x16 and 0x20d9 by draw(7)), passes the punishment weight to the side (0x716450: the player, those on the pitch (p + 1) / 2, those off it p / 6 + 0.83) and 0x716e10 (1 on a sending-off); a sending-off takes the player off (0x6cda20). At the end, for each marked side and outside matches with a human manager who does not delegate: with +0x9049 the staff change the goalkeeper (0x6e1090 with 1), otherwise they redo the team (0x6e1aa0), give the armband to the best rated (0x6e1a40/0x897760), clear +0xf59e/+0xf5a2 and recompose (0x6ce110); store the minute and redo strength and marking. Ends with +0x1a3 = 0 and +0x8eb4 = 2. Compared through the tape (narration, staff and captain; 0x6cda20 comes in through `jogador_saiu_com`) |

Mutations: 8 of 8 caught (joint run with batch 124); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9). 0x716630 and 0x716e10 narrate inside: they got ports that record on the tape only during this comparison.

### Match engine, 124th batch — the penalty shootout (official on 2026-10-12)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `penaltis(modo)` | 0x6be710 | thiscall, ret 4 | match_eng.cpp (~1.9 KB): one step of the shootout by state +0x8eb3. 1 and 4 prepare (clear the incident, save and mute the commentator in +0x8ea4, five kicks per side in +0x8ea1/+0x8ea2, side 1 or a drawn side starts, 0x6be520) and go to 2 and 5. 2 and 5: with no kicks left comes sudden death (+0x8ea3); the taker is next on the side's list (+0x1af + 11 × side, +0x8e9f counts), the goalkeeper is the other side's (0x6ce110); it marks who kicked (+0x8eda), the strength comes from draw(card +0x35) and the kicks left, and grows (×2.25 with narration 0x2005 when the side must score, ×0.75 with 0x2006 when it can decide). With `modo` 3 the shot (0x700bd0, 0x702050) is taken directly; otherwise it positions taker, goalkeeper and the others (0x6f46b0, 0x7160e0), the direction comes from the strength and the goalkeeper reacts (0x714d40, up to 4 times 0x7109f0) before the shot. It passes the turn and, once the score is decided (+0xf5bc × +0xf5f2 with the kicks left), goes to 3 (mode 4) or 6. 3 and 6 narrate the result (0x2004), store the shootout score in the match (+0x49/+0x4a) and give the commentator back. Compared through the tape: narration, the shot (0x702050, a new port that records only in this comparison), positioning, ball control (0x6f0630) and the goalkeeper's reaction are units. Automatic careers only use the direct shot (`modo` 3): the branch with positioning and the goalkeeper's reaction (mutations n_direcao and n_reacao survive) and a side's 11th kick (n_vez) wait for a watched match |

Mutations: 5 of 8 caught (joint run with batch 123); survivors n_direcao and n_reacao (the watched-match branch, which automatic careers do not use) and n_vez (only on a side's 11th kick); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9).

### Match engine, 125th batch — the called foul (official on 2026-10-12)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `aplicar_falta()` | 0x6bac40 | thiscall, returns al | match_eng.cpp (~3.2 KB): applies the pending incident in +0x475a (type; +0x4761 who committed it, +0x4765 who suffered it, +0x475b/+0x475c where, +0x475f the code). With no type it returns 0; with a set piece under way and the incident behind the marked position (+0x8ea8) of the side that had the advantage, it discards it and returns 0 (otherwise it narrates 0x20f0). Type 8: narrates 0x20ee, counts in the side's score (+0xf5ca) and the player's (+0x13), set piece 7. With no victim: foul punishment (0x6e9630) and 0x20fb; with one: incident punishment (0x6ea4e0) and narration by code (0x20fc to 0x2107). The returned type picks the card (0x1ff5 to 0x1ff9, or 0x210a by severity); near the opponent's box it becomes a penalty (+0x8eb2 = 5, 0x1f7a, −500 in +0x35), otherwise a free kick (6, 0x1f79). With a card, 1 in 4 narrates 0x2109 (and 0x20f5), narrates the card and a sending-off takes the player off (0x6cda20). Outside the set-piece codes: narrations 0x20df/0x20e0/0x20d9 by draw, the offender's team wears down by the punishment weight (0x716450) and the offender tires (0x716e10). Clears the incident and returns 1. Compared through the tape: narration and tiring (0x716e10) are units |

Mutations: 9 of 12 caught; survivors f_vazio (callers only call with a pending incident), f_marcada (incident exactly on the marked position during a set piece) and f_parada (set-piece code 0x32, rare); p_achou, from the new procurar_no_lado comparator, is caught; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9). The 1st base flagged procurar_no_lado on day 271: the old comparator replayed 0x6f5c30's answers without its effects; it now compares through the tape (see How to continue).

### Match engine, 126th batch — the tackle (official on 2026-10-12)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `dividida(forte)` | 0x70ec80 | thiscall, ret 4 | match_pl.cpp (~5.9 KB, called by 0x7109f0): the tackle on the ball holder. It draws by stamina (+0x4d) and only goes on with `j` in possession (+0xf582); with `forte` the opposing side gets 2.25 (0x6bb950) and `j`'s side +10 possession. The attack is draw(+0x19c − stamina + +0x13a − draw(+0x8ea9 × 50)); the challenger, the score and the move come from 0x6c87e0 (move 0x10), and the tackle injury (0x6cfe10) follows. With no challenger or the attack above the score, `j` gets past: outside the box he may grow (+0x1a2 × 1.05, 0x211d), counts possession and narrates 0x2133/0x216d/0x21de/0x1f40. Otherwise the referee decides (0x6cfc60): a foul with punishment (0x6cfcd0, narration by move, card 0x1ff6 to 0x1ff9 or 0x210a, set piece 7, penalty 5 or free kick 6, taker 0x6ceb90 and narrations 0x21df to 0x21e2, sending-off 0x6cda20, team wear 0x6bb8d0, tiring) or a clean challenge (the challenger marks the move, loses 59/70 stamina, narrates the move or defends 0x70d430, controls 0x6f0630 with +0x19c = score − attack − 200; below the move's limit the ball breaks loose 0x6cd770 and he goes down, or `j` suffers 0x716c90 with a penalty given; otherwise he turns to the move 0x6f4920 and may narrate 0x21cd/0x21ce by success rate; gains stamina and 1 in 7 narrates 0x1f41). Compared through the tape: narration, tiring, 0x716c90 (a new port under the same switch as 0x716e10), control, the loose ball and the units inside the injury, the defence and the substitution |

Mutations: 12 of 12 caught; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9). The 1st base flagged the referee's decision on day 262: the comparisons left out the end of the engine (see How to continue); all of them now use the allocated size, 0xf63b.

### Match engine, 127th batch — the loose-ball contest (official on 2026-10-12)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `disputa(x, y)` | 0x6c9cd0 | thiscall, ret 8, returns the winner | match_eng.cpp (~6.5 KB, called by 0x6b98c0): the loose ball at square (x, y) contested by the challengers of both sides (0x6c87e0, move 0x15; the score corrected by the angle of the direction against +0x8ed8). Off the pitch it is ball out (0x6cd490); with no challenger, it stores the square and returns 0. The higher score wins; the tackle injury (0x6cfe10) follows. With both, the referee looks at the winner's foul (0x6e9820, +4 stamina first, 0x2168 in the box) and then at the loser's; with no foul, 0x6e9540 may give one. The foul becomes a pending incident (+0x475a) by draw(distance to goal × 10 + 2) = 0 with no risk, or is punished at once (0x6ea4e0 or 0x6e9630): narration by move (0x1fe8 to 0x1ff4), card, set piece 7, penalty 5 or free kick 6, sending-off (0x6cda20), narrations 0x20df/0x20e0/0x20d9, wear of the offender's team (0x716450) and tiring; special moves leave before the wear. With no foul, the clean contest: on move 1 the defence (0x70d430); with both, the reading (+0xad: 0x2164, 0x214f or 0x1f5d) and the attempts (+7, +8, +9 with danger 0x70d630); the winner marks the move, turns to the play (0x6f4920), controls (0x6f0630) with +0x19c = score − other score − 225 (±200), the loser adds the challenge's 2nd byte to +0x2b (0x7160d0) and the winner goes on (0x7103e0). The two halves (winner's and loser's foul) are not symmetric in the narrated side nor in who gets +0x16. Compared through the tape: narration, tiring, ball out, control, the winner's move and the units inside the injury, the defence and the substitution |

Mutations: 9 of 11 run caught; t_fora skipped (the ball-off-the-pitch branch did not run in the base: measured gap); survivors t_angulo (only with an angle difference of exactly 180) and t_defesa (the defence with both challengers, strength ±1); real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9).

### Match engine, 128th batch — the ball reaching the player (official on 2026-10-12)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `responder(forca, tipo, altura, conta)` | 0x6f5c30 | thiscall, ret 0x10, returns 0/1 | match_pl.cpp (~3.9 KB, called by 0x6cd330 `procurar_no_lado`): the player contests the incoming ball. The set-piece taker (+0xf576) does not; on the side without the ball, with +0x10d × 2 < draw(6) and the line beyond the limit (0x6cb930), neither. The attack is draw(2000 − +0xa5 × 40) for the goalkeeper in the box, otherwise by type (500, 600, 700, 1000, 1800; 5 is draw(500000) + 200 and becomes 4); with height < 1.5 types 4/5 do not contest; with height, attack × (+0x8eaa + 1) / (height + 1). With the ball on his own side (+0x8eab), 0x1f5e and the ball breaks loose. Otherwise the move and the attribute (control 1 with +0x11c, 0x15 +0x144, 0x24 +0x162, 0x29 +0x16c, header 0x25 +0x164) give the score = draw(attribute) − strength / 2 − attack. The referee looks at the foul (0x6cfca0): 1 in 10 logs it (0x6bb9a0), otherwise punishes (0x6cfd10, 0x1fe8, card, penalty with the +0x17 adjustment and performance 0x6eecb0, taker, 0x21df to 0x21e2, wear 0x6bb8d0, tiring). Without punishment, with the score at the limit (1500 − height² × 100 + +0x8eaa × 100, minimum 160) the player controls (0x1fe2/0x1f5f, 0x6f0630, 0x21cd/0x21ce); otherwise the ball breaks loose (0x20d4/0x213e/0x1f71, 0x6cd770). In comparison the hook is the unit's own port (which only records inside `procurar_no_lado`); in Rust mode the copy goes in. Compared through the tape: narration, tiring, the loose ball, control and the units inside the substitution |

Mutations: 12 of 12 caught; real game to 2002-01-01 with 0 divergences; Rust ×2 and original careers equal (7cb2218eb00107c9). The 1st base did not exercise the Rust copy (the branch count showed zero): the routine only runs as a unit inside procurar_no_lado; the port now compares it inside the unit (nested comparison).

### Match engine, 129th batch — the play (official on 2026-10-13)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `jogada(forca, altura, estilo, forte)` | 0x70dfd0 | thiscall, ret 0x10, returns 0/1 | match_pl.cpp (~7 KB, called by 0x6cd3d0 `procurar_jogada`): the player tries the play against the opponent's strength. The side's goalkeeper (0x6ce110), the set-piece taker (+0xf582) and, on the side without the ball, anyone with +0x10d zero do not play. The attack is a draw by +0x8ea9 (300, 400, 500, 1000, 1800) divided by height + 1. With the ball on his own side (+0x8eab), 0x1f5e and the ball breaks loose. Move and attribute follow 0x6f5c30, but by +0x8ea9; the header 0x25 is worth twice +0x164 and, in the air, the attribute is worth 85%. Score = draw(attribute) − strength / 3 − attack. Hard foul: strength > draw(150) + 200. The referee looks at the foul (0x6cfca0): 1 in 5, without a hard foul, logs it and carries on; otherwise punishes as in 0x6f5c30 (the penalty only takes 500 from +0x35). A score of 400 wins (0x1ffe/0x21d6/0x1f66/0x1f65, 0x6f0630); otherwise the goalkeeper narrates 0x1fe7, from the arc with score > 200 0x1ffd and the ball breaks loose with no shooter, or 0x21d8/0x1f85/0x1f84 and the ball breaks loose. The narration's 7th argument (the style) goes in the code's high bits. In comparison the hook is the unit's own port, which compares inside it; in Rust mode the copy goes in. Compared through the tape, including the narration's 7th argument |

Mutations: `mutacoes/leva129.py` (14, to 2001-10-01): 12 caught; `j_falta_forte` (weak: only a draw tie changes it) and `j_estilo` (narration 0x1f65 did not run in the window) escaped. Reinforcements: `leva129b.py` (`j_estilo2` on 0x1f66, caught: the tape kept 0x51f66 and the copy sent 0x61f66; `j_falta_forte2` with limit 100 escaped: only 3 fouls in the window) and `leva129c.py` (`j_falta_forte3` always on, caught). The `jogada goleiro` branch (0x1fe7) is unreachable: the goalkeeper leaves at the start. Real game comparing to 2002-01-01: 0 divergences; Rust ×2 and original careers: 7cb2218eb00107c9

### Match engine, 130th batch — the post-match events (official on 2026-10-13)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `eventos_do_pos_jogo(partida)` | 0x6b74a0 | cdecl | match_day.cpp (called at 0x6b60c2, after 0x6b6cc0): walks the event list at [0xb67dc8] (cursor reset; 0x68d1b0) and handles those of the match's two clubs that are not out (0x53b900). Event: +0 person, +4 club, +8 mark, +0x13 type, +0x15 other person. 0x82: 0x759610. 0x13: without a mark clears bits 0x40 and 0x2000000 of the contract (0x4cf5f0); with a mark, news 0x72c1f0 and 0x2000000, or 0x40 by draw(+0x5b). 0x0a: 0x4000000 and, by draw, the news if the sheet's +0xb > draw(2000) + 4500. 0x0b: 0x7daf50. 0x1e: 0x40000 by draw, 0x733a60 and −50 (with 0x547af0) or −75/−75 on words +0xa/+0xc of the club's row in [[0xb67a44]] (between 1 and 10000). Others: 0x40000 by draw(+0x5a) < draw(15/8/6 by age) and the news. **No side-by-side comparison** (calls 14 database routines with effects): compare mode keeps the original; it is checked by the career, with the database equal to the original's on every day |

Mutations: `mutacoes/leva130.py` (11, through the career to 2002-06-01): 7 caught (`e_0b` crashes the game). Escaped: `e_0a_sorteio` and `e_0a_limite` (type 0x0a ran only twice, no flip), `e_1e_tecnico` and `e_1e_sem`. With the daily database fingerprint (`leva130b.py`) `e_1e_tecnico` is now caught (the [[0xb67a44]] table changes and is erased later); `e_1e_sem` is not: the game resets the row to 5000 the same day (measured: 5000 → 4925 → 5000). Reinforcements (`leva130c.py`): `e_0a_limite2` (without the 4500) caught; `e_1e_sem2` (adding instead of subtracting) escapes for the same reason: a known gap, only a side-by-side comparison would see it. Branches measured to 2002-06-01: others 16+, 0x1e 4+ (with manager 2+, without 1), 0x0a 2, 0x0b 2, 0x13 mark 2; 0x82, 0x13 without mark and 0x13 news never ran (not even to 2003-06-01 with seed 777). Real game comparing to 2002-01-01: 0 divergences; Rust ×2 and original careers: 7cb2218eb00107c9, matches 0ac236ae64589294, database 923a4e0edd9f2e09

### Match engine, 131st batch — the players after the match (official on 2026-10-13)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `jogadores_do_pos_jogo(partida, t)` | 0x6b6cc0 | cdecl | match_day.cpp (called at 0x6b60bc, before 0x6b74a0): for both sides of record `t` (club at t+0/t+4; side at t + side × 0x625), the 20 players (0x49-byte rows from +0x1c5; person at −0x35) with a sheet (+0x61) and from the base. In the match's competition, with its club and without +0x4c: by contract 0x10/0x20/0x30, 0x849ce0/0x600a50, rating +3 above 5000 and above 1.1 × +0x18e (or +7 above 0.9 ×, or the sheet's +5 above 0.024 ×), +0 up to 6, the side's score not above the other's, draw(10) + 5 < rating 1 (0x4a31a0), draw(10) above the sheet's +0x45 and rating 0x11 within 0.2 of the competition average ([0xb74350]+0x10/+0x18, or 6.8): news 0x72c1f0 type 0x11. With 0x76d940, +0 from 9 up, the score not below, draw(5) + 2 < rating 1 and rating 0x11 above draw(9) × 0.1 + average + 0.5: 0x744240 (0x34), one per match. Copies the row's −4, +3, +5, +7 into the sheet's +0x45, +0xb, +9, +0xd. Same nation for both clubs and −8 in the row: with 0x7c1e30 nonzero and the previous-club, sheet and person conditions, 0x20 on the contract (draw(3) for the own club [0x9cf300] or draw(10)). The manager (+0x184) gets +0x188/+0x18a/+0x18c on his sheet (+0x69) and the assistant (+0xd3) and the 5 at +0x19f a quarter of the difference (between 1 and 10000); club +0x80 = +0x18e. **No side-by-side comparison**: compare mode keeps the original; checked by the career (database equal every day) |

Mutations: `mutacoes/leva131.py` (12, through the career to 2002-06-01): 11 caught — the three on the sheet (+0x45, +0xb, +9), the manager, the assistants, the 10000 limit (`j_limite` only by the database fingerprint), the club's +0x80 and news 0x11/0x34. `j_n34` escaped (limit 0.1 lower); the reinforcement `leva131b.py` (`j_n34_forte`, 1.0 lower) is caught. Branches to 2002-06-01: manager 8192+, news 0x11 8+, 0x34 4+; the 0x20 mark only runs in the long career (seed 777 to 2003-06-01: 2+), where the three fingerprints also match (database 9a32333b47dbbdcd). Real game comparing to 2002-01-01: 0 divergences; Rust ×2 and original careers: 7cb2218eb00107c9, matches 0ac236ae64589294, database 923a4e0edd9f2e09

### Match engine, 132nd batch — the club after the match (official on 2026-10-13)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_do_pos_jogo(clube, outro, partida, n, t)` | 0x6b79a0 | cdecl | match_day.cpp (called twice at 0x6b61a9/0x6b61bd, once per club): without club, other or match, error box. Without a manager (+0xcf), nothing; without 0x600a50, with +0x80 below 4750 and draw(100) ≠ 0, nothing. With 0x6b7cf0, match +0x40/+0x41 and the club not out (0x53b900): among the 50 at +0xd7 with a contract (0x4cf5a0) without +0x45, sheet +0xb above 4250, +0x59 > 5, +0x57 > 15, +0x58 > 10, `n` > 2 and draw(n) ≠ 0: 0x5896d0(person, club, other, n) and done. Then the 20 in the club's row of record `t` (+0x1ce, person at −0x3e, only with +0 in the row): sheet, from the base, age up to 20, contract without +0x45, the match in the club's competition, +0x59 > 15, +0x57 above draw(5) + 10, sheet +0xb below 4250 and 0x7c3a40 zero: 0x589bc0 and, without 0x547d60, 0x547d90(person, club, 1). **No side-by-side comparison**: compare mode keeps the original; checked by the career |

Mutations: `mutacoes/leva132.py` (11, through the career to 2002-06-01): 6 caught (draw(100), +0x80, manager, age, +0x59, side); `c_acao` skipped (the 0x547d90 branch does not run). Escaped: `c_57`, `c_ficha`, `c_n` and `c_elenco57`: filters right before the final actions, which never fire in automatic careers (not even to 2003-06-01 with seed 777; the body runs — `clube segue` 4096+, `clube elenco` 32768+, `clube jovem` 512+). Reinforcements (`leva132b.py`) forcing the actions: `c_589bc0_forcada` and `c_5896d0_forcada` caught — the fingerprints see those paths. Measured gap: 0x5896d0, 0x589bc0 and 0x547d90 never run. Real game comparing to 2002-01-01: 0 divergences; Rust ×2 and original careers: 7cb2218eb00107c9, matches 0ac236ae64589294, database 923a4e0edd9f2e09; seed 777 to 2003-06-01 also equal

### Match engine, 133rd batch — the match without the human (official on 2026-10-13)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `partida_sem_humano(partida)` | 0x6b66a0 | cdecl, returns 0/1 | match_day.cpp (called at 0x6b4ae3 and 0x6b5c9a): 1 if the match does not involve the human, through 0x7f9720. First, once ([0xa15704]), marks the humans' national teams at 0xb67a58 (0x48-byte slots at 0xb63d60 with +0x11c bit 2 and national team +0x71). Without [0xdd7118]: 1. Returns 0 if one of the clubs (+0x1c, +0x20) is a human's (0x600a50) or its linked club (0x540a50); if the competition (+0) is [0x9cf7bc] or [0x9cf6e4]; if it is [0x9cf958] with club [0x9cf2e4]/[0x9cf510] human; if one of the 16 humans at the end of the people table (+0x5f = 1) manages a club of the same nation (+0x57). In [0x9cf684] without a human club, straight to 0x7f9720. Outside a national team (0x53b900) with 0x5c2990, or in [0x9cf6f8]: 0 if both +0x80 add up to more than 12000. Otherwise the slot from 0x48dd70 (table at 0xb63d74): 0 or 1 gives 0; 3 to 7 with the competition's national team marked, 0; a human's slot with +0x11c bit 2, 0. Side-by-side comparison (answer and marks) |

Mutations: `mutacoes/leva133.py` (12, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): 7 caught (`s_flag`, `s_684`, `s_5f`, `s_80`, `s_vaga`, `s_marcada`, `s_final`); `s_ligado` skipped (the linked-club branch does not run). Escaped in both modes: `s_humano`, `s_comp1`, `s_958` and `s_16`: conditions always false in automatic careers (no human club in the matches, fixed competitions that do not appear, the 16th human never a national manager) — only a career by Lucas reaches them. Branches to 2002-06-01: direct competition 1024+, nation manager 512+, marked national team 256+, by reputation 128+, 0x7f9720 2+. Real game comparing to 2002-01-01: 0 divergences; Rust ×2 and original careers: 7cb2218eb00107c9, matches 0ac236ae64589294, database 923a4e0edd9f2e09

### Match engine, 134th batch — the human's interest in the match day (official on 2026-10-13)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `interesse_do_dia(d)` | 0x6b5040 | thiscall, returns 1 | match_day.cpp (called from human_manager.cpp and match_day.cpp): marks in match day `d` what interests the human. First the humans' list (0x5fe8d0 on [0xb63c98]); zeroes +0x29, +0x2a, +0x2b. For each group (+1, 0x18 apart, count at +9) and each of its competitions (+4 to +8, at +0xd 0x54 apart): the competition is of interest (+0xc) if one of the 16 humans at the end of the people table has his club (+0x39) or other link (+0x24) in it (0x48db80). Each match (+4 to +8, at +0x19 0x69 apart): with a human club (0x600a50 with +0x20/+0x44 or +0x24/+0x45) or by 0x5fe420, it is marked (+0x53) and counts the group (+0x29) and the competition (+0x2a) the first time; otherwise, by 0x6b61e0, only the competition is of interest. Any mark sets +0x2b. Side-by-side comparison (all the day's marks) |

Mutations: `mutacoes/leva134.py` (11, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): 4 caught — `i_zera29` (only by the comparison), `i_2b` and `i_6b61e0` (the game stops on a screen waiting for the human and hits the limit), `i_comp_zera`; `i_marcada` skipped (no match has a human club in automatic careers). Escaped in both modes: `i_16`, `i_k`, `i_outro`, `i_byte44`, `i_5fe420` (only the holiday manager's slot is filled among the 16 humans; no human club on the pitch) and `i_marca_comp` (the competition it would fail to mark is marked later by 0x6b61e0). Branches to 2002-06-01: through 0x6b61e0 1024+, human in the competition 128+. Real game comparing to 2002-01-01: 0 divergences; Rust ×2 and original careers: 7cb2218eb00107c9, matches 0ac236ae64589294, database 923a4e0edd9f2e09

### Match engine, 135th batch — the restart of play (official on 2026-10-13)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `reiniciar_lance()` | 0x6c54a0 | thiscall | match_eng.cpp (3.2 KB, called by 0x6b98c0): resets the play state (+0x1c5, +0x4782.., +0x8ea9/+0x8eaa, +0xf576.., the pending play +0x475a..) and, without +0x1a8: the 15 minutes since each side's last change (+0x1d2/+0x1d4); after 60 minutes, 1 in 6, the first player out of breath (+0x4d < 6750) narrated (0x21cf); each side's posture (0x6df460) stored in +0x1ab with the previous one; with posture 3/4 for the side with the ball, draw(10) of stoppage time (+0x8ed0, half in +0x8eb7) and the other side tiring in posture 1 (0x716450); the AI substitution (0x6e1090 on +0x9032 + 0xd0 × side; no human, or an automatic human) with minute > draw(50) + 10 and 55 of play since the last, 0x6bbb60 and 0x6bbf60; the narrated stoppage time (0x21c1) near the end of each half; the AI tactics by posture (0x5b50d0 marks on object +0x91e2 + 0x18e3 × side, by the manager's +0xe/+0x13, the formation +0x905d and +0x476a; narrations 0x2135 to 0x213d, 0x6d10a0, the squares' 0x400000 mark); with +0x8ebb the crowd chants (score 0x6ce310, fixed clubs [0x9d0230].., nation +0x76, "Ruud Gullit" 0x84b8b0); and the set piece by +0x8eb2 (throw-in, corner, kick-off, penalty, free kick, kick-off). Ghidra's pseudocode gets postures 2 and 3 wrong ("previous 3/4/5" and "previous 1/2" become "== true"): ported from the disassembly. Compared through the tape (set pieces are units with `GRAVAR_REINICIO`, since they reposition players through 0x6f09f0) |

Mutations: `mutacoes/leva135.py` (14, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): 12 caught — `r_acresc_narr` only by the comparison (narration 0x21c1's 7th argument); `r_canto` skipped (crowd chants only run in a narrated match, +0x8ebb); `r_postura` escaped (minute ≤ 2 instead of ≤ 1: at minute 2 the computed posture is the stored one). Branches to 2002-06-01: AI substitution 32768+, stoppage time 16384+, out of breath 8192+, postures 1/2 4096+, 3 2048+, 4/5 1024+, narrated stoppage time 4096+. Real game comparing to 2002-01-01: 0 divergences. Careers in the test folder `jogo_testes` (new reference, see COMO_CONTINUAR): Rust ×2 and original 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 136th batch — the shot (official on 2026-10-15)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `chute()` | 0x6f6c00 | thiscall, ret 0xc | match_pl.cpp (12.7 KB, called only by 0x6b98c0 with power +0x8ed6 and target +0x1c8/+0x1c9): the chance by distance to goal (table 0xa15e44 on side 0, 0x6f09b0 on side 1), stamina, the direction angle (+0x19a against +0x8ed8) weighted by the play +0x8ea9, power minus 20 × (+0x8eaa − 1)³ (and more by the engine's byte 1); with an angle up to 90 the player turns to the goal or to the target (0x6f4920). The play and the attribute: header 0x25 (+0x164); with +0x8ea9 4+ the long-range 0xc (power and clear path 0x6c41b0, +0x8ea9 becomes 2), 0x2e (+0x176) or 0x15 (+0x144); with 2/3 the 0x3b (+0x190) near goal, else 0xc (+0x132); the others 0xd (+0x134). Score = draw(attribute) − draw(stamina) − draw(angle) + draw(power). Three possible fouls: the shooter's before (0x6cfca0; recorded by draw(chance × 5 + 2), else punished as in 0x6f5c30), the challenger's (0x6c87e0, injury 0x6cfe10; punished as in the tackle, plus the +0x17 adjustment on a penalty) and the shooter's on him (narration by the play table, penalty given to the challenger). A challenger with the higher score takes the ball (foul narrated in the box, keeper save 0x70d430 or the play's narration; the ball spills below the limit; counts, stamina, +0x17; 1 in 7 0x1f41). Otherwise the shot goes on: +0x4782/+0x4786 lose score × 2/11; headed 0x2164/0x21cb/0x214f/0x1f5d and 0x7103e0; else 0x216e or the long-range narrations (0x21cc, 0x21db, 0x21dc, 0x1f47), +0x19c gains the difference (±200) and by draw 0x7103e0 (with +0x1a7 and +100 possession); outside the attacking box the player may grow (0x211d). The tackle's punished foul became a shared function (`punir_desafiante_com`); the shooter's (`punir_falta_de_quem_disputa_com`) repeats 0x6f5c30's, which keeps its inline copy (see COMO_CONTINUAR). Compared through the tape |

Mutations: `mutacoes/leva136.py` (17, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): all caught. The original `c_3b` (chance < 1.5 instead of < 1.41) was equivalent — table 0xa15e44 has no value between 1.41 and 1.5 — and was redone (< 1.4, removing the diagonal's 1.41). Comparing to 2001-08-20 the shot went through every marked branch (65,536+ shots without a challenger, 8,192+ with the challenger winning, recorded and punished fouls on both sides, penalty, sending-off, keeper save, 0x716c90 once). Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 137th batch — the referee's grade (official on 2026-10-15)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `nota_do_arbitro()` | 0x6d4a20 | thiscall | match_eng.cpp (336 bytes, called by 0x6b98c0): the match referee (+0xf5b6, id ≠ −1), except in friendly competitions (0x5c6710 on [0xae2c74]). Cards (+0xf5cb and +0xf601 on both sides) and fouls (+0xf5cc and +0xf602); the grade starts at +0xf5ba / 1000, loses 2 with more than 2 cards, 1 with 2, gains 1 with none; loses 1 with more than 5 fouls, gains 1 with fewer than 3; below 4 becomes 4 (except 1 in 500), above 8 becomes 8 (except 1 in 25), and stays within 1 to 10. Records through 0x79ce10 on [0xb74340] (8-byte record: id, cards, fouls, grade; the 8th byte is stack garbage and is not read). Compared side by side (draws and the referee's record) |
| `registrar_nota_do_arbitro()` | 0x79ce10 | thiscall, ret 4 | officials_manager.cpp (208 bytes): adds the grade to the referee's record (17 bytes at [this] + id × 17, id in 0..[0xae2390]): +0xf the cards, words +8 and +0xa the fouls and the grade, +0x10 the last grade, +0xc the matches and +4 the average (+0xa / +0xc, float). Compared side by side (the record) |

Mutations: `mutacoes/leva137.py` (9, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): all caught — 3 by the career (`a_cartoes`, `a_sem_cartao`, `a_faltas`: they change how many grades fall below 4 and therefore the draws) and 6 only by the comparison (`a_minuto`, `a_quatro`, `a_oito`, `a_cartoes_ficha`, `a_media`, `a_ultima`: the referee's record does not weigh on a 5-month career). 0x5c6710 (friendly competition), which the grade checks, has been Rust since batch 48 (`e_amistoso`). Comparing to 2001-08-20: 0 divergences; the branches `nota do arbitro minima`/`maxima` did not run (a grade below 1 or above 10 is rare). Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 138th batch — the stored taker, the stored instructions and the competition number (official on 2026-10-15)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `batedor_guardado()` | 0x897990 | thiscall, ret 0x10 | tactics.cpp (544 bytes, called by the taker choice 0x6ceb90): who takes set piece `tipo` (0..4) on `lado` (0..1): the cell (dword) in the club's tactics row (0x8982b0) at +0x601 + row × 0x91 + 0x5e + (side + type × 2) × 4. Above [0xae2364] + 0x52d it becomes −1; negative, 0; a person (0x6e bytes at [0xae23bc]) without +0x61 clears the cell; from the club (+0x39), in a national team (0x53b900) or with link 0x775790 they are returned; otherwise it clears. Null club, type or side out of range: the original (error box). Compared side by side (the cell) |
| `guardar_instrucoes()` | 0x896fe0 | thiscall, ret 0x10 | tactics.cpp (624 bytes, called by the engine step 0x6b98c0 and others): saves or restores the club's 20 instructions (dwords of the tactics row + 2) in one of 16 slots of 0x5c bytes (+0x30; used count at +0x5f0), the current human's ([0xb74c26 + [0xb77c7a] × 0x300]; [0xae2364] − id − 1); when saving, the instructions become −1. Compared side by side (the slots and the row) |
| `numero_da_competicao()` | 0x6b6570 | cdecl | match_day.cpp (304 bytes): the number by code (0: 5/4, 1: 11/10, 0x1ef: 1/0, 0x3de: 3/2, 0x483: 7/6, 0x528: 9/8; the first with the second argument 1); any other code: the original (error box). Compared by the result |

Mutations: `mutacoes/leva138.py` (12, through the career to 2002-06-01): the automatic career only goes through the trivial paths — the stored taker always with an empty cell (16 million times: the AI does not choose takers), the instructions only cleared (32 thousand times), the competition number never. It catches `i_menos_um` (the 20th instruction left uncleared changes the matches fingerprint); the other 11 are skipped as gaps (branch not reached) and the branches went into `docs/PENDENCIAS.en.md` for Lucas's comparing session (a human club with chosen takers, more than one human, a match day with a human). First batch with three routines at once (Lucas's proposal, 2026-10-15). Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 139th batch — the match wrap-up and the determinized original (official on 2026-10-15)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `favorito_de()` | 0x547790 | cdecl | does `b` have `a` among its three favourites (+0x9b, +0x9f, +0xa3)? Compared by the result |
| `rejeitado_por()` | 0x5477d0 | cdecl | does `b` have `a` among its three rejected (+0xa7, +0xab, +0xaf)? Compared by the result |
| `fechamento()` | 0x6d1cc0 | cdecl | match_eng.cpp (2.5 KB, called by the engine step 0x6b98c0): with the day's event list (+0x478e) and the match, walks the events (0x68d1b0): 0x82 goes to the global list [0xb67dc8] (0x68d0d0); 0x1e by the score, the clubs' strength and the other side's manager (0x544410) marks the event (+0xe, +0xf) or compares it with the club's strength; the others, from the match player (6 by marks +0x24/+0x23; the rest by draw). Then, per side, each player's record (0x6d26b0), the request to leave (event 0xa) and dissatisfaction (event 0xb, rival, favourite/rejected 0x547790/0x5477d0, preferences 0x547d30/0x547d60), and the average rating by minutes (±0.25 home/away) for the reputations (0x6d3610, 0x6d3de0). First routine of the **determinized original**: a side's average with no minutes and 4 bytes of the event built on the stack were garbage; the zeroing at entry leaves them at zero in every mode. Compared through the tape (the record, the reputations and the event copy are units; the event's content is checked) |

Mutations: `mutacoes/leva139.py` (12, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): all caught — 8 by the career (favourite, rejected, home average, the 90 minutes, substitute minutes, player event, event 6, offer) and 4 only by the comparison (`f_insatisfacao`, `f_rival`, `f_gosta`: the value asked of the dissatisfaction draw, checked by the tape; `f_lixo_evento`: one of the event's 4 bytes that used to be stack garbage, checked byte by byte at 0x68d0d0's input). The first `f_tipo6` (+0x24 = 3 instead of 2) was equivalent on the data (in the career's type-6 events +0x24 is never 2 or 3) and was redone by inverting +0x23. First batch of the determinized original: 0x6d1cc0 read a side's average with no minutes and 4 bytes of the event built on the stack with no value; with the zeroing at entry (`ZERADAS`) the original career did not change (459815f5e0dca92b) and the copy uses zero. Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 140th batch — the tactics row and the quick simulation (official on 2026-10-15)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ler_linha_tatica()` | 0x8983f0 | thiscall, ret 0xc | tactics.cpp: copies the club's tactics row (0x91 bytes at +0x601, row from 0x8982b0) to the destination. Null club: the original (error box); row -1: 0. Compared side by side |
| `gravar_linha_tatica()` | 0x8984d0 | thiscall, ret 0xc | tactics.cpp: the reverse, writes the 0x91 bytes into the row (row -1 or beyond the total +0x605: 0). Called by 0x6dbb10 (`guardar_tatica`). Compared side by side |
| `simulacao_rapida()` | 0x6beeb0 | thiscall | match_eng.cpp (6 KB, called by the engine step 0x6b98c0): the match without moves, in periods of two rounds. Per period, the team strength (0x6c0c60) with four pairs clamped between half and double, and the side that starts; per round, three attacks with a duel of an active player, the second and third duel (0x6c2880), the wear (0x6c2750), the other side's cards (0x6c2310; the second yellow sends off, 0x6cda20; more than 4 sent off: abandonment 3-0), the score factor, the contest (0x6c06b0/0x6c07a0/0x6c0a70), the shot (0x6c23f0, 0x6c0890/0x6c0980) and the goal (0x6c2050, keepers 0x717410, commentary 0x1fcc/0x1fcf, or 0x1fce on a missed penalty). At minutes 45/90/105/120 the scores (+0x45/+0x43/+0x47), extra time, golden goal and penalties; the aggregate; the home goal; the injuries (0x6d0980); the ratings (friendly: by the club's reputation). Used to read high bytes of stack dwords with no value (determinized original). Compared through the tape (commentary, injuries and the player-leaving calls are units) |

Mutations: `mutacoes/leva140.py` (23, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): 21 caught by the career and 2 gaps (abandonment by sendings-off and the restart of the attack counter never happen in the career; in `PENDENCIAS.en.md`). The first `t_gravar` (writing one byte less) was equivalent on the data and was redone copying from the shifted source; the first `s_contador` escaped the career and the comparison and revealed the gap (it got the branch and the sibling `s_contador_igual`, caught). In the first comparison, 421 draw divergences in 0x6beeb0 with equal memory: the comparators of 0x6c0c60 and of the choices (0x6c2750...) do the side-by-side even inside the tape recording, and their copy's draws went into the original's trace (the team strength showed up twice). Fixed in `comparacao::lado_a_lado`: the Rust side runs with the trace paused. Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 141st batch — the injury and the player highlight (official on 2026-10-15)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `lesionar()` | 0x6d0980 | thiscall, ret 0x10 | match_eng.cpp: the player's injury in the match. With more than 3 injuries in the match (+0xf639) draws may avoid it; more than 6, never. Registers it (0x62be00), marks the culprit (+0x55, -10 at +0x31), takes stamina by severity (above 50: the square; otherwise a draw), writes +0x59 and reduces +0xd5; with no stamina the player leaves, otherwise stops (0x7160e0 with 5 or 3) by the person's strength. Compared through the tape when it runs outside another tape (the injury list and the 0x68d0d0 copy); inside, it is a unit |
| `registrar_lesao()` | 0x62be00 | thiscall, ret 0xc | injury.cpp: the record's injury in the injury list (+0x28 of [0xb63d28]): same injury adds the severity (up to 100), another keeps the worse; a new record goes to the list (0x68d0d0, the record built on the stack checked by the tape). No list: the original (creates the list) |
| `destaque_do_jogador()` | 0x717470 | thiscall, ret 4 | can the player earn highlight `codigo` (0x1f40..0x21e4)? Not if the side already had it (table at engine +0x21e) or the record is from outside; 0x219e..0x21b8 with rules on performance, goals, cards, time on the pitch and the history search (0x757da0, last 7 or 30 days). Compared side by side |

Mutations: `mutacoes/leva141.py` (15, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): 14 caught by the career and 1 gap (`l_para3`: the light injury that stops and leaves never happens; in `PENDENCIAS.en.md`). The highlight is only granted 16 times in the career: its first mutations (limits, range, default) and three of the redone ones (the 0x219e search date, the 0x21b1 code, the common condition) change no result on the data and left the list; `d_sim` (forces 0x21b1) and `d_21b5` (without the +0x30 condition) prove the comparison sees the result. The injury (0x6d0980) only runs inside other tape comparisons, as a unit: its copy is proven by the career and the mutations. In the first comparison, compare mode changed the career (5,264 extra draws): the highlight comparator's Rust side was missing `sem_contar`. Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 142nd batch — the shot on goal and the shot outcome (official on 2026-10-15)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `chute_a_gol()` | 0x70a910 | thiscall, ret 4 | match_pl.cpp (5.9 KB): `p`'s shot against the other side's keeper (0x6ce110, save 0x700420). Quality from strength and draws gives the type: wide (0x1fc6..0x1fcb, goal kick), on target; on target, the shot's share against the save decides goal (player counts, hat-trick, career milestones 0x200a..0x2012, score and time commentary, penalty shoot-out 0x6bba50/0x6bac40), corner, byline or save (keeper highlight 0x717470). Compared through the tape (commentary, set piece, 0x716e10, 0x6f0630 and the foul's calls as units) |
| `desfecho_do_chute()` | 0x70c040 | thiscall, ret 4 | match_pl.cpp (5.1 KB): the same structure against the keeper of the shooter's own side (the goal goes to the other side, commentary and side factor mirrored). Determinized original. Compared through the tape |

Mutations: `mutacoes/leva142.py` (20, through the career to 2002-06-01, confirmed in compare mode to 2001-10-01): 17 caught by the career and 3 gaps (the career milestones and the late equaliser of the shot on goal; in `PENDENCIAS.en.md`). The first `c_fator` changed only the last bit of the 0.1 factor (no effect after rounding) and was redone with 0.2. A batch of two large routines (11 KB together, the last of the shot). 0x70c040 reads the save's rebound and exit flags with no value when the keeper himself shoots: it joined the determinized original (`ZERADAS`). The milestone commentaries pass word counts as the last argument: the tape checks the low byte and the Rust copy passes them whole. Both (and batch 141's injury) only ran as units of other tapes, uncompared: now, inside the unit, they run their own comparison (nested tape); for the injury this needed the list's nodes and pointer in the regions. Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 143rd batch — the player move (official on 2026-10-16)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `mover_jogador()` | 0x6f09f0 | thiscall, ret 0x14 | match_pl.cpp (9 KB): the player moves one cell (two, if fast) towards (x, y) with the given action. Without the ball, the tackle injury and, if not facing the target, only turns. With the ball: the attack from stamina and action, the approach commentary, the challenger (foul with event 0x82, clean contest), possession and the action commentary; then the grid, the furthest forward, the marked players released, wear from the position's marks and the ball (follows it or takes the loose one). Compared through the tape (commentary, 0x714d40, control, set piece, ball out, the event and the injury and foul calls as units) |

Mutations: `mutacoes/leva143.py` (22, by the career to 2002-06-01, confirmation in compare mode to 2001-10-01): 20 caught by the career and 2 gaps (`m_vaga` and `m_adiantado`: the player with no grid slot and the side with no furthest-forward player recorded; in `PENDENCIAS.md`). A batch of one large routine (9 KB). It only ran as a unit of other tapes: the comparator records the unit and, inside it, runs its own comparison (as in batch 142). Event 0x82 goes to the list built on the stack: 0x68d0d0 checks the contents, not the address. In the first chain the Rust career timed out: the Rust-mode dispatch sent 0x714d40 and 0x68d0d0 to the dividida's default call; the movement's own dispatch fixed it (new pitfall). Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 144th batch — the player on the ball (official on 2026-10-16)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `conduzir()` | 0x7109f0 | thiscall, ret 4 | match_pl.cpp (13 KB): the decision of the player on the ball. Set pieces: corners, throw-in and free kicks pass (0x6c6a30 picks the team-mate, otherwise a drawn cell); penalty and free kick near goal shoot (0x700bd0 + 0x702050). In play, shot, pass and dribble scores from the distance to goal, the pressure (0x6bcbd0, with list), the score, the keeper, the position's marks and the clock; long ball (0x6fa490), dividida (0x70ec80), shot, pass (0x6fa250) or dribble (0x6f09f0) to 0x6bce80's cell. Mode 4: the keeper in the shoot-out (returns 1 when he stays). Determinized original. Compared through the tape |

Mutations: `mutacoes/leva144.py` (24, by the career to 2002-06-01, confirmation in compare mode to 2001-10-01): 23 caught by the career and 1 gap (`c_goleiro`: the position-1 guard without the goalkeeper mark; in `PENDENCIAS.md`). A batch of one large routine (13 KB, the decision of the player on the ball). It reads variables with no value in mode 4 (the keeper in the penalty shoot-out: the pass score at 0x712ec4 and the team-mate at 0x713107 are never written) and passes bytes with an unset high part to the pass and the shot: it joined the determinized original (`ZERADAS`). The pressure list (0x68d020) lives on the stack: the copy into the list (0x68d0d0) enters the tape without the list's address and the shot (0x702050, still original) enters as a unit with the list head ignored. The pass, the long ball and the dividida run inside the copy (their units go into the dribble's tape). Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 145th batch — the engine loop (official on 2026-10-16)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `laco_do_motor()` | 0x6b98c0 | thiscall, ret 4 | match_eng.cpp (4.8 KB): `n` match instants. Match already decided: quick simulation, highlight, closing, humans' instructions restored, referee rating (2). Each instant: punishments, shoot-out, kick-off, drawn substitution, marked incident, restart and taker; on the clock, abandonment (8), period (4) and full time (2 or 4); then possession, each player's time, lines, nearest to the ball, marking and each active player's decision (0x7103e0), with the pending shot and the contest. Validated by the career |

Mutations: `mutacoes/leva145.py` (18, by the career only to 2002-06-01): 16 caught and 2 gaps (`l_limpeza`, the on-field record with no engine; `l_direcao`, the set-piece taker with no ball owner; in `PENDENCIAS.md`). Three from the first round were equivalent or weak (−5 for −6 in a mark only tested as negative; the clock at kick-off is never 2; 19 versus 20 draw attempts) and were redone (`leva145b.py`). A batch of one routine: the engine loop, which only orders calls. No side-by-side comparator (the tape would snapshot the engine at every player decision): validated by the career, like the routines with many effects. The routines it calls stay compared. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba. Seed 777 to 2003-06-01, Rust and original: 4,738,450,581 draws, fingerprint 17dcb32ab70df0bd, matches b006b77a7a0cc3ec, database 480fccdff099e75f

### Match engine, 146th batch — the pass execution (official on 2026-10-16)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `executar_passe()` | 0x6fad40 | thiscall, ret 0x1c | match_pl.cpp (22 KB): `p`'s pass (type 6..=0x5e) from (x, y) to (tx, ty), with receiver and the output (1 when it does not reach him). Score by type and distance; challenger with foul or dispossession; without him, stamina, inaccurate pass by +0x4756's factor, cost in +0x35, approach and pass-type commentary, the next set piece (+0x8ea9/+0x8eaa) and the trace (0x6ccb60): shot (0x70c040/0x70a910) or ball in transit (+0x1c5..+0x1c9, +0x477e). Compared through the tape, with comparison inside the unit |

Mutations: `mutacoes/leva146.py` (25, by the career to 2002-06-01, confirmation in compare mode to 2001-10-01): 24 caught by the career and 1 by the confirmation (`p_cai`: the draw changes, the outcome does not, and the tape checks the draws). A batch of one routine, the largest so far (22 KB: the function map cut at 0x6fe364 a routine that runs to 0x6fffee). First batch read from the package (`pacote.py`), which showed the real size, that the pass trace 0x6ccb60 has this single caller (it became a unit of 0x6fad40's tape) and the old note on the output pointer. Challenger, foul and dispossession in their own versions (the write order, the penalty draws, event 0x82 outside the card test and the commentary differ from batch 143); the commentary by pass type in tables (four groups of seven by the output and +0x1a7, the pass direction picks within the group). Comparing to 2001-08-20: 0 divergences, 12 of 17 branches. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 147th batch — the shot (official on 2026-10-16)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `chutar()` | 0x702050 | thiscall, ret 0x18 | match_pl.cpp (35 KB): `p`'s shot (type, value, commentary, mode 1 match / 2 no challenger / 3 shoot-out / 4 set-piece kick / 5-6 repeat, the pressure teammates list). Challenger (0x6c87e0) with foul or dispossession; commentary; offside; interception (0x6cc540); the outcome class; goal (can be disallowed), save, post or wide, with the commentary. Compared through the tape, with comparison inside the unit |
| `atras_no_agregado()` | 0x6d5950 | thiscall, ret 4 | match_eng.cpp: the side is losing on aggregate (0x6ce310 without bonus) or, in a first leg with no third phase and no qualified side, level |
| `companheiro_livre()` | 0x6d5a70 | thiscall, ret 4 | match_eng.cpp: the first active teammate in the deep attack (0x70df40), central (0x70df20) and unpressed (0x6bcbd0) |

Mutations: `mutacoes/leva147.py` (26, by the career to 2002-06-01, confirmation in compare mode to 2001-10-01): 25 caught by the career and 1 gap (`k_bloqueio`: the shot blocked on the way to goal did not happen; see `PENDENCIAS.en.md`). The largest routine ported (35 KB, 10,356 instructions, recursive: the foul on a set-piece kick and the repeated move shoot again). The challenger, the foul and the dispossession in their own versions; the rest in parts: the outcome class by the score against the limit, the ratio shot/(save + shot) decides goal (above 71), parry (61), corner (51) or held save; the goal goes through up to five chances of being disallowed by the marked incident (0x6bba50/0x6bac40), the scorer's milestones, the comeback and the score (0x6d4b70 stays in the original); the shot off target has the commentary by class, by the moment (+0x4782/+0x4786) and by the teammates in the pressure list. The interception (0x6cc540) runs inside the copy (its comparator records no unit) and the plays are the units. The 0x702050 recording port goes away: the comparator records the unit whenever there is a tape. Comparing to 2001-08-20: 0 divergences, 23 of 31 branches. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 148th batch — the engine is born, prepares the match and dies (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `construir_motor()` | 0x6b7e50 | thiscall | match_eng.cpp: the engine constructor (the player, tactics and tactical line vectors through the runtime vector constructor 0x9463af; the referee strictness 0x6e9460; the empty list) |
| `preparar_partida()` | 0x6b7f20 | thiscall, ret 0x10 | match_eng.cpp (6 KB): clears the state, the weather (0x923010/0x923970/0x923630), the kits, the referee, the pitch (0x85cba0, +0x1ad/+0x1ae), the squads (0x6db500), the commentator (given or new), the side strength, the marking, the angle table and the pre-match event list (unhappiness 0x13, interest 0x1e, form 5/4, contract end 6); draws who kicks off |
| `destruir_motor()` | 0x6b97a0 | thiscall | match_eng.cpp: the destructor (its own commentator, the list, the strictness and the vectors) |

Mutations: `mutacoes/leva148.py` (18, by the career to 2002-06-01) and `mutacoes/leva148b.py` (the 4 that escaped, redone stronger): 16 caught by the career and 2 gaps (`i_ceu`: the rainy-sky limit does not matter in the automatic career; `i_insatisfeito`: the code of event 0x13 does not change the result; see `PENDENCIAS.en.md`). No comparison of their own: the three allocate (the commentator and the event list) or free, and the tape would have to reproduce the allocations; validated by the career (Rust × original), like the engine loop. The routines they call stay compared. The angle table (0xb67c30), computed in the first match with fpatan, comes from Rust's atan and matches. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 149th batch — each side's selection and the match day (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `escalar_lado()` | 0x6db500 | thiscall, ret 0x24 | match_man.cpp: sets up one side's tactics (manager, players, club, opponent, match, home side) and selects: at the human's club the stored lines (0x5fbe20/0x5fbd00) and the automatic 0x6dbc50 mode 1 or the regulars (0x6dbb80); otherwise the automatic mode 2 and the count of the home players; the substitutes by the bench and the club's tactical line. Returns how many it selected |
| `encerrar_dia()` | 0x6b4190 | thiscall | match_day.cpp: frees the global list, the day's vectors and, for each match, the engine (0x6b97a0), the summary and the +0x5f object |
| `escolher_partida_do_dia()` | 0x6b42a0 | thiscall | match_day.cpp: clears each of the day's matches' fields and picks the human's (or the first marked) for the screen (0x717cc0, 0x7fc7e0) |

Mutations: `mutacoes/leva149.py` (8, by the career to 2002-06-01): 7 caught and 1 gap (`e_ficha`: the second field of the manager's record does not matter in the automatic career; see `PENDENCIAS.en.md`). No comparison of their own: each side's selection calls the automatic one (0x6dbc50, still in the original, which allocates the list and reads the stack) and the match day frees engines and lists; validated by the career (Rust × original). The routines they call stay compared. Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 150th batch — the automatic selection (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `escalar_automatico()` | 0x6dbc50 | thiscall, ret 8 | match_man.cpp (~7.7 KB): the staff selects the team against the opponent. Outside the human, weighs today's match against the next one on the list (0x5acbe0: competition reputation, rival, same city, days until it) to rest players (+0x1a 0/1/2 and the slot masks); filters the 50 of the lines (state, suspension, already played, barred, on loan), selects (0x6e2000), picks the formation (0x6e0230), the three references (+6/+0xa/+0xe), the rotation (0x8cf880/0x6e8910), the slots (0x6e0d00), the captain (0x897760), the instructions (0x6ddab0) and the general orders by the stance (0x5b50d0). The locals live in a frame with the original stack offsets (zeroed, like the determinised original) |

Mutations: `mutacoes/leva150.py` (17, by the career to 2002-06-01): 12 caught and 5 escaped for being weak; redone stronger (`mutacoes/leva150b.py`: the days weight, the “already played” filter, the mask, the rotation and stance 3), all 5 caught. No comparison of its own: the automatic selection allocates the list and the rotation vectors and calls 24 routines still in the original; validated by the career (Rust × original). The routines it calls stay compared. Real game comparing to 2002-01-01: 0 divergences (6500 selections by the probe, none with garbage). Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 151st batch — the progress of the match day (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `iniciar_proxima_partida()` | 0x6b5a70 | thiscall | match_day.cpp: starts the day's next match after the current one (+0x2f): skips the decided ones, those of a human club and those already started; creates the engine (0x6b7e50) and prepares it (0x6b7f20); in the 1st round with none, moves to the 2nd. Returns 1 if it started one |
| `andar_partida_do_dia()` | 0x6b5ce0 | thiscall, ret 4 | match_day.cpp: one engine step of the current match (1 in 3 or 6 calls, by the modes in [0xae3138]) or to the end; when it finishes, creates the summary, records it (0x7293f0), frees the engine and takes it off the queue. Returns 1 if it finished |
| `fechar_partida_do_dia()` | 0x6b5f00 | thiscall, ret 4 | match_day.cpp: copies the day's match into the match, records the result in the competition (method 1, or 0x924050 and 0x5ab510/0x5aa680 for the away game) and runs the post-match: players, events, contracts, suspensions, states, the history and each side's club (0x6b79a0) |

Mutations: `mutacoes/leva151.py` (14 and 2 redone, by the career to 2002-06-01): 6 caught and 8 escaped; with the branch counters in the base: 2 were weak and, redone (`mutacoes/leva151b.py`: half time as the end, the match copy), both caught; 1 is equivalent (the decided-match test, -2, is redundant: the chosen one needs -1); 5 become 4 gaps in `PENDENCIAS.en.md` (the step by step and the human's round do not run in the automatic career; the finished counter only serves the bar; the history is not part of the fingerprints). No comparison of their own: the match day creates and frees engines and summaries and calls dozens of routines still in the original; validated by the career (Rust × original). Real game comparing to 2002-01-01: 0 divergences. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba

### Match engine, 152nd batch — the match day loop (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `jogar_o_dia()` | 0x6b4360 | thiscall | match_day.cpp (~3.3 KB): plays the day. On a network, ties the humans to their matches (0x11d-byte scoreboard, 0x765480, 0x5fcff0/0x5fd100) and plays them to the end; then each match without an engine: with a human on a network, steps the current one (0x6b5ce0) and restarts; otherwise creates the engine, plays to the end, creates the summary, records it and writes the score with the percentage on the bar (0x66f4e0, 0x802f80). Finally closes each match (0x6b5f00), redoes the humans' stored tactics and frees the summaries |

Mutations: `mutacoes/leva152.py` (8, by the career to 2002-06-01): 5 caught and 3 escaped: the queue one was weak and, redone without the call (`mutacoes/leva152b.py`), was caught; the humans' lines one does not run in the automatic career (new gap in `PENDENCIAS.en.md`); the finished counter one is the gap already noted in batch 151. No comparison of its own: the day loop creates and frees engines and summaries and touches the screen; validated by the career (Rust × original). Real game comparing to 2002-01-01: 0 divergences (new fingerprint 963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: all three equal, on a new reference (with the day loop hooked, the originals it calls run on the private stack and read other garbage, the same in both modes): 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 153rd batch — the match day setup (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `montar_o_dia()` | 0x6b3c10 | thiscall, ret 8 | match_day.cpp: sets up the day (date, mode 0..2): zeroes the vectors, gathers the pending matches of each list (0x5ae6a0), records the already decided ones (0x923630/0x923cc0), groups by list and by competition (qsort 0x6b6510), points each match to its competition (+0x63), creates the global list [0xb67dc8] and works out the day's interest (0x6b5040). Returns the day |
| `lado_do_jogador()` | 0x714d30 | thiscall | match_pl.cpp: the player's side (+0x27) |
| `sons_terminados()` | 0x6d8c70 | thiscall, ret 4 | match_events.cpp: have the sounds of the 16 marked events played and passed their window (clock 0x90c510; stops the finished channels, 0x802a50)? |

Mutations: `mutacoes/leva153.py` (8 and 1 redone, by the career to 2002-06-01): 4 caught and 4 escaped; by the branch counters, they become 3 gaps in `PENDENCIAS.en.md`: the day's 1st round picks nothing in the automatic career (2 mutations), no match reaches the setup marked -2 (redone strong in `mutacoes/leva153b.py`, escaped too) and the event sounds only run with the screen. No comparison of their own: the setup allocates the day's vectors and calls routines still in the original; the side is read-only and the sounds are screen; validated by the career (Rust × original), with branch counters. Real game comparing to 2002-01-01: 0 divergences (fingerprint 963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e. Tests run with another project on the machine (CMRTE) open at the same time, after adjusting the `mutacoes.py` lock (only this folder's exes): times equal to the previous batches

### Match engine, 154th batch — the recorded events and the network step (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `gravar_eventos()` | 0x6d98b0 | thiscall, ret 8 | match_events.cpp: records the events (+0x720, 0xe bytes each) in one of the two "events1/2.tmp" files (human or computer) in CM3_TEMP; header version 1, number, count; with both full (2500), recreates the oldest. In `desc`: position, count, number, human |
| `ler_eventos()` | 0x6d9c40 | thiscall, ret 4 | match_events.cpp: reads back the events described in `desc` from the file with the right number |
| `passo_em_rede()` | 0x6b55c0 | thiscall, ret 4 | match_day.cpp: one step of the human's match on a network, by the clock and the humans' scoreboard (everyone sees each state; sounds, events, engine step, end, half time, goals, minute); records it (0x729ff0) and picks the day's human match |

Mutations: `mutacoes/leva154.py` (6, by the career to 2002-06-01): all 6 escaped, and the branch counters explain it: recording the events runs ("eventos arquivo com espaco" above 2048, "eventos arquivo novo" 2), but goes to temporary files that are not part of the fingerprints; reading them back and the network step do not run in the automatic career. They become 2 gaps in `PENDENCIAS.en.md`; their test is in the game (replaying the plays of a watched match). No comparison of their own: they touch files and only run with the screen or on a network; validated by the career (Rust × original). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 155th batch — the match events reader (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ler_eventos_cfg()` | 0x6d9e50 | cdecl | match_events.cpp (~5.2 KB): reads the `events_<language>.cfg` (or `events_eng.cfg`) into the table of the 677 event codes at [0xb68788] (7 option bytes, the 0x110-byte sounds with weight, type, name and the two texts `G`/`R`); checks `<X` tokens, events without text and, with `-verifysounds`, the sound files; at the end converts the weights and gives the default sound to those without one |

Mutations: `mutacoes/leva155.py` (6 and 2 redone, by the career to 2002-06-01): 2 caught (the sounds' largest weight and, redone, option 2) and 6 escaped: the sounds' weights, types and names and options 0, 1 and 4 only serve the audio, the screen and the network step (gap in `PENDENCIAS.en.md`; in Lucas's game the commentary and sounds worked). No comparison of its own: it reads a file, allocates the texts and writes the global table once at startup; validated by the career (Rust × original). Alongside, batch 154's read of the plays (0x6d9c40) got a real comparison (it only reads a file): in Lucas's game, 0 divergences. Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 156th batch — the event text and the start of the network match (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `texto_do_evento()` | 0x6d8d10 | stdcall, ret 0x10 | match_events.cpp: the event sound's `R` text with the tokens replaced (`<P1>` player, `<T1>`/`<T2>` club, `<C>` competition, `<F>` stage, `<D>`/`<N>` number, `<rf>`/`<Rf>` referee), `{...}` prefix and suffix (0x6701f0) and upper case; writes to [0xb67ea0] |
| `confirmar_partida_em_rede()` | 0x6b5260 | thiscall, ret 4 | match_day.cpp: each human confirms the network match; with all of them, creates the engine and prepares it for watching, or plays it to the end and records it |

Mutations: `mutacoes/leva156.py` (3, by the career to 2002-06-01): all 3 escaped; the branch counters ("texto do evento", "partida em rede confirmada") stay at zero: both only run with the match screen (gap in `PENDENCIAS.en.md`). The event text got a real comparison (it only writes to the text buffers): in `JOGAR_COMPARANDO`, each commentary line is built by the original and the copy and checked. The network confirmation has no comparison (it creates and frees engine and summary). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 157th batch — the commentary on the panel (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `narrar_no_painel()` | 0x6d7b00 | thiscall, ret 0x20 | match_events.cpp (~4.4 KB): restores the crowd sound, builds the event's `G` text (like 0x6d8d10) into a line with the club colours (`<O>` the other's, `<Y>`/`<R>` the default ones), wraps or cuts with "..." by the width (0x5e5ad0, 0x90cb20), draws on the panel (0x801040…0x800580), plays the event sound (0x8025e0) and marks the time and duration (+0x2d6c, +0x2dac) |

Mutations: `mutacoes/leva157.py` (2, by the career to 2002-06-01): both escaped; the counter "narracao no painel" stays at zero: the routine only runs with the match screen (gap in `PENDENCIAS.en.md`; the test is watching matches in `JOGAR.bat`). No comparison of its own: it draws on the panel and plays sound. With it, the match engine without the screens is 100% in Rust. Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Save, 158th batch — saves above 2 GB (the game container) (official on 2026-10-17)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `abrir_no_conteiner()` | 0x9326e0 | cdecl | zipdir.cpp (`zopen`): opens item `nome` of the container (12-byte header at +0x19, index of 1000 items of 0x10c bytes at +0x25: position, size, name); writes appending at the end (max. 999) or reads seeking to the position. **Deliberate difference:** unsigned position and 64-bit seek/tell (the original uses signed 32-bit `fseek`/`ftell`: above 2 GB the index gets 0xffffffff and reading fails); above 4 GB it refuses the item without damaging the file |
| `buscar_no_conteiner()` | 0x933080 | cdecl | zipdir.cpp (`zseek`): moves within the item open for reading (origin 0/1/2); outside the item, -1; type 4 (RLE) goes back to the start and re-reads up to the point, type 3 seeks directly. Unsigned positions, 64-bit seek |
| `posicao_no_conteiner()` | 0x933170 | cdecl | zipdir.cpp (`ztell`): type 4 returns item position + z+8; type 3, `ftell` (64-bit here, discounting the buffer like 0x946aae) minus the item position |

Mutations: `mutacoes/leva158.py` (4, by the career to 2002-06-01): all 4 escaped; the automatic career neither saves nor loads (gap in `PENDENCIAS.en.md`). The proof is `save_grande.py`: a quick start with a 2.2 GB filler makes a 2.38 GB `.qst`; in the original the 45 items beyond 2 GB get position 0xffffffff, with batch 158 the index is right. In Lucas's game: a 2.55 GB save (187 items beyond 2 GB) saved and loaded with our DLL; with the originals (`CM_ORIGINAL=1`) loading fails (`Friendly.cpp`, first item beyond 2 GB). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 159th batch — the formation: player positions and pitch cells (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `mascara_de_posicoes()` | 0x5b7d20 | cdecl | formation.cpp: the strong-position mask by the ratings (+0xf..+0x15, level 15 down to 10) and the sides rated 15 (+0x17..+0x19); returns only `ax` |
| `casa_no_campo()` | 0x5b7e60 | cdecl | formation.cpp: column 0..26 and row 0..35 of the position (sector and side via 0x5b73a0) by attack and band (tables of 4 and 5, nudges of 1 or 5 in the bands, limits) |
| `vagas_sem_conflito()` | 0x5b8960 | thiscall | formation.cpp: 0 if any of the 11 slots with an open side (0x880) is a striker's, midfielder's, defensive or attacking midfielder's (cells +0x12d/+0x14e and 0x5b2ae0); returns only `al` |

Mutations: `mutacoes/leva159.py` (7, by the career to 2002-06-01, confirming the escaped ones in comparison mode to 2001-08-20): 5 caught; `m_nivel` (stepping the level down by 2) and `v_a20` (0x10 instead of 0x20) escaped both proofs: in the game database the natural position is rated 20 (the loop stops at level 15) and no slot with an open side is a midfielder's (gap in `PENDENCIAS.en.md`). First batch outside the match engine: the candidates come from `candidatos.py --simulacao` (coverage of the automatic career in 26 simulation modules, `docs/cobertura_simulacao.txt`). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 160th batch — player statistics and centre-backs in the formation (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `estatistica_do_jogador()` | 0x7c2150 | stdcall | player_stats.cpp: statistic `qual` 1..22 of the record (bytes, percentages ×100.0f, per-match averages, the two ratings completed with under 20 matches ×0.05f), in x87 like the original |
| `media_das_notas()` | 0x7c1950 | thiscall | player_stats.cpp: the average of the record's five 4-bit ratings (zero counts 27), 16-bit, divided by 4 |
| `zagueiros_sem_conflito()` | 0x5b8dc0 | thiscall | formation.cpp: 0 if any slot with an open side is a centre-back's or defensive midfielder's |

Mutations: `mutacoes/leva160.py` (7, by the career to 2002-06-01): all 7 caught. The floating-point statistic (f64 divisions and products) matches the original's x87 over the whole career. Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 161st batch — shortlist slot, position code and staff criterion (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `serve_para_a_vaga()` | 0x844300 | stdcall | shortlist_manager.cpp: the player fits shortlist slot 0..15 (rating 15 in sector and side; slot 15 needs a contract, an id and 0x600a50 at zero); returns only `al` |
| `codigo_da_posicao()` | 0x83bd00 | stdcall | shortlist_manager.cpp: the position code from the sector and side mask (28 combinations; the rest 0xe); returns only `al` |
| `criterio_da_comissao()` | 0x866f60 | thiscall | staff_contracts.cpp: the staff criterion: an 8-bit score (id mod 7, ratings, age, reputation, number of roles) decides by the roles whether it returns 1 |

Mutations: `mutacoes/leva161.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 6 caught. `s_id` escaped: it only changed the exact id limit (−16 to −17); redone as `s_id_b` in `mutacoes/leva161b.py` and caught (`mutacoes_refeitas.txt`). `c_idade` escaped: not even redone with +15 (`c_idade_b`) is it caught: no staff member has +0x44 between 51 and 75 in the data. `c_rep` escaped: it only changed the exact reputation 0x1c53; redone as `c_rep_b` and caught. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 162nd batch — the tactics table slots (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `gravar_vaga_tatica()` | 0x896a90 | thiscall | tactics.cpp: stores the person in slot 0..19 of the club row (0x8982b0) and clears the club's pending instructions |
| `pessoa_da_vaga_tatica()` | 0x8968f0 | thiscall | tactics.cpp: the slot's person; without a record or from another nation in a national team, the slot becomes −1 |
| `gravar_formacao_tatica()` | 0x8967d0 | thiscall | tactics.cpp: stores the formation in the club row |

Mutations: `mutacoes/leva162.py` (5, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `g_pendente` escaped: pending instructions (+0x5f0) only exist with a human manager. `p_ficha` escaped: in the automatic career the tactics table slots are empty (−1): not even the inverted condition (`p_ficha_b`, `mutacoes/leva162b.py`) is caught; the part with a person only runs on the human manager's tactics. `p_nacao` escaped: likewise (the inverted `p_nacao_b` also escapes). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 163rd batch — the formation (flags and cells) and resetting the club records (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `casa_guardada()` | 0x5b3c70 | thiscall | formation.cpp: the stored cell of the slot by attack and band |
| `ligar_marca_da_formacao()` | 0x5b50d0 | thiscall | formation.cpp: sets the flag in +0x584 clearing the same group and zeroes +0x187f |
| `reiniciar_recordes()` | 0x452fe0 | cdecl | club_records.cpp: −1.0f and −1 in the club record fields |

Mutations: `mutacoes/leva163.py` (5, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 5 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 164th batch — the league calendar (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `girar_rodada()` | 0x68a030 | cdecl | league.cpp: the next position in the calendar rotation (special league, jumps of 3 by draw, wrapping at n) |
| `registrar_jogo_da_rodada()` | 0x68a1c0 | cdecl | league.cpp: stores the round match (three bytes and a dword) |
| `marcar_ano_no_calendario()` | 0x5af180 | cdecl | fix_man.cpp: marks the match year in both clubs' entries (year × 2 + bit, 2001 rule) |

Mutations: `mutacoes/leva164.py` (7, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 5 caught. `g_meio` escaped: the automatic career only plays English leagues, without the rotation's 0x80 flag; not even redone stronger (`g_meio_b`, `mutacoes/leva164b.py`) is it caught. `g_volta` escaped: wrapping at the end of the rotation does not happen in the career; not even redone stronger (`g_volta_b`) is it caught. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 165th batch — extra tactic slots, national team ages and the human manager field (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `vaga_extra_tatica()` | 0x897ee0 | thiscall | tactics.cpp: the two extra slots (+0x56, +0x5a) of the club row |
| `idades_da_selecao()` | 0x76d430 | stdcall | national_teams.cpp: the age limits (23/23 in youth competitions, 24/22, 26/22) |
| `gravar_dado_do_humano()` | 0x5fdd30 | thiscall | human_manager.cpp: stores +0x9a65 of the human manager record (0xa6ef bytes) |

Mutations: `mutacoes/leva165.py` (5, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 4 caught. `v_desce` escaped: when the first extra slot is cleared, the second is already empty (−1) in AI clubs: moving −1 down or writing −1 is the same (the counters "vaga extra apagada" 65536 and "vaga extra gravada" 4096 show both paths run). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 166th batch — the human's scout, the human at the club and the contract category (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `registro_do_observador()` | 0x7f6cd0 | thiscall | scout_manager.cpp: the human manager's scouting record 0..9 (0x2b0 bytes) |
| `humano_manda_no_clube()` | 0x600c90 | cdecl | human_manager.cpp: does the current human manager run the club (or the nation's main team)? |
| `categoria_do_contrato()` | 0x4ca470 | stdcall | contract_manager.cpp: the player's category 1..8 in the contract (status, happiness, age, ambition, reputation) |

Mutations: `mutacoes/leva166.py` (7, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught. `h_forcar` escaped: counter "humano manda no clube" stays at zero: with no human manager the routine never returns 1 in the career. `c_k0` escaped: exact limit; not even redone stronger (`c_k0_b`, `mutacoes/leva166b.py`) is it caught: the status-0 range with happiness 40 to 75 does not appear. `c_k5` escaped: exact limit; redone as `c_k5_b` and caught. `c_tipo5` escaped: type-5 contracts do not appear in the career; not even redone stronger (`c_tipo5_b`) is it caught. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 167th batch — closing the engine (1): the ball's goal end, the abandoned match and the side's slots (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `bola_na_area()` | 0x6d58f0 | thiscall | match_eng.cpp: 1 with the ball in the top box, −1 in the bottom one, otherwise 0; returns only `al` |
| `partida_abandonada()` | 0x6d5930 | thiscall | match_eng.cpp: the abandonment minute × 11 below the clock (16-bit); returns only `al` |
| `vagas_do_lado_sem_conflito()` | 0x6d5af0 | thiscall | match_eng.cpp: 0x5b8960 on the side's formation |

Mutations: `mutacoes/leva167.py` (4, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught. `a_onze` escaped: no match is abandoned in the automatic career (the abandonment minute stays at −1). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 168th batch — closing the engine (2): the side's centre-backs, the referee and the match player (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `zagueiros_do_lado_sem_conflito()` | 0x6d5b20 | thiscall | match_eng.cpp: 0x5b8dc0 on the side's formation |
| `construir_arbitro()` | 0x6e9460 | thiscall | match_official.cpp: the referee constructor (zeroes +0 and +4) |
| `destruir_jogador_da_partida()` | 0x6ebd60 | thiscall | match_pl.cpp: the match player destructor (zeroes +0x19e) |

Mutations: `mutacoes/leva168.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 169th batch — closing the engine (3): the commentary queue (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `evento_na_frente()` | 0x6d5d00 | thiscall | match_events.cpp: puts an event at the front of the commentary queue (steps the 12-slot ring back, drops the repeat) and queues it via 0x6d6f70 |
| `construir_fila_da_narracao()` | 0x6d5bf0 | thiscall | match_events.cpp: the commentary queue constructor: clears the queue, reads the events and, if asked, plays the crowd loop |
| `liberar_tabelas_de_eventos()` | 0x6d5b50 | cdecl | match_events.cpp: frees the loaded event tables and each record's texts |

Mutations: `mutacoes/leva169.py` (4, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `c_tempo` escaped: equivalent mutation: the initial +0x2db0 (125) is only compared with 20 and 101 and the match screens rewrite it. `l_fim` escaped: equivalent mutation: 0x6d9e50, which calls the freeing, rewrites 0xa1599c (the events language) right after. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 170th batch — closing the engine (4): the match and player orderings and the event variant (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ordem_das_partidas_do_dia()` | 0x6b6510 | cdecl | match_eng.cpp: the day's match order (qsort): competition, record fields and the club name |
| `ordem_de_acao()` | 0x6bd840 | cdecl | match_eng.cpp: the players' order on the engine's turn (qsort): the marked ones first, then by strength |
| `variante_do_evento()` | 0x6d5d80 | stdcall | match_events.cpp: swaps the commentary event for a variant, by a draw and by the play type |

Mutations: `mutacoes/leva170.py` (5, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 4 caught. `o_campos` escaped: in the English league the day matches tied up to +0x36 never disagree in sign between +0x38 and +0x34 (missing data; goes to the broad career). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 171st batch — closing the engine (5): the combined commentary, the context commentary and the match player (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `combinar_com_o_anterior()` | 0x6d60b0 | thiscall | match_events.cpp: combines the new commentary event with the last queued one (returns the combined event and copies the previous player, side and value) |
| `comentario_de_contexto()` | 0x6d4b70 | thiscall | match_eng.cpp: decides whether a context commentary (league race, club news) may play now |
| `construir_jogador_da_partida()` | 0x6eada0 | thiscall | match_pl.cpp: the match player constructor (clears the fields, with initial values for some) |

Mutations: `mutacoes/leva171.py` (7, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 7 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 172nd batch — closing the engine (6): reading the commentary, the player name and the perceived ability (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `proximo_evento()` | 0x6d78b0 | thiscall | match_events.cpp: the next commentary event to show, part by part of the text (split by |), advancing the queue reading |
| `nome_do_jogador_na_narracao()` | 0x6db2d0 | cdecl | match_events.cpp: the player name in the commentary: the short form, or the long one when another player of either club has the same short name |
| `habilidade_vista()` | 0x6e86d0 | cdecl | match_man.cpp: a player's current or potential ability as an observer sees it (reputation, noise and the observer's knowledge) |

Mutations: `mutacoes/leva172.py` (6, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught. `p_parte` escaped: 0x6d78b0 only runs with the commentary on screen, which the automatic career does not show (goes to a match watched by Lucas). `p_anel` escaped: 0x6d78b0 only runs with the commentary on screen, which the automatic career does not show. `n_longa` escaped: 0x6db2d0 only runs with the commentary on screen, which the automatic career does not show. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 173rd batch — closing the engine (7): the squad rule and the player scores (official on 2026-10-18)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `cumprir_regra_do_elenco()` | 0x6e8910 | cdecl | match_man.cpp: applies the squad rule (foreigners, non-EU or age): swaps restricted players for the candidates with the smallest score loss |
| `nota_fora_da_partida()` | 0x6e6340 | cdecl | match_man.cpp: the player's score for a position outside a match (transfers and scouting): position, form, perceived ability, contract, age, scout knowledge |
| `nota_para_a_vaga()` | 0x6e2f20 | cdecl | match_man.cpp: the player's score for a lineup slot: like the off-match score, plus the second position, in-match stamina, tactics, veterans and holidays |

Mutations: `mutacoes/leva173.py` (12, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `r_banco` escaped: the bench phase of the squad rule (0x6e8910) had no restricted bench players in the English league (goes to the broad career). `r_idade` escaped: the age rule (type 3) never had a player exactly at the limit. `n_veterano` escaped: the contract adjustment (flag 0x2000000) never met a veteran (over 34) with no games. `v_ferias` escaped: the holiday bonus of 0x6e2f20 only runs with a human manager's holiday list. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 174th batch — person in the manager list, the contract record and the daily wear (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `pessoa_na_lista()` | 0x69fc60 | thiscall | manager_manager.cpp: is the person in the list (iterator 0x68d1b0, restored at the end)? |
| `ficha_de_contrato()` | 0x4cf6d0 | thiscall | contract_manager.cpp: the person's contract record (0x50 bytes) |
| `desgaste_diario()` | 0x62c2f0 | thiscall | injury.cpp: the day's wear on word +0xc (age, draws of 25 and 13, goalkeeper cap) |

Mutations: `mutacoes/leva174.py` (7, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 7 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 175th batch — the tactics table formation, the player's competition weight and the news field (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `registro_da_formacao()` | 0x896570 | thiscall | tactics.cpp: the formation record (0x18e3 bytes) in the two tables |
| `peso_da_competicao()` | 0x7c6930 | stdcall | player_stats.cpp: the competition weight by the club's two competitions (500/1000/300/100) |
| `gravar_dado_da_noticia()` | 0x785790 | thiscall | news.cpp: stores news field 0..49 |

Mutations: `mutacoes/leva175.py` (6, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 5 caught. `f_limite` escaped: the limit only changes for formations beyond the default ones asked with the human flag (a human manager's own formations), which the automatic career does not use. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 176th batch — injury recovery, the person in the transfer list and the discipline penalty (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `recuperar_lesao()` | 0x62bd10 | thiscall | injury.cpp: the recovery day (below −1000 in x87 with −0.0035 and 2.5; otherwise the pace or a draw) |
| `pessoa_na_lista_de_transferencias()` | 0x8c1870 | thiscall | transfer_manager.cpp: is the person in list +0x92c? |
| `punicao_da_tabela()` | 0x55cae0 | thiscall | discipline.cpp: the penalty at the table row and column, if the type matches |

Mutations: `mutacoes/leva176.py` (6, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 5 caught. `t_item` escaped: the transfer manager list +0x92c never holds the person looked up in the automatic career (the routine always returns 0); goes to the broad career. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 177th batch — the contract bit, the competition flag and the club record in the league (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `bits_do_registro()` | 0x7a7eb0 | stdcall | player_regen.cpp: bits 4..5 of byte +0xb of the person's 79-byte record |
| `competicao_marcada()` | 0x684790 | thiscall | league.cpp: does the competition have +0xbe and flag 2 of +0xd9? |
| `registro_da_liga()` | 0x6826d0 | thiscall | league.cpp: the record (59 bytes) of table +0xb1 |

Mutations: `mutacoes/leva177.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `m_marca` escaped: in the English leagues the competitions with +0xbe set never have bits 1 and 2 of +0xd9 different (missing data; goes to the broad career). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 178th batch — the game average, the contract value and the national team level (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `media_do_jogo()` | 0x5d5a60 | thiscall | game.cpp: the f64 average +0x20/+0x28 or +0x10/+0x18 (zero divisor: 6.8) |
| `valor_do_contrato()` | 0x4d0c20 | thiscall | contract_manager.cpp: dword +0xc of the person's contract record |
| `nivel_para_a_selecao()` | 0x76d810 | stdcall | national_teams.cpp: the player's level 1..3 for the national team by reputation and position ratings |

Mutations: `mutacoes/leva178.py` (5, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 4 caught. `m_padrao` escaped: the average divisor (+0x18/+0x28) is never zero when the career asks for the average: the default 6.8 never shows. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 179th batch — the contract offers and the closed transfer offer (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `oferta_de_contrato()` | 0x4cc9e0 | thiscall | contract_manager.cpp: the person's contract offer for the club, in the contract manager's list |
| `oferta_acertada()` | 0x4cca30 | thiscall | contract_manager.cpp: the person's agreed contract offer (state 2) |
| `oferta_fechada()` | 0x8d0d90 | stdcall | transfer_offer.cpp: whether the transfer offer is closed (by its state and, in state 0xf, by its phase) |

Mutations: `mutacoes/leva179.py` (4, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `f_estado` escaped: in the career the transfer offers asked about are never in state 4. `f_fase` escaped: no offer in state 0xf is in phase 2 when it is asked about. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 180th batch — the match screens (1): the control label, closing the screen and the waiting screen (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `rotulo_do_controle()` | 0x725a70 | cdecl | match_screens.cpp: the match control button label (waiting, kick off, restart, pause or resume, continue) and whether it is disabled |
| `fechar_tela_da_partida()` | 0x71b120 | cdecl | match_screens.cpp: closes the match screen: destroys and frees screen manager parameters 0x2d and 0x1d |
| `abrir_tela_de_espera()` | 0x727f00 | cdecl | match_screens.cpp: opens the human manager's waiting screen in the screen manager |

Mutations: `mutacoes/leva180.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 181st batch — the player's media stories and the cup match importance (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `noticia_com_resposta_2()` | 0x735780 | thiscall | media.cpp: whether the player has a story with response 2 in the media log |
| `tem_noticia()` | 0x735840 | thiscall | media.cpp: whether the player has any story in the media log |
| `importancia_da_partida()` | 0x5221f0 | thiscall | Cup.cpp: the cup match importance by round and the first-leg score difference |

Mutations: `mutacoes/leva181.py` (4, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `m_alguma` escaped: the media log stories asked about in the career never have response 0. `c_fase` escaped: equivalent mutation: at round 1, replacing it with 1 changes nothing (not a test gap). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 182nd batch — the match screens (2): the latest scores screen (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `abrir_ultimos_placares()` | 0x71b1b0 | cdecl | match_screens.cpp: opens the latest scores screen of a competition and writes its parameters |
| `montar_ultimos_placares()` | 0x71b300 | cdecl | match_screens.cpp: builds the latest scores screen: background, title, menu, panel and back and next buttons |
| `responder_ultimos_placares()` | 0x71b3d0 | cdecl | match_screens.cpp: the latest scores screen's response to a click (competition switch, redraw, menu) |

Mutations: `mutacoes/leva182.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 183rd batch — the match screens (3): the match details screen (official on 2026-10-19)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `abrir_detalhes_da_partida()` | 0x71b4a0 | cdecl | match_screens.cpp: opens the details screen of a match and writes its parameters |
| `responder_detalhes_da_partida()` | 0x71bdc0 | cdecl | match_screens.cpp: the match details screen's response to a click (tab switch, club tactics, tab redraw, menu) |
| `fechar_detalhes_da_partida()` | 0x71c170 | cdecl | match_screens.cpp: closes the details screen: destroys and frees the objects kept in the parameters |

Mutations: `mutacoes/leva183.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 184th batch — discipline: the new season, the day and the table slot (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `disciplina_temporada_nova()` | 0x556210 | thiscall | discipline.cpp: starts the new season of the competition's discipline rules (virtual method 0) |
| `disciplina_do_dia()` | 0x556240 | thiscall | discipline.cpp: the day of each of the 43 discipline rules (virtual method 1) |
| `disciplina_gravar_vaga()` | 0x55cb20 | thiscall | discipline.cpp: writes the type and value of one of the person's slots in the discipline table |

Mutations: `mutacoes/leva184.py` (4, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `d_sem_regra` escaped: the competition with discipline rules index 0 does not start a season in the career (league outside the loaded ones). `d_43` escaped: the 43rd discipline rule is empty or changes nothing in the loaded leagues. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 185th batch — the agreed-contract flag and the offer slot (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `marcar_contrato_acertado()` | 0x8c5410 | thiscall | transfer_manager.cpp: sets the person's agreed-contract flag (0x40) in the transfer manager |
| `desmarcar_contrato_acertado()` | 0x8c5430 | thiscall | transfer_manager.cpp: clears the person's agreed-contract flag |
| `vaga_da_oferta()` | 0x8c61e0 | thiscall | transfer_manager.cpp: the address of an offer slot (out of range, the table start) |

Mutations: `mutacoes/leva185.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 1 caught. `t_desmarca` escaped: when the agreed-contract flag is cleared, the person never also has flag 0x80. `t_vaga` escaped: the career never asks for the offer slot equal to the count (out of range). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 186th batch — the match screens (4): opening the match screen and the responses of the latest scores and team stats panels (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `abrir_tela_da_partida()` | 0x718aa0 | cdecl | match_screens.cpp: opens the live match screen, writes its parameters and marks the human as a spectator |
| `responder_ultimos_placares_da_partida()` | 0x725860 | cdecl | match_screens.cpp: the latest scores panel of the match screen: the page switch every 5 s and the clicks (competition, pages, pause) |
| `responder_estatisticas_do_time()` | 0x722dc0 | cdecl | match_screens.cpp: the team stats panel: the redraw with new data and the clicks that move the row up and down |

Mutations: `mutacoes/leva186.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 187th batch — the match screens (5): the latest scores panel, the next screen and the heading colour (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `painel_dos_ultimos_placares()` | 0x723d20 | cdecl | match_screens.cpp: draws the latest scores panel: title, badge, the day's groups and matches, table and buttons |
| `sem_tela_seguinte()` | 0x7febb0 | thiscall | scrman.cpp: the screen manager has no next screen to go forward to |
| `cor_do_titulo()` | 0x839590 | cdecl | Setup.cpp: the heading text colour (two colours by the flag) |

Mutations: `mutacoes/leva187.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 188th batch — the shortlist manager, the training entry and the training load (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `lista_de_observados()` | 0x5d5ab0 | thiscall | game.cpp: the shortlist manager kept in the transfer manager |
| `entrada_do_treino()` | 0x5d5ac0 | thiscall | game.cpp: the player's 6-byte entry in the training table |
| `gravar_carga_de_treino()` | 0x62c660 | thiscall | injury.cpp: stores the player's training load in the fitness record |

Mutations: `mutacoes/leva188.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 189th batch — the friendly competition, the international date and the manager record (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `e_competicao_do_amistoso()` | 0x5c2990 | thiscall | friendly.cpp: tells whether the competition is the friendlies one |
| `preencher_data_internacional()` | 0x5cb9b0 | thiscall | friendly.cpp: fills an international date: day, month, span, year, continent and competition |
| `limpar_registro_de_tecnico()` | 0x86fc20 | thiscall | staff_records.cpp: clears the manager record and sets the owner, league and club ids to −1 |

Mutations: `mutacoes/leva189.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `r_ids` escaped: the last spell id (+0x72) of a cleared manager record is never read in the career before being rewritten. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 190th batch — the loan contract, the shortlist bid markers and the stand-in manager (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `tirar_contrato_de_emprestimo()` | 0x4d0bd0 | thiscall | contract_manager.cpp: frees the person's loan contract slot, if any |
| `desmarcar_lances_pelo_jogador()` | 0x84aeb0 | thiscall | shortlist_manager.cpp: clears the bid marker of the club shortlist slots that target the player |
| `tecnico_substituto()` | 0x600e90 | cdecl | human_manager.cpp: the club's assistant or, without one, the highest-reputation staff member |

Mutations: `mutacoes/leva190.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `s_empate` escaped: the stand-in manager is only asked for the human club, and its staff never tie on reputation. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 191st batch — the agreed contract club, the national team appearance and the held ban (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_do_contrato_acertado()` | 0x8c5450 | thiscall | transfer_manager.cpp: the club of the person's agreed contract (clears the flag when the contract is gone) |
| `registrar_jogo_pela_selecao()` | 0x8ce920 | thiscall | transfer_manager.cpp: records the player's national team match in the recent-matches bits |
| `punicao_guardada()` | 0x55cb60 | thiscall | discipline.cpp: the player's held ban in a slot, or none |

Mutations: `mutacoes/leva191.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 1 caught. `c_clube` escaped: in the career the agreed contract club is never asked for with the flag set and the contract found. `p_vaga` escaped: no player has two held bans in different slots when the search runs. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 192nd batch — the retirement entry, the friendly competitions and the news slot (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `entrada_da_aposentadoria()` | 0x7a7ed0 | thiscall | player_regen.cpp: the person's entry in the retirement list (by injury or already announced) |
| `registrar_competicoes_do_amistoso()` | 0x5ca030 | thiscall | friendly.cpp: stores the friendly and tournament objects in the competition table |
| `vaga_da_noticia()` | 0x785830 | thiscall | news.cpp: the value of one of the 50 news item slots |

Mutations: `mutacoes/leva192.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `a_torneio` escaped: friendly tournament slot 0 is never taken in the career. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 193rd batch — the contract clauses: next sale percentage, appearance bonus and national team bonus (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `tirar_percentual_da_proxima_venda()` | 0x8cb4f0 | thiscall | transfer_manager.cpp: removes the player's next sale percentage (flag and list record) |
| `tirar_bonus_por_jogo()` | 0x8cb590 | thiscall | transfer_manager.cpp: removes the first-team appearance bonus (flag and list record) |
| `tirar_bonus_pela_selecao()` | 0x8cb630 | thiscall | transfer_manager.cpp: removes the international appearance bonus (flag and list record) |

Mutations: `mutacoes/leva193.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught. `l_venda` escaped: no contract clause (next sale percentage, appearance bonus, international bonus) is removed in the career up to 2002-06. `l_jogo` escaped: no contract clause (next sale percentage, appearance bonus, international bonus) is removed in the career up to 2002-06. `l_conta` escaped: no contract clause (next sale percentage, appearance bonus, international bonus) is removed in the career up to 2002-06. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 194th batch — the player generator reserve lists and the league prize fund (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `tirar_reserva_de_tecnico()` | 0x7a82b0 | thiscall | player_regen.cpp: removes a non-player record from the spare list by id |
| `tirar_jogador_do_banco()` | 0x7a8470 | thiscall | player_regen.cpp: removes the player data record from the free player pool |
| `premio_da_liga()` | 0x682f70 | thiscall | league.cpp: the league total prize fund (one more share per place above the last) |

Mutations: `mutacoes/leva194.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `r_marca` escaped: field +8 of the slot freed at the end of the reserve list is never read before being rewritten. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 195th batch — the player's last rating and the player search slot and list type (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ultima_nota_do_jogador()` | 0x7c1a60 | thiscall | player_stats.cpp: the player's last match rating (6.5 with none) |
| `vaga_da_busca()` | 0x7ae640 | thiscall | player_search.cpp: stores the player search slot (0 to 9) |
| `tipo_da_lista_da_busca()` | 0x7ae6e0 | thiscall | player_search.cpp: stores the player search list type (1 to 4) |

Mutations: `mutacoes/leva195.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 196th batch — the league head-to-head and the terms offered in the transfer (official on 2026-10-20)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ordem_do_confronto()` | 0x6834f0 | cdecl | league.cpp: the head-to-head mini table order: points and the league tie-breakers |
| `atualizar_confronto_direto()` | 0x6869a0 | thiscall | league.cpp: adds one match to the head-to-head record (played, result, goals and points) |
| `marcar_termos_oferecidos()` | 0x8c1f80 | thiscall | transfer_manager.cpp: marks that the transfer offer had contract terms offered to the player |

Mutations: `mutacoes/leva196.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 1 caught. `c_saldo` escaped: no career league breaks ties by head-to-head goal difference (the mini table does not decide). `c_empate` escaped: head-to-head draw points decide no standings in the career. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 197th batch — the human manager record (holiday stamp and saved line-up) and the search manager (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `hora_das_ferias()` | 0x5fdc30 | thiscall | human_manager.cpp: the clock stamp stored when the human manager went on holiday |
| `escalacao_guardada()` | 0x5fed50 | thiscall | human_manager.cpp: the address of the human manager's saved line-up (16 pairs) |
| `tecnico_da_busca()` | 0x7ae570 | thiscall | player_search.cpp: stores the human manager id in the player search |

Mutations: `mutacoes/leva197.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 198th batch — the match screens (6): the day fixtures screen and the default captain (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `abrir_jogos_do_dia()` | 0x717cc0 | cdecl | match_screens.cpp: opens the day fixtures screen in a slot with the fixture group (error box without a group) |
| `responder_jogos_do_dia()` | 0x718930 | cdecl | match_screens.cpp: the day fixtures screen input: another fixture group, the competition progress or the menu bar |
| `vaga_do_mais_bem_avaliado()` | 0x727e20 | cdecl | match_screens.cpp: the first eleven slot with the highest staff rating (the default captain), -1 for none |

Mutations: `mutacoes/leva198.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 199th batch — the match statistics: the player league stat, the fixture entry and the record bucket (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `estatistica_da_liga_do_jogador()` | 0x7c47b0 | thiscall | player_stats.cpp: the player's league appearances (field 1) or league goals (field 2) at the club, as a float |
| `entrada_do_jogo_bate()` | 0x729500 | thiscall | match_screens.cpp: reports whether a match statistics entry holds the two given clubs in either order and the given date |
| `balde_do_registro()` | 0x729bd0 | stdcall | match_screens.cpp: the bucket (0 to 3) where the match statistics record is stored: competition type, national team and fixture flags |

Mutations: `mutacoes/leva199.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 1 caught. `e_data` escaped: in the career the fixture entry second date never decides (when the first matches, the second matches too). `b_tipo` escaped: the record bucket branch without the +0x40 flag does not run in the automatic career. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 200th batch — the match screens (9): the team block and player row copies and the live score (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `atribuir_bloco_do_time()` | 0x71dd70 | thiscall | match_screens.cpp: copies the match team block (0x625 bytes) field by field, with its 20 player rows |
| `atribuir_linha_do_jogador()` | 0x71df00 | thiscall | match_screens.cpp: copies one player row of the team block (0x49 bytes), the date through Date::operator= |
| `contar_placar_ao_vivo()` | 0x728e90 | cdecl | match_screens.cpp: counts the goals and shoot-out goals of a match up to the current minute of the live match |

Mutations: `mutacoes/leva200.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 201st batch — the match screens (7): the match Continue, opening team selection and the team selection response (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `responder_continuar_da_partida()` | 0x725ca0 | cdecl | match_screens.cpp: the match Continue: the pending team selection command (online, the club's free slot or waiting for the others), the network matches and team selection |
| `abrir_escalacao_da_partida()` | 0x725f70 | cdecl | match_screens.cpp: opens the Team Selection screen for the human's next match and stores the club, the match and the home flag |
| `responder_escalacao_da_partida()` | 0x727c30 | cdecl | match_screens.cpp: the Team Selection buttons: appoints the captain (the best starter) when missing and stores the pending command; the rest go to the common handler |

Mutations: `mutacoes/leva201.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 202nd batch — the match screens (8): the team stats, player ratings and match stats panels (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `painel_das_estatisticas_do_time()` | 0x7217a0 | cdecl | match_screens.cpp: draws one team's stats table (page header, arrows, column titles) or the no-stats message |
| `painel_das_notas_dos_jogadores()` | 0x722ed0 | cdecl | match_screens.cpp: draws both teams' player ratings: shirt number, name (button, with the captain) and rating |
| `painel_das_estatisticas_da_partida()` | 0x71e040 | cdecl | match_screens.cpp: draws the match stats: 13 labelled rows (shots, corners, fouls, passes, tackles, headers, cards) |

Mutations: `mutacoes/leva202.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 203rd batch — the match screens (11): the match stats, action zones and player ratings values (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `valores_das_estatisticas_da_partida()` | 0x71e750 | cdecl | match_screens.cpp: sends the match stats cells the values that changed since the match state copy (param 0x1d) and updates the copy |
| `valores_das_zonas_de_acao()` | 0x720320 | cdecl | match_screens.cpp: updates the action zones: markers, possession, ball in play time, the three zone bars and the possession bar in the club colours; updates the state copy |
| `valores_das_notas_dos_jogadores()` | 0x723350 | cdecl | match_screens.cpp: updates the player ratings rows: cards, injury/dismissal/substitution minute, rating, goals and name colour; updates the state copy |

Mutations: `mutacoes/leva203.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 204th batch — the match screens (10): the action zones and match report panels and the day fixtures screen (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `painel_das_zonas_de_acao()` | 0x71f500 | cdecl | match_screens.cpp: draws the action zones: the period selector, the pitch split into three zones with both clubs' colour bars, or the no-zones message |
| `painel_do_relatorio_da_partida()` | 0x721000 | cdecl | match_screens.cpp: draws the match report: the minutes and the event texts (building the report outside the match), or the no-report message |
| `montar_jogos_do_dia()` | 0x717db0 | cdecl | match_screens.cpp: builds the day fixtures screen: title, the day's competition tree, the rounds and matches table and the scroll to the human's match |

Mutations: `mutacoes/leva204.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 205th batch — the match screens (12): the team stats rows, the latest scores refresh and the wait for the other players (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `preencher_estatisticas_do_time()` | 0x721cd0 | cdecl | match_screens.cpp: writes the 20 player rows of the team stats table (22 columns from the param 0x1c table), building the cells or updating what changed |
| `atualizar_ultimos_placares()` | 0x724520 | cdecl | match_screens.cpp: rewrites the latest scores rows in pages: stage, fixture, score, period, event scorers, aggregate and the page footer |
| `esperar_os_outros_jogadores()` | 0x727fb0 | cdecl | match_screens.cpp: in a network game, waits for the other humans of the same fixture or closes the screens, shows the notice and opens the match screen |

Mutations: `mutacoes/leva205.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 206th batch — the match statistics: reading and writing the record in the temporary file and recording the match (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ler_registro_da_partida()` | 0x729600 | thiscall | match_stats.cpp: reads one match statistics record (0xd7a bytes) from match_stats_matches.tmp by index |
| `registrar_partida()` | 0x7293f0 | thiscall | match_stats.cpp: fills the record of a finished match and writes it, except for excluded matches |
| `gravar_registro_da_partida()` | 0x729820 | thiscall | match_stats.cpp: writes the events and the record into the fixture's slot (or the bucket cursor's slot, which wraps around) and updates the index |

Mutations: `mutacoes/leva206.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 1 caught. `l_tamanho` escaped: reading the match record (0x729600) only runs when the report of a played match is opened; the automatic career opens no screens. `r_excluido` escaped: in the career the fixture is never the [0x9cf684] competition one, so the "or" is decided by its first half and the record +0x4c flag is never read. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 207th batch — the match screens (15): the team selection check and the match statistics index in the saved game (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `desenhar_conferencia_da_escalacao()` | 0x7260f0 | cdecl | match_screens.cpp: draws the team selection check: warns (with linked names) about players on international duty, on loan, unavailable, ineligible, banned, injured or without a work permit, the bench limit or missing starters; otherwise asks for confirmation (captain, set piece takers, substitutes) with "No" and "Yes" |
| `carregar_estatisticas_da_partida()` | 0x72aa80 | thiscall | match_stats.cpp: reads the record index, the four bucket cursors and the counts from match_stats.dat in the saved game |
| `salvar_estatisticas_da_partida()` | 0x72adf0 | thiscall | match_stats.cpp: writes the record index, the four bucket cursors and the counts to match_stats.dat in the saved game |

Mutations: `mutacoes/leva207.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 208th batch — the interface areas (1): laying out the grid, initialising and drawing an area (with the 30-row clamp: the 1080 report "Area.cpp 507") (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `organizar_area()` | 0x403350 | thiscall | Area.cpp: splits the area width and height by the column and row weights (up to 30 each; more raises the Area.cpp 497/507 box) and sets up the scrollbar |
| `iniciar_area()` | 0x402ac0 | thiscall | Area.cpp: initialises the area: rectangle clamped to the screen, fields, cleared scrollbar and the column and row weights; the visible rows clamped at 30 (the grid size), the rest goes to the scroll bar |
| `desenhar_area()` | 0x402e70 | thiscall | Area.cpp: draws the area: the panel, the grid lines or each visible widget in its cell, and the scrollbar |

Mutations: `mutacoes/leva208.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Match engine, 209th batch — the match screens (14): building and handling the live match screen and building the match details screen; the match screens (13): the possession bar, the match overview panel and the live league table (official on 2026-10-21)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `montar_tela_da_partida()` | 0x718e90 | cdecl | match_screens.cpp: builds the live match screen: team headers and score, commentary, clock, control, tactics buttons, commentary speeds, tabs, the selected tab panel, the stadium and the commentary panel |
| `responder_tela_da_partida()` | 0x71a240 | cdecl | match_screens.cpp: handles the live match screen: commentary volume and speed, the new event (clock, commentary, modes), watching or not, a team's tactics, switching tabs and each tab's input |
| `montar_detalhes_da_partida()` | 0x71b630 | cdecl | match_screens.cpp: builds the details screen of a stored match: loads the match record and snapshot from the match stats, the team headers, the 7 tabs and the selected tab panel |
| `atualizar_barra_da_posse()` | 0x71d9e0 | cdecl | match_screens.cpp: redraws the overview possession bar when the share changes (percentages and the colour of the club with more of the ball) and stores the match state in the param 0x1d copy |
| `painel_da_visao_geral_da_partida()` | 0x71c270 | cdecl | match_screens.cpp: draws the match overview: period, possession bar and commentary, date, score with half time/extra time/penalties, competition, scorers and cards, shoot-out, referee, attendance, weather and abandoned match |
| `tabela_da_liga_ao_vivo()` | 0x7280e0 | cdecl | match_screens.cpp: draws the live league table (league, top, bottom) on a copy of the league with the day scores applied (or the draw date notice): title, headers, separators, position, up/down arrow, club and columns |

Mutations: `mutacoes/leva209.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 210th batch — the finable media story, the drawn injury type and the news category; the repeated national squad entries, the selected player slot and the former international pundit; the round rules on the cup match, the match tie and the tour nation (official on 2026-10-22)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `historia_punivel_do_jogador()` | 0x7357c0 | thiscall | media.cpp: the kind of the media story about the player when it is a misbehaviour kind, else 0 |
| `escolher_tipo_de_lesao()` | 0x62d140 | thiscall | injury.cpp: draws the injury type for the body part and severity, keeping the current injury or preferring the recurring one |
| `categoria_da_noticia()` | 0x785670 | thiscall | news.cpp: the news category by the thousands range of its type |
| `apagar_repetidos_das_convocacoes()` | 0x76c060 | stdcall | national_teams.cpp: clears repeated squad entries from the senior and B national sides of a nation |
| `vaga_do_escalado()` | 0x8972c0 | thiscall | tactics.cpp: the club selection slot (0..19) holding the player, or 0xff |
| `escolher_comentarista()` | 0x72b810 | thiscall | media.cpp: draws a former international (37+, 50+ caps) as pundit, weighted by nation reputation and caps |
| `regras_da_rodada_no_jogo()` | 0x521d60 | thiscall | Cup.cpp: copies the round rules onto the match and, with a decision on a draw, marks who goes through by both sides' levels |
| `chave_do_jogo_na_copa()` | 0x519c90 | thiscall | Cup.cpp: the round tie that holds the match's home club (-1 if none) |
| `pais_da_excursao()` | 0x5c4680 | thiscall | friendly.cpp: draws the nation for a club tour, nearer the lower the reputation |

Mutations: `mutacoes/leva210.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `m_dobro` escaped: doubling the weight (or adding 1) only decides when candidates from the subject nation compete with others; in the career the other-nation ones hardly ever pass the cut (reputation above 6500 + draw and record ≥ 6000), and among same-nation ones 2a and a + 1 keep the same order. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 211th batch — closing the contract offers of a transfer offer, the free transfer queue and the player exchange negotiations; the player leaving the national side, the fixture time slice and the shortlist fee ceiling; the offers for a player, the club target reputation and the fixture in the news item (official on 2026-10-22)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `fechar_propostas_da_oferta()` | 0x8c6db0 | stdcall | transfer_manager.cpp: closes (state 4) the contract offers linked to the transfer offer: the target player and the exchange players |
| `enfileirar_transferencia_livre()` | 0x8c16b0 | thiscall | transfer_manager.cpp: queues the player for a free transfer (id, reason and compensation), refusing players out on loan |
| `negociacoes_de_troca_do_jogador()` | 0x8c5fc0 | thiscall | transfer_manager.cpp: counts the open negotiations of the player's club that include him in a part exchange and, if asked, lists the offer indices |
| `tirar_jogador_da_selecao()` | 0x775ba0 | thiscall | national_teams.cpp: takes the player out of his level pool in the nation and out of the squad, clearing his national team fields |
| `faixa_do_horario()` | 0x5af2f0 | thiscall | fix_man.cpp: the fixture time slice (1 weekend, 2 otherwise) by competition nation and weekday |
| `teto_da_observacao()` | 0x84b230 | stdcall | shortlist_manager.cpp: the shortlist slot fee ceiling: club reputation times a factor (or -1) |
| `ofertas_pelo_jogador()` | 0x8c5cf0 | thiscall | transfer_manager.cpp: counts the transfer offers for a player (skipping closed ones unless asked) and may list their indices |
| `reputacao_alvo_do_clube()` | 0x83bf40 | thiscall | shortlist_manager.cpp: the reputation a club aims for in signings, adjusted by manager and chairman, between the club floor and 10000 |
| `gravar_jogo_na_noticia()` | 0x4afeb0 | cdecl | comp_util.cpp: writes the fixture clubs, venue, competition, score and round into 18 consecutive news slots and advances the cursor |

Mutations: `mutacoes/leva211.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `s_faixa` escaped: in the career the player leaving the national team is never in his level band with the level mark below the record one, so the band size never shrinks (the comparator covers the whole nation record, where the size lives). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e

### Simulation, 212th batch — the match record, the order of the events and the language a player loses (official on 2026-10-22)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `preencher_registro_da_partida()` | 0x729ff0 | thiscall | match_screens.cpp: fills the match statistics record: clubs, dates, attendance, gate receipts, weather, sorted events, formation, captain and the 20 players of each side |
| `ordem_dos_eventos()` | 0x72b160 | cdecl | match_screens.cpp: orders two match events by type group, then by minute and arrival order (qsort comparator) |
| `tirar_idioma_do_jogador()` | 0x7c6bf0 | thiscall | player_stats.cpp: removes a language from the player's language table (binary search) and shifts the later entries down |

Mutations: `mutacoes/leva212.py` (3, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 2 caught. `i_ultima` escaped: in the career 0x7c6bf0 is only called by the regens (languages 1..64 of the freshly created player, who has none yet): the search never finds the entry, so the last slot is never cleared (the comparator covers the whole table). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (5afd987e1a1a3ef3). Rust ×2 and original careers to 2002-06-01: 2,361,839,396 draws, fingerprint b11bd5cb63f51d72, matches 2461da5b0d296dba, database 4d07a03c94c6fda5

### Simulation, 213th batch — the signing by the nation rules and the exchanges offering the player; the unread news, the slot written into the inbox news item and the human-managed club; the draw and match dates of the cup round and the club removed from the planned friendlies (official on 2026-10-22)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `pode_contratar_jogador()` | 0x8ced40 | thiscall | transfer_manager.cpp: whether the club may sign the player under the nation rules, with the refusal reason and the human club's second try on the foreign player limit |
| `trocas_oferecendo_o_jogador()` | 0x8c5e40 | thiscall | transfer_manager.cpp: how many of the club's open offers include the player in the exchange, listing their indices when asked |
| `noticias_nao_lidas()` | 0x786fe0 | thiscall | news.cpp: the unread news in a human manager's inbox, in total or for one category |
| `gravar_marca_da_noticia()` | 0x786d30 | thiscall | news.cpp: writes a value into a slot of a news item in the human manager's inbox and into its local copy |
| `clube_de_tecnico_humano()` | 0x7875e0 | stdcall | news.cpp: whether a human manager runs the club, ignoring holidays |
| `data_do_sorteio_da_rodada()` | 0x521e60 | cdecl | Cup.cpp: stores the cup round draw date: the day number and the year offset from the base year |
| `data_do_jogo_da_rodada()` | 0x521eb0 | cdecl | Cup.cpp: stores the cup round match date, its match time and the venue flags |
| `tirar_clube_dos_amistosos()` | 0x5c64e0 | thiscall | friendly.cpp: removes the club from the planned AI friendlies and from the friendly lists |

Mutations: `mutacoes/leva213.py` (8, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 7 caught. `tr_recusa` escaped: in the career the transfer rules never refuse a signing asked without the second chance (a5 = 0), so the "refused, no second question" path does not run (not even in comparison mode). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (5afd987e1a1a3ef3). Rust ×2 and original careers to 2002-06-01: 2,361,839,396 draws, fingerprint b11bd5cb63f51d72, matches 2461da5b0d296dba, database 4d07a03c94c6fda5

### Simulation, 214th batch — the league table row order, the league round-robin grid and the last-day goal margin; the match stored in the news item, the cup tie result for the news and the cup round match day; the formation position lookup, the club score for a youth player and the match watched by the human (official on 2026-10-22)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ordem_das_linhas_da_tabela()` | 0x683210 | cdecl | league.cpp: the table qsort comparison: points, the league tiebreak rules and the club id |
| `montar_grade_de_turnos()` | 0x6844b0 | stdcall | league.cpp: builds the grid where every club meets each other once, with balanced home and away games |
| `margem_do_ultimo_dia()` | 0x68a4d0 | thiscall | league.cpp: on the last matchday, the goal margin a club needs for the title, the top places or survival, and the deciding-match flag |
| `partida_das_vagas_da_noticia()` | 0x4b0010 | cdecl | comp_util.cpp: rebuilds the fixture from eighteen consecutive news slots and resolves its clubs, stadium and competition |
| `resultado_da_eliminatoria_na_noticia()` | 0x4b0890 | cdecl | comp_util.cpp: the category of the cup tie result for the club, from the final margin, the side and the reputation gap |
| `dia_da_rodada_da_copa()` | 0x521f10 | cdecl | Cup.cpp: stores one of the eight cup round match days: the venue flags, the weekday, the time slot and the code |
| `mapa_de_casas_da_formacao()` | 0x5b7160 | thiscall | formation.cpp: expands each formation slot's positions into the pitch cell tables and their mirrored copy |
| `nota_do_clube_para_o_jovem()` | 0x7ad220 | stdcall | player_regen.cpp: the random score of how well the club suits the youth player: reputation against ability, youth facilities, human control and nation |
| `partida_assistida_pelo_humano()` | 0x5fe290 | thiscall | human_manager.cpp: whether the fixture is the one the human manager chose to watch |

Mutations: `mutacoes/leva214.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `lt_ultimo_dia` escaped: the margin is only asked by 0x68a3e0 (match importance), which stores the byte in a field; in the real game comparing (to 2002-01-01) it is not called, and in the career to 2002-06-01 0xfe instead of 0xff (no margin) changes no decision. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (5afd987e1a1a3ef3). Rust ×2 and original careers to 2002-06-01: 2,361,839,396 draws, fingerprint b11bd5cb63f51d72, matches 2461da5b0d296dba, database 4d07a03c94c6fda5

### Match engine, 215th batch — the end of the match screens: the match the human leaves and the fixture of a statistics record (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `largar_partida_assistida()` | 0x6b6970 | cdecl | match_day.cpp: the current human leaves the match he was watching; with no other human watching, the match runs to the end, is recorded and leaves the queue |
| `ficha_do_registro()` | 0x729e30 | thiscall | match_stats.cpp: finds in the fixture list the match of a statistics record (date, competition and club) and copies the fixture |

Mutations: `mutacoes/leva215.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 216th batch — whether the league has relegation, the league schedule round date and the league match importance; the club records copies of the record holder, the match record and the run record; the fixture watched by any human, the nickname given by the human and the pending retirement copy (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `liga_tem_rebaixamento()` | 0x6847b0 | thiscall | league.cpp: whether the league has relegation places at the bottom of the table |
| `data_da_rodada_da_liga()` | 0x68a160 | cdecl | league.cpp: stores in a league schedule round record the date with a relative year, the code and a value |
| `importancia_do_jogo_da_liga()` | 0x68a3e0 | thiscall | league.cpp: the league match importance for the club: the league base, +1 early in the season or with the final-day margin |
| `copiar_recordista()` | 0x448490 | thiscall | club_records.cpp: copies the record holder: four identity fields and the date |
| `copiar_registro_de_partida()` | 0x448110 | thiscall | club_records.cpp: copies the match record: the date, the opponent, the score and the fixture fields |
| `copiar_registro_de_sequencia()` | 0x448170 | thiscall | club_records.cpp: copies the run record: the game count and the start and end dates |
| `humano_assiste_a_partida()` | 0x5fe420 | thiscall | human_manager.cpp: whether any of the active human managers chose to watch the fixture |
| `humano_deu_apelido()` | 0x600740 | thiscall | human_manager.cpp: whether the human manager (or the current one) gave the person a nickname, by binary search in his list |
| `copiar_aposentadoria_pendente()` | 0x7a8780 | thiscall | player_regen.cpp: copies the pending retirement record: the person, the club, the two dates, the reason and the announced flag |

Mutations: `mutacoes/leva216.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `ha_motivo` escaped: the reason (+0x18) of the copied pending retirement is not read in a deciding way up to 2002-06-01, and in the real game comparing (to 2002-01-01) the copy catches no retirement whose reason changes (retirements are settled at season end). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Match engine, 217th batch — the end of the match statistics: the constructor (with the CD check) and the destructor (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `montar_estatisticas_da_partida()` | 0x729000 | thiscall | match_stats.cpp: checks for the CM01/02 CD and builds the statistics: loads from the save or creates the index, the buckets and the zeroed temporary file |
| `desmontar_estatisticas_da_partida()` | 0x7293c0 | thiscall | match_stats.cpp: frees the record index and clears its pointer and count |

Mutations: `mutacoes/leva217.py` (0, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 0 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 218th batch — the saved game container: the queries (name pattern, id and item size); the saved game container: packing and unpacking the folder files and clearing the temporary folder; the saved game container: reading, writing and closing an item (the type 4 RLE) (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `casar_nome_com_padrao()` | 0x933a00 | cdecl | zipdir.cpp: matches a container item name against a pattern with `*` (`*.` goes to the last dot), case-insensitively; the "{ Empty }" slot never matches |
| `id_do_conteiner()` | 0x9331c0 | cdecl | zipdir.cpp: reads the container header and returns dword +4 (0x16 .sav, 0x17 .qst; 0x14 on failure) |
| `tamanho_dos_itens_do_padrao()` | 0x933860 | cdecl | zipdir.cpp: adds up the uncompressed sizes of the items matching the pattern (signed int: negative above 2 GB, as in the original) |
| `empacotar_no_conteiner()` | 0x933610 | cdecl | zipdir.cpp: each folder file matching the pattern (FindFirstFile) becomes a container item: zopen for writing, fread/zwrite in 0x30d40 blocks, zclose |
| `desempacotar_do_conteiner()` | 0x933320 | cdecl | zipdir.cpp: each container item whose name matches the pattern becomes a folder file (zopen for reading, zread in 0x30d40 blocks, fwrite) |
| `limpar_pasta_temporaria()` | 0x90cd50 | cdecl | utils.cpp: builds the CM3_TEMP folder path in [0xde279c] and deletes its files matching the pattern, in groups of 100 |
| `ler_do_conteiner()` | 0x932c30 | cdecl | zipdir.cpp: reads from the item like fread, clipped at the item end; on type 4 decodes the RLE (a run may span calls) |
| `gravar_no_conteiner()` | 0x932e10 | cdecl | zipdir.cpp: writes to the item like fwrite and adds to the size in the index; on type 4 encodes the RLE (runs of up to 0x7f kept across calls) |
| `fechar_conteiner()` | 0x932b00 | cdecl | zipdir.cpp: when writing: flushes the buffer, closes the pending RLE run and rewrites header and index at the start; closes the file and frees the item |

Mutations: `mutacoes/leva218.py` (9, by the save proof): 6 caught. `sv_padrao_letra` escaped: the name pattern match (0x933a00) only runs when loading (unpacking "*.lng" and "*.tmp"); the proof writes the .qst and the load step needs a save with a manager on holiday, which does not exist yet. `sv_padrao_ponto` escaped: the name pattern match (0x933a00) only runs when loading (unpacking "*.lng" and "*.tmp"); the proof writes the .qst and the load step needs a save with a manager on holiday, which does not exist yet. `sv_padrao_fim` escaped: the name pattern match (0x933a00) only runs when loading (unpacking "*.lng" and "*.tmp"); the proof writes the .qst and the load step needs a save with a manager on holiday, which does not exist yet. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL fora dos campos ignorados. Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 219th batch — the saved game: the file box filters (saved game, quick start and version); the saved game: the FIFA transfer rules (save and load) and the save summary; the saved game: the database links (save and load) and freeing the game data (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `filtro_do_jogo_salvo()` | 0x838e10 | cdecl | Setup.cpp: file box filter for saved games: 1 if the container has id 0x16 (.sav), else 2 (ax only); a null path gives the error box |
| `filtro_do_inicio_rapido()` | 0x839290 | cdecl | Setup.cpp: file box filter for quick starts: 1 if the container has id 0x17 (.qst) and the saved version is the exe one, else 2 (ax only) |
| `versao_do_jogo_salvo_valida()` | 0x838eb0 | cdecl | Setup.cpp: opens the container general.dat, reads the version and the summary (0xf70 or 0xf74 bytes) and returns 1 if the summary version text is the exe one; on success it does not close the item (as the original) |
| `gravar_regras_fifa()` | 0x8395b0 | cdecl | Setup.cpp: writes the last saved game item, fifa_transfer_rules.zip: 2 bytes ([0xae233c] and the rule in force [0xa83608]); errors give the "Error" box |
| `carregar_regras_fifa()` | 0x839760 | cdecl | Setup.cpp: reads the 2 bytes of fifa_transfer_rules.zip; if the first is 1, the rule in force becomes 5 |
| `montar_resumo_do_save()` | 0x591f80 | cdecl | file_screens.cpp: builds the general.dat summary: the active nations (two groups), the human managers with club and nation, the game date, the version, the play time and time() |
| `gravar_banco_no_save()` | 0x524d30 | cdecl | Database.cpp: save_game_data step 1: save_database(save, 1, a4, type), the database tables into the container |
| `carregar_banco_do_save()` | 0x524d10 | cdecl | Database.cpp: load_game_data step 2: load_database(save, shared, 1), the database read from the container |
| `liberar_dados_do_jogo()` | 0x82ff40 | cdecl | Setup.cpp: releases the screens, destroys the subsystems held in globals (destructor + delete, free, virtual destructor) in the original order, deletes the *.tmp files, frees the database tables, resets the game date and clears three flags; returns 1 |

Mutations: `mutacoes/leva219.py` (4, by the save proof): 3 caught. `sv2_banco_a4` escaped: the 0x524d30 copy is switched off (DESLIGADAS: the save_database it calls writes 408 bytes of unset locals into index.dat and, called from the copy on the private stack, the garbage changes), so the mutation does not run. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL fora dos campos ignorados. Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 220th batch — the saved game: creating the game from the setup file (quick start and automatic career); the saved game: saving the whole game (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `criar_jogo_pela_configuracao()` | 0x827110 | cdecl | Setup.cpp: reads auto_setup.cfg/quick_start.cfg (database, stats, foreground and background leagues, language, engine, bids, holiday), creates the game (0x828110) and saves the .qst (0x82e430 with 0x17) and frees everything, or puts the human manager on holiday at a drawn club |
| `gravar_jogo()` | 0x82e430 | cdecl | Setup.cpp: saves the game: deletes the old file, the database (0x524d30), general.dat (16 fields and the summary), the subsystem saves in the original order with disk and memory checks between them, packs the *.tmp files, the FIFA rules; on a network game, writes to temp_save_file and renames |

Mutations: `mutacoes/leva220.py` (5, by the save proof): 3 caught. `sv3_criar_id` escaped: the 0x827110 copy is switched off (DESLIGADAS: running on the private stack it changes the residue create_game 0x828110 reads and club.dat comes out different), so the mutation does not run. `sv3_criar_frente` escaped: the 0x827110 copy is switched off (DESLIGADAS: running on the private stack it changes the residue create_game 0x828110 reads and club.dat comes out different), so the mutation does not run. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 221st batch — the saved game: loading the whole game (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `carregar_jogo()` | 0x82a9c0 | cdecl | Setup.cpp: loads the saved game: disk, memory, CD, the database (0x524d10), unpacks the *.tmp files, general.dat (version, summary, ids, dates, leagues, default nation, the runtime data of people, clubs and reputation, converting old saves), the subsystem constructors in the original order with disk and memory checks between them, the FIFA rules and the last save date |

Mutations: `mutacoes/leva221.py` (3, by the save proof): 3 caught.  Save proof (the .qst with and without the routines): EQUAL fora dos campos ignorados; carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 222nd batch — the reputation scope, the reputation seen by a viewer and whether a person is foreign; the person's age, whether the nation has free transfers and whether the person is an EU national; whether the person is transfer listed and the 1 to 20 player attributes (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `escopo_da_reputacao()` | 0x5408c0 | cdecl | Database.cpp: how far the viewer stands from the person (0 same club or league, 2 same nation, 3 club nation or high reputation, 4 world): picks the reputation |
| `reputacao_vista_por()` | 0x5407e0 | cdecl | Database.cpp: the person's reputation as seen by a viewer: current, home, world or a blend, by the scope; player or non-player data |
| `pessoa_e_estrangeira()` | 0x53d980 | cdecl | Database.cpp: whether the person is foreign in the nation: nationalities, groups (British Isles, Australia, [0x9cf360]) and the Portugal rule for contracts up to 1998 |
| `idade_da_pessoa()` | 0x53ab90 | cdecl | Database.cpp: the person's age in whole years on a date, from the birth date |
| `nacao_com_transferencia_livre()` | 0x544670 | cdecl | Database.cpp: whether the nation has free transfers: new FIFA rules, Brazil, or Europe except Poland, Turkey, Croatia and Russia |
| `pessoa_e_comunitaria()` | 0x53d740 | cdecl | Database.cpp: whether the person counts as an EU national for a club of the nation: the nationality EU mark, Switzerland and the Germany, [0x9cf360] and Denmark exceptions |
| `pessoa_listada_para_transferencia()` | 0x544520 | cdecl | Database.cpp: whether the person's link with the club (or the contract club) is transfer listed, by the club or at request |
| `atributo_calculado_do_jogador()` | 0x543540 | cdecl | Database.cpp: a player attribute (1 to 31) as 1 to 20: the record byte or, for relative ones, the rating of 2 × byte + current ability |
| `atributo_da_pessoa()` | 0x5431e0 | cdecl | Database.cpp: a player attribute of the person, from the person's player data (0x543540) |

Mutations: `mutacoes/leva222.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 9 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 223rd batch — the competition history order, the player attribute vector and whether the nation is British or Ireland; whether the club is Basque and whether the person is on the club transfer list or the loan list; whether the person is listed at request, the rating of a relative attribute and whether the club may have a reserve side (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ordem_do_historico_de_competicao()` | 0x540780 | cdecl | Database.cpp: qsort comparison of the competition history: rows without a competition last, by competition id and by season, newest first |
| `vetor_de_atributos_do_jogador()` | 0x53a610 | cdecl | Database.cpp: widens the player's positions, sides and attributes into a 63 cell vector (value and weight) and returns the group ends (9, 0xc, 0x20) |
| `nacao_britanica_ou_irlanda()` | 0x5446d0 | cdecl | Database.cpp: whether the nation is England, Scotland, Ireland, Wales or Northern Ireland |
| `clube_basco()` | 0x53d7f0 | cdecl | Database.cpp: whether the club is Spanish and one of the 23 Basque clubs named in the code |
| `pessoa_listada_pelo_clube()` | 0x5444a0 | cdecl | Database.cpp: whether the person's link with the club (or the contract club) is transfer listed by the club; job 0xf at a club without a human manager does not count |
| `pessoa_listada_para_emprestimo()` | 0x544590 | cdecl | Database.cpp: whether the person's link with the club (or the contract club) is loan listed |
| `pessoa_listada_a_pedido()` | 0x544600 | cdecl | Database.cpp: whether the person's link with the club (or the contract club) is transfer listed at request |
| `nota_relativa_do_atributo()` | 0x544380 | cdecl | Database.cpp: the 1 to 20 rating of 2 × the attribute byte + the player's current ability |
| `clube_pode_ter_reservas()` | 0x542740 | cdecl | Database.cpp: whether the club may have a reserve side: not national teams, A/B pairs, Spanish, German and Portuguese clubs and some second national sides |

Mutations: `mutacoes/leva223.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 9 caught.  Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 224th batch — the first and second name draw by nation and the formatted person name; the club's kit colours, the club's manager and the club's English name; the morale adjustment, the non-player attribute vector and the person's squad flags (official on 2026-10-23)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `primeiro_nome_da_nacao()` | 0x539c60 | cdecl | Database.cpp: draws a first name for the nation (20 draws by weight when it has over 100; otherwise borrows from the language group) |
| `segundo_nome_da_nacao()` | 0x53a1b0 | cdecl | Database.cpp: draws a second name for the nation, the same way |
| `formatar_nome_da_pessoa()` | 0x53dba0 | cdecl | Database.cpp: writes the person's name in one of 7 styles (first last, F.last, last, first, initials...), preferring the common name or nickname |
| `cores_do_uniforme_do_clube()` | 0x53b640 | cdecl | Database.cpp: the club's home, away or third kit colours as screen pixels, with defaults when missing |
| `tecnico_do_clube_ou_selecao()` | 0x53ca90 | cdecl | Database.cpp: the manager of the club, of its A/B partner or, for an under-21 side, the senior national team's |
| `nome_ingles_do_clube()` | 0x545880 | cdecl | Database.cpp: the club's name, or its English one from eng.lng when another language is loaded |
| `ajustar_moral_da_pessoa()` | 0x544710 | cdecl | Database.cpp: adds a delta to the player's morale, damped by a draw near the ends, clamped to 1..20; returns whether it changed |
| `vetor_de_atributos_do_nao_jogador()` | 0x53a8b0 | cdecl | Database.cpp: widens the non-player's coaching attributes (+0x0e..+0x22) into a 21-cell vector of 3 bytes, with the split at 13 |
| `marcar_elenco_da_pessoa()` | 0x540980 | cdecl | Database.cpp: sets the person's club or national squad bits, keeping the other pair |

Mutations: `mutacoes/leva224.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `dq8_ingles` escaped: the English club name only reads eng.lng when the game runs in another language; the career and the real game comparing run in English (language 0/1), where the routine returns the name itself. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 225th batch — the club's squad count and squad list and whether the club can be managed; the attribute vectors back into the non-player and the player and the rebalancing of the non-player's attributes; the colour family, whether two colours clash and the non-player's computed attribute (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `contar_elenco_do_clube()` | 0x53c590 | cdecl | Database.cpp: how many valid people are in the club's squad (with the A/B partner, the reserve squad flags and the loanee filter) |
| `reunir_elenco_do_clube()` | 0x541f90 | cdecl | Database.cpp: copies the club's squad (and the A/B partner's, up to 50 or 100) into a vector, padding with zeros |
| `clube_pode_ser_dirigido()` | 0x544820 | cdecl | Database.cpp: whether the club can be managed: senior national team of a nation with +0x88 > 150, or a club that is not a B team, with an active competition and a flagged nation |
| `vetor_para_o_nao_jogador()` | 0x53a980 | cdecl | Database.cpp: writes the low byte of the vector's 21 cells back to the non-player's coaching attributes (+0x0e..+0x22) |
| `vetor_para_o_jogador()` | 0x53aa10 | cdecl | Database.cpp: writes the low byte of the vector's cells 0..53 back to the player's attributes and cell 54 to staff +0x58 |
| `normalizar_atributos_do_nao_jogador()` | 0x539490 | cdecl | Database.cpp: rebalances the non-player's attributes in two passes: mean of the near-zero cells pulled to −5, positive excess trimmed, cap −80 for job 10 |
| `familia_da_cor()` | 0x540520 | cdecl | Database.cpp: the colour's family (1..10) from the index.cpp colour keys |
| `cores_se_confundem()` | 0x540490 | cdecl | Database.cpp: whether two colours clash: same family or the pairs 8-9, 8-7, 6-7 and family 6 against the colour keyed at [0x9cf1dc] |
| `atributo_calculado_do_nao_jogador()` | 0x543fd0 | cdecl | Database.cpp: the coaching attribute 1..12: four straight from the record, eight through the 1-to-20 formula with the non-player word +4 |

Mutations: `mutacoes/leva225.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `dq10_reunir` escaped: the 0x541f90 copy is switched off (DESLIGADAS: even the original through the passthrough hook changes the career, because the 0x540a50 it calls leaves its residue on the private stack), so the mutation does not run. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 226th batch — the adjective form per language, the person's coaching attribute and the rating with the reduced ability; the first name by weight, the competition's colours and whether the club has room in the job; adding a person to a club (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `forma_do_adjetivo()` | 0x545a60 | cdecl | Database.cpp: the adjective form (0..4) for index 1..35 in the inflecting languages 3..10 (jump tables); any other language or index: 0xff |
| `atributo_do_nao_jogador()` | 0x543f30 | cdecl | Database.cpp: the person's coaching attribute 1..12: 0x543fd0 with their non-player record (+0x69) |
| `nota_relativa_com_habilidade_reduzida()` | 0x5442f0 | cdecl | Database.cpp: the 1-to-20 rating of 2 × value + current ability × 0.1 (0x544380 with the ability scaled down) |
| `primeiro_nome_pelo_peso()` | 0x539b90 | cdecl | Database.cpp: a first name: nation with more than 15 names, the heaviest of 3 draws in its range; otherwise a draw over the whole table |
| `cores_da_competicao()` | 0x53b7c0 | cdecl | Database.cpp: the competition's colours (foreground and background) in pixels, from the +0x5d record or the competition itself |
| `clube_tem_vaga_para_cargo()` | 0x53b9a0 | cdecl | Database.cpp: whether the club has room for the person in job 1..15 (an empty slot or one already theirs; 12, 13 and 15 also in the squad) |
| `incluir_pessoa_no_clube()` | 0x53bd10 | cdecl | Database.cpp: puts the person in the job slot at the club (jobs 1..15; a player above 85 without a human manager moves from 6/8 to 13/15), copies the date and writes club and job into the person |

Mutations: `mutacoes/leva226.py` (9, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 8 caught. `dq13_reduzida` escaped: the score with reduced ability (0x5442f0) decides nothing in the career up to 2002-06-01, and in the real game comparing (to 2002-01-01) there was no call where ×0.2 and ×0.1 gave different scores after clamping to 1..20. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 227th batch — the fixed club and nation keys by name and the club key remap after the database trim; the fixed club competition and staff competition/award keys by name (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `casar_chave_de_clube()` | 0x6146b0 | cdecl | index.cpp: matches the database club name with the 313 fixed club keys (372 names in 4 chains, _stricmp): the first hit of each chain stores the row in the key global |
| `remapear_chave_de_clube()` | 0x60ba80 | cdecl | index.cpp: after the club table trim, the first club key holding the old id gets the new one (returns 1; none: 0) |
| `casar_chave_de_nacao()` | 0x60d810 | cdecl | index.cpp: matches the database nation name with the 209 fixed nation keys (217 names in 2 chains, _stricmp) |
| `casar_chave_de_competicao_de_clube()` | 0x60efd0 | cdecl | index.cpp: matches the club competition long name with the 301 fixed keys (351 names in 3 chains, _stricmp; the two Croatian divisions also store 0x9cf8a4) |
| `casar_chave_de_competicao_de_pessoa()` | 0x6115e0 | cdecl | index.cpp: matches the staff competition or award name with the 417 fixed keys (442 names in 5 chains, _stricmp) |

Mutations: `mutacoes/leva227.py` (6, by the save proof): 4 caught. `ix2_comp_pessoa` escaped: the person competition keys (0x9cfa20..0x9d00a0) are exe globals, not written into the .qst, and the quick start does not read them before saving; the save proof (which evaluates the save batch mutations) does not see them. `ix2_devolucao` escaped: the caller (set_key_ids 0x601a40) ignores the return value of the key matching routines (the result of the last _stricmp). (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 228th batch — reading the .dat files: continents, nations, cities; reading the .dat files: stadiums, staff 157, staff history; reading the .dat files: competition histories, staff competitions, staff competition history (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ler_continentes()` | 0x54d5e0 | cdecl | Db_files.cpp: reads the continent.dat table (id 3) of 198-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_nacoes()` | 0x54d740 | cdecl | Db_files.cpp: reads the nation.dat table (id 4) of 290-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came |
| `ler_cidades()` | 0x54d8a0 | cdecl | Db_files.cpp: reads the city.dat table (id 21) of 56-byte records in one go, through zread (save) or fread (Data), with the entry version 1 to 2 (other: box and exit); 1 if everything came |
| `ler_estadios()` | 0x54da00 | cdecl | Db_files.cpp: reads the stadium.dat table (id 5) of 78-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_pessoas_157()` | 0x54e780 | cdecl | Db_files.cpp: reads the staff.dat (staff, 157-byte file version) table (id 6) of 157-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_historico_de_pessoas()` | 0x54fd60 | cdecl | Db_files.cpp: reads the staff_history.dat table (id 17) of 17-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_historicos_de_competicao()` | 0x550230 | cdecl | Db_files.cpp: reads the club_comp_history.dat and nation_comp_history.dat table (id 19, 20) of 26-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_competicoes_de_pessoal()` | 0x550380 | cdecl | Db_files.cpp: reads the staff_comp.dat table (id 11) of 101-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came |
| `ler_historico_de_competicoes_de_pessoal()` | 0x5504d0 | cdecl | Db_files.cpp: reads the staff_comp_history.dat table (id 18) of 58-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |

Mutations: `mutacoes/leva228.py` (9, by the save proof): 8 caught. `dbf_historico_de_competicoes_de_pessoal` escaped: the staff competition history table is not a .qst item (there is no staff_comp_history.dat in the container) and the quick start writes nothing derived from it; the save proof does not see it. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 229th batch — reading the .dat files: officials, colours, names; reading the .dat files with a converter: clubs, competitions, non-players; reading the .dat files: players, staff 110, preferences (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ler_arbitros()` | 0x5505c0 | cdecl | Db_files.cpp: reads the officials.dat table (id 7) of 43-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_cores()` | 0x550710 | cdecl | Db_files.cpp: reads the colour.dat table (id 2) of 58-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_nomes()` | 0x550860 | cdecl | Db_files.cpp: reads the first_names.dat, second_names.dat and common_names.dat tables (ids 13, 14, 15) of 60-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |
| `ler_clubes()` | 0x54db50 | cdecl | Db_files.cpp: reads the club.dat and nat_club.dat table (ids 0, 1) of 581-byte records in one go, through zread (save) or fread (Data), with the entry version 2, and turns reputation 0 into 1; version 1: the 0x54dc90 converter (other: box and exit); 1 if everything came |
| `ler_competicoes()` | 0x54feb0 | cdecl | Db_files.cpp: reads the club_comp.dat and nation_comp.dat table (ids 12, 16) of 107-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54ffd0 converter (other: box and exit); 1 if everything came |
| `ler_nao_jogadores()` | 0x54e880 | cdecl | Db_files.cpp: reads the non-player table of staff.dat (id 9; the community map calls it players) of 68-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54e9a0 converter (other: box and exit); 1 if everything came |
| `ler_jogadores()` | 0x54f030 | cdecl | Db_files.cpp: reads the player table of staff.dat (id 10; the community map calls it non_players) of 70-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54f150 converter (other: box and exit); 1 if everything came |
| `ler_pessoas_110()` | 0x54e630 | cdecl | Db_files.cpp: reads the staff.dat (staff, 110-byte file version: the save and community databases) table (id 6) of 110-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came |
| `ler_preferencias()` | 0x54fc10 | cdecl | Db_files.cpp: reads the staff preferences table (id 22: the save and community databases) of 52-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came |

Mutations: `mutacoes/leva229.py` (9, by the save proof): 9 caught.  Save proof (the .qst with and without the routines): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 230th batch — writing the .dat files: continents, nations, cities; writing the .dat files: stadiums, clubs, staff 110; writing the .dat files: non-players, players, preferences (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `gravar_continentes()` | 0x54d6e0 | cdecl | Db_files.cpp: writes the continent.dat (id 3) table of 198-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_nacoes()` | 0x54d840 | cdecl | Db_files.cpp: writes the nation.dat (id 4) table of 290-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_cidades()` | 0x54d9a0 | cdecl | Db_files.cpp: writes the city.dat (id 21) table of 56-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_estadios()` | 0x54daf0 | cdecl | Db_files.cpp: writes the stadium.dat table of 78-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_clubes()` | 0x54e5d0 | cdecl | Db_files.cpp: writes the club.dat and nat_club.dat (ids 0, 1) table of 581-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_pessoas_110()` | 0x54e720 | cdecl | Db_files.cpp: writes the staff.dat (staff, 110 bytes; id 6) table of 110-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_nao_jogadores()` | 0x54efd0 | cdecl | Db_files.cpp: writes the staff.dat non-player (id 9; the community map calls it players) table of 68-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_jogadores()` | 0x54fbb0 | cdecl | Db_files.cpp: writes the staff.dat player (id 10; the community map calls it non_players) table of 70-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |
| `gravar_preferencias()` | 0x54fd00 | cdecl | Db_files.cpp: writes the staff preferences (id 22) table of 52-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out |

Mutations: `mutacoes/leva230.py` (9, by the save proof): 9 caught.  Save proof (the .qst with and without the routines): EQUAL (16856662 bytes). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 231st batch — writing the .dat files: staff history, competitions, competition histories; writing the .dat files: staff competitions, officials, colours; writing the .dat files: names; opening and creating the .dat files (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `gravar_historico_de_pessoas()` | 0x54fe50 | cdecl | Db_files.cpp: writes n 17-byte records (staff_history.dat, id 17) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went |
| `gravar_competicoes()` | 0x5501d0 | cdecl | Db_files.cpp: writes n 107-byte records (club_comp.dat, in two calls: the competitions and the 127 reserve ones; nation_comp.dat) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went |
| `gravar_historicos_de_competicao()` | 0x550320 | cdecl | Db_files.cpp: writes n 26-byte records (club_comp_history.dat and nation_comp_history.dat, ids 19 and 20) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went |
| `gravar_competicoes_de_pessoal()` | 0x550470 | cdecl | Db_files.cpp: writes n 101-byte records (staff_comp.dat, id 11) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went |
| `gravar_arbitros()` | 0x5506b0 | cdecl | Db_files.cpp: writes n 43-byte records (officials.dat, id 7) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went |
| `gravar_registros_58()` | 0x550800 | cdecl | Db_files.cpp: writes n 58-byte records (colour.dat, id 2, and staff_comp_history.dat, id 18: the same routine for both) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went |
| `gravar_nomes()` | 0x550950 | cdecl | Db_files.cpp: writes n 60-byte records (first_names.dat, second_names.dat and common_names.dat, ids 13 to 15) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went (it only writes, despite the community name) |
| `abrir_arquivo_do_banco()` | 0x54d580 | cdecl | Db_files.cpp: builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "rb"); the comparator checks the path |
| `criar_arquivo_do_banco()` | 0x54d5b0 | cdecl | Db_files.cpp: builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "wb"); only when writing the database to the folder; the comparator checks the path |

Mutations: `mutacoes/leva231.py` (9, by the save proof): 7 caught. `dbh_gravar_historico_de_pessoas` escaped: save_database only calls 0x54fe50 when [0xae23cc] ≠ 0 (0x5311ac); in the game the staff history has already become staff history.tmp, so the write only runs when saving the database to the Data folder, never into the .qst. `dbh_criar` escaped: 0x54d5b0 is only called from save_database(…, 0, …), saving the database to the Data folder; the .qst is opened by zopen 0x9326e0. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 232nd batch — freeing the staff and award histories and growing the competition history; the club-by-reputation and squad-by-ability orders; the physical and general normalisation of player attributes and the second name by weight; the version 1 converters: clubs, non-players, players; the version 1 competition converter and the nation's English name and the club's English short name; game creation: the senior and B team squads, the B team staff, the per-nation name index; game creation: removing a person from the club, the club scouts with the partner team ones (official on 2026-10-24)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `liberar_historico_de_pessoas()` | 0x527ee0 | cdecl | Database.cpp: with the staff history table present, frees it (GlobalFree in the shared database, free otherwise) and zeroes pointer and count |
| `liberar_historico_de_premios()` | 0x527f20 | cdecl | Database.cpp: the same for the award (staff competition) history |
| `crescer_historico_de_competicoes()` | 0x527f60 | cdecl | Database.cpp: reallocs the competition history to n more rows (out of memory: 0); count, national part pointer and the new rows with their own index at +0 and the rest zero; 1 |
| `ordem_dos_clubes_pela_reputacao()` | 0x540020 | cdecl | Database.cpp: qsort comparison of club ids by reputation (+0x80), highest first; negative id = 0 |
| `ordem_do_elenco_pela_habilidade()` | 0x5402f0 | cdecl | Database.cpp: qsort comparison of squads: null staff last, then the player word +0xb, highest first |
| `ordem_do_elenco_pela_habilidade_atual()` | 0x549c30 | cdecl | Database.cpp: qsort comparison of a squad: no player last, then current ability (+5) and, when tied, the word +0xb, highest first |
| `normalizar_atributos_fisicos_do_jogador()` | 0x538ec0 | cdecl | Database.cpp: lowers the physical attributes above the current ability (cut by the weighted sum with the .rdata constants) and rebalances the attribute vector without cells 0x18 and 0x1e, target −40 to 0; with +0xf = 20 removes pair and trio averages; removes and restores the training progress |
| `segundo_nome_pelo_peso()` | 0x53a0e0 | cdecl | Database.cpp: a second name: with the nation having more than 10, the heaviest of 3 draws in its range; otherwise a draw over the whole table |
| `normalizar_atributos_do_jogador()` | 0x538bc0 | cdecl | Database.cpp: rebalances the player attribute vector with target −20 (12 cells within the limit, excess above 85 spread); with +0xf ≥ 15 removes the +0x2a/+0x3a average; removes and restores the training progress |
| `ler_clubes_v1()` | 0x54dc90 | cdecl | Db_files.cpp: reads version 1 club.dat field by field (577 bytes on file for 581-byte records, holes at +0x37, +0x52 and +0x244), the reputation in one byte (0 becomes 1) × 500; 0 at the first read that does not come, 1 if everything came |
| `ler_nao_jogadores_v1()` | 0x54e9a0 | cdecl | Db_files.cpp: reads the version 1 non-player table (id 9; the community map calls it players) field by field (65 bytes on file for 68-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came |
| `ler_jogadores_v1()` | 0x54f150 | cdecl | Db_files.cpp: reads the version 1 player table (id 10; the community map calls it non_players) field by field (67 bytes on file for 70-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came |
| `ler_competicoes_v1()` | 0x54ffd0 | cdecl | Db_files.cpp: reads version 1 club_comp.dat / nation_comp.dat field by field (104 bytes on file for 107-byte records, holes at +0x37 and +0x52), the reputation in one byte; 0 at the first read that does not come, 1 if everything came |
| `nome_ingles_da_nacao()` | 0x5453f0 | cdecl | Database.cpp: the nation's name, or its English one from eng.lng (109-byte rows, by id) when another language is loaded |
| `nome_curto_ingles_do_clube()` | 0x545770 | cdecl | Database.cpp: the club's short name, or its English one from eng.lng (79-byte rows, through the id map or by id) when another language is loaded |
| `cortar_elencos_de_a_e_b()` | 0x541e60 | cdecl | Database.cpp: flags every club that has a partner team (+0x244) and, for every senior team, trims the joint senior+B squad to 48 by reputation, from the bottom, keeping 3 goalkeepers |
| `juntar_o_time_b_ao_a()` | 0x544970 | cdecl | Database.cpp: for every senior team with a B team: trims the B squad to what fits in 50 with the senior one and moves the B backroom staff (coaches, scouts, physios and the manager when the senior team has none) to the senior club |
| `montar_indice_de_nomes()` | 0x539750 | cdecl | Database.cpp: allocates and builds the per-nation index of the first, second and common name tables (first row, weight total and count of every group); the last group of each table is left unwritten, as in the original |
| `tirar_pessoa_do_clube()` | 0x53c190 | cdecl | Database.cpp: removes the person from the club slot her job points to (writing the date into her, when given, and, for the senior national team manager, from the under-21 side too) and, if asked, clears the club in the person; 0 if she was not in the slot |
| `olheiros_do_clube()` | 0x53cd60 | cdecl | Database.cpp: copies the 7 club scouts into an array; with a partner team (senior/B), merges both up to 7 and clears the rest |

Mutations: `mutacoes/leva232.py` (20, by the save proof): 11 caught. `dbi_crescer_historico_de_competicoes` escaped: only runs at the end of a competition; the save proof only creates and saves, and the load goes to 2001-08-28. `dbi_ordem_do_elenco_pela_habilidade_atual` escaped: only runs in the AI squad strength during the season, not in the creation or the .qst save. `dbj1_clubes` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj1_nao_jogadores` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj1_jogadores` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj2_competicoes` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj2_clube_curto` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbk1_juntar_comissao` escaped: no B team of the test leagues seems to have a manager, scout or physio at creation, so nothing moves to the A team (to be checked against the database). `dbk2_olheiros` escaped: the creation does not call 0x53cd60 (only the career); the save proof does not see it. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 233rd batch — the contract rules: club in season and club and nation among the six major ones; the contract opinion links: clear, add and remove; the club staff with the partner team ones: the assistant, the coaches and the coach count; the English names of the nation's three-letter code, the competition and the staff competition; the stadium's English name, does the language's .lng exist? and freeing the English tables; game creation: generating the missing backroom staff, trimming the club table, pruning the competition history; database load: resolving indexes to pointers and the club vector by reputation (official on 2026-10-25)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_em_temporada()` | 0x547770 | cdecl | Database.cpp: null club: 0; otherwise key_nation_club_in_season 0x669bd0(club, null, null): 1 when today is inside the season of the club nation (the whole eax) |
| `clube_em_nacao_grande()` | 0x549e50 | cdecl | Database.cpp: 1 when the club nation (+0x53) is England, France, Germany, Italy, Spain or Holland (the index.cpp keys); null club or nation: 0 |
| `nacao_grande()` | 0x549ea0 | cdecl | Database.cpp: the same six-key test on the nation itself; null: 0 |
| `limpar_vinculos_de_opiniao()` | 0x549c90 | cdecl | Database.cpp: −1 in both links (id and value) of the person contract; null person or id ≥ [0xae2364]: nothing |
| `juntar_vinculo_de_opiniao()` | 0x549cf0 | cdecl | Database.cpp: writes the other person id and the value into the first empty link (+0x27 = −1) or else into the second, overwriting; both persons must be valid |
| `tirar_vinculo_de_opiniao()` | 0x549d60 | cdecl | Database.cpp: empties the first link with the given value or, if not, the second |
| `auxiliar_do_clube()` | 0x53cb80 | cdecl | Database.cpp: the club assistant (+0xd3); without one, the partner (senior/B) team one; none: 0 |
| `treinadores_do_clube()` | 0x53cc30 | cdecl | Database.cpp: copies the 5 club coaches into an array; with a partner team (senior/B), merges both up to 5 and clears the rest |
| `contar_treinadores_do_clube()` | 0x53c8d0 | cdecl | Database.cpp: how many valid coaches (non-null, id < [0xae2364]) the club and the partner team have together |
| `sigla_inglesa_da_nacao()` | 0x5454c0 | cdecl | Database.cpp: the nation's three-letter code, or its English one from eng.lng (109-byte rows, by id, code at +0x4f) when another language is loaded |
| `nome_ingles_da_competicao()` | 0x5455a0 | cdecl | Database.cpp: the competition's name, or its English one from eng.lng (83-byte rows: club ones by id, nation ones by id minus the club count) when another language is loaded |
| `nome_ingles_da_competicao_de_pessoal()` | 0x5456a0 | cdecl | Database.cpp: the staff competition's name, or its English one from eng.lng (79-byte rows, by id) when another language is loaded |
| `nome_ingles_do_estadio()` | 0x545990 | cdecl | Database.cpp: the stadium's name, or its English one from eng.lng (52-byte rows, by id) when another language is loaded |
| `arquivo_do_idioma_existe()` | 0x535930 | cdecl | Database.cpp: builds CM3_TEMP\<language>.lng (0, 1, 11 and 12: eng.lng; 2 to 10: fra, ger, ita, esp, dan, hol, nor, por, swe) and tells whether it exists; any other language, 0 |
| `liberar_tabelas_em_ingles()` | 0x544f30 | thiscall | Database.cpp: frees the 8 non-null rows of the eng.lng table, +0x20 to +0x3c, without clearing the pointers |
| `gerar_comissao_extra()` | 0x527240 | cdecl | Database.cpp: fills coaches, scouts and physios of the clubs of chosen nations by reputation (reserve rows, names of the nation, random birth date) and creates unattached managers for clubs without one; over 23,000: box and 0 |
| `cortar_tabela_de_clubes()` | 0x5267c0 | cdecl | Database.cpp: trims the club table by the database size: the best per nation, those referred by staff and histories, national teams, keys and clubs of chosen nations; swaps the pointers, compacts, builds [0xae23fc] and reallocates in place |
| `podar_historico_de_competicoes()` | 0x539ae0 | cdecl | Database.cpp: keeps in the club competition history only the rows of competitions with no nation or a chosen one, reallocates, sets the national part and sorts (qsort with 0x540780) |
| `resolver_indices_do_banco()` | 0x5315a0 | cdecl | Database.cpp: replaces every id field of the 12 tables with the record pointer, with the limits, the 0xde1f64 log lines, the boxes for staff with bad player, preferences or non-player data and the original bugs (club +0x5b clears +0x60; stadium +0x48 clears the city) |
| `ordenar_clubes_pela_reputacao()` | 0x53db40 | cdecl | Database.cpp: with rebuild, fills the vector [0xae2d1c] with the club ids; then sorts it by reputation (qsort with 0x540020) |

Mutations: `mutacoes/leva233.py` (21, by the save proof): 9 caught. `dbl_clube_em_temporada` escaped: the 0x547770 copy is switched off (DESLIGADAS: the hook, even passthrough, changes the career in both modes), so the mutation does not run. `dbl_nacao_grande` escaped: only runs in the youth regeneration (0x7abac0), during the season; the save proof does not reach it. `dbl_juntar_vinculo` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_tirar_vinculo` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_auxiliar` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_treinadores` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_contar_treinadores` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbm1_competicao` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm1_pessoal` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm2_estadio` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm2_lng` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm2_soltar` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 234th batch — the scout: the club has enough defenders, midfielders and strikers; the scout: the club has hard workers, a fit goalkeeper and ball-winning midfielders; the scout: the club lacks aerial strikers, aerial defenders and fast defenders; the scout: the club has a fast striker and two creative midfielders, and the squad strength band; the club reputation for a person and how many of the favoured and disliked staff of the person are at the club; removing from the preferences: the favoured club of a person, the favoured person of a club and the favoured person of a person; removing a disliked person from the preferences and the favoured staff into an array (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_tem_defensores()` | 0x549100 | cdecl | Database.cpp: club conceding more than 1.35 goals a game: 0; otherwise 1 when more than 2 defenders (+0x11 = 20) without serious injury are above the ability limit (reputation/65 + 40, 40 to 145) |
| `clube_tem_meias()` | 0x549200 | cdecl | Database.cpp: 1 when more than 2 midfielders (+0x14, +0x12 or +0x13 = 20) without serious injury are above the limit (reputation/65 + 40, 40 to 145) |
| `clube_tem_atacantes()` | 0x5492c0 | cdecl | Database.cpp: club scoring fewer than 1.5 goals a game: 0; otherwise 1 when more than 1 striker (+0x15 = 20) without serious injury is above the limit (reputation/65 + 40, 40 to 150) |
| `clube_tem_esforcados()` | 0x5493c0 | cdecl | Database.cpp: 1 when more than 4 players without serious injury are above the ability limit (reputation/65 + 40, 40 to 140) with attribute +0x44 above 14 |
| `clube_tem_goleiro()` | 0x549470 | cdecl | Database.cpp: 1 when some goalkeeper (+0xf = 20) in the squad has no serious injury and a valid id |
| `clube_tem_meias_que_roubam()` | 0x549850 | cdecl | Database.cpp: 1 when more than 1 midfielder without serious injury is above the limit (reputation/75 + 25, 40 to 135) with attributes +0x44 and +0x3e above reputation/500 + 5 (at most 14) |
| `faltam_atacantes_pelo_alto()` | 0x5494d0 | cdecl | Database.cpp: 1 when at most 1 striker (+0x15 = 20) without serious injury is above the ability limit (reputation/75 + 25, 40 to 135) with attribute +0x2e above reputation/500 + 5 (at most 13) |
| `faltam_zagueiros_pelo_alto()` | 0x5495b0 | cdecl | Database.cpp: the same with defenders (+0x11 = 20) and attribute +0x2e (at most 13) |
| `faltam_zagueiros_rapidos()` | 0x549690 | cdecl | Database.cpp: the same with defenders (+0x11 = 20) and attribute +0x36 (at most 10) |
| `clube_tem_atacante_rapido()` | 0x549770 | cdecl | Database.cpp: 1 when some striker (+0x15 = 20) without serious injury is above the ability limit (reputation/75 + 25, 40 to 135) with attribute +0x36 above reputation/500 + 5 (at most 17) |
| `clube_tem_meias_criativos()` | 0x549940 | cdecl | Database.cpp: 1 when more than 1 midfielder without serious injury is above the limit with attributes +0x43 and +0x37 above reputation/500 + 5 (at most 14) |
| `faixa_de_forca_do_elenco()` | 0x549a30 | cdecl | Database.cpp: the 0..6 band from the average ability of the best in the squad (up to 16, qsort with 0x549c30) or, without them, from the reputation; adjusted by the scout club reputation and, in the same competition, by the reputation gap and the [0xae2d14] table; null club: 3 |
| `reputacao_do_clube_para_pessoa()` | 0x545e90 | cdecl | Database.cpp: the club reputation (+0x80) as seen by the person: with a division stronger than the club, weighs it by the league position (table 0x9b59c0, mixed with last season before 20 games) and, for someone playing at the club, by the 0x7c1e30 average; ±500 when the club is a favoured or disliked one; writes [0xae1080] |
| `contar_rejeitados_no_clube()` | 0x547c90 | cdecl | Database.cpp: how many of the three disliked staff (preferences +0x28..+0x30) are at the club (+0x39) |
| `contar_favoritos_no_clube()` | 0x547ce0 | cdecl | Database.cpp: how many of the three favoured staff (preferences +0x1c..+0x24) are at the club (+0x39) |
| `tirar_clube_favorito()` | 0x547f50 | cdecl | Database.cpp: clears the first of the three favoured clubs of the person (preferences +4..+0xc) that is the club |
| `clube_tirar_pessoa_favorita()` | 0x547bd0 | cdecl | Database.cpp: clears the first of the three favoured staff of the club (+0x9b..+0xa3) that is the person |
| `tirar_pessoa_favorita()` | 0x547c10 | cdecl | Database.cpp: clears the first of the three favoured staff of the person (preferences +0x1c..+0x24) that is the target |
| `tirar_pessoa_rejeitada()` | 0x547c50 | cdecl | Database.cpp: clears the first of the three disliked staff of the person (preferences +0x28..+0x30) that is the target |
| `pessoas_favoritas()` | 0x548090 | cdecl | Database.cpp: clears the counter and copies the non-null favoured staff (preferences +0x1c..+0x24) into the array, counting them |

Mutations: `mutacoes/leva234.py` (20, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 4 caught. `dbo_defensores` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_meias` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_atacantes` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_esforcados` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_goleiro` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_meias_que_roubam` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_atacantes_pelo_alto` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_zagueiros_pelo_alto` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_zagueiros_rapidos` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_atacante_rapido` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_meias_criativos` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbo_faixa_de_forca` escaped: only runs in the type-4 scout report with a target and a club (PLAYER_SEARCH::run_scout_day 0x7b3e20, range 0x7b5607..0x7b57da), which the automatic holiday does not reach; the answer only becomes a parameter of the report news; also escapes the real game comparing (0 divergences). Test: session with scout reports on clubs. `dbp_tirar_clube_favorito` escaped: the path is rare in the career to 2002-06-01 (disputes, board, or the person without a preference at the queried club) and did not change the career; the comparator runs in the real game comparing (0 divergences). Test: broad career. `dbp_clube_tirar_favorita` escaped: the path is rare in the career to 2002-06-01 (disputes, board, or the person without a preference at the queried club) and did not change the career; the comparator runs in the real game comparing (0 divergences). Test: broad career. `dbp_tirar_rejeitada` escaped: the path is rare in the career to 2002-06-01 (disputes, board, or the person without a preference at the queried club) and did not change the career; the comparator runs in the real game comparing (0 divergences). Test: broad career. `dbp_pessoas_favoritas` escaped: the path is rare in the career to 2002-06-01 (disputes, board, or the person without a preference at the queried club) and did not change the career; the comparator runs in the real game comparing (0 divergences). Test: broad career. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 235th batch — adding to the preferences: the favoured and the disliked person of a club and the favoured person of a person; the disliked club of a person, the fading of personal relationships and the list of disliked staff; the natural position mask, how many of the squad have it and the picked favoured team mate; the nation rules: club outside Argentina, outside Argentina, France and Croatia, and Melanesian nation; the Polynesian nation, the coaching job name and the years between two dates as text; the club order by short name and the staff order by full name, and whether a person matches the names; the club order by latitude and by longitude of the stadium city (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_acrescentar_pessoa_favorita()` | 0x547870 | cdecl | Database.cpp: puts the person among the three favoured staff of the club (+0x9b..+0xa3): already favoured, nothing; disliked (+0xa7..+0xaf), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3) |
| `clube_acrescentar_pessoa_rejeitada()` | 0x547940 | cdecl | Database.cpp: puts the person among the three disliked staff of the club (+0xa7..+0xaf): already disliked, nothing; favoured (+0x9b..+0xa3), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3) |
| `acrescentar_pessoa_favorita()` | 0x547a10 | cdecl | Database.cpp: puts the other person among the three favoured staff of the person (preferences +0x1c..+0x24): already favoured, nothing; disliked (+0x28..+0x30), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3) |
| `acrescentar_clube_rejeitado()` | 0x547e70 | cdecl | Database.cpp: puts the club among the three disliked clubs of the person (preferences +0x10..+0x18): the club already dislikes the person (+0xa7..+0xaf), nothing; favoured (+4..+0xc), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot (3) |
| `esmaecer_relacoes()` | 0x547f90 | cdecl | Database.cpp: each favoured person (preferences +0x1c..+0x24) leaves when a random draw of (+0x59) + 15 gives 0; each disliked one (+0x28..+0x30), with 35 − (+0x5d) |
| `pessoas_rejeitadas()` | 0x5480f0 | cdecl | Database.cpp: copies the present disliked staff (preferences +0x28..+0x30) to the output, counting in the byte of the third argument (always cleared) |
| `mascara_da_posicao_natural()` | 0x5481a0 | cdecl | Database.cpp: the mask of the positions where the player is natural (20): goalkeeper 1; full backs, centre backs, midfielders, forwards (bits 2..0xc00); with none, the first position at 15 or more; otherwise 0x10 |
| `contar_elenco_com_mascara()` | 0x548150 | cdecl | Database.cpp: how many of the 50 in the squad (+0xd7) with a player record have the given natural position mask |
| `escolher_companheiro_favorito()` | 0x548da0 | cdecl | Database.cpp: the squad mate the person picks as favoured: filters by the person (age, +0x57, +0x59) and by the mate (disliked, position, contract, reputation, 0x8cca10), score by reputation, the 0x7c1e30 average, already favoured, ids, contract level and nationality; the highest score |
| `clube_fora_da_argentina()` | 0x5487b0 | cdecl | Database.cpp: 1 when the club nation (+0x53) is not Argentina (key 0x9cf218); null club or nation: 0; the rest of eax comes from the id |
| `clube_fora_de_argentina_franca_croacia()` | 0x5487d0 | cdecl | Database.cpp: 1 when the club nation is not Argentina, France or Croatia (keys 0x9cf218, 0x9cf300, 0x9cf2ac); null: 0 |
| `nacao_da_melanesia()` | 0x53d900 | cdecl | Database.cpp: 1 when the nation is one of the five Melanesian ones (keys 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500) |
| `nacao_da_polinesia()` | 0x53d940 | cdecl | Database.cpp: 1 when the nation is one of the five Polynesian ones (keys 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514) |
| `nome_do_cargo_de_treino()` | 0x53ac40 | cdecl | Database.cpp: the job name (5/12 manager, 6/13 assistant, 8/15 coach; others: empty) translated into 0xae1980; with language 4 writes the gender (4 or 1) into the second argument |
| `anos_entre_datas_em_texto()` | 0x540340 | cdecl | Database.cpp: the full years between the dates (end year minus start year, minus 1 when the start day is later) as "%d" text of an unsigned word; returns 1 |
| `ordem_dos_clubes_pelo_nome_curto()` | 0x540080 | cdecl | Database.cpp: qsort comparison of clubs by short name (+0x38), case-insensitive (0x90c1d0); null club: empty text |
| `ordem_das_pessoas_pelo_nome_completo()` | 0x540270 | cdecl | Database.cpp: qsort comparison of staff by style-1 name (0x53dba0), case-insensitive; null person: empty text |
| `pessoa_confere_com_os_nomes()` | 0x53e990 | cdecl | Database.cpp: 1 when the person (with a player or non-player record) has the requested first, second and common name ids (on the non-null ones) and, with a date, the same birth date |
| `ordem_dos_clubes_pela_latitude()` | 0x53fd40 | cdecl | Database.cpp: qsort comparison of clubs by the latitude (+0x23, f64) of the city (+0x38) of the stadium (+0x69), lowest first; no city: 0 |
| `ordem_dos_clubes_pela_longitude()` | 0x53fe50 | cdecl | Database.cpp: the same comparison by longitude (+0x2b, f64) |

Mutations: `mutacoes/leva235.py` (20, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 13 caught. `dbr_fora_da_argentina` escaped: only matters in discipline, managers and news of Argentine clubs, absent from the career leagues; also escaped the real game comparing (0 divergences). `dbr_fora_de_arg_fra_cro` escaped: only changes something for French clubs (ban appeal), absent from the career leagues; also escaped the real game comparing (0 divergences). `dbr_nome_do_cargo` escaped: the job text only appears in retirement news and regeneration, with no effect on the state; also escaped the real game comparing (0 divergences). `dbr_anos_em_texto` escaped: only in the records screens; also escaped the real game comparing (0 divergences). `dbr_pessoas_pelo_nome_completo` escaped: transfer manager sort whose result did not change the career to 2002-06-01; also escaped the real game comparing (0 divergences). `dbr_confere_nomes` escaped: the person search by names had no call where the surname decided; also escaped the real game comparing (0 divergences). `dbr_longitude` escaped: the longitude sort is only used in the cup draws of Belgium, Netherlands, Italy (C), Spain and Turkey, outside the English career; also escaped the real game comparing (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 236th batch — the name from the ids, the official name and the position mask text; the position name, the player description and the best favoured candidate; the player strengths and weaknesses; the club and its B team: physios, whether it is a B team and the non-playing staff; attributes as the viewer sees them: whether a player attribute is visible, its value for the viewer and the non-player attribute name; the staff order by surname and the club order by reputation, and the EEC club; the regions sorted by name and the person of an opinion link (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `nome_por_ids()` | 0x53e140 | cdecl | Database.cpp: the first, second and common name records by index ([0xae23f0], [0xae23f4], [0xae23f8]; the nickname given by a human instead of the common name) and the text by 0x53e2e0 |
| `nome_oficial()` | 0x53eab0 | cdecl | Database.cpp: the official name by format: 1 "%s %s", 2 "%c.%s", 3 "%s, %s", 4 "%s, %c", 5 the surname, 6 the initials; nation [0x9cf488]: surname only |
| `texto_da_mascara_da_posicao()` | 0x5482f0 | cdecl | Database.cpp: the translated position name of the mask (goalkeeper, support, striker, wing back, wingers, midfielders, centre back, full backs, sweeper, substitute, defender) and the gender by language |
| `nome_da_posicao()` | 0x5461b0 | cdecl | Database.cpp: the translated position name by mask (the last one written wins; more than 3 positions "utility player", none "player") and the gender by language |
| `descricao_do_jogador()` | 0x5465a0 | cdecl | Database.cpp: the player description by strengths (0x546ab0) and position: forward, midfielder, full back, defender, veteran or rookie by age; the gender by language |
| `melhor_candidato_a_favorito()` | 0x548810 | cdecl | Database.cpp: 1 when the candidate passes the favoured team mate filters and none of the 50 in the squad has a higher score (the 0x548da0 score) |
| `forcas_do_jogador()` | 0x546ab0 | cdecl | Database.cpp: the player strength bits: natural positions, the [0xae2d0c] nibble and 19 draws against bytes, attributes (0x543540) and averages (0x7c1e30) |
| `fraquezas_do_jogador()` | 0x5470d0 | cdecl | Database.cpp: the player weakness bits: natural positions and draws against bytes, attributes, averages, the 0x62c3d0 word and the starter score (0x8972c0, 0x5b2ae0, 0x6e6340) |
| `contar_fisioterapeutas()` | 0x53c9b0 | cdecl | Database.cpp: how many of the 3 physios (+0x1cf) of the club and of its linked B team are valid people; in `al` |
| `clube_e_time_b()` | 0x5448c0 | cdecl | Database.cpp: 1 when the club has a partner (0x540a50 with the flag) and is its B team |
| `pessoas_fora_do_elenco()` | 0x544c70 | cdecl | Database.cpp: the non-playing staff (6 from +0xbf, 15 from +0x19f) of the club and its B team into an array, up to the maximum; the count in `al` |
| `atributo_visivel()` | 0x543280 | cdecl | Database.cpp: 1 when the viewer sees the attribute: no fog or knowledge 3, always; knowledge 1 or 2, by the attribute and the id modulo 4 (divisible by 4 or 2, or by 3); in `al` |
| `atributo_para_quem_ve()` | 0x543440 | cdecl | Database.cpp: the player attribute (0x543540) or 0xf9 (unknown) when the fog hides it from the viewer; in `al` |
| `nome_do_atributo_de_tecnico()` | 0x543b10 | cdecl | Database.cpp: the translated name (0x66f4e0) of the 12 non-player attributes, short or full; returns 1 |
| `ordem_das_pessoas_pelo_sobrenome()` | 0x53ff60 | cdecl | Database.cpp: qsort comparison of staff by style-4 name (0x53dba0), case-insensitive; null person: empty text |
| `ordem_dos_ponteiros_de_clube_pela_reputacao()` | 0x53ffe0 | cdecl | Database.cpp: qsort comparison of club pointers by reputation (+0x80), highest first; null club: 0 |
| `nacao_do_clube_na_cee()` | 0x53d7d0 | cdecl | Database.cpp: 1 when the club nation (+0x53) is in the EEC (+0x7f = 2) |
| `regioes_pelo_nome()` | 0x545dc0 | cdecl | Database.cpp: the global array of the 14 regions (0xae2104) sorted by name in the given language (qsort 0x945333 with 0x545e00) |
| `ordem_das_regioes_pelo_nome()` | 0x545e00 | cdecl | Database.cpp: qsort comparison of regions by name (0x616f20), case-insensitive; null a: 1; null b: -1 |
| `pessoa_da_opiniao()` | 0x549dd0 | cdecl | Database.cpp: the person at the other end of the opinion link (record [0xae2d0c] + id × 79: +0x27/+0x37 or +0x3b/+0x4b), or 0 |

Mutations: `mutacoes/leva236.py` (20, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 4 caught. `dbs_nome_oficial` escaped: only changes text (news, media, match commentary and panel) that does not reach the fingerprint; also escaped the real game comparing (0 divergences). Test: comparing session with news and a watched match. `dbs_mascara_texto` escaped: only changes text (news, media, match commentary and panel) that does not reach the fingerprint; also escaped the real game comparing (0 divergences). Test: comparing session with news and a watched match. `dbs_nome_da_posicao` escaped: only changes text (news, media, match commentary and panel) that does not reach the fingerprint; also escaped the real game comparing (0 divergences). Test: comparing session with news and a watched match. `dbs_descricao` escaped: only changes text (news, media, match commentary and panel) that does not reach the fingerprint; also escaped the real game comparing (0 divergences). Test: comparing session with news and a watched match. `dbt_fisioterapeutas` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_time_b` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_fora_do_elenco` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_atributo_visivel` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_atributo_para_quem_ve` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_nome_do_atributo` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_pessoas_pelo_sobrenome` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_clubes_pela_reputacao` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_clube_na_cee` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_regioes_pelo_nome` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_ordem_das_regioes` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. `dbt_pessoa_da_opiniao` escaped: screen-only routine (person profile, compare players, search and filters, club screen, friendlies, offer contract): the hidden career and the real game comparing do not reach it. Test: Lucas comparing session on those screens. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 237th batch — the name from strings, the job name and the language name; the national team name, the player attribute name and the positions text; the region name and the continent adjective (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `nome_pelos_textos()` | 0x53e2e0 | cdecl | Database.cpp: the person name by format (1..7) from the first, second and common names: "%s %s", "%c.%s", "%s, %s", "%s, %c", one of them or the initials |
| `nome_do_cargo()` | 0x53ad00 | cdecl | Database.cpp: the translated job name (0..0x11), short or full, lower or upper case, in [0xae19b4]; the gender with language 4 |
| `nome_do_idioma()` | 0x53cee0 | cdecl | Database.cpp: the translated language name (1..64, else "Unknown") in [0xae21d4] |
| `nome_da_selecao()` | 0x542100 | cdecl | Database.cpp: the national team club name by style (1..4): the name, "Reserves", "B"/"'B'", "U21"/"Under 21"/"Under 21's"/"Under 21 Squad" (or 23) and "Squad"/"Main Squad" |
| `nome_do_atributo_do_jogador()` | 0x5428a0 | cdecl | Database.cpp: the translated name of the 31 player attributes, short or full; returns 1 |
| `texto_das_posicoes()` | 0x53ee50 | cdecl | Database.cpp: the positions text from the ratings (15 to 10) and the sides (15), short ("D/DM RC"), full or as letters |
| `nome_da_regiao()` | 0x616f20 | cdecl | index.cpp: the region name (1..14): from the language 4, 5 and 9 tables with the adjective form, else the translated adjective or noun by form |
| `adjetivo_do_continente()` | 0x6177f0 | cdecl | index.cpp: the continent adjective: the record one (+0x23) or, in languages 9, 5 and 4, the text by continent and adjective form |

Mutations: `mutacoes/leva237.py` (8, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 1 caught. `dbu_cargo` escaped: screen/news text that does not reach the fingerprint (media, transfers, person, club and contract screens); also escaped the real game comparing (0 divergences). `dbu_idioma` escaped: screen/news text that does not reach the fingerprint (search and person screen); also escaped the real game comparing (0 divergences). `dbu_selecao` escaped: screen/news text that does not reach the fingerprint (national and youth team screens, club screen, line-up, hall of fame); also escaped the real game comparing (0 divergences). `dbu_atributo` escaped: screen/news text that does not reach the fingerprint (player, club, search and training screens); also escaped the real game comparing (0 divergences). `dbu_posicoes` escaped: screen/news text that does not reach the fingerprint (person screen with a wide player, search, tactics, training); also escaped the real game comparing (0 divergences). `dbu_regiao` escaped: screen/news text that does not reach the fingerprint (region filter of the player search); also escaped the real game comparing (0 divergences). `dbu_continente` escaped: screen/news text that does not reach the fingerprint (search screens, menu bar, matches of the day); also escaped the real game comparing (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 238th batch — the game's fixed keys: ids from the database, colours and continents, and the report of the missing ones (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `definir_chaves_fixas()` | 0x601a40 | cdecl | index.cpp: the fixed keys (colours, continents, nations, competitions, awards, clubs) set to -1 and then to the database row by name (from eng.lng or from the database); the ones left without a value go to the "Unable to find the %s index" box (7 fatal by nation); in cm0102-1080.exe through the 0x603834 entry, with -1 over the whole ranges |

Mutations: `mutacoes/leva238.py` (5, by the save proof): 4 caught. `dbw_continentes` escaped: the continent keys (0x9cfa08..0x9cfa1c) decide nothing in the .qst creation nor in the load to 2001-08-28; Asia and Europe swapped did not change the save. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 238th batch — the game's fixed keys: ids from the database, colours and continents, and the report of the missing ones (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `definir_chaves_fixas()` | 0x601a40 | cdecl | index.cpp: the fixed keys (colours, continents, nations, competitions, awards, clubs) set to -1 and then to the database row by name (from eng.lng or from the database); the ones left without a value go to the "Unable to find the %s index" box (7 fatal by nation); in cm0102-1080.exe through the 0x603834 entry, with -1 over the whole ranges |

Mutations: `mutacoes/leva238.py` (5, by the save proof): 4 caught. `dbw_continentes` escaped: the continent keys (0x9cfa08..0x9cfa1c) decide nothing in the .qst creation nor in the load to 2001-08-28; Asia and Europe swapped did not change the save. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 238th batch — the game's fixed keys: ids from the database, colours and continents, and the report of the missing ones (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `definir_chaves_fixas()` | 0x601a40 | cdecl | index.cpp: the fixed keys (colours, continents, nations, competitions, awards, clubs) set to -1 and then to the database row by name (from eng.lng or from the database); the ones left without a value go to the "Unable to find the %s index" box (7 fatal by nation); in cm0102-1080.exe through the 0x603834 entry, with -1 over the whole ranges |

Mutations: `mutacoes/leva238.py` (5, by the save proof): 4 caught. `dbw_continentes` escaped: the continent keys (0x9cfa08..0x9cfa1c) decide nothing in the .qst creation nor in the load to 2001-08-28; Asia and Europe swapped did not change the save. (gap in `PENDENCIAS.en.md`) Save proof (the .qst with and without the routines): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 239th batch — game creation: initialising the players and the staff (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `inicializar_jogadores()` | 0x535a80 | cdecl | Database.cpp: prepares every person of the new game: missing names, dates, staff attributes, nation; for the player the abilities, potential, current ability, age, positions and missing attributes (from the most similar of 250 random players); for the non-player the reputations, attributes (template among 250), job and the famous names; then the ability-relative vectors and the normalisations; "Player Initialisation" bar |

Mutations: `mutacoes/leva239.py` (5, by the save proof): 5 caught.  Save proof (the .qst with and without the routines): EQUAL (16856054 bytes). Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 240th batch — the nationality adjective by language and form (official on 2026-10-26)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `adjetivo_da_nacao()` | 0x6181d0 | cdecl | index.cpp: the nation's adjective into the text (size >= 50): with gender 0 or outside languages 9, 4, 5, 10 and 6 the nation's name; in those, the form from 0x545a60 and the chains of 937 nations (generated from the exe by adjetivos_nacao.py, texts by address), with the name if the text stays empty (except in languages 4 and 5); returns 1 (0 with a null nation or text or a short size) |

Mutations: `mutacoes/leva240.py` (5, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 3 caught. `dbx_forma` escaped: form 5 (min(5) → min(4)) only appears in the Italian and Spanish tables, which the English game does not use; also escaped the real game comparing (0 divergences). `dbx_vazio` escaped: the name at the end only changes something in languages 4 and 5 (Italian, Spanish); also escaped the real game comparing (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 241st batch — the league table: sorting, the head-to-head mini table, freeing and checking completion; the league season end: the TV rights, the season honours and the competition history record; the competition helpers: the league clash type, the club strength score and the latest placed club; competitions: the arranged friendly of a club and the fixture list for a date; competitions: the next fixture by filter, the stadium in use on a day and the international friendly date (official on 2026-10-27)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `ordenar_tabela_da_liga()` | 0x6827d0 | thiscall | league.cpp: sorts the table by the rules (+0xc5): rule 6 only sorts (0x683210); rule 4 builds the mini table and, when first, sorts by head-to-head (0x683410) and bubbles the ties; stores the position (+4) and the position history (+9); returns 1 |
| `mini_tabela_do_confronto()` | 0x682b30 | thiscall | league.cpp: mini table of the clubs level on points with their summed head-to-head results, sorted by 0x6834f0; the position (+4) starts at the club count and drops at each separated pair |
| `liberar_tabela_da_liga()` | 0x687970 | thiscall | league.cpp: frees the +5 and +9 row buffers and the table (null: the league one, then clears +0x3e and +0xb1) |
| `liga_completa()` | 0x689ad0 | thiscall | league.cpp: 1 when every club played the +0xa7 rounds (and the stages, if asked); at the end of the league, the records (0x86a070), the prize money (0x682e10), the notice (+0x94) and +0xdd |
| `pagar_direitos_de_tv()` | 0x683010 | thiscall | league.cpp: adds the amount to the TV revenue of every club in the table (with a filter, only those with +0x5b equal) and sends the news (0x48ea40) |
| `registrar_honras_da_temporada()` | 0x684640 | thiscall | league.cpp: champion, runner-up and third into the history (0x4afce0), with the host nation team as the featured entry when the competition asks (+0x88, 0x5fa730, 0x53b3d0); a champion of this division clears +0x5b of the clubs pointing at it |
| `acrescentar_ao_historico()` | 0x4afce0 | cdecl | comp_util.cpp: stores competition, year and four values in the free row of the competition history (without one, grows 20 rows by 0x527f60); returns the row |
| `tipo_de_confronto_na_liga()` | 0x4b02e0 | cdecl | comp_util.cpp: 1 = title clash, 3 = zone clash, 0 = none, between two clubs of the same league, from the weighted statistics ratings (0x4a4850) against a third of the clubs or 0.2 × clubs + +0xbe (+ +0xbf) |
| `forca_do_clube()` | 0x4c0720 | cdecl | comp_util.cpp: money (+0x65 / 400 or balance / 300) + 16 × reputation + the off-match ratings (0x6e6340) of the 15 picked (0x6e2000) from the squad; null club: −10,000,000 |
| `clube_colocado_mais_recente()` | 0x4afd80 | cdecl | comp_util.cpp: sorts the competition history and returns the winner, runner-up or third (+0xa, +0xe, +0x12) of the first row of the competition; not found: 0 |
| `amistoso_do_clube()` | 0x5c88f0 | thiscall, ret 8 | friendly.cpp: the friendly of the club (+0xc) in the friendly list (+0x34c8): with a date, those on that date (0x4a7f20), the complete one (+0x10 and +0x14) or the second found at once, otherwise the last kept; without a date, the first incomplete one with +8 |
| `lista_da_data()` | 0x5ae6a0 | stdcall, ret 0xc | fix_man.cpp: the fixture list (0x5abf10 of the manager [0xae2a58]) for the year (+2) and day (+0) of the date, in the given slice and group |
| `proxima_partida()` | 0x5ac920 | thiscall, ret 0x2c | fix_man.cpp: the next (or previous) fixture from the year/day/slice cursor, inclusive, by the competition (+0), stage (+0x34) and two club (+0xc/+0x10) filters, each -1 = any; up to 5 years; writes the cursor back and returns the fixture, or 0 |
| `estadio_em_uso()` | 0x5ae870 | thiscall, ret 0x10 | fix_man.cpp: 1 when a fixture (+0x18) in the day lists (groups 0 to 7 and the stadium one, 0x48df40) uses the stadium, in the given slice or in slices 0 to 2 (slice -1) |
| `data_do_amistoso_internacional()` | 0x5cbdd0 | thiscall, ret 4 | friendly.cpp: the date (+3) through 0x54a130 from the day (+1), month (+2), given year and weekday (+0); the high half of the arguments comes from the year (the caller eax) |

Mutations: `mutacoes/leva241.py` (15, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 11 caught. `cpa_ordem_posicao` escaped: ordenar_tabela_da_liga 0x6827d0: disabled routine (DESLIGADAS in motor.rs): no hook in any mode, the copy does not run. `cpa_completa_rodadas` escaped: 0x689ad0: disabled routine (DESLIGADAS in motor.rs): no hook in any mode, the copy does not run (disabled on 26/10/2026). `cpa_tv_valor` escaped: pagar_direitos_de_tv 0x683010 is not called before 2002-06-01 (the "liga direitos de tv" branch does not appear in the base); also escaped the game comparing and the pt load (0 divergences). `cpa_confronto_titulo` escaped: tipo_de_confronto_na_liga 0x4b02e0: disabled routine (DESLIGADAS in motor.rs): no hook in any mode, the copy does not run. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 242nd batch — national teams: ending the call-ups of a competition and pruning the call-up records; national teams: adding to and removing from the squad and releasing the over-age under-21 players; national teams: the current competition of each nation and calling up a replacement; national teams: the call-up pools of each nation and their two orderings; friendlies: the friendly opponent and the tour opponent; the league table: adding a result and the points for a result; the league table: the order by stored head-to-head, the date order, the club row and whether the league has the club (official on 2026-10-27)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `encerrar_convocacoes_da_competicao()` | 0x774f30 | thiscall, ret 4 | national_teams.cpp: in the call-up record list, those of the competition (+4) end tomorrow (+0x10, the day from 0x54c5b0) |
| `encerrar_convocacoes_da_nacao()` | 0x774f90 | thiscall, ret 8 | national_teams.cpp: like 0x774f30, only the records of the competition (+4) and the nation (+0) |
| `podar_registros_de_convocacao()` | 0x7774b0 | thiscall | national_teams.cpp: in each club record (+0x20, 6 bytes), drops the expired entries (0x456770) at the start, moves the rest down (0x777590) and frees the emptied buffer |
| `por_no_elenco_da_selecao()` | 0x776250 | thiscall, ret 0xc | national_teams.cpp: the first of the 50 free squad slots of the team (+0xd7; type 0/1 the senior team, else the under-21) takes the person, who gets the team (+0x24), today's date (+0x29), mark 0xb (+0x28) and squad bit 4 or 8 (0x540980) |
| `tirar_do_elenco_da_selecao()` | 0x7762f0 | thiscall, ret 0xc | national_teams.cpp: removes the person from the team tactics (0x896c60) and, if found in a slot, empties it and clears +0x24, +0x28 and the national bits of +0x5e |
| `dispensar_acima_da_idade_do_sub21()` | 0x76d880 | thiscall, ret 4 | national_teams.cpp: once the nation record date (+0x62) has passed, removes from the under-21 squad everyone older than 21 (+0x18) and clears record +0x6b |
| `limpar_competicao_atual()` | 0x775000 | thiscall, ret 4 | national_teams.cpp: the nation current competition (record +4) becomes −1 and, if there is a next one ([this+8][id]), that one takes its place, notifying the managers (0x6a3770) |
| `liberar_competicoes_do_continente()` | 0x775070 | thiscall, ret 8 | national_teams.cpp: in the nations of the continent (+0x71), the current and next competitions that ended (competition +0x59 null, or stage [0x9cfa10]) become −1; empty current and a next one: the next moves up, with the notice (0x6a3770) |
| `convocar_substituto()` | 0x775f90 | thiscall, ret 0x10 | national_teams.cpp: in the pool of the level, the first eligible player of the nation who plays a position of the one leaving (0x5b7d20) is called up (0x776250, leaving another squad first through 0x7762f0) with the news (0x76fd10); nobody: clears the type slot count (+0x45) |
| `montar_reservas_das_selecoes()` | 0x76abf0 | thiscall | national_teams.cpp: clears the 4 levels of each nation record and puts every eligible person in the nation pool (and the under-21 one, up to 21 years and under 5 caps), also by the second nationality; full section: enters by rating, and sorts (qsort with 0x778ed0) |
| `ordem_das_reservas()` | 0x778ed0 | cdecl | national_teams.cpp: the pool qsort order: rating (+9 of the player data) descending |
| `ordem_pelo_clube()` | 0x778ef0 | cdecl | national_teams.cpp: the order by club (+0x39): no club last, clubs by id, within a club squad bit 4 before 8 (+0x5e) |
| `adversario_do_amistoso()` | 0x5c4cb0 | thiscall, ret 0x14 | friendly.cpp: searches (bsearch) the candidates of the band (1 to 20, going up until found) and, from there, the first one that is not the club, excluded, already an opponent, out by date, of crossed bands, a rival, of a rival nation (half the time), from the same city or above the drawn band, that passes the nation check by the club reputation and is drawn (1 in 5); else 0 |
| `adversario_da_excursao()` | 0x5c5040 | thiscall, ret 0x14 | friendly.cpp: searches (bsearch) the candidates of the tour nation and, from its first one, the one not above the maximum band nor out by date, the club itself, already an opponent, a rival, of a rival nation (half the time), from the same city or above the drawn band, drawn half the time; stops at the first one of another nation; else 0 |
| `aplicar_resultado_na_tabela()` | 0x685f60 | thiscall, ret 0x18 | league.cpp: finds the home and away rows and adds the game at home and away (played, goals, win/draw/loss, by extra time and penalties, marking +0x3d/+0x3e), the win points by the +0x90 virtual and the draw points +0xc3; redoes the totals, returns both rows points and adds the head-to-head (0x6869a0) |
| `pontos_do_resultado()` | 0x68aad0 | thiscall, ret 8 | league.cpp: the win points (+0xc2) to the winner, 0 to the loser; when level, by extra time (+0xab bit 2), by penalties (bits 0x21: the draw points +0xc3 to the winner) or the draw points |
| `ordem_pela_posicao_do_confronto()` | 0x683410 | cdecl | league.cpp: qsort callback: the own compare, the points, the club id with no games, the stored head-to-head position (+4) and the name |
| `ordem_das_datas_da_liga()` | 0x685cf0 | cdecl | league.cpp: bsearch callback: the year (+2, unsigned) and the day (+0): −1, 0 or 1 |
| `linha_do_clube()` | 0x68ab70 | thiscall, ret 8 | league.cpp: the club row in the given rows (null: the league table), or 0 |
| `liga_tem_o_clube()` | 0x68abd0 | thiscall, ret 4 | league.cpp: 1 when the club has a row in the league table |

Mutations: `mutacoes/leva242.py` (20, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 16 caught. `cpd_atual_aviso` escaped: no nation with a next competition before 2002-06-01 (the "seleção a competição seguinte" branch is not in the base); also escaped the game comparing and the pt load (0 divergences). `cpd_continente_aviso` escaped: the next competition becoming the current one in the continent does not happen before 2002-06-01 (the "seleção continente a seguinte vira a atual" branch is not in the base); also escaped the game comparing and the pt load (0 divergences). `cpe_datas_ano` escaped: ordem_das_datas_da_liga 0x685cf0: in the career searches the year always ties, only the day decides; also escaped the game comparing and the pt load (0 divergences). `cpe_linha_do_clube` escaped: linha_do_clube 0x68ab70 (31 direct calls): the row found changes nothing the prints and the comparator see before 2002-06-01; also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 243rd batch — club records: the copies of the rating, man of the match and discipline records; club records: the copy of the count and fans player records and the construction of the tally and count records; club records: the copies of the transfer fee record and the goals in a match record; the copies of the news item and of the past and future transfers; the copy of the regen queue entry, the shortlist lookup and the choice of the non-player data donor; the copy of the ban record, the default contract wage and the player match condition; the staff contract: the days to the end date, clearing all grievances and checking the transfer request withdrawal (official on 2026-10-27)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `copiar_recorde_de_nota()` | 0x448230 | thiscall, ret 4 | club_records.cpp: copies the rating record (0x1f bytes): the rating (+0), the holder (+4, through 0x448490), +0x1c and +0x1d |
| `copiar_recorde_de_melhor_em_campo()` | 0x4484d0 | thiscall, ret 4 | club_records.cpp: copies the man of the match record (0x1b bytes): the count (+0), the holder (+1) and the year (+0x19) |
| `copiar_recorde_de_disciplina()` | 0x448500 | thiscall, ret 4 | club_records.cpp: copies the discipline record (0x1c bytes): yellows (+0), reds (+1), the holder (+2) and the year (+0x1a) |
| `copiar_recorde_de_contagem()` | 0x448530 | thiscall, ret 4 | club_records.cpp: copies the count record (0x1a bytes): the count (+0, word) and the holder (+2, through 0x448490) |
| `copiar_recorde_do_torcedor()` | 0x4485b0 | thiscall, ret 4 | club_records.cpp: copies the fans player record (0x1a bytes): the holder (+0) and the year (+0x18) |
| `construir_recorde_de_ocorrencias()` | 0x450c60 | thiscall | club_records.cpp: constructs the tally record: the holder date at +0x11 (0x54a110) |
| `construir_recorde_de_contagem()` | 0x450d30 | thiscall | club_records.cpp: constructs the count record: the holder date at +0x12 (0x54a110) |
| `copiar_recorde_de_valor()` | 0x448550 | thiscall, ret 4 | club_records.cpp: copies the transfer fee record (0x29 bytes): five dwords, the dates (+0x14, +0x20), +0x1c and the part-exchange flag (+0x28) |
| `copiar_recorde_de_gols_na_partida()` | 0x451570 | thiscall, ret 4 | club_records.cpp: copies the goals in a match record (0x38 bytes): the goals (+0), the holder (+1) and the match (+0x19) |
| `copiar_noticia()` | 0x786010 | thiscall, ret 4 | news.cpp: copies the news item (0xdf bytes): +0, +4, the 50 tokens (+5), the date (+0xcd), +0xd5, +0xd6, +0xda and +0xde |
| `copiar_transferencia_passada()` | 0x8cb940 | thiscall, ret 4 | transfer_manager.cpp: copies the past transfer (0x25 bytes): seven dwords, the byte +0x1c and the date (+0x1d) |
| `copiar_transferencia_futura()` | 0x8cbf40 | thiscall, ret 4 | transfer_manager.cpp: copies the future transfer (0xc bytes): the date (+0) and the offer index (+8) |
| `copiar_entrada_da_fila_de_regens()` | 0x7a8b70 | thiscall, ret 4 | player_regen.cpp: copies the regen queue entry (0xc bytes): the person (+0) and the regen date (+4) |
| `esta_na_lista_de_observados()` | 0x842080 | thiscall, ret 8 | shortlist_manager.cpp: is the (player, club or −1) pair in the shortlist pairs? bsearch with 0x757ea0 |
| `escolher_doador_de_dados()` | 0x7a6e60 | stdcall, ret 4 | player_regen.cpp: the non-player with the highest positive score (0x7a7150) of the nation; with none, of any nation; else 0 |
| `copiar_registro_de_suspensao()` | 0x556c70 | thiscall, ret 4 | discipline.cpp: copies the ban record (0x14 bytes): fields, the date (+4) and bits 0..2 of +0x13 |
| `salario_padrao_do_contrato()` | 0x8626e0 | thiscall, ret 4 | staff_contracts.cpp: the wage (+0xc) = the default for the person at the club (0x862720) truncated, minimum 55 |
| `condicao_de_jogo_do_jogador()` | 0x62c510 | thiscall, ret 8 | injury.cpp: updates the day fitness (0x62c680) and returns the match condition (out of 10000, or as a percentage) |
| `dias_ate_o_fim_do_contrato()` | 0x85fa00 | thiscall | staff_contracts.cpp: the days from the game date to the contract end date (+0x2d), through 0x54cdb0 |
| `limpar_todas_as_queixas()` | 0x867520 | thiscall, ret 4 | staff_contracts.cpp: clears the grievances (+0x45), the opinion links (0x549c90) and, when asked, checks the withdrawal (0x867240) |
| `conferir_retirada_do_pedido()` | 0x867240 | thiscall, ret 4 | staff_contracts.cpp: a player of the contract club with a transfer request, no staff criterion and no agreed contract: withdraws the request (0x4d9460), morale becomes +0x5b + +0x59 + 5, +0x3e = 0, +0x3d at least 15, and clears each grievance checking again |

Mutations: `mutacoes/leva243.py` (22, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 18 caught. `cpg_doador_zero` escaped: escolher_doador_de_dados 0x7a6e60: a score of exactly zero only changes the choice when no other person scores positive; does not happen before 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpg_salario_minimo` escaped: salario_padrao_do_contrato 0x8626e0: no default wage falls to 55 or below before 2002-06-01 (the minimum does not act); also escaped the game comparing and the pt load (0 divergences). `cpg_condicao_porcento` escaped: Injury::get_match_condition 0x62c510: disabled routine (DESLIGADAS in motor.rs, 27/10/2026): no hook in any mode, the copy does not run. `cph_queixas_zero` escaped: limpar_todas_as_queixas 0x867520: the three high grievance bytes (+0x46..+0x48) are already zero when the routine runs in the career; also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 244th batch — the player squad placement and the moves between the first team and the reserves; the staff contract copy, the morale after a good game and removing the contract talks; removing from sorted arrays: the media transfer interests, the scout request and the nickname from every human manager; retirements: removing the pending one and the unemployed decision; work permits: the offer that needs a permit and the yearly update; the search scout, the club current run, building the record holder and the rival manager who comments in the media (official on 2026-10-27)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `lugar_no_elenco()` | 0x4cbb60 | thiscall, ret 8 | contract_manager.cpp: 0 none, 1 reserves, 2 first team (bit 0 of +0x5e at his own club with reserves), 3 at the linked B side, 4 at the main club with a B side (through the partner club 0x540a50 and bit 6 of the contract +0x35) |
| `subir_para_o_principal()` | 0x4cc0b0 | thiscall, ret 8 | contract_manager.cpp: in the reserves or at the B side (placement 1 or 3), swaps through 0x4cbd20; else 0 |
| `descer_para_os_reservas()` | 0x4cc0f0 | thiscall, ret 8 | contract_manager.cpp: in the first team or at the club with a B side (placement 2 or 4), swaps through 0x4cbd20; else 0 |
| `copiar_contrato_de_pessoal()` | 0x4cc810 | thiscall, ret 4 | contract_manager.cpp: copies the contract (0x50 bytes) field by field, the dates through 0x417c10 and the bit fields +0x35 and +0x4f in steps |
| `subir_moral_pela_partida()` | 0x4cb0a0 | stdcall, ret 0xc | contract_manager.cpp: the morale gain from the rating against the club reputation (up to 10), morale between −100 and 100 and under the grievance cap; clears grievances 4 and 0x10 (0x8674e0) and checks the withdrawal (0x867240) |
| `tirar_conversas_de_contrato()` | 0x4ca1c0 | thiscall, ret 0xc | contract_manager.cpp: removes the person talks (and of the club and manager, when given) moving the last one into the gap; sorts again |
| `tirar_interesses_de_transferencia()` | 0x7549b0 | thiscall, ret 4 | media.cpp: removes the person interests (25 bytes) moving the last one into the gap; sorts again (0x757ea0) |
| `tirar_pedido_de_observacao()` | 0x7f8ee0 | thiscall, ret 8 | scout_manager.cpp: finds the request (club, person; 17 bytes), pulls the next ones up and sorts again (0x7fc270); null club = the current human manager club |
| `tirar_apelido_de_todos()` | 0x5fff30 | thiscall, ret 4 | human_manager.cpp: in each human manager, finds the person nickname (30 bytes), pulls the next ones up, blanks the last one and lowers the count; sorts the last manager array |
| `tirar_aposentadoria_pendente()` | 0x7a8690 | thiscall, ret 4 | player_regen.cpp: finds the pending one (26 bytes), pulls the next ones up (0x7a8780), blanks the last one (person −1) and lowers the count |
| `aposentadoria_do_desempregado()` | 0x7a8050 | stdcall, ret 8 | player_regen.cpp: outside the shortlist: the weak manager without a club (draw of 5), the player unemployed for over a year (draw by rating < 750) or whoever has no record retires: fills the entry (reason 0x10, the game date) and returns 1 |
| `oferta_precisa_de_permissao()` | 0x8c7470 | thiscall, ret 4 | transfer_manager.cpp: 1 when the offer player needs a permit at the offer club (0x8c7560) or one of the exchange players at his contract club; else 0 |
| `atualizar_permissoes_de_trabalho()` | 0x8c7330 | thiscall | transfer_manager.cpp: clears the flag of whoever lacks bit 0x20; after more than 4 years grants the second nationality and drops the permit; drops the permit of non-players; sorts and trims the dropped ones from the end |
| `olheiro_da_busca()` | 0x7ae950 | thiscall | player_search.cpp: the person at index +0x180 when it still matches the stored names and date (0x53e990); else 0 |
| `sequencia_atual_do_clube()` | 0x690c30 | thiscall, ret 0xc | manager_manager.cpp: the first run (types 1, 3, 4, 2) of more than 1 game with the last one under 10 days ago, from the club line in the league statistics; else 5 and 0 |
| `construir_recordista()` | 0x450d40 | thiscall | club_records.cpp: builds the record holder: only the date at +0x10 (0x54a110) |
| `tecnico_rival_para_comentar()` | 0x72b940 | thiscall, ret 4 | media.cpp: the manager of a rival (+0xb3/+0xb7/+0xbb) of the same league and much bigger, or of a drawn club in a league clash, who passes the draws; returns the id or −1 |

Mutations: `mutacoes/leva244.py` (21, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 11 caught. `cph_lugar_marca` escaped: squad placement 0x4cbb60: swapping 3 and 4 only matters for someone at the affiliate club with bit 6 of the contract; the result only tells the two cases apart on screens; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 434.096 calls); also escaped the game comparing and the pt load (0 divergences). `cph_conversas_clube` escaped: remove talks records 0x4ca1c0: no person has talks with two clubs at once when the routine is called with a club; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 741 calls); also escaped the game comparing and the pt load (0 divergences). `cph_interesse_ordem` escaped: remove transfer interest 0x7549b0: the array stays sorted after the swap with the last one, or nobody reads it before it is sorted again; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 2.372 calls); also escaped the game comparing and the pt load (0 divergences). `cph_pedido_data` escaped: remove scout request 0x7f8ee0: the date only shifts when the removed request is not the last one, and the shifted date is not read before 2002-06-01; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 741 calls); also escaped the game comparing and the pt load (0 divergences). `cph_apelido_fim` escaped: remove nickname from all 0x5fff30: the slot past the end of the array (count already decremented) is not read; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 248 calls); also escaped the game comparing and the pt load (0 divergences). `cph_pendente_fim` escaped: remove retirement 0x7a8690: the slot past the end of the array (count already decremented) is not read; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 263 calls); also escaped the game comparing and the pt load (0 divergences). `cph_desempregado_tecnico` escaped: unemployed retirement 0x7a8050: no unemployed manager rated exactly 49 hits the 1-in-5 draw; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 18.426 calls); also escaped the game comparing and the pt load (0 divergences). `cph_permissao_marca` escaped: update work permits 0x8c7330: the mark without bit 0x20 only has bit 0 set (clearing all of it or only bit 0 is the same); the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 48 calls); also escaped the game comparing and the pt load (0 divergences). `cph_olheiro_confere` escaped: scout of the search 0x7ae950: the slot of a search scout does not change person (regen) before 2002-06-01; the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 143 calls); also escaped the game comparing and the pt load (0 divergences). `cph_sequencia_dias` escaped: current run of the club 0x690c30: no last game of a run falls exactly 9 days back (and the result only picks the text of a news item); the routine runs in the career (call counters, batch 243 DLL, up to 2002-06-01: 15 calls); also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,368,478,169 draws, fingerprint b28542870027e432, matches 959a02ada4a23639, database ce037e0141221551

### Simulation, 245th batch — club records: the construction of the transfer fee paid and received records and of the run record; club records: the construction of the won, lost and unbeaten run records; club records: the construction of the winless run, goals in a match and youngest player records; the nation transfer rules: the deadline news text and the loan ending; the game.cpp pow; removing the held offers of a person, holding the bans of a loaned player and filling the vacated staff job; the news for the human managers of a nation, the search scout assignment and whether the cup is complete; the oldest player record and the work permits: finding the player permit and whether he passes the foreign player limit (official on 2026-10-29)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `construir_recorde_de_valor_pago()` | 0x4485d0 | thiscall | club_records.cpp: constructs the highest fee paid record: the dates at +0x14 and +0x20 (0x54a110) |
| `construir_recorde_de_valor_recebido()` | 0x448620 | thiscall | club_records.cpp: constructs the highest fee received record: the dates at +0x14 and +0x20 (0x54a110) |
| `construir_recorde_de_sequencia()` | 0x448670 | thiscall | club_records.cpp: constructs the run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_vitorias_seguidas()` | 0x450ad0 | thiscall | club_records.cpp: constructs the won run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_derrotas_seguidas()` | 0x450b20 | thiscall | club_records.cpp: constructs the lost run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_invencibilidade()` | 0x450b70 | thiscall | club_records.cpp: constructs the unbeaten run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_sem_vencer()` | 0x450bc0 | thiscall | club_records.cpp: constructs the winless run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_gols_na_partida()` | 0x450c10 | thiscall | club_records.cpp: constructs the goals in a match record: the holder date (+0x11) and the match date (+0x19) (0x54a110) |
| `construir_recorde_do_mais_novo()` | 0x450c70 | thiscall | club_records.cpp: constructs the youngest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110) |
| `texto_do_fim_do_prazo()` | 0x8e05a0 | thiscall, ret 0xc | transfer_manager.cpp: the deadline news text through the rules of the nation in news slot 0 (method +0x30) |
| `pode_encerrar_emprestimo()` | 0x8d0530 | thiscall, ret 0x14 | transfer_manager.cpp: with the player parent club in the same nation as the club, that nation rules answer (method +0x24); else 1 |
| `pow_do_game()` | 0x5d5a20 | cdecl | game.cpp: the game CRT pow (0x945ff0) of two doubles, result in st0 |
| `tirar_ofertas_guardadas()` | 0x4cf560 | thiscall, ret 4 | contract_manager.cpp: removes from the held offers list (+0x38) every item of the person (0x68d1f0) |
| `guardar_suspensoes_do_emprestado()` | 0x5563f0 | thiscall, ret 4 | discipline.cpp: the bans (slots 0 to 2) that do not apply at the loan club go to the held list (+0x1c), the slot becomes −1 and the +0x1c counter goes up |
| `preencher_vaga_da_comissao()` | 0x8cb6d0 | stdcall, ret 8 | transfer_manager.cpp: the first unemployed person of the club nation in the wanted job (6, 8, 9, 10; 13 → 6, 15 → 8) joins the club (0x53bd10) |
| `noticia_para_os_tecnicos_da_nacao()` | 0x786520 | thiscall, ret 8 | news.cpp: nation scope (+0xde = 2) and delivery (0x785d40) to every active human manager with a club of the nation; the first refusal stops |
| `missao_do_olheiro()` | 0x7aefd0 | thiscall, ret 0x18 | player_search.cpp: with the scout checked and some assignment set (+0x190, +0x18c, +0x194, +0x195, +0x199, +0x19d), copies the six and returns 1; else 0 |
| `copa_completa()` | 0x521e00 | thiscall, ret 8 | Cup.cpp: the cup started (+0xb1) and, asking for the stages with no stage set (+0x44), all up to the current complete and the current is the last |
| `construir_recorde_do_mais_velho()` | 0x450cd0 | thiscall | club_records.cpp: constructs the oldest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110) |
| `permissao_de_trabalho_de()` | 0x8c7260 | thiscall, ret 4 | transfer_manager.cpp: with the 0x20 flag of the id in [+0x86e], the binary search for the permit (0xe-byte entries in [+0x93a]); else 0 |
| `passa_na_permissao()` | 0x84b180 | stdcall, ret 0xc | shortlist_manager.cpp: without rule 0xe in the buyer nation, same nation, second nation or EU: yes; no national side: no; else the permit (0x8c7950) |

Mutations: `mutacoes/leva245.py` (21, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 5 caught. `cpi_vitorias_data` escaped: construir_recorde_de_vitorias_seguidas 0x450ad0: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_derrotas_data` escaped: construir_recorde_de_derrotas_seguidas 0x450b20: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_invencibilidade_data` escaped: construir_recorde_de_invencibilidade 0x450b70: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_sem_vencer_data` escaped: construir_recorde_sem_vencer 0x450bc0: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_gols_data` escaped: construir_recorde_de_gols_na_partida 0x450c10: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_mais_novo_idade` escaped: construir_recorde_do_mais_novo 0x450c70: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_prazo_vaga` escaped: texto_do_fim_do_prazo 0x8e05a0: slot 1 is another nation, with other rules, in every call (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 16 or more), but the rules method +0x30 only writes the news text, which the career does not read; also escaped the game comparing and the pt load (0 divergences). `cpi_emprestimo_nacao` escaped: pode_encerrar_emprestimo 0x8d0530: the loaned player parent club is in the same nation in every call (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 256 or more), but the rules method +0x24 of another nation answers the same up to 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpi_ofertas_pessoa` escaped: tirar_ofertas_guardadas 0x4cf560: the mutation removes the other people held offers (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 4,096 or more items of another person), but nobody reads the held offers list (+0x38) up to 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpi_suspensao_nacao` escaped: guardar_suspensoes_do_emprestado 0x5563f0: no ban without a competition arrives with the nation [+0x71] different from +0x1b (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 8 or more calls, 0 in that case); also escaped the game comparing and the pt load (0 divergences). `cpi_vaga_15` escaped: preencher_vaga_da_comissao 0x8cb6d0: no job 15 is vacated (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 128 or more vacancies, 0 of job 15); also escaped the game comparing and the pt load (0 divergences). `cpi_noticia_recusa` escaped: noticia_para_os_tecnicos_da_nacao 0x786520: no delivery is refused (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 128 or more news items, 0 refusals); also escaped the game comparing and the pt load (0 divergences). `cpi_missao_comp` escaped: missao_do_olheiro 0x7aefd0: no search has the competition set (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 4,096 or more calls, 0 with +0x195 above −1); also escaped the game comparing and the pt load (0 divergences). `cpi_copa_fase` escaped: copa_completa 0x521e00: no incomplete stage shows up in the loop (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 4,096 or more calls, 0 incomplete stages); also escaped the game comparing and the pt load (0 divergences). `cpj_mais_velho_data` escaped: construir_recorde_do_mais_velho 0x450cd0: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpj_permissao_marca` escaped: permissao_de_trabalho_de 0x8c7260: whoever lacks the 0x20 flag has no permit in the table: searching without checking the flag also gives 0 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 8,388,608 or more without the flag, none found); also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b

### Simulation, 245th batch — club records: the construction of the transfer fee paid and received records and of the run record; club records: the construction of the won, lost and unbeaten run records; club records: the construction of the winless run, goals in a match and youngest player records; the nation transfer rules: the deadline news text and the loan ending; the game.cpp pow; removing the held offers of a person, holding the bans of a loaned player and filling the vacated staff job; the news for the human managers of a nation, the search scout assignment and whether the cup is complete; the oldest player record and the work permits: finding the player permit and whether he passes the foreign player limit (official on 2026-10-29)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `construir_recorde_de_valor_pago()` | 0x4485d0 | thiscall | club_records.cpp: constructs the highest fee paid record: the dates at +0x14 and +0x20 (0x54a110) |
| `construir_recorde_de_valor_recebido()` | 0x448620 | thiscall | club_records.cpp: constructs the highest fee received record: the dates at +0x14 and +0x20 (0x54a110) |
| `construir_recorde_de_sequencia()` | 0x448670 | thiscall | club_records.cpp: constructs the run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_vitorias_seguidas()` | 0x450ad0 | thiscall | club_records.cpp: constructs the won run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_derrotas_seguidas()` | 0x450b20 | thiscall | club_records.cpp: constructs the lost run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_invencibilidade()` | 0x450b70 | thiscall | club_records.cpp: constructs the unbeaten run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_sem_vencer()` | 0x450bc0 | thiscall | club_records.cpp: constructs the winless run record: the start (+4) and end (+0xc) dates (0x54a110) |
| `construir_recorde_de_gols_na_partida()` | 0x450c10 | thiscall | club_records.cpp: constructs the goals in a match record: the holder date (+0x11) and the match date (+0x19) (0x54a110) |
| `construir_recorde_do_mais_novo()` | 0x450c70 | thiscall | club_records.cpp: constructs the youngest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110) |
| `texto_do_fim_do_prazo()` | 0x8e05a0 | thiscall, ret 0xc | transfer_manager.cpp: the deadline news text through the rules of the nation in news slot 0 (method +0x30) |
| `pode_encerrar_emprestimo()` | 0x8d0530 | thiscall, ret 0x14 | transfer_manager.cpp: with the player parent club in the same nation as the club, that nation rules answer (method +0x24); else 1 |
| `pow_do_game()` | 0x5d5a20 | cdecl | game.cpp: the game CRT pow (0x945ff0) of two doubles, result in st0 |
| `tirar_ofertas_guardadas()` | 0x4cf560 | thiscall, ret 4 | contract_manager.cpp: removes from the held offers list (+0x38) every item of the person (0x68d1f0) |
| `guardar_suspensoes_do_emprestado()` | 0x5563f0 | thiscall, ret 4 | discipline.cpp: the bans (slots 0 to 2) that do not apply at the loan club go to the held list (+0x1c), the slot becomes −1 and the +0x1c counter goes up |
| `preencher_vaga_da_comissao()` | 0x8cb6d0 | stdcall, ret 8 | transfer_manager.cpp: the first unemployed person of the club nation in the wanted job (6, 8, 9, 10; 13 → 6, 15 → 8) joins the club (0x53bd10) |
| `noticia_para_os_tecnicos_da_nacao()` | 0x786520 | thiscall, ret 8 | news.cpp: nation scope (+0xde = 2) and delivery (0x785d40) to every active human manager with a club of the nation; the first refusal stops |
| `missao_do_olheiro()` | 0x7aefd0 | thiscall, ret 0x18 | player_search.cpp: with the scout checked and some assignment set (+0x190, +0x18c, +0x194, +0x195, +0x199, +0x19d), copies the six and returns 1; else 0 |
| `copa_completa()` | 0x521e00 | thiscall, ret 8 | Cup.cpp: the cup started (+0xb1) and, asking for the stages with no stage set (+0x44), all up to the current complete and the current is the last |
| `construir_recorde_do_mais_velho()` | 0x450cd0 | thiscall | club_records.cpp: constructs the oldest player record: the age (+0), the holder date (+0x18) and the match date (+0x20) (0x54a110) |
| `permissao_de_trabalho_de()` | 0x8c7260 | thiscall, ret 4 | transfer_manager.cpp: with the 0x20 flag of the id in [+0x86e], the binary search for the permit (0xe-byte entries in [+0x93a]); else 0 |
| `passa_na_permissao()` | 0x84b180 | stdcall, ret 0xc | shortlist_manager.cpp: without rule 0xe in the buyer nation, same nation, second nation or EU: yes; no national side: no; else the permit (0x8c7950) |

Mutations: `mutacoes/leva245.py` (21, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 5 caught. `cpi_vitorias_data` escaped: construir_recorde_de_vitorias_seguidas 0x450ad0: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_derrotas_data` escaped: construir_recorde_de_derrotas_seguidas 0x450b20: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_invencibilidade_data` escaped: construir_recorde_de_invencibilidade 0x450b70: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_sem_vencer_data` escaped: construir_recorde_sem_vencer 0x450bc0: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_gols_data` escaped: construir_recorde_de_gols_na_partida 0x450c10: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_mais_novo_idade` escaped: construir_recorde_do_mais_novo 0x450c70: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpi_prazo_vaga` escaped: texto_do_fim_do_prazo 0x8e05a0: slot 1 is another nation, with other rules, in every call (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 16 or more), but the rules method +0x30 only writes the news text, which the career does not read; also escaped the game comparing and the pt load (0 divergences). `cpi_emprestimo_nacao` escaped: pode_encerrar_emprestimo 0x8d0530: the loaned player parent club is in the same nation in every call (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 256 or more), but the rules method +0x24 of another nation answers the same up to 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpi_ofertas_pessoa` escaped: tirar_ofertas_guardadas 0x4cf560: the mutation removes the other people held offers (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 4,096 or more items of another person), but nobody reads the held offers list (+0x38) up to 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpi_suspensao_nacao` escaped: guardar_suspensoes_do_emprestado 0x5563f0: no ban without a competition arrives with the nation [+0x71] different from +0x1b (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 8 or more calls, 0 in that case); also escaped the game comparing and the pt load (0 divergences). `cpi_vaga_15` escaped: preencher_vaga_da_comissao 0x8cb6d0: no job 15 is vacated (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 128 or more vacancies, 0 of job 15); also escaped the game comparing and the pt load (0 divergences). `cpi_noticia_recusa` escaped: noticia_para_os_tecnicos_da_nacao 0x786520: no delivery is refused (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 128 or more news items, 0 refusals); also escaped the game comparing and the pt load (0 divergences). `cpi_missao_comp` escaped: missao_do_olheiro 0x7aefd0: no search has the competition set (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 4,096 or more calls, 0 with +0x195 above −1); also escaped the game comparing and the pt load (0 divergences). `cpi_copa_fase` escaped: copa_completa 0x521e00: no incomplete stage shows up in the loop (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 4,096 or more calls, 0 incomplete stages); also escaped the game comparing and the pt load (0 divergences). `cpj_mais_velho_data` escaped: construir_recorde_do_mais_velho 0x450cd0: the routine does not run in the career to 2002-06-01 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpj_permissao_marca` escaped: permissao_de_trabalho_de 0x8c7260: whoever lacks the 0x20 flag has no permit in the table: searching without checking the flag also gives 0 (counters in the batch 245 DLL, career to 2002-06-01, same be54a138 fingerprint: 8,388,608 or more without the flag, none found); also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b

### Simulation, 246th batch — the pending lists: dropping the held bans, removing the pending offers of a person and marking the decision after an agreed transfer; the news for the human managers: for all, for those of the division and for those of the nation (club or national side); the offer that breaks the fee paid record, the player condition and the match selection limit; clearing the watched fixtures the humans may not watch, the club scout targets and changing the search scout; after the match, the board and the discipline; the club shortlist entries and the highest offer for a player (official on 2026-10-29)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `soltar_suspensoes_guardadas()` | 0x5567d0 | thiscall, ret 4 | discipline.cpp: removes from the held list (+0x1c) the items of the person while the +0x1c counter of her record is positive |
| `tirar_ofertas_pendentes()` | 0x4ccbf0 | thiscall, ret 4 | contract_manager.cpp: with pending offers (+8 of the 79-byte record), unmarks the agreed contract (0x8c5430) and removes the person offers (+0x34) until the byte is zero |
| `marcar_decisao_da_transferencia()` | 0x4cc930 | thiscall, ret 4 | contract_manager.cpp: marks (+0x62) the first offer of the person and club still undecided and schedules the decision (0x4cd230); returns ability / 500 > club reputation |
| `noticia_para_todos_os_tecnicos()` | 0x786780 | thiscall, ret 4 | news.cpp: all scope (+0xde = 4) and delivery (0x785d40) to every active human manager; 1, or 0 on the first refusal |
| `noticia_para_os_tecnicos_da_divisao()` | 0x7862d0 | thiscall, ret 8 | news.cpp: division scope (+0xde = 1) and delivery to every active human manager with a club in the division (+0x57); 1, or 0 on the first refusal |
| `noticia_para_os_tecnicos_do_pais()` | 0x7863f0 | thiscall, ret 8 | news.cpp: nation scope (+0xde = 2) and delivery to every active human manager whose club or national side belongs to the nation (+0x53); returns 0 |
| `oferta_bate_o_recorde()` | 0x4486c0 | thiscall, ret 8 | club_records.cpp: the offer fee passes 50 × reputation + 500,000 (and reputation² × 0x967880 when required) and the record (+0x313) of the buyer all-time records |
| `condicao_do_jogador()` | 0x62c590 | thiscall, ret 8 | injury.cpp: fitness × condition / 10,000 (minimum 100, maximum 9000 while recovering), or as a percentage (maximum 100) |
| `limite_de_escalacao()` | 0x8cf7e0 | thiscall, ret 8 | transfer_manager.cpp: from the match restriction table (0x8cf6b0), the first one present: type at +0, count at +5 (and the age at +1 for type 3) |
| `limpar_partidas_assistidas()` | 0x5fe8d0 | thiscall | human_manager.cpp: clears (79 bytes) the watched fixture of each active human manager who may no longer watch it (0x5fe700); returns 1 |
| `alvos_dos_olheiros_do_clube()` | 0x7f9690 | thiscall, ret 8 | scout_manager.cpp: empties the list and adds the person of each scout request of the club; returns whether there is any |
| `trocar_olheiro_da_busca()` | 0x7ae8a0 | thiscall, ret 4 | player_search.cpp: new scout: stores the ids (0x7e4920) or −1, and the nation, region and competition assignments go back to −1; returns 1 |
| `diretoria_depois_da_partida()` | 0x6951e0 | thiscall, ret 8 | manager_manager.cpp: played match: the board confidence of each side with a manager changes (0x695240, home with 1) |
| `disciplina_da_partida()` | 0x55ca00 | thiscall, ret 8 | discipline.cpp: the competition ruling body judges the match (0x7e7ff0); without it, in friendlies, the league ruling bodies of both clubs |
| `entradas_do_clube_na_shortlist()` | 0x8420e0 | thiscall, ret 8 | shortlist_manager.cpp: adds to the list every shortlist entry (8 bytes) of the club, found by binary search |
| `maior_oferta_pelo_jogador()` | 0x8cc930 | thiscall, ret 8 | transfer_manager.cpp: of the offers for the player from the club (or flagged) in state 2, the highest (0x8e7a70); 0 when none |

Mutations: `mutacoes/leva246.py` (16, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 5 caught. `cpj_suspensoes_desconto` escaped: soltar_suspensoes_guardadas 0x5567d0: the routine does not run in the career to 2002-06-01 (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpj_pendentes_fim` escaped: tirar_ofertas_pendentes 0x4ccbf0: the routine does not run in the career to 2002-06-01 (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls); also escaped the game comparing and the pt load (0 divergences). `cpj_decisao_500` escaped: marcar_decisao_da_transferencia 0x4cc930: dividing by 400 never changes the comparison (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 256 or more calls, 0 change); also escaped the game comparing and the pt load (0 divergences). `cpj_noticia_selecao` escaped: noticia_para_os_tecnicos_do_pais 0x7863f0: no human manager belongs to the nation only through the national side (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 64 or more managers seen, 0 only through the national side); also escaped the game comparing and the pt load (0 divergences). `cpj_noticia_alcance` escaped: noticia_para_todos_os_tecnicos 0x786780: the news goes to the same managers; only the stored scope (+0xde) changes, and the career does not read it up to 2002-06-01 (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 8 or more calls); also escaped the game comparing and the pt load (0 divergences). `cpj_recorde_minimo` escaped: oferta_bate_o_recorde 0x4486c0: no offer falls between the two reputation minimums (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 32 or more calls, 0 between 2×(25×rep+200,000) and 2×(25×rep+250,000)); also escaped the game comparing and the pt load (0 divergences). `cpj_assistida_79` escaped: limpar_partidas_assistidas 0x5fe8d0: the 79th byte of every cleared watched match is already zero (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 512 or more cleared, 0 with the byte set); also escaped the game comparing and the pt load (0 divergences). `cpj_alvos_clube` escaped: alvos_dos_olheiros_do_clube 0x7f9690: the scouting request list is empty whenever it is walked (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 requests seen); also escaped the game comparing and the pt load (0 divergences). `cpj_olheiro_mesmo` escaped: trocar_olheiro_da_busca 0x7ae8a0: with the same scout (4 or more of 8 or more calls, counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint) the mutation rewrites the same ids and sets the nation, region and competition assignments to −1, which the career does not tell apart up to 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpj_disciplina_liga` escaped: disciplina_da_partida 0x55ca00: the next league tribunal is a different one in 512 or more of the 2,048 or more judgements through the league (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint), but it judges friendlies the same up to 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpj_oferta_empate` escaped: maior_oferta_pelo_jogador 0x8cc930: no value tie among the eligible offers (counters in the batch 246 DLL, career to 2002-06-01, same be54a138 fingerprint: 4 or more offers, 0 ties); also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b

### Simulation, 247th batch — club records: finding or creating the all-time and season records and queueing the broken record news; the regen lists: the spare non-player record, the pending retirement and the regen slot; contracts and suspension: the relegation clauses of a relegated club, removing the contract offer of a person to a club and whether the player is suspended for the match; the match rules and the friendly: the scope of the match selection rules, the reason a player cannot play and the slot of a scheduled friendly; the news for the human managers of a continent and of the competition region, and whether the news is important for the person; the text of a news slot, the news of a club in financial trouble and whether the club scouts cover a nation; a person's fluency in a language, rewinding a person's history and the position of a formation slot (official on 2026-10-29)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `recordes_de_sempre()` | 0x452710 | thiscall, ret 8 | club_records.cpp: the all-time records of the club (index +0 of the 0x6a-byte entry, 0x38b-byte vector at +4); with create, the new index is the count (+0xc), the vector grows by 50 and the record is reset (0x452fe0) |
| `recordes_da_temporada()` | 0x4528d0 | thiscall, ret 8 | club_records.cpp: the same for the season records (index +4, vector +0x10, capacity +0x14, count +0x18) |
| `guardar_noticia_de_recorde()` | 0x452cb0 | thiscall, ret 8 | club_records.cpp: appends (club id, record type) to the 6-byte record news vector (+0x24), growing by 5; without memory, error box and 0 |
| `acrescentar_reserva_de_tecnico()` | 0x7a81a0 | thiscall, ret 0xc | player_regen.cpp: appends (id, nation, second nation) to the 16-byte spare non-player list (+0x1c); grows by 250 when the player pool (+0x40/+0x44) is full, as the original |
| `acrescentar_aposentadoria()` | 0x7a8510 | thiscall, ret 4 | player_regen.cpp: appends the record fields to the 26-byte pending retirement table (+4), growing by 250 |
| `guardar_vaga_para_regen()` | 0x7a87d0 | thiscall, ret 4 | player_regen.cpp: the person enters the regen queue (+0x10) with the dates; the non-player data in the spare list (+0x1c, random nation if none) and the player data in the free pool (+0x3c) |
| `acionar_clausulas_de_rebaixamento()` | 0x4cd030 | thiscall, ret 8 | contract_manager.cpp: in the relegated club and its A/B partner, the 50 at +0xd7 with a pending relegation clause (+0x1f = 1) go to 2 and the news is posted (0x8e3db0, type 3) |
| `tirar_oferta_de_contrato()` | 0x4cca70 | thiscall, ret 8 | contract_manager.cpp: the first offer of the person to the club (+0x34): returns the record reservation (0x7a81a0), unmarks the agreed one (+0x50 = 2), leaves the list and the pending count drops by one |
| `esta_suspenso()` | 0x55d270 | thiscall, ret 0x10 | discipline.cpp: suspended by the general body (0x7e85b0 with 1) or by the competition one (+0x50, with 0); in that case, in a human club, he leaves his slot among the 20 at +0x1e0 |
| `escopo_das_regras_da_partida()` | 0x8d2fa0 | thiscall, ret 0x10 | transfer_manager.cpp: reserves: the home nation and type 1; friendly: 0xff; otherwise the key nation of the competition (0x669470), the continent (+0x59) and type 4 (0, 1), 2 (3 to 7) or 1 |
| `codigo_de_inelegibilidade()` | 0x8cf5e0 | thiscall, ret 8 | transfer_manager.cpp: 9 for an AWOL player (outside national teams); otherwise the first of the 28 rules (+0x8ac) of the continent or nation with the same type answers (virtual +0x18); none: 0 |
| `vaga_do_amistoso()` | 0x5c6740 | thiscall, ret 0x10 | friendly.cpp: in the friendlies list (+0x34cc), the 25-byte slot of the club with the requested day, year and opponent, or 0 |
| `noticia_para_os_tecnicos_do_continente()` | 0x786640 | thiscall, ret 8 | news.cpp: continent scope (+0xde = 3) and delivery to every active human manager whose club or nation is in the continent |
| `noticia_para_a_regiao_da_competicao()` | 0x786860 | thiscall, ret 8 | news.cpp: to the managers of the competition's nation (0x786520), else of its continent (0x786640), else to all (0x786780) |
| `noticia_importante_para_a_pessoa()` | 0x784eb0 | thiscall, ret 4 | news.cpp: whether the news is important for the person, by category and type (bids on the person's club, the humans...) |
| `texto_do_dado_da_noticia()` | 0x785a20 | thiscall, ret 0xc | news.cpp: copies the text stored from the slot (+5 + 4 × slot) into the buffer (strncpy) and clears the last byte; returns 1 |
| `noticia_de_apuro_financeiro()` | 0x74ae60 | thiscall, ret 4 | media.cpp: news 0x1790 (id, reputation, draw of 2) to the managers of the nation, or to all for a big club (9000+) |
| `olheiros_cobrem_a_nacao()` | 0x7f9190 | thiscall, ret 8 | scout_manager.cpp: poor (balance < 1,000,000) and small (reputation < 5000) club outside the nation's region: 0; else 1 |
| `fluencia_no_idioma()` | 0x7c6830 | thiscall, ret 0x10 | player_stats.cpp: from 0 to 10: the contract level, 10 for a nation (or the second one) speaking the language, or the learned language (binary search) |
| `recomecar_historico_da_pessoa()` | 0x86fb80 | thiscall, ret 4 | staff_records.cpp: the last offset of the person's history list (+4 of the pair at [+0xc] + 8 × id) = −1 (0x58f110) |
| `posicao_na_formacao()` | 0x5b2ba0 | thiscall, ret 0xc | formation.cpp: stores the general or extra position of the slot, the marks, and recomputes the pitch cells without and with the ball (0x5b7e60) |

Mutations: `mutacoes/leva247.py` (20, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 13 caught. `cpk_noticia_recorde_conta` escaped: guardar_noticia_de_recorde 0x452cb0: with the count not rising the record news are not issued; they only go to the news inboxes, which the career does not read up to 2002-06-01 (counters in the batch 247 DLL, career to 2002-06-01, same be54a138 fingerprint: 64 or more calls, 32 or more with the count already above 0); also escaped the game comparing and the pt load (0 divergences). `cpk_aposentadoria_conta` escaped: registrar_aposentadoria 0x7a8510: the routine is in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run. `cpk_oferta_pendente` escaped: tirar_oferta_de_contrato 0x4cca70: the person's pending count (+8 of the 79-byte record) stays at 1 instead of 0, which the career does not tell apart up to 2002-06-01 (counters in the batch 247 DLL, career to 2002-06-01, same be54a138 fingerprint: 1,024 or more calls, all with the byte at 1 or more); also escaped the game comparing and the pt load (0 divergences). `cpk_importante_oferta` escaped: noticia_importante_para_a_pessoa 0x784eb0: no news of type 0xfa1 is delivered up to 2002-06-01 (counters in the batch 247 DLL, career to 2002-06-01, same be54a138 fingerprint: 0 calls in the branch); also escaped the game comparing and the pt load (0 divergences). `cpk_apuro_sorteio` escaped: noticia_de_apuro_financeiro 0x74ae60: the routine is in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run. `cpk_olheiro_regiao` escaped: olheiros_cobrem_a_nacao 0x7f9190: every call past the gate (balance below 1,000,000 and reputation below 5000) is for a nation outside the region, and the mutation returns 1 instead of 0; the callers (the player search 0x7b7490 and the scouts 0x7f8ff0) do not change the career up to 2002-06-01 with it (counters in the batch 247 DLL, career to 2002-06-01, same be54a138 fingerprint: 256 or more calls, 256 or more past the gate, 256 or more outside the region); also escaped the game comparing and the pt load (0 divergences). `cpk_historico_ultimo` escaped: recomecar_historico_da_pessoa 0x86fb80: both offsets of the pair are already −1 on every call: writing −1 to the first instead of the second gives the same pair (counters in the batch 247 DLL, career to 2002-06-01, same be54a138 fingerprint: 128 or more calls, all with both at −1); also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b

### Simulation, 248th batch — offers for a player: whether a club has an offer for him, the club offer and whether he has a pending offer; player offers and contract: whether he is under offer, whether his club follows the Bosman rule and the board veto on the offer; transfer records: storing the past transfer, granting the work permit and storing the first-team appearance money; staff contracts and decision dates: finding the contract slot, giving the new person a contract and scheduling the decision date; staff contracts: building the offer from the current contract, resetting the new contract's morale and the expiry band; the media story log: logging a story, expiring the old ones and whether a story was already told; the media transfer interests: adding an interest and the comparisons of id keys and stories (official on 2026-10-29)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `clube_tem_oferta_pelo_jogador()` | 0x8cdc90 | thiscall, ret 8 | transfer_manager.cpp: 1 when the person flagged as under offer (bit 0 of [+0x86e] + id × 8) has, among the open offers (0x8c5cf0), one from the club (offer +8); else 0 |
| `oferta_do_clube_pelo_jogador()` | 0x8cdd80 | thiscall, ret 0xc | transfer_manager.cpp: the club offer for the person (with `all`, closed ones too and without requiring the flag), or 0 |
| `jogador_com_oferta_pendente()` | 0x8cd150 | thiscall, ret 4 | transfer_manager.cpp: 1 when the person is under offer (0x8cde70) and an open offer is in state (+0x2c) 0, 1, 5 or 6; else 0 |
| `jogador_sob_oferta()` | 0x8cde70 | thiscall, ret 8 | transfer_manager.cpp: without the human view, the "under offer" flag (bit 0 of [+0x86e] + id × 8); with it, 1 when an offer is known (+0x2d), comes from a club of the current human or is for a player of his club, when the person is on his shortlist or when he has an accepted contract offer for the person |
| `clube_do_jogador_com_lei_bosman()` | 0x8d0a70 | stdcall, ret 4 | transfer_manager.cpp: 1 when the nation of the club of the person contract has free transfers at contract expiry (0x544670); without contract, club or nation: 1 |
| `veto_da_diretoria_na_oferta()` | 0x8cdb80 | thiscall, ret 0x18 | transfer_manager.cpp: with an empty exchange part, the deciding club board answers (0x696b10, by the 0x8e7b50 value); 0x2000: reason 0x24 and the player loses 10% of his reputations (+0xb current, +9 home, +0xd world); 0x800: 0x17; 0x200: 0x18; each returns 1 |
| `guardar_transferencia_passada()` | 0x8cb810 | thiscall, ret 4 | transfer_manager.cpp: appends the past transfer (0x25 bytes, 0x8cb940) to the +0x862 vector (capacity +0x866, count +0x86a), growing by 100; returns the new record |
| `dar_permissao_de_trabalho()` | 0x8c7070 | thiscall, ret 0x14 | transfer_manager.cpp: appends the permit (14 bytes: person, year +8, nation +0xa) to the +0x93a vector (growing by 30), sets the person 0x20 flag at +0x86e and, when asked, sorts (0x8e55c0) |
| `guardar_premio_por_jogo_no_time()` | 0x8cac40 | thiscall, ret 4 | transfer_manager.cpp: appends the appearance money (13 bytes) to the +0x88c vector (growing by 50) and sorts by person (0x8b4e00) |
| `vaga_livre_de_contrato()` | 0x4c79a0 | thiscall, ret 4 | contract_manager.cpp: the slot in the 0x50-byte contract vector: the first empty one (id −1) or, append only, a new one (grows by 200; without memory, error box and −1) |
| `acrescentar_contrato_da_pessoa()` | 0x4c7950 | thiscall, ret 8 | contract_manager.cpp: the appended slot for a new person (hired manager, regen): links the person to the slot in [0xae2d0c], stores the id and generates the initial contract from the value (0x85e960) |
| `acrescentar_data_de_decisao()` | 0x4cd6d0 | thiscall, ret 0x10 | contract_manager.cpp: appends (person, date, time slice, type) to the 14-byte decision date vector (grows by 100); type 1 first adds type 2 reminders from the contract morale |
| `zerar_moral_do_contrato()` | 0x866ee0 | thiscall, ret 8 | staff_contracts.cpp: clears the contract morale, grievances and happiness, the person's opinion links (0x549c90), sets +0x4e to 4 and the club nation language level (0x7c6830, up to 10) in +0x4f |
| `contrato_pelo_atual()` | 0x85e590 | thiscall, ret 0x18 | staff_contracts.cpp: the offer from the current contract: person, club, wage × percent / 100, today, the end date, flags, morale reset as 0x866ee0 and the two clauses; without contract, error box |
| `faixa_do_vencimento()` | 0x867d30 | thiscall | staff_contracts.cpp: the 0..=14 band of the contract end date against today and the CONTRACT_MANAGER limit dates (0xae1010..0xae1090) |
| `registrar_historia_da_midia()` | 0x757a50 | thiscall, ret 0x14 | media.cpp: appends the story (today, subject ids or −1, type, subtype, response) and sorts again (0x757ee0); grows by 20 |
| `expirar_historias_da_midia()` | 0x757c70 | thiscall, ret 0 | media.cpp: removes stories older than 100 days (team ones, 0xbdf, older than 40 + random(40)) moving the last one into the gap; sorts again |
| `historia_ja_registrada()` | 0x757da0 | thiscall, ret 0x14 | media.cpp: team story: linear search by second subject id and subtype; the others: binary search by the key and, with a date, whether it is not older |
| `acrescentar_interesse_de_transferencia()` | 0x754740 | thiscall, ret 0x14 | media.cpp: if the {player, club} key is missing, appends the interest (today, fee up to 50 million, valuation × 100) and sorts again; grows by 20 |
| `ordem_das_chaves_de_ids()` | 0x757ea0 | cdecl | media.cpp: null first; by the +0 id and, when equal, by +4 (−1 matches anything) |
| `ordem_das_historias()` | 0x757ee0 | cdecl | media.cpp: null first; by subjects +8 and +0xc, type +0x13 and subtype +0x11 |

Mutations: `mutacoes/leva248.py` (19, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 18 caught. `cpl_contrato_valor` escaped: acrescentar_contrato_da_pessoa 0x4c7950: half the value passed to 0x85e960 (the initial contract) does not change the career up to 2002-06-01 (counters in the batch 248 DLL, career to 2002-06-01, same be54a138 fingerprint: 128 or more calls, all with a club and value above 1); in 0x85e960 the value only enters through conditional branches (the type 2 competition via vtables +0x98/+0x9c and the comparison at 0x85ef76), which in the career do not run or end the same; the exact cause was not isolated; also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b

### Simulation, 249th batch — the cup: who goes through in the match and in the tie and the seed order in the draw; the league table: setting up the club row, the club array and the final positions; the club division move and the shuffles of the club lists; discipline: counting a served match of a ban, releasing a ban and clearing a person's ban history; injuries: rolling a recurring injury, rewinding the injury history and resetting a player's fitness; club records: adding the transfer fee to the sale and purchase totals, the club's career appearances and goals records and the season rating record; friendlies: whether a competition is current on the date, whether the club already has a friendly near the date and adding the planned friendly (official on 2026-10-29)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `decidir_jogo_da_copa()` | 0x51c020 | thiscall, ret 4 | Cup.cpp: who goes through in a cup match (1 home, 2 away, 0 nobody) by the score, extra time, penalties, walkover and points of the rule (+0x38), storing each side's code (+0x3d/+0x3e) |
| `decidir_confronto_da_copa()` | 0x51f2f0 | thiscall, ret 8 | Cup.cpp: in the first leg marks the winner and returns 3; in the second leg decides by aggregate, walkover, extra time, away goals and penalties (1 or 2 the side through, 4 undecided) |
| `ordem_dos_cabecas_de_chave()` | 0x51ebc0 | cdecl | Cup.cpp: the qsort comparison of the 7-byte draw entries: the higher seed (+6) first |
| `iniciar_linha_da_tabela()` | 0x687430 | thiscall, ret 0x10 | league.cpp: puts the club in table row i: totals cleared, status 0xff, flag, total rounds (+0xa7) if still −1, position history and, under rule 4, the head-to-head entries (the last row fills every row's opponents) |
| `vetor_dos_clubes_da_liga()` | 0x687a20 | thiscall, ret 8 | league.cpp: a new array with the table clubs and the count; without memory, error box and count 0 |
| `gravar_posicoes_finais_da_liga()` | 0x68aa30 | thiscall | league.cpp: each club still in the league division stores last season's division (+0x5b) and the final position (+0x5f) |
| `mudar_clube_de_divisao()` | 0x6830b0 | thiscall, ret 0xc | league.cpp: the current division becomes last season's and the new one is set; clears the division clauses (and the squad numbers when the new one uses them), opens the requested manager vacancy, upgrades the stadium to the league minimum and turns status 3 into 2 |
| `embaralhar_ponteiros()` | 0x4afb40 | cdecl | comp_util.cpp: shuffles an array of n pointers with 3 × n swaps of drawn pairs |
| `ordenar_e_embaralhar_os_melhores()` | 0x4afbd0 | cdecl | comp_util.cpp: sorts the clubs by reputation (nulls last) and shuffles the first min(3 × picks, n) |
| `contar_jogo_da_suspensao()` | 0x556e90 | thiscall, ret 0xc | discipline.cpp: the person's ban in the rule's ruling body: match bans, from the start date (+4/+6) and when counting, +1 to the counter (+1); on reaching the total (+0), or day bans once today reaches start + days (+2, 0x54c5b0), it goes to the history (0x58e340) and is released (0x556d00) |
| `soltar_suspensao()` | 0x556d00 | thiscall, ret 8 | discipline.cpp: each ruling body of the ban's 4 rules (+0xc..+0xf) forgets it (0x7e8590); on the fourth, the person's item leaves the nation index (last one moved in, qsort); the ban becomes free (+0 = 0xff, +2 = −1) |
| `apagar_historico_de_suspensoes()` | 0x55e1d0 | thiscall, ret 4 | discipline.cpp: the last offset of the person's ban history (+0x18 of the 29-byte record) = −1 (0x58f110) |
| `sortear_lesao_recorrente()` | 0x62e500 | thiscall, ret 8 | injury.cpp: with 1 in 20, a recurring-capable injury becomes recurring in the record (+0x12 type, +0x13 = 0); among the recurring records, the best score (roll 0x65 − 5 × level − proneness, stops at 95): a player at a big club (reputation > 4250) without a human and 1 in 3 gets a new injury 0x4d/0x4e/0x4f (0x62d250); otherwise his mark is cleared; in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run |
| `recomecar_historico_de_lesoes()` | 0x631f90 | thiscall, ret 4 | injury.cpp: the last offset of the person's injury history (+0x18 of the 31-byte record) = −1 (0x58f110) |
| `reiniciar_fisico_do_jogador()` | 0x62bfa0 | thiscall, ret 4 | injury.cpp: removes the id items from list +0x28, ends the injury (0x62e700), rewinds the history (0x631f90) and the 31-byte record (today, condition and form 10000, wear 0, no injury nor recurrence); bit 0 of +0xc of the 79-byte record is cleared |
| `somar_totais_de_transferencia()` | 0x448ad0 | thiscall, ret 0xc | club_records.cpp: adds the fee to the seller's season income and the buyer's season spending, raises the all-time records and queues the record news (0x1f, 0x1e) above (+0x80 + 500) × 1000 |
| `recordes_de_carreira_no_clube()` | 0x448cb0 | thiscall, ret 0x10 | club_records.cpp: the club's all-time league appearances (+0x247) and goals (+0x261) records, with the player reference (0x7e4920), when the record type applies to the club (0x449590) |
| `recorde_de_nota_da_temporada()` | 0x452a90 | thiscall, ret 4 | club_records.cpp: the squad player with the best average rating (statistic 0x11) among those who played at least half the games becomes the season rating record |
| `competicao_na_data_do_amistoso()` | 0x5c29b0 | thiscall, ret 8 | friendly.cpp: returns 1 for an ordinary competition; for a friendly tournament, only when the date is within 7 days of the slot date |
| `amistoso_perto_da_data()` | 0x5c45a0 | thiscall (this sem uso), ret 8 | friendly.cpp: returns 1 when the club already has a planned friendly on the date, the day before or the day after |
| `acrescentar_amistoso_planejado()` | 0x5c4b60 | thiscall, ret 0x10 | friendly.cpp: appends the friendly (date and three ids) to the 0x14-byte array, which grows by 100 |

Mutations: `mutacoes/leva249.py` (16, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 14 caught. `cpm_confronto_adversario` escaped: iniciar_linha_da_tabela 0x687430: with the club itself in place of the opponent, the head-to-head tiebreak (League::rank_by_head_to_head 0x682b30, which finds the slot by the opponent id) finds no slot and leaves the order to the next rule; the change only applies to tables built during the career (32 or more last rows with rule 4, counters in the batch 249 DLL) and up to 2002-06-01 no tie decided by it changed the career; in the game comparing the comparator runs only the original; the exact cause was not isolated. `cpm_vetor_conta` escaped: vetor_dos_clubes_da_liga 0x687a20: the only caller is the nation manager of the year award (0x768940, nation_awards.cpp, through vtable +0xa0), which sorts the clubs by manager points (0x86fab0) and only goes on with 3 candidates; without the last club of the table (8 or more calls) the candidates are the same and the award does not enter the career. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b

### Simulation, 250th batch — news delivery: by scope, to the club manager and into the human manager's inbox; the human manager's inbox: replying to every item, whether an item is pending and the text in an item slot; the media and the inbox: a player's rating for a story, the financial trouble text and the human's inbox items; the retiring player leaves the squads and the choice of the youngster's club: the best scored among all and the drawn one from his nation's list; manager jobs and offer scheduling: opening the missing jobs, the best manager candidate score and scheduling the offer; the club shortlist: the squad status the club gives the person, the human club estimate and releasing the targets that are not the club's; the scout search and the friendlies: whether the search covers the nation, removing the scheduled friendly and the nation pairs that play no friendlies (official on 2026-10-29)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `noticia_pelo_alcance()` | 0x786090 | thiscall, ret 0x18 | news.cpp: delivers the news item by scope: 0 the club manager, 1 the division's managers, 2 the country's, 3 the continent's, 4 all, 5 the competition region's; anything else shows the "Error" box |
| `noticia_ao_tecnico_do_clube()` | 0x7861e0 | thiscall, ret 8 | news.cpp: marks the scope as club (+0xde = 0) and delivers the item to the club manager (+0xcf) when he is human (among the last 16 people); in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run |
| `entregar_noticia_ao_tecnico()` | 0x785d40 | thiscall, ret 0xc | news.cpp: copies the item into the human's 100-slot ring (created on the first, reallocating the array), gives it an id, skips a repeat of the last one and, with the manager on holiday, applies the automatic reply for the club and the national team; in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run |
| `responder_todas_as_noticias()` | 0x7874b0 | thiscall, ret 8 | news.cpp: from the newest to the oldest item of the human's inbox, up to the empty slot, applies the automatic reply (0x784100) for the club |
| `tecnico_tem_noticia_pendente()` | 0x787350 | thiscall, ret 8 | news.cpp: without "urgent only", whether there is an unread item; with it, whether an item needs a reply (the text returns 2) or is important and unread |
| `gravar_texto_na_noticia()` | 0x7858c0 | thiscall, ret 0xc | news.cpp: copies the text into the item slots from the given slot and returns the first free slot after it (at most 50) |
| `nota_do_jogador_para_a_historia()` | 0x736720 | thiscall, ret 0x18, st0 | media.cpp: the player's rating for a story kind: stats and the average of ratings against the league average, the weight, the games and the reputation; then, by kind, the contract, age, days at the club and attributes |
| `texto_da_crise_financeira()` | 0x74af60 | thiscall, ret 0xc | media.cpp: the headline or body of the club in financial trouble story, by severity, with the club name linked |
| `noticias_da_caixa()` | 0x786960 | thiscall, ret 0xc | news.cpp: a new array with the human's inbox items, newest first, only those of one category when asked, and the count |
| `aposentar_do_elenco()` | 0x7aace0 | stdcall, ret 4 | player_regen.cpp: removes the player from the tactics and the 50 slots of his club (+0x39) and national team (+0x24) and changes the player job to the staff job (12 → 5, 13 → 6, 15 → 8, the others 0x10 retired); without a kept job, clears the club and the national team |
| `melhor_clube_para_o_jovem()` | 0x7ad0e0 | thiscall, ret 4 | player_regen.cpp: among the clubs where the youngster is not a foreigner, that are not a B team, that may approach him and with a squad under 50, the best scored one (0x7ad220) |
| `clube_da_nacao_para_o_jovem()` | 0x7ad590 | thiscall, ret 4 | player_regen.cpp: in the club list of the youngster's nation (some nations mapped to [0x9cf2e4]), the one with the highest draw of the score by reputation against ability, among those that may approach him and with a squad under 50 |
| `abrir_vagas_que_faltam()` | 0x69d060 | thiscall, ret 0 | manager_manager.cpp: each club without a manager (0x53ca90), manageable (0x544820), not a B team and without an entry in the [this+8] list gets the "Job not found" note and a job vacancy (0x69ba50) |
| `melhor_nota_de_candidato_a_tecnico()` | 0x69c540 | thiscall, ret 8 | manager_manager.cpp: the highest manager score (0x69d1b0, job 5) among the available staff accepted for the club (nation, continent, reputation, contract, 0x69f810), ignoring the current manager; nobody: −10000 |
| `agendar_oferta()` | 0x8c6220 | thiscall, ret 0xc | transfer_manager.cpp: gives the offer today's date + days and the day slice (0 to 2, every 3 slices one more day) and puts it in the bucket (days since the base date + start, modulo 50, × 3 + slice) of the +0x18 lists |
| `estatuto_no_elenco_pelo_clube()` | 0x849f00 | thiscall, ret 0xc | shortlist_manager.cpp: the squad status the club gives the person: from her entry in the club shortlist (bits 1..4 of +0x24, job from +0), from the contract with the club (high nibble of +0x4f), from the human club estimate, or 7 |
| `estimar_estatuto_no_elenco()` | 0x849d60 | stdcall, ret 8 | shortlist_manager.cpp: the estimate of the status the human club gives: the ratio between the person's reputation seen by the manager (1.5 × when on the human shortlist) and the club's, and the age |
| `liberar_alvos_da_lista_do_clube()` | 0x844660 | thiscall, ret 8 | shortlist_manager.cpp: in the club's 40 slots, a target with an invalid id becomes −1 and a target not at the club (contract nor +0x39) and without +0x17 gets shortlist event 3 (0x843e10) |
| `busca_cobre_a_nacao()` | 0x7b7490 | thiscall, ret 4 | player_search.cpp: whether the scout search (type 4, scout checked, with a club) covers the nation: by the set assignments (club, nation, region, base) or, without one, by the region of the club's nation, the scout's nation and the club's scouts |
| `tirar_amistoso_marcado()` | 0x5c87b0 | thiscall, ret 8 | friendly.cpp: removes the match's items from the +0x34d4 list, clears the scheduled slot (with clear_linked) and removes the match from the +0x34c8 list (freeing it) |
| `iniciar_pares_proibidos_de_amistoso()` | 0x5cc2d0 | thiscall | friendly.cpp: allocates the 4 nation pairs that play no friendlies with each other (Iran and USA, Iraq and anyone, Vietnam and USA, the Koreas) at +0x3502, count at +0x3506 |

Mutations: `mutacoes/leva250.py` (17, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 15 caught. `cpn_responder_todas` escaped: responder_todas_as_noticias 0x7874b0: in the automatic career it only runs when the human goes on holiday at the start (counter: 64 passes, the news already in their inbox); the automatic replies to those news (0x784100) change nothing the career reads (same fingerprint with and without them, and with only the newest); in the game comparing the comparator runs only the original and checks the return value. `cpn_responder_so_a_nova` escaped: responder_todas_as_noticias 0x7874b0: in the automatic career it only runs when the human goes on holiday at the start (counter: 64 passes, the news already in their inbox); the automatic replies to those news (0x784100) change nothing the career reads (same fingerprint with and without them, and with only the newest); in the game comparing the comparator runs only the original and checks the return value. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b

### Simulation, 251st batch — board confidence: the transfer deal, the confidence after the transfer and the ultimatum; contracts: removing the main contract, resubmitting the held offers and the month of the club's season end; manager points: the turn of the month and the year, the end of the competition season and the manager of the month; leaving clubs: the managers' pending departures, removing the player from the squad and removing the person from the job; the new news item and the supporters' and board's confidence after a media story; the AI manager's response to a media story, marking a news item read and finding a news item by id; the player's news after a media story: gratitude to the manager, request for backing and reaction to criticism; the shortlist position lists: building the lists, sorting them every day and the clubs interested in the player; the shortlist and loans: adding the pair to the shortlist, predicting the selling club's response and the loan period; transfers: the offer response delay, paying the month's instalments and removing the offer; bans on a club move: adding a ban, dropping the slots that do not apply at the new club and restoring the held bans when a loan ends; the player's languages (adding and improving) and resetting a regenerated player's statistics; national teams: the nation's current competition and the clubs playing on a date; discipline: removing a regenerated player (official on 2026-10-30)

| Routine | Original | Convention | Notes |
|---|---|---|---|
| `avaliar_o_negocio_da_transferencia()` | 0x696570 | thiscall, ret 0xc | manager_manager.cpp: flags the deal for buyer and seller by the fee/player value ratio (bands for values above or up to 250,000, a fee up to (reputation × 0.2)² is fair), adds and subtracts the value from each one's net spend and flags the club's and chairman's favoured or disliked player |
| `confianca_da_diretoria_na_transferencia()` | 0x6978f0 | thiscall, ret 4 | manager_manager.cpp: changes the board confidence (+6, 1..10000) and moods (+0x14, +0x16) of the buyer and the seller from the deal flags and removes the player from the seller chairman's favoured people |
| `resolver_o_ultimato_da_diretoria()` | 0x6ac470 | thiscall, ret 4 | manager_manager.cpp: removes the club from the ultimatum list and, when the manager has been in the job over 100 days, queues his departure (reason 2, or 7 without games in the table) and returns 1 |
| `tirar_o_contrato_principal()` | 0x4d0b50 | thiscall, ret 4 | contract_manager.cpp: frees the person's main contract entry (−1), puts −1 in her slot of the 79-byte record and recalculates the wage bill of the contract's club |
| `reenviar_ofertas_guardadas()` | 0x4cf4a0 | thiscall, ret 4 | contract_manager.cpp: resubmits the person's held offers as live offers (with the transfer of the same club, state 0xf becomes 8; without a transfer, with −1), removes each one from the list and returns how many |
| `mes_do_fim_da_temporada_do_clube()` | 0x4d45f0 | stdcall, ret 4 | contract_manager.cpp: the month (0..11) of the date ten days after the end of the season of the club's nation |
| `pontos_dos_tecnicos_no_dia()` | 0x86dda0 | thiscall, ret 4 | staff_records.cpp: at the turn of the month or year clears every manager's monthly points and stores the month and year; on the first day of the year passes the yearly points to the hall of fame and clears them |
| `pontos_dos_tecnicos_no_fim_da_competicao()` | 0x86d6e0 | thiscall, ret 4 | staff_records.cpp: at the end of the competition's season passes its managers' league points (with the cup points of the same competition) and cup points to the hall of fame and clears them |
| `tecnico_do_mes_na_liga()` | 0x86f950 | thiscall, ret 4 | staff_records.cpp: the league manager with the most points this month (above 0) among those two weeks in the job; a league without a table: the "Error" box and 0 |
| `saidas_pendentes_dos_tecnicos()` | 0x69b650 | thiscall, ret 0 | manager_manager.cpp: walks the pending departure list and, at each club with a manager, carries out his departure and, when the club opens a vacancy, the vacancy; removes each item from the list |
| `tirar_jogador_do_elenco_do_clube()` | 0x690730 | thiscall, ret 8 | manager_manager.cpp: takes the player out of the club (or the national team): a player-manager job goes back to the staff job (else 0xb and he leaves), out of the lists, the media, the tactics and the 50 squad slots; the loan gets state 4 |
| `tirar_pessoa_do_cargo_no_clube()` | 0x690890 | thiscall, ret 8 | manager_manager.cpp: removes the contract and the person from the job slot at the club or national team (chairman, director, manager, assistant, coach, scout, physio; the playing jobs become 0xb), from the media and, if human, from the human's lists |
| `construir_noticia()` | 0x77bbe0 | thiscall, ret 8 | news.cpp: builds the news item: type (+0), important byte (+4), the 50 data slots cleared, today's date (+0xcd), byte [0xae2c88] at +0xd5, id and +0xda cleared; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run |
| `ajustar_confianca_da_torcida()` | 0x738340 | thiscall, ret 0x14 | media.cpp: adds to the supporters' confidence (+8 of the club row in [[0xb67a44]], 1..10000) the target minus the current confidence, × 0.75, damped by a favoured or rejected manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager |
| `ajustar_confianca_da_diretoria()` | 0x7386f0 | thiscall, ret 0x18 | media.cpp: adds to the board's confidence (+6 of the club row) (confidence − 6000)/20 + target, × 0.75, damped by a chairman who likes or dislikes the manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager with the subject's flags |
| `resposta_do_tecnico_da_maquina()` | 0x735870 | thiscall, ret 0xc | media.cpp: the AI manager's response 1, 2 or 3 to a story of the given type about the player: by the contract, the player's story rating (× 5), the manager's likes and dislikes and chains of draws per family of types; some types add the player to the manager's rejected list |
| `marcar_noticia_lida()` | 0x787140 | thiscall, ret 0xc | news.cpp: marks read the first unread item with the id in the human's ring unless it needs a reply (button 0x782b70), takes it off the count and, when asked, repeats it for the humans on the same network machine |
| `achar_noticia_pelo_id()` | 0x786b70 | thiscall, ret 0xc | news.cpp: copies the item with the id to the output: in the human manager's 100-slot ring or, without a manager, among all slots |
| `noticia_de_gratidao_do_jogador()` | 0x732c80 | thiscall, ret 0x10 | media.cpp: news 0xbda to the manager: the player, the club, the type (0x12/0x21 by a draw for a player on the media list or with a flagged contract, 0xc when injured), the variant, the type's draw (which may raise +0x44 of the player record), the manager and the assistant |
| `noticia_de_pedido_de_apoio()` | 0x743370 | thiscall, ret 0x10 | media.cpp: for a human club, news 0xbcf to the manager: the player, the club, the reputation band and the request level by attributes, the type, the fourth argument and whether he is transfer-listed |
| `noticia_de_reacao_a_critica()` | 0x743a50 | thiscall, ret 0x10 | media.cpp: news 0xbdc: the player, the club, the reaction (0..4 by reputation, contract category, age, attributes and the response), the response, board confidence below 4000, the topic and the manager, delivered to the managers of the interested clubs (0x8d1da0) |
| `montar_listas_de_posicao()` | 0x839e60 | thiscall, ret 4 | shortlist_manager.cpp: rebuilds the 32 lists (16 slots × 2) at 12 bytes per entry: the size is the largest number of people fitting a slot (0x844300); each non-human person enters every slot he fits with his id, the nation id (drawn when missing) and the value or the contract +0x3d |
| `ordenar_listas_de_posicao()` | 0x83a780 | thiscall, ret 0 | shortlist_manager.cpp: recomputes the entry keys (the value when the contract band is 0, minus 1,500,000 for a listed player of a human club; in the second list −1000, 2000 or the contract +0x3d) and sorts both arrays of each slot (0x945333 with 0x83b670) |
| `clubes_interessados_no_jogador()` | 0x842180 | thiscall, ret 0x14 | shortlist_manager.cpp: empties the output list and adds the (player, club) pairs: the humans who have the player on the scout shortlist and the clubs in the by-person array that are not suspended, are not his and value him above his value |
| `acrescentar_na_lista_de_observacao()` | 0x8423f0 | thiscall, ret 8 | shortlist_manager.cpp: creates both pair arrays (capacity 100) and, when the pair is new: grows by 100, adds and sorts (by person, by club) and, outside a human club, adds 1 to the low nibble of +0xb of the person's 79-byte record |
| `prever_resposta_do_vendedor()` | 0x84af00 | stdcall, ret 0x20 | shortlist_manager.cpp: 0 accepts, 1 haggles, 2 rejects: from the contract with the seller, the loan case (listed, reputation, status), the price (human, clause, holiday, asking or 0x4d1600) against the offer |
| `periodo_do_emprestimo()` | 0x8d0590 | thiscall, ret 0x1c | transfer_manager.cpp: the period from the club nation's rules and, with a type 2 contract, moves the end back month by month until it no longer passes the contract end (the start follows); without a contract, expired, too far or before this year: the default year 1972; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run |
| `prazo_da_resposta_a_oferta()` | 0x8cc5b0 | thiscall, ret 0xc | transfer_manager.cpp: from the time until the window closes (the rules of the buyer's and the player club's nations): 0 days, 1 or 2 days with drawn slices, or a value from the player's interest (0x846920) or drawn from the status; days = value / 3, slices = value % 3 |
| `pagar_prestacoes_das_transferencias()` | 0x8caee0 | thiscall, ret 0 | transfer_manager.cpp: pays each instalment (payer's and receiver's finances), adds the fractions to the instalment figure (+0x1c) in the last 8, removes the finished ones and zeroes the figure of clubs without an instalment in its direction |
| `tirar_oferta_de_transferencia()` | 0x8d0f60 | thiscall, ret 4 | transfer_manager.cpp: destroys the linked offer, removes the pair from the loan list and the contract offers of the exchange and of the player, lowers the player's (+0x86e) and the manager's offer counts, the lowest free slot follows and the offer is destroyed |
| `acrescentar_suspensao()` | 0x556940 | thiscall, ret 8 | discipline.cpp: when the person already has a ban under one of the 4 rules, extends the old one (matches or days) and flags both; otherwise copies the new ban into the vector's first free slot (or the end, growing by 100 up to 0x7d00), links each rule to it (0x7e8510) and, with the fourth rule, adds {id, nation} to the nation index (qsort) |
| `tirar_suspensoes_de_fora_do_clube()` | 0x556270 | thiscall, ret 0xc | discipline.cpp: on a club move, frees each of the person's 4 record slots whose ruling body (nation +0x17 or continent +0x1b) does not cover the new club: releases the ban, frees the slot and the entry; with held bans, shows the "Error" box |
| `restaurar_suspensoes_guardadas()` | 0x556570 | thiscall, ret 4 | discipline.cpp: when a loan ends, puts the person's held bans back into their slots (freeing the occupied slot first; the item leaves the list) and frees slots 0..2 not restored whose ruling body does not cover the contract club |
| `acrescentar_idioma()` | 0x7c6a30 | thiscall, ret 0x10 | player_stats.cpp: appends the new entry to the language table (6 bytes: id, language, fluency up to 10; starts at 100, grows by 100) and sorts it when asked (qsort) |
| `melhorar_idioma()` | 0x7c6ca0 | thiscall, ret 0x10 | player_stats.cpp: finds the person's entry for the language (bsearch) and adds the increase to the fluency, up to 10; without one, adds the language with the increase as fluency |
| `zerar_estatisticas_do_jogador()` | 0x7c6650 | thiscall, ret 4 | player_stats.cpp: frees the player's 6 category records (+0xe = −1, +1 to the free count), clears the league totals and recent ratings and empties the history; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run |
| `definir_competicao_da_selecao()` | 0x775220 | thiscall, ret 8 | national_teams.cpp: makes the competition the nation's current one; with a major finals under way, the new one goes to the queue; when the new one is a major finals, the current one goes to the queue; notifies the manager handler (0x6a3770) |
| `marcar_clubes_com_jogo()` | 0x7775c0 | thiscall, ret 4 | national_teams.cpp: in the one-byte-per-club table, marks the clubs of the non-friendly fixtures on the date: 1 the club's league game, 2 another, 3 a club of a flagged nation (entry with no nation); in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run |
| `tirar_jogador_da_disciplina()` | 0x556830 | thiscall, ret 4 | discipline.cpp: clears the ban history, releases the bans and frees the 4 slots, drops the held bans from the list and leaves the 29-byte record free (0xff, count 0) |

Mutations: `mutacoes/leva251.py` (33, by the career to 2002-06-01, confirming the escaped ones in comparison mode): 31 caught. `cpo_tecnico_recente` escaped: the branch for a manager hired less than two weeks ago ran in the base, but his points for the month never beat the best in the league, so dropping the rule does not change the pick; the comparator (same return as the original) and the career confirm it. `cpo_tirar_ofertas_pessoa` escaped: the byte +1 of the person's 8-byte record ([+0x86e]) is only a shortcut in the highest offer search (0x8cc930): left above zero, the search walks the vector, finds no offer for the person and returns 0 as the shortcut would; the career does not change. (gap in `PENDENCIAS.en.md`) Real game comparing to 2002-01-01: 0 divergences (2f65fd6b68d1027b). Rust ×2 and original careers to 2002-06-01: 2,378,003,110 draws, fingerprint be54a1386f51073a, matches 8832310884e7ef66, database da73be6ffdbb550b
