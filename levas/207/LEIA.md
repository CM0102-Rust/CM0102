# 207ª leva — as telas da partida (15): a conferência da escalação e o índice das estatísticas da partida no save

**Rotinas:**
- `desenhar_conferencia_da_escalacao()` = 0x7260f0 (match_screens.cpp, cdecl): desenha a conferência da escalação: avisa (com os nomes ligados) os convocados, emprestados, indisponíveis, inelegíveis, suspensos, lesionados e sem permissão de trabalho, o limite do banco ou a falta de titulares; senão pede a confirmação (capitão, cobradores, reservas) com "No" e "Yes";
- `carregar_estatisticas_da_partida()` = 0x72aa80 (match_stats.cpp, thiscall): lê do match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens;
- `salvar_estatisticas_da_partida()` = 0x72adf0 (match_stats.cpp, thiscall): grava no match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 206ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 207`
