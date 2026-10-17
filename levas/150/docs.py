"""Documentacao da 150a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

M = ("`mutacoes/leva150.py` (17, pela carreira até 2002-06-01): 12 acusadas e 5 escaparam por serem fracas; refeitas mais fortes (`mutacoes/leva150b.py`: o peso dos dias, o filtro “já jogou”, a máscara, o rodízio e a postura 3), as 5 acusadas. "
     "Sem comparação própria: a escalação automática aloca a lista e os vetores do rodízio e chama 24 rotinas ainda originais; validada pela carreira (Rust × original). As rotinas que ela chama seguem comparadas. "
     "Jogo real comparando até 2002-01-01: 0 divergências (6500 escalações pela sonda, nenhuma com lixo). Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba")
ME = ("`mutacoes/leva150.py` (17, by the career to 2002-06-01): 12 caught and 5 escaped for being weak; redone stronger (`mutacoes/leva150b.py`: the days weight, the “already played” filter, the mask, the rotation and stance 3), all 5 caught. "
      "No comparison of its own: the automatic selection allocates the list and the rotation vectors and calls 24 routines still in the original; validated by the career (Rust × original). The routines it calls stay compared. "
      "Real game comparing to 2002-01-01: 0 divergences (6500 selections by the probe, none with garbage). Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba")
fazer(150, 332, 333, 'a escalação automática', 'the automatic selection',
      ["| `escalar_automatico()` | 0x6dbc50 | thiscall, ret 8 | match_man.cpp (~7,7 KB): a comissão escala o time contra o adversário. Fora do humano, mede o jogo de hoje contra o próximo da lista (0x5acbe0: reputação da competição, rival, mesma cidade, dias até lá) para poupar (+0x1a 0/1/2 e as máscaras de vagas); filtra os 50 das linhas (estado, suspensão, já jogou, impedido, emprestado), escala (0x6e2000), escolhe a formação (0x6e0230), as três referências (+6/+0xa/+0xe), o rodízio (0x8cf880/0x6e8910), as vagas (0x6e0d00), o capitão (0x897760), as instruções (0x6ddab0) e as ordens gerais pela postura (0x5b50d0). Os locais vivem num quadro com os deslocamentos da pilha original (zerado, como a original determinizada) |"],
      ["| `escalar_automatico()` | 0x6dbc50 | thiscall, ret 8 | match_man.cpp (~7.7 KB): the staff selects the team against the opponent. Outside the human, weighs today's match against the next one on the list (0x5acbe0: competition reputation, rival, same city, days until it) to rest players (+0x1a 0/1/2 and the slot masks); filters the 50 of the lines (state, suspension, already played, barred, on loan), selects (0x6e2000), picks the formation (0x6e0230), the three references (+6/+0xa/+0xe), the rotation (0x8cf880/0x6e8910), the slots (0x6e0d00), the captain (0x897760), the instructions (0x6ddab0) and the general orders by the stance (0x5b50d0). The locals live in a frame with the original stack offsets (zeroed, like the determinised original) |"],
      M, ME,
      [('escalar_automatico', 'select_automatically')],
      "'escolher_partida_do_dia': 'pick_day_match',")
