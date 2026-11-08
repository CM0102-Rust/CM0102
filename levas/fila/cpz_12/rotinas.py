# O dinheiro das transferencias (finance.cpp, fin_prm.cpp no mapa):
# - 0x599800 (Finance::add_players_sold, valor de 64 bits): pelo pagamento da transferencia
#   (TRANSFER_MANAGER::pay_fee_and_record_clauses 0x8ca8a0), pelas parcelas (0x8caf8c, 0x8cb326, 0x8cb4b4) e pela parte
#   dos clubes anteriores (0x8e4f20). Toda transferencia com valor entre clubes.
# - 0x599890 (Finance::pay_players_bought, 64 bits): pelo pagamento da transferencia (0x8ca8a0) e pelas parcelas.
# - 0x5998e0 (Finance::pay_sign_on_fee): pela oferta de contrato aceita (0x8c38b0) e pela mudanca do jogador de clube
#   (TRANSFER_MANAGER::move_player_to_club 0x8c9720).
# As transferencias rodam na carreira automatica ("cpp_10 big transfer" 8, a noticia da transferencia
# finalizada da 261a, "cps_3 compensation" 8 na base).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x599800: thiscall (mov ecx, [esp + 0x14] e test ecx antes do call em 0x8ca998; mov ecx, edi em 0x8caf87), 2 dwords
#   (push alto, push baixo: o __int64 do mapa), ret 8. 0x599890: thiscall (mov ecx, [esp + 0x34] em 0x8ca97d; mov ecx,
#   ebp em 0x8caf7e), 2 dwords (push edx/eax depois do cdq), ret 8. 0x5998e0: thiscall (mov ecx, eax = financas da
#   0x5a0590 em 0x8c39c2 e 0x8ca756), 1 arg (empurrado antes do argumento da 0x5a0590), ret 4. As tres nao chamam nada.
# Comparadores: lado a lado no registro de financas (0x167 bytes). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('receber_venda_de_jogador', 0x599800, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('pagar_compra_de_jogador', 0x599890, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('pagar_luvas', 0x5998e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x3c, 0x01, 0x00, 0x00])]
DOCS = ('o dinheiro das transferências: a venda de jogador, a compra de jogador e as luvas',
        'the transfer money: the player sale, the player purchase and the signing-on fee',
        [('receber_venda_de_jogador', 'Finance::add_players_sold', '0x599800', 'finance.cpp', 'thiscall, ret 8',
          'soma o valor (64 bits) às vendas do mês (+0x78) e do ano (+0x118) e ao saldo',
          'adds the amount (64 bits) to the sales of the month (+0x78) and of the year (+0x118) and to the balance'),
         ('pagar_compra_de_jogador', 'Finance::pay_players_bought', '0x599890', 'finance.cpp', 'thiscall, ret 8',
          'soma o valor (64 bits) às compras do mês (+0x70) e do ano (+0x110) e o tira do saldo',
          'adds the amount (64 bits) to the purchases of the month (+0x70) and of the year (+0x110) and takes it from the balance'),
         ('pagar_luvas', 'Finance::pay_sign_on_fee', '0x5998e0', 'finance.cpp', 'thiscall, ret 4',
          'soma o valor às luvas do mês (+0x9c) e do ano (+0x13c) e o tira do saldo',
          'adds the amount to the signing-on fees of the month (+0x9c) and of the year (+0x13c) and takes it from the balance')])
CAMPOS = [('finanças do clube (0x167 bytes) +0 saldo (i64), +0x70/+0x110 compras (i64, mês/ano), +0x78/+0x118 vendas (i64, mês/ano), +0x9c/+0x13c luvas (mês/ano); valores; lê e grava',
           'club finance (0x167 bytes) +0 balance (i64), +0x70/+0x110 purchases (i64, month/year), +0x78/+0x118 sales (i64, month/year), +0x9c/+0x13c signing-on fees (month/year); values; reads and writes')]
INTERESSA = ['CMRTE', 'SGE']
