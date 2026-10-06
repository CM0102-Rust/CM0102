# Rotinas migradas para Rust

Cada rotina entra por desvio (`rust/src/desvios.rs`) e só vira oficial depois de:
1. autoteste (`CM_AUTOTESTE=1`): entradas sintéticas no início do jogo, original x Rust;
2. mutação de controle: um defeito proposital na versão Rust tem de ser acusado;
3. jogo real em modo comparação (`CM_COMPARAR=1`): carreira automática de férias (`docs/VERSAO_SEM_TELA.md`) ou partida do Lucas.

| Rotina | Original | Fonte | Rust | Autoteste | Mutação | Jogo real | Oficial |
|---|---|---|---|---|---|---|---|
| `aleatorio(n)` | 0x90cfc0 (cdecl) | ? (perto de `utils.cpp`) | `aleatorio.rs` | 5.000.000 casos, 0 div. | 2 acusadas | >100 milhões, 0 div. | 06/10/2026 |
| `Data::Data()` | 0x54a110 (thiscall) | `si/code/Date.cpp` | `data.rs` | (junto das datas) | — | (junto das datas) | 06/10/2026 |
| `Data::Data(dia, ano)` | 0x549ef0 (thiscall) | `si/code/Date.cpp` | `data.rs` | 2.860.659 casos (datas), 0 div. | bissexto % 200 acusada | >10 milhões (datas), 0 div. | 06/10/2026 |
| `Data::dia_da_semana()` | 0x54a3f0 (thiscall) | `si/code/Date.cpp` | `data.rs` | (junto das datas) | — | (junto das datas) | 06/10/2026 |

Escolha dos próximos alvos: `engenharia-reversa/alvos.py` (funções-folha pequenas e muito chamadas) e `docs/MAPA_MODULOS.md`.
