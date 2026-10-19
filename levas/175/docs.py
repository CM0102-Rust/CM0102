"""Documentacao da 175a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 175
R = [('registro_da_formacao', 'formation_record', '0x896570', 'tactics.cpp', 'thiscall', 'o registro da formação (0x18e3 bytes) nas duas tabelas', 'the formation record (0x18e3 bytes) in the two tables'), ('peso_da_competicao', 'competition_weight', '0x7c6930', 'player_stats.cpp', 'stdcall', 'o peso da competição pelas duas competições do clube (500/1000/300/100)', "the competition weight by the club's two competitions (500/1000/300/100)"), ('gravar_dado_da_noticia', 'set_news_field', '0x785790', 'news.cpp', 'thiscall', 'grava o dado 0..49 da notícia', 'stores news field 0..49')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'f_limite': ('o limite só muda para as formações além das padrão pedidas com o humano (as formações próprias de um técnico humano), que a carreira automática não usa', 'the limit only changes for formations beyond the default ones asked with the human flag (a human manager\'s own formations), which the automatic career does not use'),
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
fazer(N, 398, 398 + 3, 'a formação da tabela tática, o peso das competições do jogador e o dado da notícia', "the tactics table formation, the player's competition weight and the news field",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a formação da tabela tática, o peso das competições do jogador e o dado da notícia', "the tactics table formation, the player's competition weight and the news field", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
