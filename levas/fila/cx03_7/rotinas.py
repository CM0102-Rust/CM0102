# O dia da Copa da Russia (rus_cup.cpp; leitor cx03, rodada 5, 24/11/2026). Competicoes da Russia: LIGAS do grupo da
# Polonia/Russia, ate 2002-08-01 (a virada da copa, rus_cup_start_new_season, ja e Rust e rodou na 301a com essas ligas).
# - 0x7e9ec0 RusCup::update_for_date thiscall(hoje, fase) ret 8: com fase != 0 e a virtual +0xc (Comp::all_stages_complete
#   0x48ce10, thiscall, ret 0, al) != 0, a data Date(1, 5, ano de hoje, -1) (0x549f70: lea ecx, local; 4 pushes ->
#   thiscall, ret 0x10; o ano vai numa palavra com lixo nos bytes altos) comparada com hoje (ano, depois o dia): a virtual
#   +8 (start_new_season, thiscall, ret 0); depois Cup::update_for_date 0x51f890 (push fase; push hoje; mov ecx ->
#   thiscall, ret 8), cujo eax devolve.
#   O molde (update_cup_with_turnover_on) serve tambem a SweCup::update_for_date (1 de julho, cx03_12) e a
#   AsiaSuperCup::update_for_date (25 de junho, cx03_13): conferidas instrucao a instrucao, so a data muda.
# Iguais no cm0102-1080.exe.
# Comparador: so a original (a virada libera, aloca e sorteia; o dia joga), com o plano: o ano da copa sobe 1 quando a
# virada era devida, senao fica.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('rus_cup_update_for_date', 0x7e9ec0, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x1c])]
DOCS = ('o dia da Copa da Rússia', "the Russian Cup's day",
        [('rus_cup_update_for_date', 'RusCup::update_for_date', '0x7e9ec0', 'rus_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Rússia começa a edição seguinte em 1º de junho, quando a atual terminou; depois, todo dia, o dia da copa (sorteios e jogos)',
          "rule: the Russian Cup starts its next edition on 1 June once the current one is over; then, every day, the cup's day (draws and matches)")])
CAMPOS = []
INTERESSA = []
