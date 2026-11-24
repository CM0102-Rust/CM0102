# A minicopa e a minliga dos amistosos refeitas do jogo salvo (mini_cup.cpp, mini_league.cpp; leitor cx05, rodada 5,
# 24/11/2026). Item do SAVE (CARGA): os construtores de carga de cada torneio vivo no save (chamados ao carregar as
# competicoes; 0 chamadas na carreira automatica).
# - 0x765e30 MiniCup::MiniCup thiscall(registro) ret 4: Cup::Cup 0x518640, Date::Date 0x54a110, Cup::load_from_disk
#   0x51fc00 (push 1; mov ecx -> thiscall, ret 4). Prologo push -1; push imm32 (7 bytes).
# - 0x766800 MiniLeague::MiniLeague thiscall(registro) ret 4: League::League 0x682200, Date::Date,
#   League::load_from_temp_file 0x687b10 (push 1 -> thiscall, ret 4; 0: a caixa da linha 0x85). Prologo push -1; push
#   imm32 (7 bytes).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Duvida para a prova: o save de ferias so tem minicopas e minligas vivas se for gravado com torneios de
# pre-temporada em andamento (julho); sem elas as duas nao rodam na prova do save.
# Comparadores: so a original (le o arquivo e aloca), com o plano: a vtable e o registro.
SAVE = True
CARGA = True
ROTINAS = [('load_mini_cup', 0x765e30, [0x6a, 0xff, 0x68, 0x86, 0xda, 0x95, 0x00]),
           ('load_mini_league', 0x766800, [0x6a, 0xff, 0x68, 0x2c, 0xdb, 0x95, 0x00])]
DOCS = ('a minicopa e a minliga dos amistosos refeitas do jogo salvo', 'the friendly mini cup and mini league rebuilt from the saved game',
        [('load_mini_cup', 'MiniCup::MiniCup', '0x765e30', 'mini_cup.cpp', 'thiscall, ret 4',
          'refaz uma minicopa do jogo salvo: a base da copa, a data de início e o estado lido do arquivo temporário da copa',
          'rebuilds a mini cup from the saved game: the cup base, the start date and the state read from the cup\'s temporary file'),
         ('load_mini_league', 'MiniLeague::MiniLeague', '0x766800', 'mini_league.cpp', 'thiscall, ret 4',
          'refaz uma minliga do jogo salvo: a base da liga, a data de início, 4 clubes e 3 rodadas e o estado lido do arquivo temporário da liga; leitura falha: a caixa de erro',
          'rebuilds a mini league from the saved game: the league base, the start date, 4 clubs and 3 rounds and the state read from the league\'s temporary file; a failed read: the error box')])
CAMPOS = []
INTERESSA = []
