# 200ª leva — as telas da partida (9): a cópia do bloco do time e da linha do jogador e o placar ao vivo

**Rotinas:**
- `atribuir_bloco_do_time()` = 0x71dd70 (match_screens.cpp, thiscall): copia o bloco do time da partida (0x625 bytes) campo a campo, com as 20 linhas de jogador;
- `atribuir_linha_do_jogador()` = 0x71df00 (match_screens.cpp, thiscall): copia uma linha de jogador do bloco do time (0x49 bytes), a data pela Date::operator=;
- `contar_placar_ao_vivo()` = 0x728e90 (match_screens.cpp, cdecl): conta os gols e os gols nos pênaltis de uma partida até o minuto atual da partida ao vivo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 199ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 200`
