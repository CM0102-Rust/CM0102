# O construtor da base League e um jogo fixo pelos nomes dos clubes (league.cpp; leitor cx07, rodada 5, 24/11/2026).
# Simulacao com as 8 ligas: o construtor roda na criacao do jogo (toda liga, fase de liga e selecao em torneio: chamado
# pelos construtores, exe::new_league); o jogo fixo na criacao das ligas com classicos fixos (ItaSerA::create_fixed_
# fixtures 0x642a70 e FraFirst, ja em Rust: cqn_5, cqn_6).
# - 0x682200 League::League thiscall() ret 0 -> this: Comp::Comp 0x48c580 (mov ecx -> thiscall, pelo desvio do cx01_1),
#   Date::Date 0x54a110 em +0xc9 e +0xd1 (lea ecx -> thiscall, ret 0), a vtable 0x96d020 e os campos. O quadro SEH so
#   cobre o desenrolar (nao portado). Prologo push -1; push imm32 (7 bytes).
# - 0x68aff0 fixed_fixture_set_by_club_names thiscall(liga, dia, mes, ano, mandante, visitante) ret 0x18 (this = a
#   entrada de 0x10 bytes): Date::set 0x54a130 (push -1; push ano; push mes; push dia -> thiscall, ret 0x10); +0xc e +8 =
#   -1; as linhas da tabela com club_get_english_short_name 0x545770 (cdecl, ja em Rust) e o strcmp inline; para quando
#   acha os dois. Prologo mov eax, [esp+0x10]; mov edx, [esp+8] (8 bytes).
# Iguais no cm0102-1080.exe (as duas inteiras).
# O LEAGUE_VTABLE e o LEAGUE_OBJECT_SIZE daqui sao usados pelo cx07_1.
# Comparadores: lado a lado sobre os 0xee bytes do objeto e o retorno; lado a lado sobre os 0x10 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('new_league', 0x682200, [0x6a, 0xff, 0x68, 0x26, 0xc9, 0x95, 0x00]),
           ('fixed_fixture_set_by_club_names', 0x68aff0, [0x8b, 0x44, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('o construtor da base da liga e um jogo fixo pelos nomes dos clubes',
        'the league base constructor and a fixed fixture by the clubs\' names',
        [('new_league', 'League::League', '0x682200', 'league.cpp', 'thiscall, ret 0',
          'monta a base de toda liga: a base da competição, as datas do último resultado e do último dia, o tipo liga, a categoria de jogo 2 e o modo de prêmio 2; sem dia de sorteio, jogos por clube, capacidade ou assentos exigidos (−1); as regras de base 3 (situações de campeão e de acesso); sem fases, tabela, critérios, jogos fixos nem jogos-desempate',
          'builds the base of every league: the competition base, the last result and last update dates, the league type, match category 2 and prize mode 2; no draw day, games per club, required capacity or seats (−1); the default rule bits 3 (champion and promotion statuses); no stages, table, tie-breaks, fixed fixtures or tie-break matches'),
         ('fixed_fixture_set_by_club_names', 'fixed_fixture_set_by_club_names', '0x68aff0', 'league.cpp', 'thiscall, ret 0x18',
          'monta um jogo fixo de uma liga pela data e pelos nomes dos dois clubes: procura cada nome curto (o inglês, com outro idioma carregado) na tabela da liga e grava os ids do mandante e do visitante (−1 se não achar)',
          "sets a league's fixed fixture from its date and the two clubs' names: each short name (the English one when another language is loaded) is looked up in the league table and the home and away ids stored (−1 when not found)")])
CAMPOS = [('jogo fixo +8 id do mandante; grava', 'fixed fixture +8 home club id; writes')]
INTERESSA = []
