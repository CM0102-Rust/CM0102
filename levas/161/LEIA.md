# 161ª leva — lista de observação, código da posição e critério da comissão

**Rotinas:**
- `serve_para_a_vaga()` = 0x844300 (shortlist_manager.cpp, stdcall): o jogador serve para a vaga 0..15 da lista de observação (nota 15 no setor e no lado; a 15 pede contrato, id e a 0x600a50 em zero); devolve só `al`;
- `codigo_da_posicao()` = 0x83bd00 (shortlist_manager.cpp, stdcall): o código da posição pela máscara de setor e lado (28 combinações; o resto 0xe); devolve só `al`;
- `criterio_da_comissao()` = 0x866f60 (staff_contracts.cpp, thiscall): o critério da comissão técnica: uma nota em 8 bits (id mod 7, notas, idade, reputação, número de funções) decide pelas funções se devolve 1;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 6 acusadas. `s_id` escapou: mudava só o limite exato do id (−16 para −17); refeita como `s_id_b` em `mutacoes/leva161b.py` e acusada (`mutacoes_refeitas.txt`). `c_idade` escapou: nem refeita com +15 (`c_idade_b`) é acusada: nenhuma pessoa da comissão tem +0x44 entre 51 e 75 nos dados. `c_rep` escapou: mudava só a reputação exata 0x1c53; refeita como `c_rep_b` e acusada. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 160ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 161`
