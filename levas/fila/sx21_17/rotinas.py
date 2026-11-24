# Os limites de elenco e a inscricao por outro clube da Escocia (leitor sx21, rodada 5, 24/11/2026): rodam nas partidas
# escocesas.
# Convencoes conferidas: 0x7f64c0 thiscall(partida, saida) ret 8 (le [[+0x14]] e a divisao do mandante sem conferir a
# partida); 0x7f65d0 thiscall(jogador, partida) ret 8, al; 0x4a4a30 thiscall(ficha) ret 4.
# Depende da sx21_5 (fill_squad_restrictions_with, compare_squad_restrictions, base_ineligibility_code, cup_tied_entry,
# compare_ineligibility): integrar junto ou depois dela.
# Comparadores: os 42 bytes lado a lado; o codigo (al) e G_CONTRACT_LOOKUP_RESULT.
SAVE = False
LIGAS = 'ENG SCO WAL'
ROTINAS = [
    ('scotland_rules_fill_squad_restrictions', 0x7f64c0, [0x8b, 0x54, 0x24, 0x08, 0x56]),
    ('scotland_rules_get_ineligibility_code', 0x7f65d0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
]
DOCS = ('os limites de elenco e a inscrição por outro clube da Escócia',
        "Scotland's squad limits and cup-tied players",
        [('scotland_rules_fill_squad_restrictions', 'scotland_rules_fill_squad_restrictions', '0x7f64c0', 'scotland_rules.cpp', 'thiscall, ret 8',
          'regra: nas partidas de reservas de um clube da primeira divisão, a regra da idade (+0x16 = 21, +0x17 = 3, +0x26 = reservas); na primeira divisão, +6 = 21, +8 = 2 e +0x26 = a primeira divisão',
          "rule: in a top-flight club's reserve matches, the age rule (+0x16 = 21, +0x17 = 3, +0x26 = reserves); in the top flight, +6 = 21, +8 = 2 and +0x26 = the top flight"),
         ('scotland_rules_get_ineligibility_code', 'scotland_rules_get_ineligibility_code', '0x7f65d0', 'scotland_rules.cpp', 'thiscall, ret 8',
          'regra: na Copa da Escócia e na Copa da Liga, quem jogou a copa por outro clube não joga (1) antes da rodada duas depois da última que jogou (+0x1c da entrada); senão a resposta de base',
          "rule: in the Scottish Cup and the League Cup, a player who played the cup for another club may not play (1) before the round two after the last he played (the entry's +0x1c); else the base answer")])
CAMPOS = []
INTERESSA = []
