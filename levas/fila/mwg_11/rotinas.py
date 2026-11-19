# As viradas de temporada da Copa Interamericana e do Campeonato de Clubes da Oceania e a alternância da sede da
# Oceania (inter_amer_cup.cpp, oceania_club_champ.cpp em eurocomp\, host_country.cpp; leitor mwg, rodada 3 do mundo
# inteiro, 19/11/2026). Cada uma roda uma vez até 2002-08-01 (cobertura_mundo.txt). O Campeonato da Oceania foi provado
# com a Austrália jogável (cqt_10): LIGAS = 'ENG AUS' (a Interamericana é criada pelo Setup sem depender das nações,
# 0x831282, e vai junto).
# Convenções conferidas na desmontagem (0x632910..0x632ab8, 0x797d60..0x797de8, 0x5face0..0x5fad1c):
# - InterAmerCup::start_new_season: thiscall, ret 0; o molde start_new_cup_season inteiro (os clubes 0x632480
#   InterAmerCup::select_teams; caixas nas linhas 0x164, 0x16b e 0x172);
# - OceaniaClubChamp::start_new_season: thiscall, ret 0, sobre League (o molde da Copa América, GoldCup 0x5e1530): +0x4c
#   = 0, League::free_table(0) (0x687970, push 0 -> thiscall, ret 4), +0xba liberado e zerado, Comp_Stats::
#   archive_season_and_reset(1) (0x4a1c50), as fases 0..=+0x30 destruídas (virtual 0 com 1), o ano +0x40 + 2, +0x30 =
#   -1, +0x45 = 0, HostCountry::alternate_oceania_club_champ_host (mov ecx, [0xb5e134] -> thiscall, ret 0) e a virtual
#   +0x8c (as regras, sem caixa), cujo eax devolve;
# - HostCountry::alternate_oceania_club_champ_host: thiscall, ret 0; [this] = as entradas de 0x22 bytes, +4 a contagem
#   (palavra com sinal); a entrada da Oceania ([0x9cf7cc]) troca a sede +6: a Austrália ([0x9cf224]) vira a Nova
#   Zelândia ([0x9cf3f4]), qualquer outra vira a Austrália.
# Comparadores: as viradas só no original (liberam e refazem: o ano + 1 na Interamericana, + 2 e +0x4c = 0 na Oceania);
# a alternância lado a lado sobre as entradas da tabela de sedes.
SAVE = False
LIGAS = 'ENG AUS'
ATE = '2002-08-01'
ROTINAS = [('inter_amer_cup_start_new_season', 0x632910, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('oceania_club_champ_start_new_season', 0x797d60, [0x56, 0x8b, 0xf1, 0x57, 0x6a, 0x00]),
           ('alternate_oceania_club_champ_host', 0x5face0, [0x8b, 0x01, 0x33, 0xd2, 0x66, 0x39, 0x51, 0x04])]
DOCS = ('as viradas da Copa Interamericana e do Campeonato de Clubes da Oceania e a sede alternada da Oceania',
        'the turnovers of the Inter-American Cup and the Oceania Club Championship and the alternating Oceania host',
        [('inter_amer_cup_start_new_season', 'InterAmerCup::start_new_season', '0x632910', 'inter_amer_cup.cpp', 'thiscall, ret 0',
          'a virada da Copa Interamericana: o molde das viradas de copa (tabelas liberadas, temporada arquivada, ano + 1, os clubes, a tabela de rodadas e a árvore com as caixas nas linhas 0x164, 0x16b e 0x172, e as posições previstas)',
          "the Inter-American Cup's turnover: the cup turnover template (tables freed, season archived, year + 1, the clubs, the round table and the tree with the boxes at lines 0x164, 0x16b and 0x172, and the predicted positions)"),
         ('oceania_club_champ_start_new_season', 'OceaniaClubChamp::start_new_season', '0x797d60', 'oceania_club_champ.cpp', 'thiscall, ret 0',
          'o Campeonato de Clubes da Oceania é a cada 2 anos: na virada a tabela e as datas das rodadas liberadas, a temporada arquivada, as fases destruídas, o ano + 2, a fase atual −1, o sorteio por fazer, a sede alternada entre a Austrália e a Nova Zelândia e as regras de novo',
          'the Oceania Club Championship is every 2 years: at the turnover the table and the round dates freed, the season archived, the stages destroyed, the year + 2, the current stage −1, the draw still to do, the host alternated between Australia and New Zealand and the rules again'),
         ('alternate_oceania_club_champ_host', 'HostCountry::alternate_oceania_club_champ_host', '0x5face0', 'host_country.cpp', 'thiscall, ret 0',
          'a sede do Campeonato de Clubes da Oceania alterna a cada edição: em cada entrada dele na tabela de sedes, a Austrália vira a Nova Zelândia e qualquer outra sede vira a Austrália',
          'the Oceania Club Championship host alternates every edition: in each of its entries in the host table, Australia becomes New Zealand and any other host becomes Australia')])
