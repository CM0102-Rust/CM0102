# Bloco das competições, o fim de temporada da liga: os direitos de TV (League::pay_tv_rights 0x683010, 6 chamadas na
# carreira: as ligas de pagamento por TV, como a EngPrm 0x574a20), as honras da temporada (League::record_season_honours
# 0x684640, 4: no fim de cada liga) e o registro no histórico de competições (comp_util_add_history_record 0x4afce0,
# 15: as honras das ligas e as copas). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo
# real comparando. Sem SEH, sem sorteio; a 0x684640 tem duas locais (as sedes) que só lê quando a 0x5fa730 as grava.
# Caixas só fora do caminho comum (clube nulo na 0x5a0590; nação nula na 0x53b3d0). Comparadores: os direitos de TV
# pelas finanças (0x167 bytes) de cada clube da tabela, com a cópia sem a notícia (0x48ea40, que só sai para o clube
# do técnico humano e aloca: a da original fica, uma vez só); as honras pelos clubes (o +0x5b) e o histórico; o
# registro pelo histórico. Sem linha livre no histórico (a 0x527f60 faria realloc), só a original nas duas. Nenhuma
# em laço quente.
SAVE = False
ROTINAS = [('pagar_direitos_de_tv', 0x683010, [0xa1, 0x38, 0x2a, 0xae, 0x00]),
           ('registrar_honras_da_temporada', 0x684640, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('acrescentar_ao_historico', 0x4afce0, [0xa1, 0x7c, 0x23, 0xae, 0x00])]
DOCS = ('o fim de temporada da liga: os direitos de TV, as honras da temporada e o registro no histórico de competições',
        'the league season end: the TV rights, the season honours and the competition history record',
        [('pagar_direitos_de_tv', 'League::pay_tv_rights', '0x683010', 'league.cpp', 'thiscall',
          'soma o valor à receita de TV de cada clube da tabela (com filtro, só os de +0x5b igual) e manda a notícia (0x48ea40)',
          'adds the amount to the TV revenue of every club in the table (with a filter, only those with +0x5b equal) and sends the news (0x48ea40)'),
         ('registrar_honras_da_temporada', 'League::record_season_honours', '0x684640', 'league.cpp', 'thiscall',
          'campeão, vice e terceiro no histórico (0x4afce0), com a seleção do país-sede como destaque quando a competição pede (+0x88, 0x5fa730, 0x53b3d0); o campeão que é desta divisão limpa o +0x5b dos clubes que apontam para ela',
          'champion, runner-up and third into the history (0x4afce0), with the host nation team as the featured entry when the competition asks (+0x88, 0x5fa730, 0x53b3d0); a champion of this division clears +0x5b of the clubs pointing at it'),
         ('acrescentar_ao_historico', 'comp_util_add_history_record', '0x4afce0', 'comp_util.cpp', 'cdecl',
          'grava competição, ano e quatro valores na linha livre do histórico de competições (sem ela, cresce 20 linhas pela 0x527f60); devolve a linha',
          'stores competition, year and four values in the free row of the competition history (without one, grows 20 rows by 0x527f60); returns the row')])
CAMPOS = [('[0xae2a38] tabela de finanças; finanças do clube (0x167 bytes) +0, +0xb8, +0x158 (saldo e receita de TV); grava (pagar_direitos_de_tv, pela 0x5987c0)',
           '[0xae2a38] finance table; club finance (0x167 bytes) +0, +0xb8, +0x158 (balance and TV revenue); writes (pagar_direitos_de_tv, through 0x5987c0)'),
          ('TClub +0x5b: competição (divisão anterior); lê (pagar_direitos_de_tv) e zera (registrar_honras_da_temporada); TClub +0x57: divisão; lê',
           'TClub +0x5b: competition (previous division); reads (pagar_direitos_de_tv) and clears (registrar_honras_da_temporada); TClub +0x57: division; reads'),
          ('[0xae23d4] / [0xae237c]: histórico de competições (26 bytes: +4 competição, +8 ano, +0xa/+0xe/+0x12/+0x16) e contagem; [0xae2c92] ano; lê e grava (acrescentar_ao_historico)',
           '[0xae23d4] / [0xae237c]: competition history (26 bytes: +4 competition, +8 year, +0xa/+0xe/+0x12/+0x16) and count; [0xae2c92] year; reads and writes (acrescentar_ao_historico)'),
          ('TNation (0x122 bytes, [0xae23a8]) +0x8e; lê (registrar_honras_da_temporada)', 'TNation (0x122 bytes, [0xae23a8]) +0x8e; reads (registrar_honras_da_temporada)')]
INTERESSA = ['SGE', 'CMRTE']
