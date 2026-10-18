# 159ª leva — a formação: posições do jogador e casas no campo

Primeira leva fora do motor de partidas. As candidatas vêm da cobertura da carreira automática em 26 módulos da simulação (`docs/cobertura_simulacao.txt`, `python engenharia-reversa/candidatos.py --simulacao`).

**Rotinas** (`formation.cpp`):
- `mascara_de_posicoes()` = 0x5b7d20: a máscara das posições fortes do jogador;
- `casa_no_campo()` = 0x5b7e60: a coluna e a linha da posição no campo;
- `vagas_sem_conflito()` = 0x5b8960: confere as 11 vagas da formação.

**Como foi comparada:** comparação de verdade nas três (`comparar_valor!` e `lado_a_lado_leve` na saída).

**Resultado** (`cadeia.txt`):
- **Mutações (7):** 5 acusadas. `m_nivel` e `v_a20` escaparam: no banco a posição natural tem nota 20 e nenhuma vaga com lado aberto é de meia. É uma lacuna em `docs/PENDENCIAS.md`.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 158ª e da DLL do save:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 159`
