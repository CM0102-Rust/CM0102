# O construtor, a virada e a fase seguinte do Mundial de Clubes (world_club_champ.cpp; leitor rx10, rodada 6,
# 25/11/2026). Simulacao com as 8 ligas e ATE 2003-08-15, como o cx04_6: a primeira edicao e em 2003 (o sorteio em
# 5/fev, os grupos de 29/jul a 5/ago, o mata-mata ate 12/ago); o construtor roda na criacao do jogo.
# - 0x929140 WorldClubChamp::WorldClubChamp thiscall(ano, registro) ret 8: League::League, vtable 0x97119c, +0x42 = 2,
#   +0x1c = +0x20 = -1, o ano levado ao primeiro impar >= 2003 (resto com sinal), +0x50 = 0xff, +0x51 = 15; 0x687b10(1)
#   termina; senao o dword +0x45 = 0, +0x44 = -1, +0x30 = -1, +0x2c = 3, malloc 0xc (sem teste), estatisticas (caixa
#   0x54) e WorldClubChamp::set_up_rules 0x929bb0 direto. Prologo push -1; push 0x965e69 (7 bytes).
# - 0x929d40 WorldClubChamp::start_new_season thiscall() ret 0: o molde do Oceania (oceania_club_champ_start_new_season)
#   com o ano + 2, a fase -1, HostCountry::redraw_world_club_and_confed_hosts 0x5fab70 em [0xb5e134], o dword +0x45 = 0
#   e as regras pela virtual +0x8c (devolve o eax delas). Prologo push esi; mov esi, ecx; push edi; push 0 (6 bytes).
# - 0x929dd0 WorldClubChamp::advance_stage thiscall() ret 0: fase >= +0x2c - 1: caixa 0x2df; senao fase + 1 e, na 2,
#   WorldClubChamp::create_knockout_stage 0x92a140 direto. Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# Iguais no cm0102-1080.exe. Comparadores: os tres so a original com o plano.
# A virada e o mata-mata ficam sem mutacao propria: a virada roda depois do fim (agosto de 2003, alem do que ela monta);
# mudar a fase seguinte deixa a fase 2 sem objeto (o jogo cai).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('construct_world_club_champ', 0x929140, [0x6a, 0xff, 0x68, 0x69, 0x5e, 0x96, 0x00]),
           ('world_club_champ_start_new_season', 0x929d40, [0x56, 0x8b, 0xf1, 0x57, 0x6a, 0x00]),
           ('world_club_champ_advance_stage', 0x929dd0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('o construtor, a virada e a fase seguinte do Mundial de Clubes',
        'the Club World Championship constructor, season turnover and next stage',
        [('construct_world_club_champ', 'WorldClubChamp::WorldClubChamp', '0x929140', 'world_club_champ.cpp', 'thiscall, ret 8',
          'regra: o Mundial de Clubes da FIFA é de dois em dois anos, a primeira edição no primeiro ano ímpar a partir de 2003; tem três fases (duas de grupos e o mata-mata), clubes de nações diferentes, sem divisão acima ou abaixo, importância 15 e sem regra disciplinar própria',
          'rule: the FIFA Club World Championship is played every two years, the first edition in the first odd year from 2003 on; it has three stages (two group rounds and the knockout), clubs of different nations, no division above or below, importance 15 and no discipline rule of its own'),
         ('world_club_champ_start_new_season', 'WorldClubChamp::start_new_season', '0x929d40', 'world_club_champ.cpp', 'thiscall, ret 0',
          'regra: quando o Mundial de Clubes acaba, a tabela e as fases são desfeitas, a temporada vai para o histórico das estatísticas, o ano sobe 2, novas sedes são sorteadas (para ele e a Copa das Confederações) e as regras são montadas de novo',
          'rule: when the Club World Championship ends, the table and stages are undone, the season goes to the statistics history, the year goes up 2, new hosts are drawn (for it and the Confederations Cup) and the rules are set up again'),
         ('world_club_champ_advance_stage', 'WorldClubChamp::advance_stage', '0x929dd0', 'world_club_champ.cpp', 'thiscall, ret 0',
          'regra: o Mundial de Clubes passa para a fase seguinte quando a atual acaba; ao chegar à fase 2 (depois das duas fases de grupos) monta o mata-mata; passar da última fase é erro',
          'rule: the Club World Championship moves on one stage when the current one ends; on reaching stage 2 (after the two group rounds) it builds the knockout; going past the last stage is an error')])
CAMPOS = []
INTERESSA = []
