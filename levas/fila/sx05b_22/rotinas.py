# O passo acima e abaixo de um valor em dinheiro (cash.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa
# cash_get_display_amount, cash_get_rounding_step e CURRENCY_RATE do sx05_1: integrar depois dele.
# Rodam na simulacao (as ofertas de contrato de Staff_Contracts 0x867b70 e as restricoes de oferta do
# TRANSFER_MANAGER, check_offer_restrictions 0x8cfc90 ja em Rust) e nas telas de contrato e transferencia. Item de
# simulacao sem LIGAS.
# Convencoes conferidas na desmontagem: thiscall(&dinheiro (i32), bandeiras) ret 4, devolve eax (o valor novo, que
# tambem fica guardado); o valor da tela igual ao Cash::get_display_amount 0x43f460 (fild, fmul [0xad9cb8], _ftol de
# 64 bits; sem 0x20, (passo / 2 + valor) / passo x passo com __alldiv 0x945e80 e __allmul 0x945e40); step_up soma
# cash_get_rounding_step 0x4426e0 cdecl(valor + 1 (64 bits), bandeiras), step_down subtrai o passo do proprio valor;
# fild qword / [0xad9cb8] e _ftol, o dword baixo guardado.
# Comparadores: lado a lado sobre o valor guardado e o retorno.
SAVE = False
ROTINAS = [('cash_step_up', 0x43f4e0, [0x83, 0xec, 0x08, 0x53, 0x8b, 0xd9]),
           ('cash_step_down', 0x43f580, [0x83, 0xec, 0x08, 0x53, 0x8b, 0xd9])]
DOCS = ('o passo acima e abaixo de um valor em dinheiro',
        'stepping a money amount up and down',
        [('cash_step_up', 'Cash::step_up', '0x43f4e0', 'cash.cpp', 'thiscall, ret 4',
          'sobe o valor um passo: o valor na moeda da tela (arredondado ao passo, salvo com a bandeira 0x20) mais o passo de arredondamento do valor seguinte, de volta em libras pelo câmbio (truncado); guarda e devolve',
          'raises the amount one step: the display currency amount (rounded to the step unless flag 0x20) plus the rounding step of the next amount, back in pounds by the exchange rate (truncated); stores and returns it'),
         ('cash_step_down', 'Cash::step_down', '0x43f580', 'cash.cpp', 'thiscall, ret 4',
          'desce o valor um passo: o valor na moeda da tela menos o passo de arredondamento dele, de volta em libras; guarda e devolve',
          'lowers the amount one step: the display currency amount minus its rounding step, back in pounds; stores and returns it')])
CAMPOS = []
INTERESSA = []
