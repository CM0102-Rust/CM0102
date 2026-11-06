# Os registros dos clubes nas copas da Itália e no Troféu da Liga inglesa: a Copa da Itália, a Copa da Série C e o LDV
# Vans Trophy (o slot +0x44 update_staff_records_for_status; ita_cup.cpp, ita_c_cup.cpp, eng_auto_cup.cpp, em cups\).
# Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (Itália e Inglaterra jogáveis).
# - 0x63ff00 (ItaCup::update_staff_records_for_status, 332 bytes; 78 chamadas até 2002-06-01 com as 8 ligas). Uma
#   mutação.
# - 0x63e270 (ItaCCup::update_staff_records_for_status, 332 bytes; 142 chamadas). Uma mutação.
# - 0x56bee0 (EngAutoCup::update_staff_records_for_status, 301 bytes; 110 chamadas). Uma mutação.
# Reaproveita do motor.rs os ajudantes da cpz_4 (cpz4_registros, cpz4_escala, cpz4_classificado,
# cpz4_nao_classificado, cpz4_eliminado); as duas italianas têm o mesmo corpo da 0x4c4080 da Libertadores (cqk_11) sem
# o Mundial (cqq13_italia, com o limite da fase, a rodada e o peso dos grupos de cada uma).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x44 (6 pushes; mov ecx, copa -> thiscall, ret 0x18).
# Dentro: 0x4bf850 (4 pushes; add esp, 0x10 -> cdecl), 0x86bdd0 (5 pushes; mov ecx, [0xdd7f00] -> thiscall, ret 0x14),
# 0x86c1d0 (4 pushes -> thiscall, ret 0x10), 0x86c000 (5 pushes -> thiscall, ret 0x14), 0x86a800 e 0x86b0b0 (3 pushes
# -> thiscall, ret 0xc). Palavras empurradas em registradores com lixo nos 16 bits altos (a rodada para a 0x4bf850,
# +0x30 para a 0x86c000): os chamados só leem a palavra; a cópia passa o valor limpo (como a cpz_4); o nome da rodada
# nas italianas vai com os 16 bits altos do ponteiro da fase, como a original (cqk_11).
# Comparadores: só a original (os registros vão ao histórico e mexem na reputação e no moral: não rodam duas vezes),
# com o plano da devolução 0; a cópia é provada no modo Rust (as carreiras e as mutações).
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqq13_copa_da_italia_registros', 0x63ff00, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('cqq13_copa_c_registros', 0x63e270, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('cqq13_trofeu_registros', 0x56bee0, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('os registros dos clubes nas copas da Itália e no Troféu da Liga inglesa: a Copa da Itália, a Copa da Série C e o LDV Vans Trophy',
        'the clubs\' records in the Italian cups and the English League Trophy: the Coppa Italia, the Serie C Cup and the LDV Vans Trophy',
        [('cqq13_copa_da_italia_registros', 'ItaCup::update_staff_records_for_status', '0x63ff00', 'ita_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da Itália, quem passa dos grupos ganha o registro de classificado (rodada 0xa, peso 0x28) e quem não passa o de não classificado; no mata-mata, a eliminação e a classificação valem pela rodada (peso de 50 a 100 nas 5 rodadas), e o campeão e o vice ganham os registros da final',
          'rule: in the Coppa Italia, those who get through the groups get the qualified record (round 0xa, weight 0x28) and those who do not the failed record; in the knockout, elimination and qualification weigh by round (weight 50 to 100 over the 5 rounds), and the winner and runner-up get the final records'),
         ('cqq13_copa_c_registros', 'ItaCCup::update_staff_records_for_status', '0x63e270', 'ita_c_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da Série C, como na Copa da Itália, com a classificação nos grupos na rodada 0x14 (peso 0x28)',
          'rule: in the Serie C Cup, as in the Coppa Italia, with group qualification at round 0x14 (weight 0x28)'),
         ('cqq13_trofeu_registros', 'EngAutoCup::update_staff_records_for_status', '0x56bee0', 'eng_auto_cup.cpp', 'thiscall, ret 0x18',
          'regra: no LDV Vans Trophy, nas chaves a eliminação e a classificação valem pela rodada (peso de 1 a 80 nas 5 rodadas) e o vencedor da chave ganha o registro de classificado para a final (rodada 0x96, peso 0x5a); na final, o campeão e o vice ganham os registros da final',
          'rule: in the LDV Vans Trophy, in the sections elimination and qualification weigh by round (weight 1 to 80 over the 5 rounds) and the section winner gets the record of reaching the final (round 0x96, weight 0x5a); in the final, the winner and runner-up get the final records')])
CAMPOS = [('copa +4 a competição do banco, +0xc as fases, +0xa3 a tabela das rodadas; ponteiro; lê',
           'cup +4 the database competition, +0xc the stages, +0xa3 the round table; pointer; reads'),
          ('jogo +0x30, +0x32, +0x34 (a rodada); valor; lê', 'match +0x30, +0x32, +0x34 (the round); value; reads'),
          ('[0xdd7f00] o registro de staff; ponteiro; lê (e os registros gravados pelas 0x86xxxx)',
           '[0xdd7f00] the staff records; pointer; reads (and the records written by the 0x86xxxx routines)')]
INTERESSA = ['CMRTE']
