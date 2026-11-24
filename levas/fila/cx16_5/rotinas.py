# Os destrutores das eliminatórias da CONCACAF, da Europa e da Oceania, os três destrutores escalares e os textos do
# progresso nas três (leitor cx16, rodada 5, 24/11/2026). Item de TELAS, como o cx06_1: os destrutores rodam quando o
# jogo é desmontado (carregar outro save com um jogo aberto, ou sair da carreira; as seções existem em qualquer
# carreira); o texto do progresso é o da tela de histórico de uma seleção (virtual +0x38). Caminho no jogo: com uma
# carreira aberta, abrir o histórico de uma seleção da CONCACAF, da Europa e da Oceania (Nação > Seleção > Histórico; as
# fases das eliminatórias aparecem nas seleções que jogaram; a Oceania jogou a campanha de 2000 em 2001) e depois
# carregar outro save, jogando comparando.
# Destrutores (thiscall, ret 0; prólogo push -1; push imm32, 7 bytes; o quadro SEH só cobre o desenrolar):
# - 0x914cc0 WcConcacafCup: a vtable 0x970c24, free +0xa7 e +0xa3 (sem zerar), as tabelas de jogos +0xad (cada uma e o
#   vetor, sem zerar), as fases 0..+0x30 (slot 0 com 1) e free(+0xc) zerado, free(+0xb2), Comp_Stats::~Comp_Stats
#   0x49f450 + operator delete 0x944c94, Cup::~Cup 0x518690.
# - 0x9184c0 WcEuropeLeague: a vtable 0x970cc4, League::free_table 0x687970 (push 0), free(+0xba) zerado, as fases e
#   free(+0xc) zerado, a lista +0x10 (LLIST::~LLIST 0x68d040 + operator delete), Comp_Stats, free(+0xf7), free(+0xee),
#   League::~League 0x682300.
# - 0x91e200 WcOceaniaLeague: a vtable 0x970d78, free_table(0), free(+0xba) zerado, free(+0xef), as fases e free(+0xc)
#   zerado, Comp_Stats, League::~League.
# Destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete; mov eax, esi; ret 4):
# 0x914ca0, 0x9184a0, 0x91e1e0; prólogo com 3 bytes antes de um call rel32: RELATIVAS (endereço, 3), ver
# INTEGRAR_ANTES.txt.
# Textos (thiscall(buf, tamanho, rótulo da fase, código da rodada, situação, competição) ret 0x18; o ecx não é lido;
# langlib_sprintf 0x66f4e0, cdecl; buffer menor que 30: a caixa; prólogo mov eax, [esp+8]; sub esp, 0x200, 10 bytes):
# - 0x916af0 WcConcacafCup: rodada 10/20/30 "First/Second/Third Round"; rótulo 0x473..0x474 "Central American Group
#   Stage"; rodada 0x172 "Interzone Playoff"; rótulo 0x40c..0x40e situação 6 "Qualified For Final" senão "Semi Final
#   Group Stage"; rótulo 0x96 situação 7 "Qualified For World Cup" senão "Final Group Stage"; senão nada (caixa 0x527).
# - 0x9192c0 WcEuropeLeague: rótulo 0x3e8..0x3f5 situação 7 "Qualified For World Cup", 1 "Playoff", senão "Group
#   Stage"; senão rodada 0xa0 situação 1 "Qualified For World Cup" senão "Playoff"; senão nada (caixa 0x2fc).
# - 0x91edc0 WcOceaniaLeague: rótulo 0x3e8..0x3e9 "First Round"; rótulo 0 situação 1 "Qualified For Asia Playoff"
#   (defeito do original [inferência]: a repescagem da Oceania é contra a América do Sul), 2 "Final", senão "Semi
#   Finals"; rótulo 0xa0 situação 1 "Qualified For World Cup" senão "Playoff"; senão nada (caixa 0x290).
# Convenções conferidas. Iguais no cm0102-1080.exe (inteiras). Comparadores: os destrutores só a original; os textos lado
# a lado (a cópia num buffer local do mesmo tamanho).
SAVE = False
TELAS = True
ROTINAS = [('drop_wc_concacaf_cup', 0x914cc0, [0x6a, 0xff, 0x68, 0x48, 0x58, 0x96, 0x00]),
           ('drop_wc_europe_league', 0x9184c0, [0x6a, 0xff, 0x68, 0x78, 0x59, 0x96, 0x00]),
           ('drop_wc_oceania_league', 0x91e200, [0x6a, 0xff, 0x68, 0xd8, 0x5a, 0x96, 0x00]),
           ('delete_wc_concacaf_cup', 0x914ca0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_wc_europe_league', 0x9184a0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_wc_oceania_league', 0x91e1e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('wc_concacaf_cup_write_progress_text', 0x916af0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_europe_league_write_progress_text', 0x9192c0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_oceania_league_write_progress_text', 0x91edc0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os destrutores e os textos do progresso das eliminatórias da CONCACAF, da Europa e da Oceania',
        'the destructors and progress texts of the CONCACAF, European and Oceania qualifying',
        [('drop_wc_concacaf_cup', 'WcConcacafCup::~WcConcacafCup', '0x914cc0', 'wc_concacaf_cup.cpp', 'thiscall',
          'desmonta as eliminatórias da CONCACAF: libera as tabelas de times, rodadas e jogos, apaga as fases e libera o vetor delas, libera a lista de inscritas e as estatísticas e roda o destrutor da copa',
          'destroys the CONCACAF qualifying: frees the team, round and tie tables, destroys the stages and frees their vector, frees the entrant list and the statistics and runs the cup destructor'),
         ('drop_wc_europe_league', 'WcEuropeLeague::~WcEuropeLeague', '0x9184c0', 'wc_europe_league.cpp', 'thiscall',
          'desmonta as eliminatórias europeias: libera a tabela e as datas, apaga as fases e libera o vetor delas, apaga a lista de notícias, libera as estatísticas, os grupos sorteados e a lista de inscritas e roda o destrutor da liga',
          'destroys the European qualifying: frees the table and the dates, destroys the stages and frees their vector, deletes the news list, frees the statistics, the drawn groups and the entry pool and runs the league destructor'),
         ('drop_wc_oceania_league', 'WcOceaniaLeague::~WcOceaniaLeague', '0x91e200', 'wc_oceania_league.cpp', 'thiscall',
          'desmonta as eliminatórias da Oceania: libera a tabela, as datas e a lista de inscritas, apaga as fases e libera o vetor delas, libera as estatísticas e roda o destrutor da liga',
          'destroys the Oceania qualifying: frees the table, the dates and the entrant pool, destroys the stages and frees their vector, frees the statistics and runs the league destructor'),
         ('delete_wc_concacaf_cup', 'WcConcacafCup::scalar_deleting_destructor', '0x914ca0', 'wc_concacaf_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor das eliminatórias da CONCACAF e, com o bit 0, apaga o objeto',
          'runs the CONCACAF qualifying destructor and, with bit 0, deletes the object'),
         ('delete_wc_europe_league', 'WcEuropeLeague::scalar_deleting_destructor', '0x9184a0', 'wc_europe_league.cpp', 'thiscall, ret 4',
          'roda o destrutor das eliminatórias europeias e, com o bit 0, apaga o objeto',
          'runs the European qualifying destructor and, with bit 0, deletes the object'),
         ('delete_wc_oceania_league', 'WcOceaniaLeague::scalar_deleting_destructor', '0x91e1e0', 'wc_oceania_league.cpp', 'thiscall, ret 4',
          'roda o destrutor das eliminatórias da Oceania e, com o bit 0, apaga o objeto',
          'runs the Oceania qualifying destructor and, with bit 0, deletes the object'),
         ('wc_concacaf_cup_write_progress_text', 'WcConcacafCup::write_progress_text', '0x916af0', 'wc_concacaf_cup.cpp', 'thiscall, ret 0x18',
          'regra: o progresso de uma seleção nas eliminatórias da CONCACAF no histórico: as três rodadas do mata-mata caribenho, a fase de grupos da América Central, a repescagem interzonal, a fase de grupos semifinal (ou classificada para o grupo final) e o grupo final (ou classificada para a Copa)',
          'rule: a nation progress in the CONCACAF qualifying in its history: the three Caribbean knockout rounds, the Central American group stage, the interzone play-off, the semi-final group stage (or qualified for the final group) and the final group (or qualified for the World Cup)'),
         ('wc_europe_league_write_progress_text', 'WcEuropeLeague::write_progress_text', '0x9192c0', 'wc_europe_league.cpp', 'thiscall, ret 0x18',
          'regra: o progresso de uma seleção nas eliminatórias europeias no histórico: a fase de grupos (classificada para a Copa, para a repescagem ou só a fase) e a repescagem (classificada para a Copa ou só a repescagem)',
          'rule: a nation progress in the European qualifying in its history: the group stage (qualified for the World Cup, for the play-off or just the stage) and the play-off (qualified for the World Cup or just the play-off)'),
         ('wc_oceania_league_write_progress_text', 'WcOceaniaLeague::write_progress_text', '0x91edc0', 'wc_oceania_league.cpp', 'thiscall, ret 0x18',
          'regra: o progresso de uma seleção nas eliminatórias da Oceania no histórico: a primeira fase, a final (classificada para a repescagem, finalista ou semifinal) e a repescagem (classificada para a Copa ou só a repescagem); o texto da classificada na final fala da Ásia (defeito do original)',
          'rule: a nation progress in the Oceania qualifying in its history: the first round, the final (qualified for the play-off, finalist or semi-final) and the play-off (qualified for the World Cup or just the play-off); the final winner text says Asia (defect of the original)')])
CAMPOS = []
INTERESSA = []
