# Os premios da Croacia: o objeto, os nove premios e o dia dos premios (award/croatia_awards.cpp; leitor rx12,
# rodada 6, 25/11/2026). Usa os auxiliares dos premios das nacoes do sx05b_5 (motor.rs) e do sx03_2
# (england_awards.rs), ja publicados.
# Rodam com a Croacia carregada: o construtor e o init na criacao do jogo, o daily_update todo dia; os premios em 30
# de maio de 2002.
# Convencoes conferidas na desmontagem:
# - 0x517f10 thiscall(arquivo) ret 4: 0x768290; vtable 0x969380; +0xe = nacao de KEY_NATION_CROATIA; +0x13 = 0x17;
#   +0x12 = 1; com arquivo, 0x76a280; senao 0x517fa0(arquivo); sem notas do mes;
# - 0x517fa0 thiscall(arquivo) ret 4: +8 = 9 antes do malloc(0x24); nulo: caixa linha 0x68; argumentos por emulacao
#   (rx09/emul_awards.py), o 1o conferido a mao (equipe: span 1, modo 2, indice 0 literal);
# - 0x5183a0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x1e, 4, ano, -1); por divisao 0x768cf0 (0x11) e 0x768cf0 (2)
#   nos premios 1, 2, 4, 5, 7, 8; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('croatia_awards_construct', 0x517f10, [0x6a, 0xff, 0x68, 0x08, 0x75, 0x95, 0x00]),
           ('croatia_awards_init', 0x517fa0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('croatia_awards_daily_update', 0x5183a0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Croácia',
        'the Croatian awards',
        [('croatia_awards_construct', 'CroatiaAwards::CroatiaAwards', '0x517f10', 'award/croatia_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Croácia: a base com a tabela virtual da classe, a Croácia como nação, o id 0x17 e o tipo 1; com arquivo, os prêmios salvos; senão os nove prêmios',
          'builds the Croatian awards: the base with the class vtable, Croatia as the nation, id 0x17 and type 1; with a file, the saved awards; otherwise the nine awards'),
         ('croatia_awards_init', 'CroatiaAwards::init_awards', '0x517fa0', 'award/croatia_awards.cpp', 'thiscall, ret 4',
          'cria os nove prêmios croatas: para a primeira divisão e as duas chaves da segunda (norte e sul), a equipe da semana, o jogador do ano e o artilheiro',
          'creates the nine Croatian awards: for the First Division and the two Second Division groups (north and south), the team of the week, the player of the year and the top goalscorer'),
         ('croatia_awards_daily_update', 'CroatiaAwards::daily_update', '0x5183a0', 'award/croatia_awards.cpp', 'thiscall, ret 4',
          'em 30 de maio, pelas estatísticas da temporada de cada divisão, o jogador do ano e o artilheiro; depois, todo dia, a atualização da base',
          "on 30 May, from each division's season statistics, the player of the year and the top goalscorer; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
