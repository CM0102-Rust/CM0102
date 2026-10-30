# Os registros de carreira (staff_records.cpp): o acesso (Staff_Records::add_promotion_record 0x869480, pelas ligas
# no fim da temporada: eng_second.cpp e outras 18), o rebaixamento (add_relegation_record 0x86a1c0, 24 chamadores) e a
# classificação para a fase seguinte (add_qualified_for_round_record 0x86bdd0, pela copa 0x519890 e outras 32). O SEH
# das três só cobre a data local (0x54a110, sem destrutor). Gravam os históricos no arquivo (FILE_LLIST 0x58e340: não
# rodam duas vezes); o acesso e o rebaixamento mudam a reputação do clube, as notas do técnico, o moral (0x544710,
# sorteia) e as reputações/vínculos do elenco, e podem oferecer renovação (0x4cda70); a classificação soma os pontos
# do técnico (0x86f5b0). Os bytes da entrada que a original não grava (lixo de pilha) saem zerados na cópia: só vão
# para o arquivo. Iguais no cm0102-1080.exe. Comparadores: lado a lado pelas regiões com a cópia sem os históricos
# (o estado do sorteador, o clube, o não jogador do técnico, o registro de pontos, moral, reputações e vínculos do
# elenco); com renovação, só a original.
SAVE = False
ROTINAS = [('registrar_o_acesso', 0x869480, [0x6a, 0xff, 0x68, 0x96, 0x2b, 0x96, 0x00]),
           ('registrar_o_rebaixamento', 0x86a1c0, [0x6a, 0xff, 0x68, 0x06, 0x2c, 0x96, 0x00]),
           ('registrar_a_classificacao_na_fase', 0x86bdd0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os registros de carreira: o acesso, o rebaixamento e a classificação para a fase seguinte',
        'career records: promotion, relegation and qualifying for the next round',
        [('registrar_o_acesso', 'Staff_Records::add_promotion_record', '0x869480', 'staff_records.cpp', 'thiscall, ret 0xc',
          'reputação do clube +500 (até 10000) e +0x7b; as notas do técnico pela importância da competição (× 10/61, com mínimo); os históricos do técnico, do presidente e do clube (tipo 8); os pontos do técnico; moral +15 e vínculo +75 do elenco e a renovação dos contratos que vencem',
          'club reputation +500 (up to 10000) and +0x7b; the manager ratings by the competition importance (× 10/61, with a minimum); the manager, chairman and club histories (type 8); the manager points; squad morale +15 and bond +75 and renewal of the expiring contracts'),
         ('registrar_o_rebaixamento', 'Staff_Records::add_relegation_record', '0x86a1c0', 'staff_records.cpp', 'thiscall, ret 8',
          'reputação do clube −500 (mínimo 750); as notas do técnico caem pela importância da competição (1..10000); os históricos do técnico, do presidente e do clube (tipo 9); moral −15 do elenco, reputações × 0,9 (contratos 1 e 2, com a renovação dos que vencem) ou × 0,95 (3 a 5)',
          'club reputation −500 (minimum 750); the manager ratings drop by the competition importance (1..10000); the manager, chairman and club histories (type 9); squad morale −15, reputations × 0.9 (contracts 1 and 2, renewing the expiring ones) or × 0.95 (3 to 5)'),
         ('registrar_a_classificacao_na_fase', 'Staff_Records::add_qualified_for_round_record', '0x86bdd0', 'staff_records.cpp', 'thiscall, ret 0x14',
          'o histórico do técnico (tipo 3, com a fase e a rodada base) e os pontos dele: importância² × peso no ano, no mês e na copa (seleção) ou na liga (clube)',
          'the manager history (type 3, with the round and the base round) and his points: importance² × weight in the year, the month and the cup (national team) or the league (club)')])
CAMPOS = [('clube +0x80: reputação; valor; lê e grava ±500 com limites (registrar_o_acesso, registrar_o_rebaixamento)',
           'club +0x80: reputation; value; reads and writes ±500 with limits (registrar_o_acesso, registrar_o_rebaixamento)'),
          ('clube +0x7b, +0x73; valores; lê, grava +0x7b (registrar_o_acesso)',
           'club +0x7b, +0x73; values; reads, writes +0x7b (registrar_o_acesso)'),
          ('não jogador do técnico (+0xcf, +0x69) +8, +0xa, +0xc: notas; valores; lê e grava (registrar_o_acesso, registrar_o_rebaixamento)',
           'manager non-player (+0xcf, +0x69) +8, +0xa, +0xc: ratings; values; reads and writes (registrar_o_acesso, registrar_o_rebaixamento)'),
          ('competição +0x69: importância; valor; lê (as três); objeto em [0xadadfc] +0x1c (registro de competição de clube, 0x6b bytes em [0xae23d0]) e +0x42; lê (registrar_o_acesso)',
           'competition +0x69: importance; value; reads (all three); object in [0xadadfc] +0x1c (club competition record, 0x6b bytes in [0xae23d0]) and +0x42; reads (registrar_o_acesso)'),
          ('registro de jogador (+0x61) +0x45 moral (0x544710), +9/+0xb/+0xd reputações; valores; lê e grava (registrar_o_acesso: moral; registrar_o_rebaixamento: moral e reputações)',
           'player record (+0x61) +0x45 morale (0x544710), +9/+0xb/+0xd reputations; values; reads and writes (registrar_o_acesso: morale; registrar_o_rebaixamento: morale and reputations)'),
          ('contrato (0x4cf5f0) +0x44 vínculo, +0x4f tipo (>> 4), +0x25 data; valores; lê, grava +0x44 (registrar_o_acesso), lê (registrar_o_rebaixamento)',
           'contract (0x4cf5f0) +0x44 bond, +0x4f type (>> 4), +0x25 date; values; reads, writes +0x44 (registrar_o_acesso), reads (registrar_o_rebaixamento)'),
          ('registro de pontos do técnico (0x86f5b0, 210 bytes) +0xb6 ano, +0xba liga, +0xbe copa, +0xca mês; valores; soma (registrar_o_acesso, registrar_a_classificacao_na_fase)',
           'manager points record (0x86f5b0, 210 bytes) +0xb6 year, +0xba league, +0xbe cup, +0xca month; values; adds (registrar_o_acesso, registrar_a_classificacao_na_fase)'),
          ('Staff_Records +0 e +0xc (arquivo e listas do histórico das pessoas), +4 e +0x10 (dos clubes): entradas de 8 bytes de data + clube, tipo, competição; acrescenta (as três)',
           'Staff_Records +0 and +0xc (staff history file and lists), +4 and +0x10 (club ones): entries of an 8-byte date + club, type, competition; appends (all three)')]
INTERESSA = ['CMRTE', 'SGE']
