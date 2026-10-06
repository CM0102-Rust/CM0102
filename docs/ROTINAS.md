# Rotinas migradas para Rust

Cada rotina entra por desvio (`rust/src/desvios.rs`) e só vira oficial depois de:
1. autoteste (`CM_AUTOTESTE=1`): entradas sintéticas logo que o jogo abre o vídeo, original × Rust, comparando resultado e memória tocada;
2. mutação de controle: um defeito proposital na versão Rust tem de ser acusado;
3. jogo real em modo comparação (`CM_COMPARAR=1`): carreira automática de férias (`docs/VERSAO_SEM_TELA.md`) ou partida do Lucas;
4. carreira inteira: `simular.py` com `--modo rust` e `--modo original` (mesma DLL, mesma semente) tem de dar a mesma impressão digital.

## Módulos

### Sorteio — `rust/src/aleatorio.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção |
|---|---|---|
| `aleatorio(n)` | 0x90cfc0 | cdecl |

Autoteste: 5.000.000 casos, 0 divergências. Mutações acusadas: +4 no ramo de n grande; volta da tabela. Jogo real: >100 milhões de chamadas, 0 divergências.

### Datas — `si/code/Date.cpp` → `rust/src/data.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção |
|---|---|---|
| `Data()` | 0x54a110 | thiscall |
| `Data(dia, ano)` | 0x549ef0 | thiscall |
| `Data(dia_do_mes, mes, ano, dia_da_semana)` | 0x549f70 | thiscall, ret 0x10 |
| `dia_da_semana()` | 0x54a3f0 | thiscall |
| `ajustar_semana(alvo, dest)` | 0x54a2d0 | thiscall |
| `operator=(src)` → cópia | 0x417c10 | thiscall, retorno por valor |
| `operator+(n dias)` | 0x54c5b0 | thiscall, retorno por valor, estática local |
| `operator-(n dias)` | 0x54c8e0 | thiscall, retorno por valor, estática local |
| `mais_semanas(n)` | 0x54c770 | thiscall, retorno por valor, estática local |

Autoteste: 9.446.717 casos (todos os anos de 16 bits nos construtores e no dia da semana), 0 divergências. Mutações acusadas: bissexto `% 200`; ano anterior bissexto somando 365. Jogo real: >10 milhões de chamadas, 0 divergências.

Esquisitices preservadas:
- `dia_da_semana` devolve −1 para 1600, dia 0, bissexto (resto negativo).
- `operator+`/`operator-` com n = −32768: a negação em 16 bits devolve −32768 e os dois se chamam até estourar a pilha (igual na original; o jogo nunca pede isso). Fica fora do autoteste.
- Cada operador inicializa a sua `Data` estática na primeira chamada e registra um destrutor vazio com `atexit`; a versão Rust faz o mesmo (por isso os autotestes só rodam depois de a CRT do jogo estar pronta).

Pendente no `Date.cpp`: as rotinas maiores (0x54a130, 0x54a470, 0x54a650, 0x54a950, 0x54ae10, 0x54b2f0, 0x54b550, 0x54ba20, 0x54bef0, 0x54c3b0), que formatam datas em texto.

## Escolha dos próximos alvos

`engenharia-reversa/alvos.py` (funções-folha pequenas e muito chamadas) e `docs/MAPA_MODULOS.md`.

### Motor de partidas, primeira leva — `match_eng.cpp`/`match_pl.cpp` → `rust/src/motor.rs` (oficial em 06/10/2026)

Escolhidas pela cobertura (`cobertura.py` + `simular.py --cobertura`, carreira com o motor ligado até 01/10/2001): as folhas mais chamadas. Só leem tabelas do motor.

| Rotina | Original | Convenção | Chamadas em 2 meses |
|---|---|---|---|
| `tabela_a(jogador)`: palavra da tabela do lado, pela posição 0..=10 | 0x6c38c0 | thiscall, ret 4, devolve `ax` | 870 milhões |
| `tabela_b(jogador)`: idem, outra tabela | 0x6c38f0 | thiscall, ret 4, `ax` | 18 milhões |
| `diferenca_lado(lado, v)` | 0x6c4180 | thiscall, ret 8, `al` | 9 milhões |
| `faixa(lado, b, c)` | 0x6cb960 | stdcall, ret 0xc, `al` | 8 milhões |
| `por_diferenca(a, b, c, d)`: tabela global 0xb67c30 | 0x6cb6c0 | thiscall, ret 0x10, `ax` | 6 milhões |
| `limite(lado, v)` | 0x6cb930 | stdcall, ret 8, `al` | 4 milhões |
| `condicao_jogador()` | 0x70dfa0 (match_pl.cpp) | thiscall, `al` | 21 milhões |

Retorno parcial: as originais só definem `al`/`ax`; os bits altos de `eax` são lixo (em 0x6c4180, `sub eax, ecx` mistura o `this`). A versão Rust reproduz a parte que o tipo define; a impressão digital da carreira pegaria um chamador que usasse o resto.

Autoteste: 14.000.000 casos (objeto do motor e jogador falsos, posições e lados fora das faixas), 0 divergências. Mutação acusada: `v > 8` → `v >= 8` em `limite` (divergência em `limite(0, 8)`). Jogo real, motor ligado, comparação: ~950 milhões de chamadas, 0 divergências. Carreira até 01/10/2001: Rust = original (597.180.435 sorteios, mesma impressão).

### Motor de partidas, segunda e terceira levas — `rust/src/motor.rs` (oficiais em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tabela_do_jogador()` | 0x717410 | thiscall | `tabela_a` do motor do jogador (0x19e), chamada pelo endereço do jogo |
| `diferenca_jogador(j)` | 0x6c4130 | thiscall, ret 4, `al` | 0 se `j` for o guardado em 0xf582 |
| `byte_do_lado(j)` | 0x6cfda0 | thiscall, ret 4, `al` | |
| `limite_jogador()` | 0x70df40 | thiscall, `al` | |
| `faixa_jogador(lado)` | 0x6f5b20 | thiscall, ret 4, `eax` inteiro | |
| `byte_da_posicao(i)` | 0x5b57c0 | thiscall, ret 4, `al` | `[this + 0x5b4 + i]` |
| `campo_pela_posicao()` | 0x6f4600 | thiscall, `al` | bloco do lado no motor: 0x91e2 + 6371 × lado |
| `pode_escolher(j, b, c)` | 0x6bce20 | thiscall, ret 0xc, `eax` inteiro | |
| `apontar(a, b, c, d)` | 0x6f4920 | thiscall, ret 0x10 | direção em graus (0x19a, 0..=359); **sorteia** (360, 120, 30, 30) e grava |

0x714d30 (`mov al, [ecx+0x27]; ret`, 4 bytes) não cabe um desvio; as versões Rust leem o campo direto.

Rotinas que sorteiam: na comparação, original e Rust partem do mesmo estado do gerador (`aleatorio::REGIOES_DO_ESTADO` no `lado_a_lado`), os sorteios da réplica não entram na contagem (`aleatorio::sem_contar`) e o jogo segue com o estado da original.

Autotestes: 37 e 51 milhões de casos, 0 divergências. Mutações acusadas (11): retorno sem motor; campo trocado; bytes de lado trocados; limiares `<`/`<=` e faixas; índice da tabela; bit da marca de posição; `0x2b >= 2`; ângulo fixo 180 → 181; `<` → `<=` no limiar do sorteio; ordem dos dois sorteios de ±30. Jogo real em comparação: >1 bilhão de chamadas, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust = original (2.340.778.192 sorteios), duas execuções Rust iguais.
