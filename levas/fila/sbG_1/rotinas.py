# O preenchimento da minitabela com os jogos entre os seus clubes (sub_league.cpp; leitor sbG, rodada 5, 24/11/2026).
# Simulacao com as 8 ligas, como o construtor (cqx_14, sub_league.rs): as minitabelas dos segundos colocados das
# eliminatorias em marco de 2002 (9 chamadas do construtor no mundo ate 2002-08-01); so decidem depois de junho, entao a
# mutacao e acusada pelo comparador lado a lado (como a do construtor no cqx_14).
# - 0x88c9f0 SubLeague::add_results_between_members thiscall(rodada minima, rodada maxima, &de, &ate, competicao, maximo,
#   lista) ret 0x1c -> 1: LLIST::LLIST 0x68d020 e ~LLIST 0x68d040 numa lista local; sem lista, FIXTURE_MANAGER::scan_fixtures
#   0x5acee0 (18 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x48: &ano, &dia, &faixa, 1, competicao, minima, maxima,
#   -1 x5 (ou -1 x4 e 0 x3 com minima -1), maximo, lista, 1, -2, -1, 0); o cursor volta a cabeca; LLIST::get_next 0x68d1b0;
#   Date::Date 0x549ef0 (push ano; push dia -> thiscall, ret 8) da data do jogo contra &ate (ano sem sinal, dia com sinal);
#   jogado (+0x43 != 0xff); os dois clubes na tabela; a vaga virtual +4 (push 0; push jogo -> thiscall, ret 8).
# Prologo push -1; push 0x963038 (7 bytes). Igual no cm0102-1080.exe (inteira).
# Comparador: lado a lado, a copia num objeto de rascunho com a tabela copiada (League::copy_table 0x687710), como o
# comparador do construtor.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('sub_league_add_results_between_members', 0x88c9f0, [0x6a, 0xff, 0x68, 0x38, 0x30, 0x96, 0x00])]
DOCS = ('o preenchimento da minitabela com os jogos entre os seus clubes', 'filling a mini league from the matches between its clubs',
        [('sub_league_add_results_between_members', 'SubLeague::add_results_between_members', '0x88c9f0', 'sub_league.cpp', 'thiscall, ret 0x1c',
          'regra: a minitabela recebe os jogos já disputados entre dois dos seus clubes, da competição e das rodadas pedidas, a partir de uma data e até outra (quando dada); um jogo com um clube de fora não conta',
          'rule: the mini league takes the matches already played between two of its clubs, of the given competition and rounds, from one date and up to another (when given); a match with an outside club does not count')])
CAMPOS = []
INTERESSA = []
