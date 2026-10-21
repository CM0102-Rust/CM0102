# 203ª leva — as telas da partida (11): os valores das estatísticas da partida, das zonas de ação e das notas dos jogadores

**Rotinas:**
- `valores_das_estatisticas_da_partida()` = 0x71e750 (match_screens.cpp, cdecl): passa às células das estatísticas da partida os valores que mudaram desde a cópia do estado (parâmetro 0x1d) e atualiza a cópia;
- `valores_das_zonas_de_acao()` = 0x720320 (match_screens.cpp, cdecl): atualiza as zonas de ação: marcadores, posse, tempo de bola em jogo, barras das três zonas e da posse com as cores dos clubes; atualiza a cópia do estado;
- `valores_das_notas_dos_jogadores()` = 0x723350 (match_screens.cpp, cdecl): atualiza as linhas das notas dos jogadores: cartões, minuto de lesão/expulsão/substituição, nota, gols e cor do nome; atualiza a cópia do estado;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 202ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 203`
