# Os premios da Escocia: o objeto, os 27 premios e o dia dos premios (award/scotland_awards.cpp; leitor sx04c, rodada
# 5, 23/11/2026). Usa os auxiliares do sx05b_5 e do sx03_2 e make_date_dword/is_date_today (eng_cc_cup.rs): integrar
# depois deles.
# Rodam com a Escocia carregada: construtor e init na criacao do jogo, daily_update todo dia; os premios no sabado que
# o Date::Date faz de 23 de maio (2002-05); os premios do mes viram no dia do fim da temporada da nacao.
# Convencoes conferidas na desmontagem:
# - 0x7f5170 thiscall(arquivo) ret 4: vtable 0x96f824, [KEY_NATION_SCOTLAND], +0x13 = 0x11, +0x12 = 1; notas do mes
#   (linha 0x60);
# - 0x7f52b0 thiscall(arquivo) ret 4: o ano local; +8 = 0x1b, malloc(0x6c) (nulo: linha 0x88); tres premios do ano sem
#   competicao e quatro blocos de seis (equipe da semana, tres premios do mes de agosto (7) do ano a maio (4) do
#   seguinte, tecnico do ano, equipe do ano);
# - 0x7f5dc0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x17, 4, ano, 6): o dia da semana 6, nao -1 como nas outras
#   nacoes; 0x768940 x4, 0x769380 x3 (a Escocia no 1o filtro), 0x7698f0 x4; no dia do fim da temporada 0x7672e0 nos
#   doze premios do mes; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG SCO WAL'
ROTINAS = [('scotland_awards_construct', 0x7f5170, [0x6a, 0xff, 0x68, 0x39, 0x0d, 0x96, 0x00]),
           ('scotland_awards_init', 0x7f52b0, [0x6a, 0xff, 0x68, 0xca, 0x0e, 0x96, 0x00]),
           ('scotland_awards_daily_update', 0x7f5dc0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Escócia',
        'the Scottish awards',
        [('scotland_awards_construct', 'ScotlandAwards::ScotlandAwards', '0x7f5170', 'award/scotland_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Escócia: a base com a tabela virtual da classe, a Escócia, o id 0x11 e o tipo 1; com arquivo, os salvos; senão as notas do mês e os 27 prêmios',
          'builds the Scottish awards: the base with the class vtable, Scotland, id 0x11 and type 1; with a file, the saved ones; otherwise the month ratings and the 27 awards'),
         ('scotland_awards_init', 'ScotlandAwards::init_awards', '0x7f52b0', 'award/scotland_awards.cpp', 'thiscall, ret 4',
          'cria os 27 prêmios: jogador, artilheiro e jovem escocês do ano e, em cada uma das quatro divisões, a equipe da semana, técnico, jogador e jovem do mês (de agosto a maio), técnico do ano e equipe do ano',
          'creates the 27 awards: the Scottish player, top goalscorer and young player of the year and, in each of the four divisions, the team of the week, manager, player and young player of the month (August to May), manager of the year and team of the year'),
         ('scotland_awards_daily_update', 'ScotlandAwards::daily_update', '0x7f5dc0', 'award/scotland_awards.cpp', 'thiscall, ret 4',
          'no sábado de 23 de maio, o técnico do ano de cada divisão, jogador, jovem (até 21) e artilheiro escocês do ano pelos três melhores das notas do ano e a equipe da temporada de cada divisão; no dia do fim da temporada, os doze prêmios do mês passam à temporada seguinte; depois, todo dia, a atualização da base',
          'on the Saturday of 23 May, the manager of the year of each division, the Scottish player, young player (up to 21) and top goalscorer of the year from the year ratings top three and the team of the season of each division; on the season end day, the twelve month awards move on; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
