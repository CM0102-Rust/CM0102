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
