# Os destrutores das copas continentais, da tabela do ranking da FIFA e da lista em arquivo (european_cup.cpp,
# inter_amer_cup.cpp, intertoto_cup.cpp, uefa_cup.cpp, world_club_cup.cpp, oceania_club_champ.cpp, world_club_champ.cpp,
# fifa_rankings.cpp; leitor cx04, rodada 5, 24/11/2026). Item de TELAS, como os destrutores do cx02_1: rodam quando o
# jogo e desmontado (carregar outro save com um jogo aberto, ou sair da carreira). Caminho no jogo: com uma carreira
# aberta (as europeias vem com a Inglaterra; a da Oceania e o Mundial de Clubes com as ligas da Australia e as outras
# carregadas), carregar outro save, jogando comparando.
# - 0x583a00 EuropeanCup::~EuropeanCup thiscall() ret 0: a vtable 0x96a1d0, free(+0xa7), free(+0xa3), os confrontos de
#   cada rodada e o vetor (+0xad, +0x3c rodadas), o slot 0 das fases ate a atual (+0x30, push 1), free(+0xc) e zera,
#   Comp_Stats::~Comp_Stats 0x49f450 e operator delete 0x944c94, Cup::~Cup 0x518690.
# - 0x632270, 0x6331c0, 0x9024f0, 0x92b6a0 (InterAmerCup, IntertotoCup, UefaCup, WorldClubCup): o molde das copas
#   nacionais (drop_national_cup do cx01_14) com as vtables 0x96bad8, 0x96bb8c, 0x970790, 0x971250.
# - 0x797170 OceaniaClubChamp e 0x9292d0 WorldClubChamp: a vtable (0x96e018, 0x97119c), League::free_table 0x687970
#   (push 0), free(+0xba) e zera, o slot 0 das fases (push 1), free(+0xc) e zera, as estatisticas, League::~League
#   0x682300 (o corpo do drop_bra_champ_cup do cx01_14, aqui no drop_stats_league).
# - 0x58cf20 FIFA_RANKINGS::~FIFA_RANKINGS thiscall() ret 0: com tabela, free(+4) de cada entrada de 0x2c bytes
#   ([0xae2350] nacoes) e free da tabela. Prologo push ebx; mov ebx, ecx; cmp dword [ebx], 0 (6 bytes).
# - 0x58e330 FILE_LLIST::~FILE_LLIST thiscall() ret 0: free(+0) se houver. Prologo mov eax, [ecx]; test eax, eax; je
#   rel8 (6 bytes, salto curto no 4): RELATIVAS (0x58e330, 4) no rust/src/detours.rs.
# Convencoes conferidas: free 0x9452ca (cdecl), operator delete 0x944c94 (cdecl), Comp_Stats::~Comp_Stats 0x49f450,
# Cup::~Cup 0x518690, League::~League 0x682300 (thiscall, ret 0), League::free_table 0x687970 (thiscall, ret 4). O
# quadro SEH so cobre o desenrolar de excecao (nao portado). Iguais no cm0102-1080.exe (todas inteiras).
# Ficam de fora os destrutores escalares (0x5839e0, 0x632250, 0x6331a0, 0x9024d0, 0x797150, 0x9292b0, 0x92b680: push
# esi; mov esi, ecx; call ~X, 3 bytes antes de um call rel32; entrariam pelas RELATIVAS com k = 3).
# - 0x586fa0 EuropeanCup::write_progress_text thiscall(buf, tamanho, rodada, sub-rodada, situacao, competicao) ret 0x18
#   (virtual 0x38 da Liga dos Campeoes; o ecx nao e lido): o nome do progresso de um clube (fases de grupos pelo pai da
#   rodada 0x4b01d0, cdecl; Winners, Runners Up, Third Placed; senao 0x4bffc0 cdecl(rodada, sub, buf, tamanho, 0)); buffer
#   menor que 30: a caixa (linha 0x950). Texto das telas de historico (quem chama e virtual). Prologo sub esp, 0x200.
# Comparadores: os destrutores so a original (liberam memoria); o texto do progresso lado a lado (o original no buffer
# de quem chama, a copia num local do mesmo tamanho).
# O FILE_LLIST::~FILE_LLIST roda tambem na carreira (Comp_Stats::~Comp_Stats 0x49f4c8 e outros), so com a original no
# comparador.
# Usa drop_national_cup e delete_comp_stats do cx01_14: integrar depois dele.
SAVE = False
TELAS = True
ROTINAS = [('drop_european_cup', 0x583a00, [0x6a, 0xff, 0x68, 0x98, 0x8a, 0x95, 0x00]),
           ('drop_inter_amer_cup', 0x632270, [0x6a, 0xff, 0x68, 0x38, 0xb2, 0x95, 0x00]),
           ('drop_intertoto_cup', 0x6331c0, [0x6a, 0xff, 0x68, 0x08, 0xb3, 0x95, 0x00]),
           ('drop_uefa_cup', 0x9024f0, [0x6a, 0xff, 0x68, 0xf8, 0x51, 0x96, 0x00]),
           ('drop_world_club_cup', 0x92b6a0, [0x6a, 0xff, 0x68, 0x58, 0x5f, 0x96, 0x00]),
           ('drop_oceania_club_champ', 0x797170, [0x6a, 0xff, 0x68, 0xc8, 0xea, 0x95, 0x00]),
           ('drop_world_club_champ', 0x9292d0, [0x6a, 0xff, 0x68, 0x88, 0x5e, 0x96, 0x00]),
           ('drop_fifa_rankings', 0x58cf20, [0x53, 0x8b, 0xd9, 0x83, 0x3b, 0x00]),
           ('drop_file_llist', 0x58e330, [0x8b, 0x01, 0x85, 0xc0, 0x74, 0x07]),
           ('european_cup_write_progress_text', 0x586fa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os destrutores das copas continentais, do ranking da FIFA e da lista em arquivo, e o texto do progresso na Liga dos Campeões',
        'the destructors of the continental cups, the FIFA ranking and the file-backed list, and the Champions League progress text',
        [('drop_european_cup', 'EuropeanCup::~EuropeanCup', '0x583a00', 'european_cup.cpp', 'thiscall, ret 0',
          'desmonta a Liga dos Campeões: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada, cada fase até a atual (as fases de grupos), o vetor das fases e as estatísticas e roda o destrutor da copa',
          'destroys the Champions League: frees the team table, the round table, each round\'s ties, every stage up to the current one (the group stages), the stage vector and the statistics and runs the cup destructor'),
         ('drop_inter_amer_cup', 'InterAmerCup::~InterAmerCup', '0x632270', 'inter_amer_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa Interamericana: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Inter-American Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_intertoto_cup', 'IntertotoCup::~IntertotoCup', '0x6331c0', 'intertoto_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa Intertoto: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Intertoto Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_uefa_cup', 'UefaCup::~UefaCup', '0x9024f0', 'uefa_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa da UEFA: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the UEFA Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_world_club_cup', 'WorldClubCup::~WorldClubCup', '0x92b6a0', 'world_club_cup.cpp', 'thiscall, ret 0',
          'desmonta a Copa Intercontinental: libera a tabela de times, a tabela de rodadas, os confrontos de cada rodada e as estatísticas e roda o destrutor da copa',
          'destroys the Intercontinental Cup: frees the team table, the round table, each round\'s ties and the statistics and runs the cup destructor'),
         ('drop_oceania_club_champ', 'OceaniaClubChamp::~OceaniaClubChamp', '0x797170', 'oceania_club_champ.cpp', 'thiscall, ret 0',
          'desmonta o Campeonato de Clubes da Oceania: libera a tabela, as datas das rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Oceania Club Championship: frees the table, the round dates, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('drop_world_club_champ', 'WorldClubChamp::~WorldClubChamp', '0x9292d0', 'world_club_champ.cpp', 'thiscall, ret 0',
          'desmonta o Mundial de Clubes: libera a tabela, as datas das rodadas, cada fase até a atual, o vetor das fases e as estatísticas e roda o destrutor da liga',
          'destroys the Club World Championship: frees the table, the round dates, every stage up to the current one, the stage vector and the statistics and runs the league destructor'),
         ('drop_fifa_rankings', 'FIFA_RANKINGS::~FIFA_RANKINGS', '0x58cf20', 'fifa_rankings.cpp', 'thiscall, ret 0',
          'desmonta a tabela do ranking da FIFA: libera o histórico de resultados de cada nação e a tabela',
          'destroys the FIFA ranking table: frees each nation\'s results history and the table'),
         ('drop_file_llist', 'FILE_LLIST::~FILE_LLIST', '0x58e330', 'fifa_rankings.cpp', 'thiscall, ret 0',
          'desmonta uma lista gravada em arquivo: libera o caminho do arquivo copiado pelo construtor',
          'destroys a file-backed list: frees the file path the constructor copied'),
         ('european_cup_write_progress_text', 'EuropeanCup::write_progress_text', '0x586fa0', 'european_cup.cpp', 'thiscall, ret 0x18',
          'escreve o nome do progresso de um clube na Liga dos Campeões: nas fases de grupos, "Classificado da Fase 1/2" (situações 0, 6, 7 e 8) ou "Fase de Grupos 1/2"; senão "Campeão" (situação 1), sem rodada "Vice" (2) ou "Terceiro" (3), e nos outros casos os nomes da fase e da rodada; buffer menor que 30 bytes: a caixa de erro',
          'writes the name of a club\'s progress in the Champions League: in the group stages, "Qualified From Phase 1/2" (statuses 0, 6, 7 and 8) or "Phase 1/2 Group Stage"; otherwise "Winners" (status 1), with no round "Runners Up" (2) or "Third Placed" (3), and in the other cases the stage and round names; a buffer under 30 bytes: the error box')])
CAMPOS = []
INTERESSA = []
