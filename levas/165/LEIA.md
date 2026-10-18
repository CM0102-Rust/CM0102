# 165ª leva — as vagas extras da tática, as idades da seleção e o dado do técnico humano

**Rotinas:**
- `vaga_extra_tatica()` = 0x897ee0 (tactics.cpp, thiscall): as duas vagas extras (+0x56, +0x5a) da linha do clube;
- `idades_da_selecao()` = 0x76d430 (national_teams.cpp, stdcall): as idades limite (23/23 nas competições de base, 24/22, 26/22);
- `gravar_dado_do_humano()` = 0x5fdd30 (human_manager.cpp, thiscall): grava +0x9a65 do registro do técnico humano (0xa6ef bytes);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (5):** 4 acusadas. `v_desce` escapou: ao apagar a primeira vaga extra, a segunda já está vazia (−1) nos clubes da IA: descer −1 ou gravar −1 dá o mesmo (os contadores "vaga extra apagada" 65536 e "vaga extra gravada" 4096 mostram que os dois caminhos rodam). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 164ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 165`
