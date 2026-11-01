# As receitas e a compensacao (finance.cpp, fin_prm.cpp no mapa):
# - 0x5999a0 (Finance::add_prize_money): pelas copas (Cup::process_fixture 0x51a150, EuropeanCup::process_fixture
#   0x583b10, os sorteios das fases 0x585680, 0x92a140), pelo fim das ligas (League::pay_prize_money 0x682e10) e pelas
#   honras regionais; as copas inglesas rodam na carreira ("cpx_11 premio pago" 64 na base).
# - 0x5987c0 (Finance::add_tv_revenue): so pelos direitos de TV da liga (League::pay_tv_rights 0x683010), chamados pela
#   montagem da tabela de 34 ligas (EngPrm/EngFirst/EngSecond/EngThird/EngConf::create_table entre elas): na criacao do
#   jogo e na temporada nova de cada liga.
# - 0x599920 (Finance::pay_wages): so pela liberacao sem custo (TRANSFER_MANAGER::free_transfer_staff 0x8c1af0, a
#   compensacao do contrato; "cps_3 compensacao" 8 na base, a cpy_13 da 261a).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a): as tres thiscall com ecx = as financas
# devolvidas pela 0x5a0590 (mov ecx, eax em 0x51a2fc, 0x68307f e 0x8c1d0b), 1 arg empurrado antes do argumento da
# 0x5a0590 (push eax em 0x51a2ef, push ebp em 0x68305a, push edx em 0x8c1d04), ret 4. Nao chamam nada.
# Comparadores: lado a lado no registro de financas (0x167 bytes). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('receber_premio_em_dinheiro', 0x5999a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x48, 0x01, 0x00, 0x00]),
           ('receber_direitos_de_tv', 0x5987c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x58, 0x01, 0x00, 0x00]),
           ('pagar_salarios', 0x599920, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x91, 0x30, 0x01, 0x00, 0x00])]
DOCS = ('as receitas e a compensação: o prêmio em dinheiro, os direitos de TV e o salário pago',
        'the income and the compensation: the prize money, the TV rights and the wages paid',
        [('receber_premio_em_dinheiro', 'Finance::add_prize_money', '0x5999a0', 'finance.cpp', 'thiscall, ret 4',
          'soma o valor aos prêmios do mês (+0xa8) e do ano (+0x148) e ao saldo',
          'adds the amount to the prize money of the month (+0xa8) and of the year (+0x148) and to the balance'),
         ('receber_direitos_de_tv', 'Finance::add_tv_revenue', '0x5987c0', 'finance.cpp', 'thiscall, ret 4',
          'soma o valor à TV do mês (+0xb8) e do ano (+0x158) e ao saldo',
          'adds the amount to the TV income of the month (+0xb8) and of the year (+0x158) and to the balance'),
         ('pagar_salarios', 'Finance::pay_wages', '0x599920', 'finance.cpp', 'thiscall, ret 4',
          'soma o valor aos salários do mês (+0x90) e do ano (+0x130) e o tira do saldo (a compensação da liberação sem custo)',
          'adds the amount to the wages of the month (+0x90) and of the year (+0x130) and takes it from the balance (the free release compensation)')])
CAMPOS = [('finanças do clube (0x167 bytes) +0 saldo (i64), +0x90/+0x130 salários, +0xa8/+0x148 prêmios, +0xb8/+0x158 TV (mês/ano); valores; lê e grava',
           'club finance (0x167 bytes) +0 balance (i64), +0x90/+0x130 wages, +0xa8/+0x148 prize money, +0xb8/+0x158 TV (month/year); values; reads and writes')]
INTERESSA = ['CMRTE', 'SGE']
