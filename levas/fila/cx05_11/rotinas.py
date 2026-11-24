# A situacao de uma selecao na Eurocopa e as classificadas das eliminatorias (euro_champ.cpp; leitor cx05, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2004-07-15: as classificadas entram no fim das eliminatorias da Eurocopa
# de 2004 (outubro e novembro de 2003); a situacao e a mudanca de situacao sao lidas pela avaliacao dos tecnicos das
# selecoes (manager_manager.rs, virtuais +0x70 e +0x74) quando a Eurocopa e a competicao atual da selecao (desde as
# posicoes previstas, no sorteio de fevereiro de 2004). 0 chamadas na cobertura do mundo ate 2002-08-01.
# - 0x57cd30 EuroChamp::get_qualification_status thiscall(selecao) ret 4 -> word: Comp_Stats::get_club_stat 0x4a2e10
#   (push 0x12; push selecao; mov ecx, [this+8] -> thiscall, ret 8, float) e _ftol 0x945640; comparacoes com sinal no
#   byte baixo. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# - 0x57cd80 EuroChamp::get_qualification_status_change thiscall(selecao) ret 4 -> byte: as estatisticas 0x13 (a
#   anterior) e 0x12 (a atual), cada uma pelo _ftol. Prologo push ebx; push esi; push edi; mov edi, [esp+0x10] (7 bytes).
# - 0x57c120 EuroChamp::add_qualified_team thiscall(selecao) ret 4: livro cheio (+0x36 >= +0x38, words com sinal) a
#   caixa da linha 0x365. Prologo mov ax, [ecx+0x36]; sub esp, 0x200 (10 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: as duas situacoes lado a lado no valor (so leem); a classificada lado a lado na contagem e na entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-07-15'
ROTINAS = [('euro_champ_get_qualification_status', 0x57cd30, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('euro_champ_get_qualification_status_change', 0x57cd80, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('euro_champ_add_qualified_team', 0x57c120, [0x66, 0x8b, 0x41, 0x36, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a situação das seleções na Eurocopa', 'the nations\' status in the European Championship',
        [('euro_champ_get_qualification_status', 'EuroChamp::get_qualification_status', '0x57cd30', 'euro_champ.cpp', 'thiscall, ret 4',
          'regra: a situação de uma seleção na Eurocopa pelo seu código de colocação atual: campeã −1, vice 150, semifinalista 130, quartas-de-final 120, eliminada nos grupos −4',
          'rule: a nation\'s status in the European Championship from its current position code: winner −1, runner-up 150, semi-finalist 130, quarter-finalist 120, out in the groups −4'),
         ('euro_champ_get_qualification_status_change', 'EuroChamp::get_qualification_status_change', '0x57cd80', 'euro_champ.cpp', 'thiscall, ret 4',
          'regra: a mudança da situação de uma seleção na Eurocopa, pela faixa anterior (até 4, 5 a 8, de 9 em diante) e a atual (até 2, 3 a 8, de 9 em diante): atual até 2 dá 0, −1 ou −2; atual de 3 a 8 dá 2, 0 ou −1; atual de 9 em diante dá 3, 2 ou 0 (pela faixa anterior, na ordem)',
          'rule: the change of a nation\'s status in the European Championship, by the previous band (up to 4, 5 to 8, from 9) and the current one (up to 2, 3 to 8, from 9): current up to 2 gives 0, −1 or −2; current 3 to 8 gives 2, 0 or −1; current from 9 gives 3, 2 or 0 (by the previous band, in order)'),
         ('euro_champ_add_qualified_team', 'EuroChamp::add_qualified_team', '0x57c120', 'euro_champ.cpp', 'thiscall, ret 4',
          'regra: uma seleção classificada nas eliminatórias entra na lista da Eurocopa com o status 6; com a lista cheia, a caixa de erro',
          'rule: a nation qualified from the qualifiers enters the European Championship list with status 6; with the list full, the error box')])
CAMPOS = []
INTERESSA = []
