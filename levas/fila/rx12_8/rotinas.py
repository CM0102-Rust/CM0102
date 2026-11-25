# Os premios da Holanda: o objeto, os dez premios e os dias dos premios (award/holland_awards.cpp; leitor rx12,
# rodada 6, 25/11/2026). Usa os auxiliares dos premios das nacoes do sx05b_5 (motor.rs) e do sx03_2
# (england_awards.rs), ja publicados.
# Rodam com a Holanda carregada: o construtor e o init na criacao do jogo, o daily_update todo dia; as chuteiras de
# ouro em 20 de dezembro de 2001 e os premios do ano em 25 de marco de 2002.
# Convencoes conferidas na desmontagem:
# - 0x5f8fe0 thiscall(arquivo) ret 4: 0x768290; vtable 0x96ba74; +0xe = nacao de KEY_NATION_HOLLAND; +0x13 = 0xb;
#   +0x12 = 1; com arquivo, 0x76a280; senao 0x5f9070(arquivo); sem notas do mes;
# - 0x5f9070 thiscall(arquivo) ret 4: +8 = 10 antes do malloc(0x28); nulo: caixa linha 0x67; argumentos por emulacao
#   (rx09/emul_awards.py);
# - 0x5f94c0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x14, 0xb, ano, -1): 0x7683c0 (hoje, premio 6, [[0xad9c60]
#   +0xc], -1, HOL, -1, -1, [KPN], 0, -1) e o mesmo com o premio 9 e [TOTO] (as notas lidas uma vez, em ebx) ret 0x28;
#   Date::Date (0x19, 2, ano, -1): 0x768940 (premio 0) e 0x768cf0 x5 (premios 1, 2, 4, 5 da KPN e 8 da Toto);
#   0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('holland_awards_construct', 0x5f8fe0, [0x6a, 0xff, 0x68, 0x78, 0xae, 0x95, 0x00]),
           ('holland_awards_init', 0x5f9070, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('holland_awards_daily_update', 0x5f94c0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Holanda',
        'the Dutch awards',
        [('holland_awards_construct', 'HollandAwards::HollandAwards', '0x5f8fe0', 'award/holland_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Holanda: a base com a tabela virtual da classe, a Holanda como nação, o id 0xb e o tipo 1; com arquivo, os prêmios salvos; senão os dez prêmios',
          'builds the Dutch awards: the base with the class vtable, Holland as the nation, id 0xb and type 1; with a file, the saved awards; otherwise the ten awards'),
         ('holland_awards_init', 'HollandAwards::init_awards', '0x5f9070', 'award/holland_awards.cpp', 'thiscall, ret 4',
          'cria os dez prêmios holandeses: técnico, goleiro e jovem do ano; a equipe da semana, o jogador e o artilheiro do ano e a chuteira de ouro da primeira divisão (KPN); a equipe da semana, o jogador do ano e a chuteira de ouro da segunda (Toto)',
          'creates the ten Dutch awards: manager, goalkeeper and young player of the year; the premier division (KPN) team of the week, player and top goalscorer of the year and golden boot; the second division (Toto) team of the week, player of the year and golden boot'),
         ('holland_awards_daily_update', 'HollandAwards::daily_update', '0x5f94c0', 'award/holland_awards.cpp', 'thiscall, ret 4',
          'em 20 de dezembro, as chuteiras de ouro das duas divisões pelas notas do ano dos holandeses de cada uma; em 25 de março, pelas estatísticas da temporada, o técnico, o goleiro, o jovem (até 21), o jogador e o artilheiro do ano da primeira divisão e o jogador do ano da segunda; depois, todo dia, a atualização da base',
          "on 20 December, both divisions' golden boots from the year ratings of each one's Dutch players; on 25 March, from the season's statistics, the premier division manager, goalkeeper, young player (up to 21), player and top goalscorer of the year and the second division player of the year; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
