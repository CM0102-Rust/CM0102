# As regras da Inglaterra que faltavam e a data de uma janela (england_rules.cpp, argentina_rules.cpp pelo mapa; leitor
# sx21, rodada 5, 24/11/2026): os limites de elenco das partidas inglesas (slot 0x1c), quem esta inscrito por outro
# clube na Copa da Inglaterra e na Copa da Liga (slot 0x18) e a data de uma janela num ano (de toda regra com janela,
# todo dia). Rodam na carreira ENG.
# Convencoes conferidas: 0x579c50 thiscall(partida, saida) ret 8 (a partida nao e lida); 0x57a090 thiscall(jogador,
# partida) ret 8, al; 0x40a7d0 thiscall(saida, ano) ret 8, eax = saida; 0x549f70 thiscall(dia, mes, ano, dia da
# semana) ret 0x10; 0x4a4a30 thiscall(ficha) ret 4.
# Ajudantes comuns (usados pelos itens de LIGAS do sx21): fill_squad_restrictions_with, compare_squad_restrictions,
# base_ineligibility_code, cup_tied_entry, compare_ineligibility.
# Comparadores: os 42 bytes lado a lado; o codigo (al); a data (8 bytes) lado a lado.
SAVE = False
ROTINAS = [
    ('england_rules_fill_squad_restrictions', 0x579c50, [0x8b, 0x54, 0x24, 0x08, 0x57]),
    ('england_rules_get_ineligibility_code', 0x57a090, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
    ('transfer_window_get_date', 0x40a7d0, [0x51, 0x66, 0x0f, 0xbe, 0x41, 0x02]),
]
DOCS = ('as regras da Inglaterra: os limites de elenco e quem está inscrito por outro clube nas copas; a data de uma janela',
        "England's rules: the squad limits and cup-tied players; a transfer window's date",
        [('england_rules_fill_squad_restrictions', 'england_rules_fill_squad_restrictions', '0x579c50', 'england_rules.cpp', 'thiscall, ret 8',
          'regra: sem limites (0xff) salvo três jogadores de fora da UE (+5) em toda competição',
          'rule: no limits (0xff) except three non-EU players (+5) in every competition'),
         ('england_rules_get_ineligibility_code', 'england_rules_get_ineligibility_code', '0x57a090', 'england_rules.cpp', 'thiscall, ret 8',
          'regra: na Copa da Inglaterra e na Copa da Liga, quem já jogou a copa por outro clube não joga (1); senão a resposta de base',
          'rule: in the FA Cup and the League Cup, a player who already played the cup for another club may not play (1); else the base answer'),
         ('transfer_window_get_date', 'transfer_window_get_date', '0x40a7d0', 'argentina_rules.cpp', 'thiscall, ret 8',
          'regra: a data com o dia (+3), o mês (+4) da janela, o ano dado e o dia da semana para onde ela anda (+2)',
          "rule: the date with the window's day (+3) and month (+4), the given year and the weekday it moves to (+2)")])
CAMPOS = []
INTERESSA = []
