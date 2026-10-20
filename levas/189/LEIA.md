# 189ª leva — a competição do amistoso, a data internacional e o registro de técnico

**Rotinas:**
- `e_competicao_do_amistoso()` = 0x5c2990 (friendly.cpp, thiscall): diz se a competição é a dos amistosos;
- `preencher_data_internacional()` = 0x5cb9b0 (friendly.cpp, thiscall): preenche uma data internacional: dia, mês, duração, ano, continente e competição;
- `limpar_registro_de_tecnico()` = 0x86fc20 (staff_records.cpp, thiscall): zera o registro de técnico e põe −1 nos ids do dono, da liga e dos clubes;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 2 acusadas. `r_ids` escapou: o id da última passagem (+0x72) de um registro de técnico limpo nunca é lido na carreira antes de ser regravado. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 188ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 189`
