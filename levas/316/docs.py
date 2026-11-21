"""Documentacao da 316a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, prova_save, resultado_pt_en

N = 316
R = [('injury_get_type_name', 'injury_get_type_name', '0x62ef30', 'injury.cpp', 'thiscall, ret 0x10', 'o nome traduzido de um tipo de lesão (0 a 0x51), com o artigo indefinido ("um resfriado") ou sem ele; na língua 3 sempre sem; outro valor de artigo fora da língua 3 não escreve nada; buffer abaixo de 30 bytes ou tipo fora da tabela: a caixa de erro', 'the translated name of an injury type (0 to 0x51), with the indefinite article ("a cold") or without it; in language 3 always without; another article value outside language 3 writes nothing; a buffer under 30 bytes or a type outside the table: the error box'), ('injury_write_recovery_news_text', 'injury_write_recovery_news_text', '0x631580', 'injury.cpp', 'thiscall, ret 0xc', 'o título ou o corpo da notícia da volta aos treinos leves ou completos: a manchete com o nome; o corpo pela categoria do tipo: tratamentos e férias (12) pelo tipo, pelo resultado e pelo campo 10; doenças (0) pelo tipo; as outras com o nome da lesão (a categoria, ou o tipo na categoria 6) e o gênero; nos treinos leves, o tempo previsto até os completos (aproximado); o vínculo da pessoa quando os ids e a data de nascimento ainda batem', "the headline or body of the news of the return to light or full training: the headline with the name; the body by the type's category: treatments and holidays (12) by the type, the outcome and slot 10; illnesses (0) by the type; the others with the injury's name (the category, or the type in category 6) and gender; in light training, the expected time to full training (approximate); the person's link when the ids and the birth date still match"), ('news_manager_departure_text', 'news_manager_departure_text', '0x6a1d60', 'manager_manager.cpp', 'stdcall, ret 0xc', 'o título ou o corpo da notícia de que o técnico de um clube ou de uma seleção foi demitido, pediu demissão ou se aposentou: a manchete pelo modo da saída e pelo cargo (técnico, jogador-técnico, ou com motivo), com o nome curto do clube; o corpo, numa seleção, pela primeira ou segunda seleção, pelo modo e pelo motivo, com o adjetivo da nação; num clube, demitido como técnico ou jogador-técnico com ou sem um segundo nome, ou com um dos 16 motivos de duas tabelas; demissão pedida pelo cargo; os outros modos com um texto só; os vínculos da pessoa (quando os ids batem), do clube e do adjetivo (quando aparecem no texto)', "the headline or body of the news that a club or national team manager was sacked, resigned or retired: the headline by how he left and by the job (manager, player-manager, or with a reason), with the club's short name; the body, for a national team, by the first or second national team, how he left and the reason, with the nation's adjective; for a club, sacked as manager or player-manager with or without a second name, or with one of 16 reasons from two tables; resigned by the job; the other ways with a single text; the links of the person (when the ids match), of the club and of the adjective (when they appear in the text)")]
# the escapes are explained in escapadas.py next to this file, when there are any (18/11/2026)
_e = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'escapadas.py')
ESCAPADAS = {k: (a.rstrip('.'), b.rstrip('.')) for k, (a, b) in (__import__('runpy').run_path(_e)['ESCAPADAS'].items() if os.path.exists(_e) else [])}  # nome: (pt, en)
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia com divergência ou carreira que não chegou à data'
RES_PT, RES_EN = resultado_pt_en(N)
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
PS = prova_save(N)  # leva do save: mutações e prova pelo .qst gravado com e sem as rotinas (cadeia.sh)
VIA_PT = 'pela prova do save' if PS else 'pela carreira até 2002-06-01, confirmando no modo comparação as que escapam'
VIA_EN = 'by the save proof' if PS else 'by the career to 2002-06-01, confirming the escaped ones in comparison mode'
PS_PT = f'Prova do save (o .qst com e sem as rotinas): {PS}. ' if PS else ''
PS_EN = f'Save proof (the .qst with and without the routines): {PS.replace("IGUAL", "EQUAL").replace("DIFERENTE", "DIFFERENT")}. ' if PS else ''
M = (f"`mutacoes/leva{N}.py` ({tot}, {VIA_PT}): {ac} acusadas. {lac_pt} {PS_PT}"
     f"{RES_PT}")
ME = (f"`mutacoes/leva{N}.py` ({tot}, {VIA_EN}): {ac} caught. {lac_en} {PS_EN}"
      f"{RES_EN}")
fazer(N, 2770, 2770 + len(R), 'os nomes dos tipos de lesão e o texto da notícia da volta aos treinos; o texto da notícia da saída de um técnico', 'the injury type names and the return to training news text; the manager departure news text',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'os nomes dos tipos de lesão e o texto da notícia da volta aos treinos; o texto da notícia da saída de um técnico', 'the injury type names and the return to training news text; the manager departure news text', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en,
     campos=[], interessa=[])
