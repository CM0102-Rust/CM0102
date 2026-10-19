"""Documentacao da 181a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 181
R = [('noticia_com_resposta_2', 'story_with_response_two', '0x735780', 'media.cpp', 'thiscall', 'o jogador tem uma notícia com a resposta 2 no registro da mídia', 'whether the player has a story with response 2 in the media log'), ('tem_noticia', 'has_story', '0x735840', 'media.cpp', 'thiscall', 'o jogador tem alguma notícia no registro da mídia', 'whether the player has any story in the media log'), ('importancia_da_partida', 'cup_match_importance', '0x5221f0', 'Cup.cpp', 'thiscall', 'a importância da partida da copa pela fase e a diferença do placar da ida', 'the cup match importance by round and the first-leg score difference')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'm_alguma': ('as notícias do registro da mídia consultadas na carreira nunca têm a resposta 0', 'the media log stories asked about in the career never have response 0'),
    'c_fase': ('mutação equivalente: com a fase 1 trocar por 1 não muda nada (não é lacuna do teste)', 'equivalent mutation: at round 1, replacing it with 1 changes nothing (not a test gap)'),
}
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
M = (f"`mutacoes/leva{N}.py` ({tot}, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): {ac} acusadas. {lac_pt} "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = (f"`mutacoes/leva{N}.py` ({tot}, by the career to 2002-06-01, confirming the escaped ones in comparison mode): {ac} caught. {lac_en} "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(N, 416, 416 + 3, 'as notícias do jogador na mídia e a importância da partida de copa', "the player's media stories and the cup match importance",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as notícias do jogador na mídia e a importância da partida de copa', "the player's media stories and the cup match importance", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
