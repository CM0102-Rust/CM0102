# Os construtores das duas copas galesas em grupos (wel_lge_cup.cpp, wel_prm_cup.cpp, em cups\). Com o mwb_18 a mwb_25
# fecham os dois arquivos (ficam fora só o build_fate_news_text, o load_extra_data e o set_club_progress, que não estão
# na lista). Leitor r4_mwab (20/11/2026), lista do mundo (Escócia e País de Gales), continuação do mwb.
# - 0x925e90 (WelLgeCup::WelLgeCup): estava fora da lista mwb (o mwb_18 diz "ainda na original"); same.py contra
#   0x636dd0 (IreLgeCup::IreLgeCup): 22 diferenças: o vtable 0x971034, a disciplina 0x28 (Gales), as bandeiras da liga 8
#   (0x28 no irlandês), 6 fases, 5 grupos, as rotinas, o arquivo e as linhas 0x53, 0x5d, 0x64, 0x6a e 0x71.
# - 0x9277f0 (WelPrmCup::WelPrmCup): o mesmo, vtable 0x9710e8, disciplina 0xff (nenhuma), 2 fases, 1 grupo (uma chamada
#   só), as rotinas da Copa Premier; as mesmas linhas.
# Convenções conferidas: thiscall, ret 8 (como os irlandeses: League::League 0x682200, load_from_temp_file 0x687b10
# push 1, as rotinas da copa thiscall, ret 0, o grupo push g -> ret 4).
# Comparadores: só a original, com o plano (ita_cups_check_constructor_plan com as bandeiras 8).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_wel_lge_cup', 0x925e90, [0x6a, 0xff, 0x68, 0x39, 0x5d, 0x96, 0x00]),
           ('construct_wel_prm_cup', 0x9277f0, [0x6a, 0xff, 0x68, 0xd9, 0x5d, 0x96, 0x00])]
DOCS = ('os construtores das copas galesas em grupos',
        "the Welsh group cups' constructors",
        [('construct_wel_lge_cup', 'WelLgeCup::WelLgeCup', '0x925e90', 'wel_lge_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Liga galesa na criação da carreira: o ano e a competição do banco de dados, 5 reservas no banco e 3 substituições, a regra de disciplina de Gales, importância 12; sem nada salvo, 6 fases (5 grupos e o mata-mata), os 18 inscritos, as estatísticas, as posições previstas, as regras, a tabela do primeiro grupo (a copa é a própria liga do primeiro grupo), os seus jogos e ordem e as fases dos outros 5 grupos',
          "rule: the Welsh League Cup at career creation: the year and the database competition, 5 substitutes on the bench and 3 substitutions, the Welsh discipline rule, importance 12; with nothing saved, 6 stages (5 groups and the knockout), the 18 entrants, the statistics, the predicted positions, the rules, the first group's table (the cup is itself the league of the first group), its fixtures and order and the stages of the other 5 groups"),
         ('construct_wel_prm_cup', 'WelPrmCup::WelPrmCup', '0x9277f0', 'wel_prm_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa Premier galesa na criação da carreira: o ano e a competição do banco de dados, 5 reservas no banco e 3 substituições, sem regra de disciplina própria, importância 12; sem nada salvo, 2 fases (o segundo grupo e o mata-mata), os 12 inscritos, as estatísticas, as posições previstas, as regras, a tabela do primeiro grupo, os seus jogos e ordem e a fase do segundo grupo',
          "rule: the Welsh Premier Cup at career creation: the year and the database competition, 5 substitutes on the bench and 3 substitutions, no discipline rule of its own, importance 12; with nothing saved, 2 stages (the second group and the knockout), the 12 entrants, the statistics, the predicted positions, the rules, the first group's table, its fixtures and order and the second group's stage")])
CAMPOS = []
INTERESSA = []
