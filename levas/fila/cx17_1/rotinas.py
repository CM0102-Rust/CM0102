# Os destrutores das eliminatórias sul-americanas e da Copa do Mundo e o texto do progresso nas sul-americanas
# (wc_south_american_league.cpp, world_cup.cpp; leitor cx17, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do
# cx04_1 e do cx16_5: rodam quando o jogo é desmontado (carregar outro save com um jogo aberto, ou sair da carreira); as
# duas competições existem em qualquer carreira. Caminho no jogo: com uma carreira aberta, carregar outro save, jogando
# comparando; o texto do progresso, no histórico de uma seleção sul-americana (as telas de competições da seleção).
# Destrutores (thiscall, ret 0; prólogo push -1; push imm32, 7 bytes; o quadro SEH só cobre o desenrolar):
# - 0x920500 WcSouthAmericanLeague: a vtable 0x970e2c, League::free_table 0x687970 (push 0), free(+0xba) zerado,
#   Comp_Stats::~Comp_Stats 0x49f450 + operator delete 0x944c94 (sem zerar +8), League::~League 0x682300. Sem fases.
# - 0x92c1d0 WorldCup: a vtable 0x9712f0 e o corpo do drop_stats_league do cx04_1 (free_table(0), free(+0xba) zerado, as
#   fases 0..=+0x30 com a virtual 0 e 1, free(+0xc) zerado, Comp_Stats, League::~League).
# Destrutores escalares (push esi; mov esi, ecx; call ~X; test byte [esp+8], 1; operator delete; mov eax, esi; ret 4):
# 0x9204e0 e 0x92c1b0; prólogo com 3 bytes antes de um call rel32 (e8 18 00 00 00 nos dois): RELATIVAS (endereço, 3),
# ver INTEGRAR_ANTES.txt.
# - 0x921390 WcSouthAmericanLeague::write_progress_text thiscall(buf, tamanho, rodada, sub-rodada, situacao,
#   competicao) ret 0x18 (virtual +0x38; o ecx não é lido): tamanho < 30 (com sinal): a caixa (linha 0x2ae); situação 7:
#   langlib_sprintf(buf, 0xad4658 "Qualified {}<%s - Ignore>For World Cup", gênero +0x37 da competição, 0xad9c64 "");
#   senão langlib_sprintf(buf, 0xad4ddc "League Stage"). Prólogo mov eax, [esp+8]; sub esp, 0x200 (10 bytes).
# Convenções conferidas (free e operator delete cdecl; os demais thiscall). Iguais no cm0102-1080.exe (inteiras).
# Comparadores: os destrutores só a original (liberam memória); o texto do progresso lado a lado (a original no buffer de
# quem chama, a cópia num local do mesmo tamanho).
SAVE = False
TELAS = True
ROTINAS = [('drop_wc_south_american_league', 0x920500, [0x6a, 0xff, 0x68, 0xc8, 0x5b, 0x96, 0x00]),
           ('drop_world_cup', 0x92c1d0, [0x6a, 0xff, 0x68, 0xa8, 0x5f, 0x96, 0x00]),
           ('delete_wc_south_american_league', 0x9204e0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_world_cup', 0x92c1b0, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('wc_south_american_league_write_progress_text', 0x921390, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os destrutores das eliminatórias sul-americanas e da Copa do Mundo e o texto do progresso nas sul-americanas',
        'the destructors of the South American qualifying and of the World Cup, and the South American progress text',
        [('drop_wc_south_american_league', 'WcSouthAmericanLeague::~WcSouthAmericanLeague', '0x920500', 'wc_south_american_league.cpp', 'thiscall',
          'desmonta as eliminatórias sul-americanas: libera a tabela e as datas, apaga as estatísticas e roda o destrutor da liga',
          'destroys the South American qualifying: frees the table and the dates, deletes the statistics and runs the league destructor'),
         ('drop_world_cup', 'WorldCup::~WorldCup', '0x92c1d0', 'world_cup.cpp', 'thiscall',
          'desmonta a Copa do Mundo: libera a tabela e as datas, apaga as fases e libera o vetor delas, apaga as estatísticas e roda o destrutor da liga',
          'destroys the World Cup: frees the table and the dates, deletes the stages and frees their vector, deletes the statistics and runs the league destructor'),
         ('delete_wc_south_american_league', 'WcSouthAmericanLeague::scalar_deleting_destructor', '0x9204e0', 'wc_south_american_league.cpp', 'thiscall, ret 4',
          'roda o destrutor das eliminatórias sul-americanas e, com o bit 0, apaga o objeto',
          'runs the South American qualifying destructor and, with bit 0, deletes the object'),
         ('delete_world_cup', 'WorldCup::scalar_deleting_destructor', '0x92c1b0', 'world_cup.cpp', 'thiscall, ret 4',
          'roda o destrutor da Copa do Mundo e, com o bit 0, apaga o objeto',
          'runs the World Cup destructor and, with bit 0, deletes the object'),
         ('wc_south_american_league_write_progress_text', 'WcSouthAmericanLeague::write_progress_text', '0x921390', 'wc_south_american_league.cpp', 'thiscall, ret 0x18',
          'regra: o progresso de uma seleção nas eliminatórias sul-americanas é "Classificada para a Copa do Mundo" com a situação 7 e "Fase de liga" nas outras',
          'rule: a nation progress in the South American qualifying reads "Qualified For World Cup" with status 7 and "League Stage" otherwise')])
CAMPOS = []
INTERESSA = []
