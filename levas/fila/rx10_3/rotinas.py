# A virada da Football Conference (eng_conf.cpp; leitor rx10, rodada 6, 25/11/2026). Carreira ENG com a base NORMAL
# (a Conference so existe com a divisao extra carregada; ver rx10_1), ate 2002-08-01: a virada inglesa e no verao de
# 2002 (a data dos itens mwf; com a data padrao nao roda).
# - 0x56ee80 EngConf::start_new_season thiscall() ret 0: o molde da Second (eng_second_start_new_season, conferido
#   instrucao a instrucao: +0x4c = 0, rebuild_season com a tabela 0x56ecb0 e as caixas 0x259, 0x260, 0x266, 0x26d,
#   set_expectations_and_renew_officials). Prologo sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe. Comparador: so a original (o ano sobe um). Sem mutacao propria: o molde da virada ja e
# provado nas quatro divisoes; o que ela monta so e usado na temporada seguinte.
SAVE = False
BASE = 'NORMAL'
ATE = '2002-08-01'
ROTINAS = [('eng_conf_start_new_season', 0x56ee80, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada da Football Conference', 'the Football Conference season turnover',
        [('eng_conf_start_new_season', 'EngConf::start_new_season', '0x56ee80', 'eng_conf.cpp', 'thiscall, ret 0',
          'regra: na virada a Conference arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa o ano e refaz as regras, a tabela de 22, os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: at the season turn the Conference archives the season in its statistics, clears the table, the dates and the stages, moves the year on and rebuilds the rules, the 22-club table, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = []
INTERESSA = []
