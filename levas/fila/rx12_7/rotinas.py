# Os premios da Belgica: o objeto, os onze premios e os dias dos premios (award/belgium_awards.cpp; leitor rx12,
# rodada 6, 25/11/2026). Usa os auxiliares dos premios das nacoes do sx05b_5 (motor.rs) e do sx03_2
# (england_awards.rs), ja publicados.
# Rodam com a Belgica carregada: o construtor e o init na criacao do jogo, o daily_update todo dia; a chuteira de ouro
# em 29 de dezembro de 2001 e os premios do ano em 6 de junho de 2002 (ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x424480 thiscall(arquivo) ret 4: 0x768290; vtable 0x967bfc; +0xe = nacao de KEY_NATION_BELGIUM; +0x13 = 4;
#   +0x12 = 1; com arquivo, 0x76a280 (eax != 0: fim); senao 0x424510(arquivo); sem notas do mes;
# - 0x424510 thiscall(arquivo) ret 4: +8 = 11 antes do malloc(0x2c); nulo: caixa linha 0x69; argumentos por emulacao
#   (rx09/emul_awards.py);
# - 0x4249c0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x1d, 0xb, ano, -1): 0x7683c0 (hoje, premio 1, [[0xad9c60]
#   +0xc], -1, BEL, -1, -1, -1, 0, -1) ret 0x28; Date::Date (6, 5, ano, -1): 0x768940 (premio 0) e 0x768cf0 x6
#   (premios 3, 4, 5, 7, 10, 8, nessa ordem); 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('belgium_awards_construct', 0x424480, [0x6a, 0xff, 0x68, 0xf8, 0x41, 0x95, 0x00]),
           ('belgium_awards_init', 0x424510, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('belgium_awards_daily_update', 0x4249c0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Bélgica',
        'the Belgian awards',
        [('belgium_awards_construct', 'BelgiumAwards::BelgiumAwards', '0x424480', 'award/belgium_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Bélgica: a base com a tabela virtual da classe, a Bélgica como nação, o id 4 e o tipo 1; com arquivo, os prêmios salvos; senão os onze prêmios',
          'builds the Belgian awards: the base with the class vtable, Belgium as the nation, id 4 and type 1; with a file, the saved awards; otherwise the eleven awards'),
         ('belgium_awards_init', 'BelgiumAwards::init_awards', '0x424510', 'award/belgium_awards.cpp', 'thiscall, ret 4',
          'cria os onze prêmios belgas: o técnico do ano e a chuteira de ouro; a equipe da semana, o artilheiro, o goleiro e o jovem do ano da primeira divisão; a equipe da semana, o artilheiro e a revelação da segunda; a equipe da semana e o artilheiro da terceira',
          'creates the eleven Belgian awards: the manager of the year and the golden boot; the First Division team of the week, top goalscorer, goalkeeper and young player of the year; the Second Division team of the week, top goalscorer and revelation; the Third Division team of the week and top goalscorer'),
         ('belgium_awards_daily_update', 'BelgiumAwards::daily_update', '0x4249c0', 'award/belgium_awards.cpp', 'thiscall, ret 4',
          'em 29 de dezembro, a chuteira de ouro pelas notas do ano dos belgas; em 6 de junho, pelas estatísticas da temporada, o técnico do ano, o artilheiro, o goleiro e o jovem (até 23) da primeira divisão, os artilheiros da segunda e da terceira e a revelação da segunda (até 23); depois, todo dia, a atualização da base',
          "on 29 December, the golden boot from the Belgians' year ratings; on 6 June, from the season's statistics, the manager of the year, the First Division top goalscorer, goalkeeper and young player (up to 23), the Second and Third Division top goalscorers and the Second Division revelation (up to 23); then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
