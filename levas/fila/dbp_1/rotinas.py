# As preferências da pessoa no diário (game_daily_staff_update 0x5ce2d0, que roda para cada pessoa a cada dia): a
# reputação do clube vista pela pessoa (0x545e90, duas chamadas: o interesse em sair para um clube maior) e quantos
# favoritos e rejeitados dela estão num clube (0x547c90, 0x547ce0). Simulação: as mutações pela carreira até
# 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio (a 0x7c1e30, a 0x4a4850 e a 0x4cf5a0 são
# chamadas pelo endereço do jogo; nenhuma sorteia), sem variável local lida sem valor (a local de 8 bytes da
# 0x545e90 é gravada antes de lida; a original escreve nos próprios argumentos, a cópia não). As três rodam em laço
# quente (cada pessoa, cada dia): correm o risco da 0x547770 (233ª), de o desvio mudar o resíduo de pilha que uma
# original lê depois.
SAVE = False
ROTINAS = [('reputacao_do_clube_para_pessoa', 0x545e90, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('contar_rejeitados_no_clube', 0x547c90, [0x8b, 0x4c, 0x24, 0x04, 0x32, 0xc0]),
           ('contar_favoritos_no_clube', 0x547ce0, [0x8b, 0x4c, 0x24, 0x04, 0x32, 0xc0])]
DOCS = ('a reputação do clube para a pessoa e quantos favoritos e rejeitados dela estão no clube',
        'the club reputation for a person and how many of the favoured and disliked staff of the person are at the club',
        [('reputacao_do_clube_para_pessoa', 'club_reputation_for_staff', '0x545e90', 'Database.cpp', 'cdecl',
          'a reputação do clube (+0x80) vista pela pessoa: com a divisão mais forte que o clube, pesa pela posição na liga (tabela 0x9b59c0, mistura com a temporada anterior antes de 20 jogos) e, para quem joga no clube, pela média 0x7c1e30; ±500 se o clube é favorito ou rejeitado da pessoa; grava [0xae1080]',
          'the club reputation (+0x80) as seen by the person: with a division stronger than the club, weighs it by the league position (table 0x9b59c0, mixed with last season before 20 games) and, for someone playing at the club, by the 0x7c1e30 average; ±500 when the club is a favoured or disliked one; writes [0xae1080]'),
         ('contar_rejeitados_no_clube', 'staff_count_disliked_staff_at_club', '0x547c90', 'Database.cpp', 'cdecl',
          'quantas das três pessoas rejeitadas (preferências +0x28..+0x30) estão no clube (+0x39)',
          'how many of the three disliked staff (preferences +0x28..+0x30) are at the club (+0x39)'),
         ('contar_favoritos_no_clube', 'staff_count_favoured_staff_at_club', '0x547ce0', 'Database.cpp', 'cdecl',
          'quantas das três pessoas favoritas (preferências +0x1c..+0x24) estão no clube (+0x39)',
          'how many of the three favoured staff (preferences +0x1c..+0x24) are at the club (+0x39)')])
CAMPOS = [('TClub +0x57/+0x5b divisão e divisão anterior, +0x5f posição anterior, +0x80 reputação; competição +0x69 reputação; liga [0xadadfc] +8 tabela, +0x3e clubes; linha da tabela +4 jogos, +0x73 posição; lê (reputacao_do_clube_para_pessoa)',
           'TClub +0x57/+0x5b division and previous division, +0x5f previous position, +0x80 reputation; competition +0x69 reputation; league [0xadadfc] +8 table, +0x3e clubs; table row +4 games, +0x73 position; reads (reputacao_do_clube_para_pessoa)'),
          ('[0xae1080]: o contrato achado pela 0x4cf5a0; grava (reputacao_do_clube_para_pessoa)', '[0xae1080]: the contract found by 0x4cf5a0; writes (reputacao_do_clube_para_pessoa)'),
          ('preferências (TStaff +0x65) +4..+0x18 clubes favoritos e rejeitados, +0x1c..+0x30 pessoas favoritas e rejeitadas; TStaff +0x39 clube; lê (as três)',
           'preferences (TStaff +0x65) +4..+0x18 favoured and disliked clubs, +0x1c..+0x30 favoured and disliked staff; TStaff +0x39 club; reads (all three)')]
INTERESSA = ['SGE', 'CMRTE']
