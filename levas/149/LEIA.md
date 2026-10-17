# 149ª leva — a escalação de cada lado e o dia de jogos (0x6db500, 0x6b4190, 0x6b42a0)

**Rotinas:**
- `escalar_lado()` = 0x6db500 (match_man.cpp): monta a tática de um lado e escala o time. No clube do humano, usa as linhas guardadas e a escalação automática (0x6dbc50) ou os titulares; nos outros, a automática.
- `encerrar_dia()` = 0x6b4190 (match_day.cpp): libera o que o dia de jogos alocou, inclusive os motores (0x6b97a0, da 148ª).
- `escolher_partida_do_dia()` = 0x6b42a0 (match_day.cpp): escolhe a partida do humano para a tela.

**Por que a 0x6dbc50 (a escalação automática) ficou de fora:**
- Ela tem 7,7 KB e chama 24 rotinas ainda originais (datas, competições, telas de tática).
- Aloca a própria lista e lê lixo de pilha (está nas `ZERADAS`).
- Fica para uma leva própria.

**Como foi comparada:** sem comparação própria, pela carreira (Rust × original), como as da 145ª e da 148ª. As rotinas que elas chamam seguem comparadas.

**Resultado** (`cadeia.txt`):
- Mutações (8): 7 acusadas e 1 lacuna, o segundo campo da ficha do técnico (em `docs/PENDENCIAS.md`).
- Jogo real comparando até 2002-01-01: 0 divergências.
- Carreiras Rust ×2 e original até 2002-06-01: impressão 459815f5e0dca92b.

**Refazer:** sobre o commit da 148ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 149 -`
