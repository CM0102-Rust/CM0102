ROTINAS = [('e_competicao_do_amistoso', 0x5c2990, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('preencher_data_internacional', 0x5cb9b0, [0x8a, 0x44, 0x24, 0x04, 0x8a, 0x54, 0x24, 0x08]),
           ('limpar_registro_de_tecnico', 0x86fc20, [0x8b, 0x44, 0x24, 0x04, 0x57])]
DOCS = ('a competição do amistoso, a data internacional e o registro de técnico',
        'the friendly competition, the international date and the manager record',
        [('e_competicao_do_amistoso', 'Friendly::is_own_comp_record', '0x5c2990', 'friendly.cpp', 'thiscall',
          'diz se a competição é a dos amistosos',
          'tells whether the competition is the friendlies one'),
         ('preencher_data_internacional', 'friendly_international_date_set', '0x5cb9b0', 'friendly.cpp', 'thiscall',
          'preenche uma data internacional: dia, mês, duração, ano, continente e competição',
          'fills an international date: day, month, span, year, continent and competition'),
         ('limpar_registro_de_tecnico', 'Staff_Records::clear_manager_record', '0x86fc20', 'staff_records.cpp', 'thiscall',
          'zera o registro de técnico e põe −1 nos ids do dono, da liga e dos clubes',
          'clears the manager record and sets the owner, league and club ids to −1')])
