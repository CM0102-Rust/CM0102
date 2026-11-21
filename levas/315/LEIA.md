# 315ª leva — as viradas da Copa Interamericana e do Campeonato de Clubes da Oceania e a sede alternada da Oceania

**Rotinas:**
- `inter_amer_cup_start_new_season()` = 0x632910 (inter_amer_cup.cpp, thiscall, ret 0): a virada da Copa Interamericana: o molde das viradas de copa (tabelas liberadas, temporada arquivada, ano + 1, os clubes, a tabela de rodadas e a árvore com as caixas nas linhas 0x164, 0x16b e 0x172, e as posições previstas);
- `oceania_club_champ_start_new_season()` = 0x797d60 (oceania_club_champ.cpp, thiscall, ret 0): o Campeonato de Clubes da Oceania é a cada 2 anos: na virada a tabela e as datas das rodadas liberadas, a temporada arquivada, as fases destruídas, o ano + 2, a fase atual −1, o sorteio por fazer, a sede alternada entre a Austrália e a Nova Zelândia e as regras de novo;
- `alternate_oceania_club_champ_host()` = 0x5face0 (host_country.cpp, thiscall, ret 0): a sede do Campeonato de Clubes da Oceania alterna a cada edição: em cada entrada dele na tabela de sedes, a Austrália vira a Nova Zelândia e qualquer outra sede vira a Austrália;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (1):** 1 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (798998b1).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 9f530e7a, original bd0bb86f (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 314ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 315`
