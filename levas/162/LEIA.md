# 162ª leva — as vagas da tabela tática

**Rotinas:**
- `gravar_vaga_tatica()` = 0x896a90 (tactics.cpp, thiscall): grava a pessoa na vaga 0..19 da linha do clube (0x8982b0) e apaga as instruções pendentes desse clube;
- `pessoa_da_vaga_tatica()` = 0x8968f0 (tactics.cpp, thiscall): a pessoa da vaga; sem ficha ou de outra nação numa seleção, a vaga vira −1;
- `gravar_formacao_tatica()` = 0x8967d0 (tactics.cpp, thiscall): grava a formação na linha do clube;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 2 acusadas. `g_pendente` escapou: as instruções pendentes (+0x5f0) só existem com técnico humano. `p_ficha` escapou: na carreira automática as vagas da tabela tática estão vazias (−1): nem a condição invertida (`p_ficha_b`, `mutacoes/leva162b.py`) é acusada; a parte com pessoa só roda na tática do técnico humano. `p_nacao` escapou: idem (`p_nacao_b` invertida também escapa). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 161ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 162`
