"""Documentacao da 173a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 173
R = [('cumprir_regra_do_elenco', 'apply_squad_rule', '0x6e8910', 'match_man.cpp', 'cdecl', 'cumpre a regra do elenco (estrangeiros, extracomunitários ou idade): troca os restritos pelos candidatos de menor perda na nota', 'applies the squad rule (foreigners, non-EU or age): swaps restricted players for the candidates with the smallest score loss'), ('nota_fora_da_partida', 'off_match_score', '0x6e6340', 'match_man.cpp', 'cdecl', 'a nota do jogador para uma posição fora da partida (transferências e observação): posição, forma, habilidade vista, contrato, idade, conhecimento do observador', "the player's score for a position outside a match (transfers and scouting): position, form, perceived ability, contract, age, scout knowledge"), ('nota_para_a_vaga', 'lineup_slot_score', '0x6e2f20', 'match_man.cpp', 'cdecl', 'a nota do jogador para uma vaga da escalação: como a nota fora da partida, mais a segunda posição, o fôlego na partida, a tática, os veteranos e as férias', "the player's score for a lineup slot: like the off-match score, plus the second position, in-match stamina, tactics, veterans and holidays")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'r_banco': ('a fase do banco da regra de elenco (0x6e8910) não teve restritos no banco na liga inglesa (vai para a carreira ampla)', 'the bench phase of the squad rule (0x6e8910) had no restricted bench players in the English league (goes to the broad career)'),
    'r_idade': ('a regra por idade (tipo 3) nunca teve um jogador exatamente no limite', 'the age rule (type 3) never had a player exactly at the limit'),
    'n_veterano': ('o ajuste do contrato (flag 0x2000000) nunca pegou um veterano (mais de 34 anos) sem jogos', 'the contract adjustment (flag 0x2000000) never met a veteran (over 34) with no games'),
    'v_ferias': ('o bônus das férias da 0x6e2f20 só roda com a lista de férias de um técnico humano', 'the holiday bonus of 0x6e2f20 only runs with a human manager\'s holiday list'),
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
fazer(N, 392, 392 + 3, 'o fechamento do motor (7): a regra do elenco e as notas do jogador', 'closing the engine (7): the squad rule and the player scores',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'o fechamento do motor (7): a regra do elenco e as notas do jogador', 'closing the engine (7): the squad rule and the player scores', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
