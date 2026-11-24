# Os premios do ano dados pelos candidatos das notas do ano e pela lista curta (nation_awards.cpp; leitor sx04b, rodada
# 5, 23/11/2026). Rodam na carreira ENG depois de 2002-06-01 (ATE): a diaria internacional (0x632e20, sx03_4) decide o
# jogador do ano mundial pela lista curta em 6 de junho de 2002 (0x768b20, 0x632ecf) e da os melhores da Africa,
# Europa, America do Sul e Oceania em 16 de junho (0x768720, 0x632f47..0x632f9e); 0x768720 tambem pelos premios da
# Europa (0x583737..0x583796).
# Convencoes conferidas na desmontagem:
# - 0x768720: thiscall(hoje, premio, notas do ano, continente, posicoes, modo, lista), ret 0x1c; ecx sem uso; malloc
#   0x944e46 cdecl(0xc); YEAR_RATINGS::collect_candidates 0x931d50 thiscall(bloco, 3, continente, posicoes, modo,
#   lista) ret 0x18 -> ax (com sinal); LLIST::add_tail 0x68d0d0 thiscall(&ponteiro da pessoa, 4) ret 8 (lista relida a
#   cada volta); YEAR_AWARD::record_winners 0x9302f0 thiscall(&registro, 1) ret 8.
# - 0x768b20: thiscall(hoje, premio), ret 8; ecx sem uso; AWARD_SHORTLIST::pick_winners 0x417c50 thiscall(bloco, 3)
#   ret 8 -> ax (movsx); clear 0x418320 thiscall (antes do teste de tres).
# Prologos sem chamada relativa; iguais no cm0102-1080.exe (0x768b20 com 10 bytes: a primeira instrucao so tem 4).
# Defeitos do original: (0x768720) sem notas do ano (caixa da linha 346) o bloco nao e liberado; nas duas, o dword da
# nota de cada vencedor no registro nunca e escrito (o historico guarda o lixo da pilha).
# ATENCAO (porte ja publicado): o AWARD_SHORTLIST::pick_winners em Rust (award_shortlist.rs, tel29_3) nao devolve
# nada, e o original devolve em ax quantos escreveu (0 sem entradas); a 0x768b20, original ou copia, le esse ax: com o
# porte atual ela le lixo. Corrigir o retorno (e o comparador) antes de provar este item.
# Depende do sx04_2 (YEAR_RECORD_*, record_year_award, compare_award_record, NATION_AWARDS_CPP, YEAR_AWARD_HISTORY_*)
# e do sx04b_3 (year_record_of, FN_AWARD_SHORTLIST_CLEAR): integrar depois deles.
# Comparadores: ensaio da copia sem o registro (e sem a lista; na lista curta, a lista, as entradas e o gerador
# guardados e repostos), depois a original; o registro mais novo do historico tem de ser o planejado, sem as notas.
SAVE = False
LIGAS = 'ENG'  # as do sx05b_1..4 e sx05b_20 (mesma leva)
ATE = '2002-08-01'
ROTINAS = [('nation_awards_award_top_three_from_ratings', 0x768720, [0x81, 0xec, 0x54, 0x02, 0x00, 0x00]),
           ('nation_awards_award_from_shortlist', 0x768b20, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x50, 0x02, 0x00, 0x00])]
DOCS = ('os prêmios do ano dados pelos candidatos das notas do ano e pela lista curta',
        'the year awards given from the year ratings\' candidates and from a shortlist',
        [('nation_awards_award_top_three_from_ratings', 'nation_awards_award_top_three_from_ratings', '0x768720', 'nation_awards.cpp', 'thiscall, ret 0x1c',
          'regra: os três melhores candidatos das notas do ano (do continente, das posições, pelo modo, fora os que já estão na lista); com menos de três, nada; cada vencedor entra na lista (se houver) e os três vão para o histórico do prêmio do ano. Defeitos: sem notas do ano o bloco não é liberado; a nota de cada vencedor no registro fica com o lixo da pilha',
          "rule: the three best candidates of the year ratings (of the continent, the positions, by the mode, leaving out those already on the list); fewer than three, nothing; each winner goes into the list (if any) and the three go to the year award history. Defects: without year ratings the block is not freed; each winner's score in the record keeps stack garbage"),
         ('nation_awards_award_from_shortlist', 'nation_awards_award_from_shortlist', '0x768b20', 'nation_awards.cpp', 'thiscall, ret 8',
          'regra: a lista curta do prêmio escolhe até três vencedores pelos votos dos clubes do computador e é limpa; com três, eles vão para o histórico do prêmio do ano. Defeito: a nota de cada vencedor no registro fica com o lixo da pilha',
          "rule: the award's shortlist picks up to three winners by the computer clubs' votes and is cleared; with three, they go to the year award history. Defect: each winner's score in the record keeps stack garbage")])
CAMPOS = []
INTERESSA = []
