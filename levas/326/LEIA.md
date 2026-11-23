# 326ª leva — o sorteio das eliminatórias da Eurocopa

**Rotinas:**
- `euro_champ_qual_draw_groups()` = 0x57dab0 (euro_champ_qual.cpp, thiscall, ret 0): O sorteio dos grupos das eliminatórias da Eurocopa: em 1998, a tabela fixa (49 seleções em 9 grupos, os grupos 1, 3, 6 e 8 com seis); nos outros anos, 9 grupos (+0xff + 2) e até seis potes: a ordem dos grupos é embaralhada com 20 trocas sorteadas e cada grupo, na ordem, recebe a próxima seleção enquanto tiver vaga; defeito: a vaga de cada grupo é a do grupo anterior (g − 1), então no formato 1 os grupos de seis ficam 1, 3 e 6;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (2):** 2 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 5e6914ad, original 2c08aef5 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 325ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 326`
