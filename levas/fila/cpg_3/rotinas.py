# Cópias de registros, 3º item: a cópia do registro de suspensão (BanRecord::operator= 0x556c70, 341 chamadas na
# carreira até 2002-06-01; discipline.cpp), o salário padrão do contrato (STAFF_CONTRACT::set_default_wage 0x8626e0,
# 181; staff_contracts.cpp; o padrão pela 0x862720, float truncado pela 0x945640, mínimo 55) e a condição de jogo do
# jogador (Injury::get_match_condition 0x62c510, 2.535; injury.cpp; atualiza o físico do dia pela 0x62c680, que sorteia
# só na primeira consulta do dia e guarda). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo
# real comparando. Sem SEH, sem caixa, sem local lida sem valor (a data local é gravada pela 0x417c10). Comparadores: a
# cópia e o salário lado a lado (registro inteiro; +0xc); a condição pelo valor (o ax), com a original primeiro (ela
# faz a atualização do dia; a cópia acha o dia feito). Nenhuma em laço quente.
SAVE = False
ROTINAS = [('copiar_registro_de_suspensao', 0x556c70, [0x83, 0xec, 0x08, 0x55, 0x56]),
           ('salario_padrao_do_contrato', 0x8626e0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('condicao_de_jogo_do_jogador', 0x62c510, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('a cópia do registro de suspensão, o salário padrão do contrato e a condição de jogo do jogador',
        'the copy of the ban record, the default contract wage and the player match condition',
        [('copiar_registro_de_suspensao', 'BanRecord::operator=', '0x556c70', 'discipline.cpp', 'thiscall, ret 4',
          'copia o registro de suspensão (0x14 bytes): campos, a data (+4) e os bits 0..2 de +0x13',
          'copies the ban record (0x14 bytes): fields, the date (+4) and bits 0..2 of +0x13'),
         ('salario_padrao_do_contrato', 'STAFF_CONTRACT::set_default_wage', '0x8626e0', 'staff_contracts.cpp', 'thiscall, ret 4',
          'o salário (+0xc) = o padrão da pessoa no clube (0x862720) truncado, mínimo 55',
          'the wage (+0xc) = the default for the person at the club (0x862720) truncated, minimum 55'),
         ('condicao_de_jogo_do_jogador', 'Injury::get_match_condition', '0x62c510', 'injury.cpp', 'thiscall, ret 8',
          'atualiza o físico do dia (0x62c680) e devolve a condição de jogo (de 10000, ou em porcento)',
          'updates the day fitness (0x62c680) and returns the match condition (out of 10000, or as a percentage)')])
CAMPOS = [('registro de suspensão (0x14 bytes: data em +4, marcas nos bits 0..2 de +0x13); escreve (copiar_registro_de_suspensao); contrato +0xc salário; escreve (salario_padrao_do_contrato); ficha de lesão (31 bytes, +0xa condição de jogo); lê (condicao_de_jogo_do_jogador)',
           'ban record (0x14 bytes: date at +4, flags in bits 0..2 of +0x13); writes (copiar_registro_de_suspensao); contract +0xc wage; writes (salario_padrao_do_contrato); injury record (31 bytes, +0xa match condition); reads (condicao_de_jogo_do_jogador)')]
INTERESSA = ['SGE', 'CMRTE']
