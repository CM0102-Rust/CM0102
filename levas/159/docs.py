"""Documentacao da 159a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva159.py` (7, pela carreira até 2002-06-01, confirmando as que escapam no modo comparação até 2001-08-20): 5 acusadas; `m_nivel` (descer o nível de 2 em 2) e `v_a20` (0x10 no lugar de 0x20) escaparam nas duas provas: no banco do jogo a posição natural tem nota 20 (o laço para no nível 15) e nenhuma vaga com lado aberto é de meia (lacuna em `PENDENCIAS.md`). "
     "Primeira leva fora do motor de partidas: as candidatas vêm de `candidatos.py --simulacao` (cobertura da carreira automática em 26 módulos da simulação, `docs/cobertura_simulacao.txt`). "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = ("`mutacoes/leva159.py` (7, by the career to 2002-06-01, confirming the escaped ones in comparison mode to 2001-08-20): 5 caught; `m_nivel` (stepping the level down by 2) and `v_a20` (0x10 instead of 0x20) escaped both proofs: in the game database the natural position is rated 20 (the loop stops at level 15) and no slot with an open side is a midfielder's (gap in `PENDENCIAS.en.md`). "
      "First batch outside the match engine: the candidates come from `candidatos.py --simulacao` (coverage of the automatic career in 26 simulation modules, `docs/cobertura_simulacao.txt`). "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(159, 350, 353, 'a formação: posições do jogador e casas no campo', 'the formation: player positions and pitch cells',
      ["| `mascara_de_posicoes()` | 0x5b7d20 | cdecl | formation.cpp: a máscara das posições fortes pelas notas (+0xf..+0x15, do nível 15 ao 10) e os lados com nota 15 (+0x17..+0x19); devolve só `ax` |",
       "| `casa_no_campo()` | 0x5b7e60 | cdecl | formation.cpp: coluna 0..26 e linha 0..35 da posição (setor e lado pela 0x5b73a0) pelo ataque e pela faixa (tabelas de 4 e de 5, encostos de 1 ou 5 nas faixas, limites) |",
       "| `vagas_sem_conflito()` | 0x5b8960 | thiscall | formation.cpp: 0 se alguma das 11 vagas com lado aberto (0x880) é de atacante, meia, volante ou meia-atacante (casas +0x12d/+0x14e e 0x5b2ae0); devolve só `al` |"],
      ["| `mascara_de_posicoes()` | 0x5b7d20 | cdecl | formation.cpp: the strong-position mask by the ratings (+0xf..+0x15, level 15 down to 10) and the sides rated 15 (+0x17..+0x19); returns only `ax` |",
       "| `casa_no_campo()` | 0x5b7e60 | cdecl | formation.cpp: column 0..26 and row 0..35 of the position (sector and side via 0x5b73a0) by attack and band (tables of 4 and 5, nudges of 1 or 5 in the bands, limits) |",
       "| `vagas_sem_conflito()` | 0x5b8960 | thiscall | formation.cpp: 0 if any of the 11 slots with an open side (0x880) is a striker's, midfielder's, defensive or attacking midfielder's (cells +0x12d/+0x14e and 0x5b2ae0); returns only `al` |"],
      M, ME,
      [('mascara_de_posicoes', 'position_mask'), ('casa_no_campo', 'pitch_cell'), ('vagas_sem_conflito', 'slots_without_conflict')],
      "'posicao_no_conteiner': 'position_in_container',",
      ("Fora do motor de partidas, as candidatas vêm de `python candidatos.py --simulacao` (lista em `docs/cobertura_simulacao.txt`, gerada por `cobertura.py <módulos>` + `simular.py --cobertura`). O `cobertura.py` pula as rotinas já desviadas: contador e desvio no mesmo início se atropelam (o Rust chamava o endereço 0).",
       "Outside the match engine, the candidates come from `python candidatos.py --simulacao` (list in `docs/cobertura_simulacao.txt`, made by `cobertura.py <modules>` + `simular.py --cobertura`). `cobertura.py` skips the routines already hooked: a counter and a hook at the same start trample each other (Rust called address 0)."),
      secao=('Simulação', 'Simulation'))
