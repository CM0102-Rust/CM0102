# Rotinas migradas para Rust

Cada rotina entra por desvio (`rust/src/desvios.rs`) e só vira oficial depois de:
1. autoteste (`CM_AUTOTESTE=1`): entradas sintéticas logo que o jogo abre o vídeo, original × Rust, comparando resultado e memória tocada;
2. mutação de controle: um defeito proposital na versão Rust tem de ser acusado;
3. jogo real em modo comparação (`CM_COMPARAR=1`): carreira automática de férias (`docs/VERSAO_SEM_TELA.md`) ou partida do Lucas.

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
