# A cópia de um registro de pessoa, Staff::operator= (Database.cpp; leitor rxs1, rodada 6, 25/11/2026). A rotina
# inteira (0x5335f0..0x53370d), igual no cm0102-1080.exe. Chamada pela troca dos registros dos jogadores falsos
# (0x533080, rxs1_9) e pelo VIRTUAL_STAFF::relocate_virtual_players (0x90e9a0, o ponto 0x90ea38: os jogadores virtuais
# de um clube visto que mudam de bloco), sem contagem própria na cobertura.
# Convenção conferida na desmontagem: thiscall(destino, origem) ret 4, devolve o destino; prólogo sub esp, 8; push esi;
# push edi (5 bytes). Os campos um a um na ordem do registro, as cinco datas pelo Date::operator= (0x417c10, uma cópia
# de 8 bytes): todos os 110 bytes.
# Comparador: lado a lado (a original no destino, a cópia num registro local com a mesma origem; os bytes e o ponteiro
# devolvido).
SAVE = False
ROTINAS = [('staff_assign', 0x5335f0, [0x83, 0xec, 0x08, 0x56, 0x57])]
DOCS = ('a cópia de um registro de pessoa',
        'the copy of a staff record',
        [('staff_assign', 'Staff::operator=', '0x5335f0', 'Database.cpp', 'thiscall, ret 4',
          'copia os 110 bytes do registro de uma pessoa em outro, campo a campo (as datas pelo Date::operator=), e devolve o destino',
          'copies the 110 bytes of a staff record into another, field by field (the dates through Date::operator=), and returns the destination')])
