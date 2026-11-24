# Os premios da Turquia: o objeto, os seis premios e o dia dos premios (award/turkey_awards.cpp; leitor sx05b, rodada
# 5, 23/11/2026). Usa os construtores e chamadas comuns dos premios das nacoes (sx05b_5) e nation_award, awards_day_is,
# award_manager_of_year e award_team_of_season (sx03_2): integrar depois dos dois.
# Rodam com a Turquia carregada: o construtor e o init na criacao do jogo (AWARD_MANAGER 0x413ec7), o daily_update
# todo dia; os premios em 30 de maio (2002-05-30, dentro da janela). LIGAS = 'ENG POL TUR GRE HRV RUS'.
# Convencoes conferidas na desmontagem (as listas de argumentos dos construtores tiradas por script e conferidas a mao
# no primeiro premio):
# - 0x901c70 thiscall(arquivo) ret 4: como o sueco, vtable 0x970748, [KEY_NATION_TURKEY], +0x13 = 0x1a, +0x12 = 1;
# - 0x901d00 thiscall(arquivo) ret 4: +8 = 6, malloc(0x18) (nulo: linha 0x64); YEAR_AWARD 0x92fe80 e TEAM_AWARD
#   0x8af520 com o tipo 0;
# - 0x901fd0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x1e, 4, ano, -1); NationAwards 0x769380 (9 argumentos),
#   0x768940 (3), 0x7698f0 (comp, premio) ret 8 duas vezes e 0x768310.
# Comparadores: construtor e init so a original (alocam) e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [('turkey_awards_construct', 0x901c70, [0x6a, 0xff, 0x68, 0x28, 0x51, 0x96, 0x00]),
           ('turkey_awards_init', 0x901d00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('turkey_awards_daily_update', 0x901fd0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Turquia',
        'the Turkish awards',
        [('turkey_awards_construct', 'TurkeyAwards::TurkeyAwards', '0x901c70', 'award/turkey_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Turquia: a base com a tabela virtual da classe, a Turquia, o id 0x1a e o tipo 1; com arquivo, os salvos; senão os seis prêmios',
          'builds the Turkish awards: the base with the class vtable, Turkey, id 0x1a and type 1; with a file, the saved ones; otherwise the six awards'),
         ('turkey_awards_init', 'TurkeyAwards::init_awards', '0x901d00', 'award/turkey_awards.cpp', 'thiscall, ret 4',
          'cria os seis prêmios turcos: jogador do ano, técnico do ano e as equipes da semana e do ano da primeira e da segunda divisão (sem reputações)',
          'creates the six Turkish awards: player of the year, manager of the year and the teams of the week and of the year of the first and second divisions (without reputations)'),
         ('turkey_awards_daily_update', 'TurkeyAwards::daily_update', '0x901fd0', 'award/turkey_awards.cpp', 'thiscall, ret 4',
          'em 30 de maio, os prêmios turcos: o jogador do ano pelos três melhores das notas do ano, o técnico do ano e as equipes do ano das duas divisões; depois, todo dia, a atualização da base',
          'on 30 May, the Turkish awards: the player of the year from the year ratings\' top three, the manager of the year and the teams of the year of both divisions; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
