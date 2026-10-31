# As receitas da semana e o centro de treino (finance.cpp): FinanceTable::add_weekly_merchandising_income 0x59b7e0 e
# Finance::sell_season_tickets 0x599a20 (as duas pelo passe diário das finanças 0x59b9c0, no dia 6 da semana, para os
# clubes de liga; os ingressos só enquanto a temporada não começou, pela 0x59b4c0) e
# FinanceTable::upgrade_training_facilities 0x59b630 (pelo fim da temporada do clube 0x59cfa0, com o pedido de treino
# aprovado pela diretoria: passageira, raramente roda até 2002-06-01). Iguais no cm0102-1080.exe. Sem SEH; x87 em
# precisão dupla (fild/fmul/_ftol 0x945640, as comparações fcom com o resultado ainda no registrador); só chamam o
# sorteio 0x90cfc0. Comparadores: as três lado a lado (registro de finanças de 0x167 bytes, lugares do estádio, nível do
# centro de treino e o estado do sorteio; a cópia sem contar os sorteios).
SAVE = False
ROTINAS = [('receita_do_merchandising', 0x59b7e0, [0x51, 0x53, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('ingressos_da_temporada', 0x599a20, [0x83, 0xec, 0x14, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00]),
           ('melhorar_o_centro_de_treino', 0x59b630, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10])]
DOCS = ('as receitas da semana e o centro de treino: o merchandising, os ingressos da temporada e a obra do centro de treino',
        'the weekly income and the training ground: merchandising, season tickets and the training facilities works',
        [('receita_do_merchandising', 'FinanceTable::add_weekly_merchandising_income', '0x59b7e0', 'finance.cpp', 'thiscall, ret 4',
          'público × reputação × fator (pelo estádio do clube de +0xbf ou pela reputação; Japão um degrau acima), × (sorteio(11) + 95)%, somado em +0xbc, +0x15c e no saldo',
          'attendance × reputation × factor (by the stadium of the +0xbf club or by reputation; Japan one step up), × (random(11) + 95)%, added to +0xbc, +0x15c and the balance'),
         ('ingressos_da_temporada', 'Finance::sell_season_tickets', '0x599a20', 'finance.cpp', 'thiscall, ret 4',
          'estádio mínimo de 1500 lugares; a procura pelo valor no gerente de técnicos, público e reputação, até 87,5% dos lugares; os novos vendidos pela fração do passo (+0x164) e sorteio; razão em +0x160; receita (com aplicar) pelo preço, tipo da nação e clubes-chave; a loja com dois sorteios pela reputação',
          'stadium of at least 1500 places; demand by the manager-table value, attendance and reputation, up to 87.5% of the places; new holders by the step fraction (+0x164) and a random draw; ratio at +0x160; income (with apply) by price, nation type and key clubs; the shop with two random draws by reputation'),
         ('melhorar_o_centro_de_treino', 'FinanceTable::upgrade_training_facilities', '0x59b630', 'finance.cpp', 'thiscall, ret 4',
          'o nível do centro de treino (+0x7f) sobe pela reputação (até 20) e cada degrau cobra a obra em +0x8c, +0x12c e no saldo',
          'the training facilities level (+0x7f) rises by reputation (up to 20) and each step charges the works to +0x8c, +0x12c and the balance')])
CAMPOS = [('finanças do clube (0x167 bytes em [[0xae2a38]]) +0 saldo (i64), +0xbc e +0x15c receita da loja, +0xac e +0x14c receita dos ingressos, +0x8c e +0x12c obras; valores; grava (as três)',
           'club finance (0x167 bytes at [[0xae2a38]]) +0 balance (i64), +0xbc and +0x15c shop income, +0xac and +0x14c ticket income, +0x8c and +0x12c works; values; writes (all three)'),
          ('finanças do clube +8 id do clube, +0x18 ingressos da temporada vendidos, +0x160 razão vendidos/lugares (f32), +0x164 passo da venda; valores; lê e grava (ingressos_da_temporada)',
           'club finance +8 club id, +0x18 season tickets sold, +0x160 sold/places ratio (f32), +0x164 sale step; values; reads and writes (ingressos_da_temporada)'),
          ('clube +0x53 nação, +0x57 liga, +0x69 estádio, +0x6d byte, +0x73 público, +0x80 reputação, +0xbf clube ligado; ponteiros/valores; lê (receita_do_merchandising, ingressos_da_temporada)',
           'club +0x53 nation, +0x57 league, +0x69 stadium, +0x6d byte, +0x73 attendance, +0x80 reputation, +0xbf linked club; pointers/values; reads (receita_do_merchandising, ingressos_da_temporada)'),
          ('clube +0x7f nível do centro de treino; valor; lê e grava (melhorar_o_centro_de_treino)',
           'club +0x7f training facilities level; value; reads and writes (melhorar_o_centro_de_treino)'),
          ('estádio +0xf nível, +0x3c lugares; valores; lê e grava os lugares abaixo de 1500 (ingressos_da_temporada)',
           'stadium +0xf level, +0x3c places; values; reads and writes places below 1500 (ingressos_da_temporada)'),
          ('nação +0 id (Japão 0x9cf36c, Alemanha 0x9cf314), +0x7e tipo; valores; lê (receita_do_merchandising, ingressos_da_temporada)',
           'nation +0 id (Japan 0x9cf36c, Germany 0x9cf314), +0x7e type; values; reads (receita_do_merchandising, ingressos_da_temporada)'),
          ('[[0xb67a44]] + 73·id + 8: valor do clube no gerente de técnicos (i16); lê (ingressos_da_temporada)',
           '[[0xb67a44]] + 73·id + 8: club value in the manager table (i16); reads (ingressos_da_temporada)')]
INTERESSA = ['CMRTE', 'SGE']
