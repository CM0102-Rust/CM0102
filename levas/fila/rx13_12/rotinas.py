# A tabela da Liga de Gales (wel_first.cpp; leitor rx13, rodada 6, 25/11/2026). Simulação com Gales carregado: o
# construtor (0x924e30, já Rust) a chama na criação do jogo e a virada de temporada a refaz; LIGAS = 'ENG SCO WAL' e
# ATE = '2002-08-01', como as outras de Gales. Usa o fill_british_division_table / compare_british_division_table do
# sco_first.rs e o LEAGUE_OF_WALES do wel_first.rs (no motor).
# - 0x925b30 WelFirst::create_table thiscall ret 0: +0x3e = 0x12; malloc(0x426) em +0xb1 (nulo: caixa 0x194 e 0); os
#   clubes de [0xae235c] com a divisão [clube+0x57] igual a [0x9cf94c], na ordem da base, cada um na linha seguinte
#   (League::init_table_row 0x687430(linha, clube, 0, 0)) até encher; sem 18: 0; com 18: +0x28 =
#   League::calc_prize_fund 0x682f70(10000), +0xe1 = 1, retorna 1. Prólogo sub esp, 0x200 (6 bytes).
# Igual no cm0102-1080.exe. Comparador: só a original (aloca), contra o plano (os clubes da divisão, a contagem, o
# fundo e o modo).
SAVE = False
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
ROTINAS = [('wel_first_create_table', 0x925b30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela da Liga de Gales',
        'the League of Wales table',
        [('wel_first_create_table', 'WelFirst::create_table', '0x925b30', 'wel_first.cpp', 'thiscall, ret 0',
          'regra: a tabela da Liga de Gales tem 18 clubes, os da divisão da liga galesa na ordem da base; fundo de prêmios de 10.000 dividido pela posição; sem 18 clubes a tabela é recusada',
          'rule: the League of Wales table has 18 clubs, the clubs of the Welsh league division in database order; a prize fund of 10,000 shared by position; without 18 clubs the table is refused')])
CAMPOS = []
INTERESSA = []
