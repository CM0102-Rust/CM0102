# O fim da temporada da liga (league.cpp): League::pay_prize_money (0x682e10, 25 chamadas na cobertura de 17/10: o
# fundo de prêmios dividido pelas posições da tabela, somado no saldo de cada clube), a virtual +0x44 da liga
# League::update_staff_records_for_status (0x686940, 4 chamadas: o título, o acesso e o rebaixamento nos registros
# das pessoas, que mexem na reputação; passageira) e League::announce_team_status (0x686670, 107 chamadas: a notícia
# da situação do clube, que chama a virtual +0x44; passageira). A 0x682e10 chama a 0x5a0590 (Rust), a soma do prêmio
# 0x5999a0 (original, só grava no registro de finanças) e a notícia do prêmio 0x48e3e0 (original, aloca); a 0x686940,
# os registros 0x868c50 (original), 0x869480 e 0x86a1c0 (Rust); a 0x686670, a notícia local (0x77bbe0, 0x785790) e
# as entregas (0x786090, 0x7861e0, 0x7862d0, 0x7863f0), todas em Rust. Quadro SEH só na 0x686670 (sem destrutor: a
# notícia local não é destruída, como na original). Iguais no cm0102-1080.exe.
# Comparadores: a 0x682e10 em ensaio (a cópia sem a notícia do prêmio) lado a lado com a original nos registros de
# finanças (0x167 bytes) de todos os clubes da tabela; com +0xe1 = 3 (a virtual da competição) ou divisor zero, só a
# original. A 0x686940 e a 0x686670 só a original (os registros das pessoas e as entregas alocam; a cópia leva os
# ramos).
SAVE = False
ROTINAS = [('pagar_premios_da_liga', 0x682e10, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('registros_pela_situacao_na_liga', 0x686940, [0x8a, 0x44, 0x24, 0x08, 0x3c, 0x05]),
           ('anunciar_situacao_na_liga', 0x686670, [0x6a, 0xff, 0x68, 0x8b, 0xca, 0x95, 0x00])]
DOCS = ('o fim da temporada da liga: os prêmios em dinheiro, os registros pela situação do clube e a notícia da situação',
        "the league's end of season: the prize money, the records for the club's status and the status news item",
        [('pagar_premios_da_liga', 'League::pay_prize_money', '0x682e10', 'league.cpp', 'thiscall, ret 0',
          'divide o fundo (+0x28) por 1 + 2 + … + n e paga (n − i) partes ao clube da linha i da tabela (0x5999a0 no registro de finanças), com a notícia do prêmio acima de 1000; com +0xe1 = 3 a competição paga pela virtual +0x6c; com +0xe1 = 2, nada',
          'divides the prize fund (+0x28) by 1 + 2 + ... + n and pays (n - i) shares to the club in table row i (0x5999a0 on the finance record), with the prize news item above 1000; with +0xe1 = 3 the competition pays through virtual +0x6c; with +0xe1 = 2, nothing'),
         ('registros_pela_situacao_na_liga', 'League::update_staff_records_for_status', '0x686940', 'league.cpp', 'thiscall, ret 0x18',
          'pela situação: 5 o título (0x868c50), 0 o acesso (0x869480, peso 100), 3 o rebaixamento (0x86a1c0) nos registros das pessoas; devolve 0',
          'by status: 5 the title (0x868c50), 0 promotion (0x869480, weight 100), 3 relegation (0x86a1c0) in the staff records; returns 0'),
         ('anunciar_situacao_na_liga', 'League::announce_team_status', '0x686670', 'league.cpp', 'thiscall, ret 0x14',
          'fora dos amistosos, chama a virtual +0x44 da competição e monta a notícia 0xc (clube, competição, fase, situação, rodada, o que a virtual devolveu), entregue pelo alcance ou, pela situação, à nação, à divisão (e à de acesso ou rebaixamento) ou ao técnico do clube',
          'outside friendlies, calls the competition virtual +0x44 and builds news item 0xc (club, competition, stage, status, round, what the virtual returned), delivered by scope or, by status, to the nation, the division (and the promotion or relegation one) or the club manager')])
CAMPOS = [('liga +0xe1: o modo dos prêmios (2 nenhum, 3 pela competição); valor; lê (pagar_premios_da_liga)',
           'league +0xe1: the prize mode (2 none, 3 through the competition); value; reads (pagar_premios_da_liga)'),
          ('liga +0x28: o fundo de prêmios; valor; lê (pagar_premios_da_liga)',
           'league +0x28: the prize fund; value; reads (pagar_premios_da_liga)'),
          ('liga +0xb1 / +0x3e: a tabela (linhas de 0x3b, +0 clube) e quantas; ponteiro e valor; lê (pagar_premios_da_liga)',
           'league +0xb1 / +0x3e: the table (0x3b-byte rows, +0 club) and the count; pointer and value; reads (pagar_premios_da_liga)'),
          ('finanças do clube +0 (64 bits), +0xa8, +0x148: o saldo e os acumulados de prêmios; valor; grava (pagar_premios_da_liga, pela 0x5999a0)',
           'club finance +0 (64 bits), +0xa8, +0x148: the balance and the prize totals; value; writes (pagar_premios_da_liga, through 0x5999a0)'),
          ('liga +0x3a / +0x44 / +0xc / +0x1c: a rodada, a fase, as fases e a divisão de cima; valor e ponteiro; lê (anunciar_situacao_na_liga)',
           'league +0x3a / +0x44 / +0xc / +0x1c: the round, the stage, the stages and the division above; value and pointer; reads (anunciar_situacao_na_liga)'),
          ('competição (objeto de [0xadadfc]) +0x1c / +0x20: as divisões de acesso e de rebaixamento; id; lê (anunciar_situacao_na_liga)',
           'competition (object of [0xadadfc]) +0x1c / +0x20: the promotion and relegation divisions; id; reads (anunciar_situacao_na_liga)'),
          ('[0xdd7f00] os registros das pessoas; ponteiro; lê (registros_pela_situacao_na_liga)',
           '[0xdd7f00] the staff records; pointer; reads (registros_pela_situacao_na_liga)')]
INTERESSA = ['CMRTE']
