# The Allsvenskan constructor and the league rules of the two Finnish divisions (reader mwe, whole-world round,
# 12/11/2026):
# - 0x88fb10 SwePrm::SwePrm (752 bytes; once, at the creation of the new game): the body of construct_division_league
#   with the filing of the Swedish clubs before the stages (the six Second Division sections into the Second Division,
#   the section kept in club +0x60; the other Swedish clubs outside the top three divisions into the lower division);
# - 0x595c90 FinPrm::set_up_rules (176 bytes) and 0x593b80 FinFirst::set_up_rules (160 bytes): at the creation and
#   at each turnover (end of 2001), the body of eng_prm_apply_league_rules (cqi_4).
# Equal in cm0102-1080.exe (whole).
# Conventions checked: SwePrm: push record; push year; mov ecx -> thiscall, ret 8; inside, as construct_division_league
# (the filing loop has no calls). The rules: mov ecx; call / vtable +0x8c -> thiscall, ret 0; the virtual +0x3c: push
# 0; push &+0x3a; push &+0xa9; push -1 -> thiscall, ret 0x10.
# Comparers: SwePrm only the original (it allocates), with compare_division_league_ctor and the filing planned before
# and checked club by club after (division +0x57 and reserve division +0x60); the rules, eng_prm_compare_league_rules.
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_swe_prm', 0x88fb10, [0x6a, 0xff, 0x68, 0x69, 0x31, 0x96, 0x00]),
           ('fin_prm_set_up_rules', 0x595c90, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x03, 0x00, 0x00, 0x00]),
           ('fin_first_set_up_rules', 0x593b80, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00])]
DOCS = ('o construtor da Allsvenskan (com a arrumação dos clubes suecos) e as regras das duas divisões finlandesas',
        'the Allsvenskan constructor (with the filing of the Swedish clubs) and the league rules of the two Finnish divisions',
        [('construct_swe_prm', 'SwePrm::SwePrm', '0x88fb10', 'swe_prm.cpp', 'thiscall, ret 8',
          'a Allsvenskan: regra de disciplina 28, só a liga; sem estado salvo, antes de tudo arruma os clubes suecos: os das seis seções regionais da segunda divisão vão para a segunda divisão (a seção fica guardada como divisão de reservas) e os outros clubes suecos fora das três primeiras divisões vão para a divisão de baixo; depois as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas',
          'the Allsvenskan: discipline rule 28, the league only; with no saved state, first it files the Swedish clubs: those of the six regional Second Division sections go into the Second Division (the section kept as the reserve division) and the other Swedish clubs outside the top three divisions go into the lower division; then the rules, the table, the fixtures, the order, the stats and the predicted positions'),
         ('fin_prm_set_up_rules', 'FinPrm::set_up_rules', '0x595c90', 'fin_prm.cpp', 'thiscall, ret 0',
          'a Veikkausliiga (12 clubes): três turnos (33 rodadas), 3 pontos por vitória; ninguém sobe, o 11º joga o play-off de rebaixamento e o 12º cai para a Ykkönen; marca de campeão; 7 reservas e 3 substituições',
          'the Veikkausliiga (12 clubs): three legs (33 rounds), 3 points for a win; nobody goes up, the 11th plays the relegation play-off and the 12th goes down to the Ykkönen; the champion status; 7 substitutes and 3 substitutions'),
         ('fin_first_set_up_rules', 'FinFirst::set_up_rules', '0x593b80', 'fin_first.cpp', 'thiscall, ret 0',
          'a Ykkönen (os grupos Norte e Sul de 10): três turnos, 3 pontos por vitória; 2 sobem, 1 joga o play-off de rebaixamento e 2 caem (sem divisão abaixo); 7 reservas e 3 substituições',
          'the Ykkönen (the North and South groups of 10): three legs, 3 points for a win; 2 go up, 1 plays the relegation play-off and 2 go down (no division below); 7 substitutes and 3 substitutions')])
CAMPOS = [('clube +0x60 divisão de reservas: a seção regional da segunda divisão sueca; ponteiro; grava (construct_swe_prm)',
           'club +0x60 reserve division: the Swedish Second Division regional section; pointer; writes (construct_swe_prm)')]
INTERESSA = ['CMRTE']
