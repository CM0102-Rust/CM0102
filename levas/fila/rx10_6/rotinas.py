# As posicoes previstas e a virada da Regionalliga alema (ger_regional.cpp; leitor rx10, rodada 6, 25/11/2026).
# Simulacao com a Alemanha e a base NORMAL (ver rx10_4). As posicoes previstas rodam na criacao do jogo e na virada
# de 2002; a virada no verao de 2002 (GerFirst::start_new_season, mwc_21).
# - 0x5de130 GerRegional::set_predicted_positions thiscall() ret 0: section_predicted_positions (o corpo do
#   PorSecondB 0x7d2cd0 com passo 3 e duas secoes): a Norte e a fase 0, cada uma ordenada por reputacao (qsort com
#   0x4b07d0) e a posicao 2 x linha + 1 (Comp_Stats 0x4a2540); caixas 0x275 e 0x27d. Prologo sub esp, 0x204 (6 bytes).
# - 0x5dda90 GerRegional::start_new_season thiscall() ret 0: divided_league_turnover (o mesmo corpo do
#   PorSecondB::start_new_season 0x7d26b0): +0x4c = 0, tabela 0x687970(0), datas, estatisticas 0x4a1c50(1), fases;
#   ano + 1, fase -1; regras (virtual +0x8c, caixa 0x1b2), tabela 0x5dd7f0 (0x1b9), ordem 0x6827d0(0) (0x1c0), jogos
#   0x6835c0 (0x1c6); a fase Sul 0x5ddc50(0), as posicoes previstas (virtual +0x5c) e os arbitros 0x79cee0. Sem as
#   expectativas da diretoria (0x68aa80), ao contrario das inglesas. Prologo sub esp, 0x200 (6 bytes).
# Depende do rx10_4 (os auxiliares) e do rx10_5 (a fase Sul pelo endereco; nada do codigo). Iguais no cm0102-1080.exe.
# Comparadores: as posicoes lado a lado; a virada so a original (o ano sobe um).
# Defeito do original: as duas secoes recebem as mesmas posicoes impares (1, 3, 5...), nao intercaladas.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('ger_regional_set_predicted_positions', 0x5de130, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('ger_regional_start_new_season', 0x5dda90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas e a virada da Regionalliga alemã',
        'the German Regionalliga predicted positions and season turnover',
        [('ger_regional_set_predicted_positions', 'GerRegional::set_predicted_positions', '0x5de130', 'ger_regional.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Regionalliga: os clubes de cada seção pela reputação, o melhor primeiro, nas posições 1, 3, 5... (as duas seções recebem as mesmas posições ímpares)',
          'rule: the Regionalliga predicted positions: each section\'s clubs by reputation, the best first, in places 1, 3, 5... (both sections get the same odd places)'),
         ('ger_regional_start_new_season', 'GerRegional::start_new_season', '0x5dda90', 'ger_regional.cpp', 'thiscall, ret 0',
          'regra: na virada a Regionalliga arquiva a temporada, limpa a tabela Norte, as datas e a fase Sul, passa o ano e refaz as regras, a tabela Norte, a ordem, os jogos e a fase Sul; depois as posições previstas e a renovação dos árbitros',
          'rule: at the season turn the Regionalliga archives the season, clears the North table, the dates and the South stage, moves the year on and rebuilds the rules, the North table, the order, the fixtures and the South stage; then the predicted positions and the referee renewal')])
CAMPOS = []
INTERESSA = []
