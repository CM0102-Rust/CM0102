# A tabela dos estaduais: Carioca, Catarinense e Paulista (o slot create_table/load_region_clubs de bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp; agente C da frente das
# competições, 04/11/2026). O mesmo molde de 253 bytes nos doze estaduais (e na Série B, com o +0x57): só mudam o
# número de clubes, o fonte e a linha da caixa e o global do id; conferido instrução a instrução contra a 0x42fd10.
# Cada uma roda duas vezes na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt): o construtor e a
# virada da temporada as chamam direto (mov ecx, esi; call -> thiscall, ret 0):
# - 0x43a500 BraRegRio::create_table (253 bytes): Carioca, 12 clubes (+0x60 = [0x9cf9dc]).
# - 0x43bb70 BraRegSanta::create_table (253 bytes): Catarinense, 10 clubes (+0x60 = [0x9cf9b0]).
# - 0x43cc40 BraRegSp::create_table (253 bytes): Paulista, 16 clubes (+0x60 = [0x9cf9e4]).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl;
# - 0x687430 (League::init_table_row): push 0; push 0; push esi (o clube); push eax (al = a linha, os bits de cima
#   do id do estadual); mov ecx, edi -> thiscall, ret 0x10 (a cópia passa a linha limpa: a 0x687430 só lê o char);
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca a tabela), conferindo o plano feito antes: os clubes do estadual na ordem do banco
# nas linhas, +0x3e e a devolução (1 se encheu).
# Determinismo: percorre o banco na ordem; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqm8_tabela_do_carioca', 0x43a500, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm8_tabela_do_catarinense', 0x43bb70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqm8_tabela_do_paulista', 0x43cc40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela dos estaduais: Carioca, Catarinense e Paulista',
        'the table of the state leagues: Rio, Santa Catarina and Sao Paulo',
        [('cqm8_tabela_do_carioca', 'BraRegRio::create_table', '0x43a500', 'bra_reg_rio.cpp', 'thiscall, ret 0',
          'O Carioca tem 12 clubes: os que têm Carioca como divisão reserva no banco (o campo que guarda o estadual ou o regional), '
          'na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios',
          'the Rio league has 12 clubs: those with it as their reserve division in the database (the field '
          'that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund'),
         ('cqm8_tabela_do_catarinense', 'BraRegSanta::create_table', '0x43bb70', 'bra_reg_santa.cpp', 'thiscall, ret 0',
          'O Catarinense tem 10 clubes: os que têm Catarinense como divisão reserva no banco (o campo que guarda o estadual ou o regional), '
          'na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios',
          'the Santa Catarina league has 10 clubs: those with it as their reserve division in the database (the field '
          'that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund'),
         ('cqm8_tabela_do_paulista', 'BraRegSp::create_table', '0x43cc40', 'bra_reg_sp.cpp', 'thiscall, ret 0',
          'O Paulista tem 16 clubes: os que têm Paulista como divisão reserva no banco (o campo que guarda o estadual ou o regional), '
          'na ordem do banco; a tabela só vale cheia; sem dinheiro da TV nem fundo de prêmios',
          'the Sao Paulo league has 16 clubs: those with it as their reserve division in the database (the field '
          'that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund')])
CAMPOS = [('clube +0x60: divisão reserva (ReserveDivision; no Brasil, o estadual); ponteiro; lê',
           'club +0x60: reserve division (ReserveDivision; in Brazil, the state league); pointer; reads'),
          ('[0x9cf9dc] / [0x9cf9b0] / [0x9cf9e4]: ids do Carioca, do Catarinense, do Paulista; id; lê', '[0x9cf9dc] / [0x9cf9b0] / [0x9cf9e4]: Rio, Santa Catarina, Sao Paulo ids; id; reads'),
          ('liga +0x3e: número de times da tabela; valor; grava', 'league +0x3e: number of teams in the table; value; writes'),
          ('liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; grava', 'league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes')]
INTERESSA = ['CMRTE']
