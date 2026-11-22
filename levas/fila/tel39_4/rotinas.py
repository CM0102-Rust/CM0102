# Os construtores e o destruidor pequenos do club_screens.cpp (leitor tel39, rodada 2 das telas, 22/11/2026).
# Convencoes conferidas na desmontagem e nos pontos de chamada (todas thiscall, ret sem argumentos):
# - 0x453bc0 match_team_pair_destructor: o vector destructor iterator da CRT (0x9462bb, stdcall) sobre os dois blocos
#   de 0x625 bytes em +0x11f com o destruidor 0x453c10 (ja Rust, match_team_block_destruct); chamada pelo
#   MATCH_DAY::~MATCH_DAY (0x6b4257), pela close_match_screen (0x71b187, 0x71c1d7, 0x71c24e) e pelos funclets de
#   desenrolar: roda na carreira a cada dia de jogos.
# - 0x453c50 rating_record_construct: Date::Date (0x54a110) em +0x14, devolve this; chamada pela
#   club_records_write_record_news_text (0x44c7b1), pelo Comp_Stats::Comp_Stats (0x49ef6b), pela
#   comp_stats_write_record_news_text (0x4a8070) e pelo construtor vetorial em 0x453c30 (por endereco em 0x453c39):
#   roda na carreira (as estatisticas das competicoes e as noticias de recorde).
# - 0x487810 squad_view_settings_construct: 15 bytes de valores fixos, devolve this; chamada pelo
#   HUMAN_RECORD::HUMAN_RECORD (0x5fb6e2, 0x5fb84a): roda ao criar o tecnico humano (inicio da carreira).
# Comparadores: o destruidor so a original (os destruidores dos elementos sao um ret: nao ha resultado a conferir);
# os construtores lado a lado sobre o que gravam e o retorno.
ROTINAS = [('match_team_pair_destructor', 0x453bc0, [0x68, 0x10, 0x3c, 0x45, 0x00]),
           ('rating_record_construct', 0x453c50, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x14]),
           ('squad_view_settings_construct', 0x487810, [0x8b, 0xc1, 0x80, 0xc9, 0xff])]
DOCS = ('os construtores e o destruidor pequenos do club_screens.cpp: o par de times de um registro de partida, um registro com data e as opções da tela do elenco do técnico humano',
        "club_screens.cpp's small constructors and destructor: a match record's team pair, a dated record and a human manager's squad view settings",
        [('match_team_pair_destructor', 'match_team_pair_destructor', '0x453bc0', 'club_screens.cpp', 'thiscall',
          'destrói os dois blocos de time (0x625 bytes, em +0x11f) de um registro de partida; os destruidores das 20 entradas de cada um não fazem nada',
          "destroys a match record's two team blocks (0x625 bytes, at +0x11f); the destructors of each one's 20 entries do nothing"),
         ('rating_record_construct', 'rating_record_construct', '0x453c50', 'club_screens.cpp', 'thiscall',
          'constrói um registro com data em +0x14 (o recorde de nota média e semelhantes): só a data',
          'constructs a record with a date at +0x14 (the average rating record and its kin): only the date'),
         ('squad_view_settings_construct', 'squad_view_settings_construct', '0x487810', 'club_screens.cpp', 'thiscall',
          'constrói as opções da tela do elenco de um técnico humano (15 bytes): visão, ordem e filtros padrão (0x41, 8, 0x154a21, 6, 0xff, 0xff)',
          "constructs a human manager's squad screen view settings (15 bytes): the default view, sort and filters (0x41, 8, 0x154a21, 6, 0xff, 0xff)")])
