# A virada da temporada das copas da CONMEBOL: a Libertadores, a Mercosul e a Merconorte (conmebol_liber.cpp,
# conmebol_merc.cpp, con_merc_cup.cpp, em eurocomp\). Frente das competições (Agente E, 07/11/2026, madrugada),
# carreira de 8 ligas (o Brasil jogável: a Libertadores de 2002 joga de fevereiro a maio).
# - 0x4c3980 (ConmebolLiber::start_new_season): a virada da Libertadores. Três mutações.
# - 0x4c5040 (ConmebolMerc::start_new_season) e 0x4c1fb0 (ConMercCup::start_new_season): as viradas da Mercosul e da
#   Merconorte (as edições de 2002 começam depois de junho). Passageiras.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual da virada, thiscall, ret 0. Dentro: 0x687970 (push 0;
# mov ecx -> thiscall, ret 4), 0x9452ca (push; add esp, 4 -> cdecl), 0x4a1c50 (push 1; mov ecx, [+8] -> thiscall,
# ret 4), a virtual 0 das fases (push 1; mov ecx -> thiscall), 0x4c61b0 (push 0; mov ecx, [0xae1000] -> thiscall, ret
# 4) e a virtual +0x8c (mov ecx -> thiscall, ret 0).
# Comparadores: só a original (liberam e arquivam), com o plano: +0x4c e +0xba zerados, o ano + 1 e +0x45 zerado (na
# Libertadores e na Mercosul).
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: comparação de 8 ligas até 2002-06-01 (os 12 itens do agente E) 0 div; Rust x2 iguais e detector igual (ab5806e63c96b235); mutações 3/3.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('conmebol_liber_start_new_season', 0x4c3980, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('conmebol_merc_start_new_season', 0x4c5040, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('con_merc_cup_start_new_season', 0x4c1fb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada da temporada das copas da CONMEBOL: a Libertadores, a Mercosul e a Merconorte',
        'the season turnover of the CONMEBOL cups: the Libertadores, the Mercosur and the Merconorte',
        [('conmebol_liber_start_new_season', 'ConmebolLiber::start_new_season', '0x4c3980', 'conmebol_liber.cpp', 'thiscall, ret 0', 'regra: na virada da Libertadores a tabela, o calendário, os convidados e as fases da edição velha são liberados e as estatísticas arquivadas; o ano sobe um; a semeadura da CONMEBOL refaz as vagas das nações; o sorteio volta a fazer e as regras são montadas de novo', "rule: at the Libertadores turnover the old edition's table, schedule, invited list and stages are freed and the statistics archived; the year goes up one; the CONMEBOL seeding redoes the nations' places; the draw is pending again and the rules are rebuilt"),
         ('conmebol_merc_start_new_season', 'ConmebolMerc::start_new_season', '0x4c5040', 'conmebol_merc.cpp', 'thiscall, ret 0', 'regra: na virada da Mercosul o mesmo, sem a semeadura (a Mercosul é por convite)', 'rule: at the Mercosur turnover the same, without the seeding (the Mercosur is by invitation)'),
         ('con_merc_cup_start_new_season', 'ConMercCup::start_new_season', '0x4c1fb0', 'con_merc_cup.cpp', 'thiscall, ret 0', 'regra: na virada da Merconorte o mesmo; o sorteio só volta a fazer se as regras se montarem', 'rule: at the Merconorte turnover the same; the draw is pending again only if the rules are built')])
CAMPOS = [('copa +0x4c zerado, +0xba calendário, +0xee convidados, +0xc/+0x30 fases; ponteiro/valor; libera e zera', 'cup +0x4c cleared, +0xba schedule, +0xee invited, +0xc/+0x30 stages; pointer/value; frees and clears'),
          ('copa +0x40 ano (word), +0x45 sorteio feito; valor; grava', 'cup +0x40 year (word), +0x45 draw done; value; writes'),
          ('[0xae1000] a semeadura da CONMEBOL; ponteiro; lê (Libertadores)', '[0xae1000] the CONMEBOL seeding; pointer; reads (Libertadores)')]
INTERESSA = ['CMRTE']
