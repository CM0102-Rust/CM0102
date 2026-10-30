# A diretoria e o técnico (manager_manager.cpp): pensar em demitir o técnico (MANAGER_MANAGER::consider_sacking_manager
# 0x69aba0, depois de cada jogo pela 0x695240 e pelas 0x6a5610/0x6a9e60), dar o ultimato (issue_board_ultimatum
# 0x6ac560, só pela 0x69aba0) e tirar o técnico do cargo (remove_club_manager 0x69b790, pela fila das saídas e por 14
# outras). A 0x69aba0 sorteia, consulta a tabela da divisão e a nota dos candidatos e põe a saída na lista [this+4]; a
# 0x6ac560 tem SEH só pela lista local (LLIST 0x68d020/0x68d040, chamadas pela cópia na mesma ordem); a 0x69b790 tira o
# contrato, solta a notícia (0x6a1b00) e grava o histórico em arquivo (0x869aa0). Sem leitura de lixo (a 0x6ac560
# empurra palavras com a metade alta do registro: reproduzida onde a 0x5acbe0 a usa, ver o comentário). Iguais no
# cm0102-1080.exe. Comparadores: a 0x69aba0 só a original (sorteia e aloca conforme o sorteio); a 0x6ac560 só a original
# com a lista dos ultimatos crescendo de 1 quando devolve 1; a 0x69b790 só a original, conferida depois (a confiança em
# 3500 sem `manter` e nenhuma marca apagável nos contratos do elenco).
SAVE = False
ROTINAS = [('pensar_em_demitir_o_tecnico', 0x69aba0, [0x83, 0xec, 0x08, 0xa1, 0xcc, 0x6e, 0xdd, 0x00]),
           ('dar_o_ultimato_da_diretoria', 0x6ac560, [0x6a, 0xff, 0x68, 0x88, 0xd0, 0x95, 0x00]),
           ('tirar_o_tecnico_do_cargo', 0x69b790, [0x83, 0xec, 0x08, 0x53, 0x55])]
DOCS = ('a diretoria e o técnico: pensar em demitir o técnico, dar o ultimato e tirar o técnico do cargo',
        'the board and the manager: considering the sacking, issuing the ultimatum and removing the manager',
        [('pensar_em_demitir_o_tecnico', 'MANAGER_MANAGER::consider_sacking_manager', '0x69aba0', 'manager_manager.cpp', 'thiscall, ret 8',
          'com a confiança da diretoria baixa e o técnico no cargo há 150 dias: o limite pela tabela da divisão, pelas séries e pela marca do técnico; sorteios do presidente seguram, dão o ultimato, deixam o técnico sair ou o demitem (a saída na lista, motivo 0x69b350), ou trocam pelo melhor candidato com a notícia da pressão',
          'with low board confidence and the manager in the job for 150 days: the limit from the division table, the runs and the manager mark; chairman draws hold back, issue the ultimatum, let the manager leave or sack him (the departure queued, reason 0x69b350), or look for a better candidate with the pressure story'),
         ('dar_o_ultimato_da_diretoria', 'MANAGER_MANAGER::issue_board_ultimatum', '0x6ac560', 'manager_manager.cpp', 'thiscall, ret 4',
          'o primeiro jogo do clube nas próximas 8 semanas (fora de hoje, ainda por jogar): a notícia do ultimato com o adversário e o clube na lista dos ultimatos; devolve 1, ou 0 sem jogo',
          'the club\'s first match in the next 8 weeks (not today, still to play): the ultimatum story with the opponent and the club in the ultimatum list; returns 1, or 0 without a match'),
         ('tirar_o_tecnico_do_cargo', 'MANAGER_MANAGER::remove_club_manager', '0x69b790', 'manager_manager.cpp', 'thiscall, ret 0x10',
          'o técnico sai: a data da saída, o elenco e a comissão, o contrato (fora da seleção), a notícia (fora do tipo 3), a confiança em 3500 (sem manter), o registro da troca, as multas e as queixas e marcas dos contratos do elenco',
          'the manager leaves: the departure date, the squad and the staff, the contract (outside national teams), the story (outside type 3), confidence at 3500 (unless kept), the job change record, the fines and the squad contracts\' grievances and flags')])
CAMPOS = [('linha da diretoria [this] + id×73: +2 bit 2 (saída marcada), +6 confiança, +0x18 data da última notícia; valores; lê e grava (pensar_em_demitir_o_tecnico), grava +6 = 3500 (tirar_o_tecnico_do_cargo)',
           'board row [this] + id×73: +2 bit 2 (departure flagged), +6 confidence, +0x18 date of the last story; values; reads and writes (pensar_em_demitir_o_tecnico), writes +6 = 3500 (tirar_o_tecnico_do_cargo)'),
          ('MANAGER_MANAGER +4, +0x10: a lista das saídas (6 bytes) e a dos ultimatos (4 bytes, o id do clube); ponteiro; acrescenta (pensar_em_demitir_o_tecnico, dar_o_ultimato_da_diretoria)',
           'MANAGER_MANAGER +4, +0x10: the departure list (6 bytes) and the ultimatum list (4 bytes, the club id); pointer; appends (pensar_em_demitir_o_tecnico, dar_o_ultimato_da_diretoria)'),
          ('não jogador do técnico (+0x69) +0xa: a segurança no cargo; valor; lê e grava −250 com piso (pensar_em_demitir_o_tecnico)',
           'manager non-player (+0x69) +0xa: job security; value; reads and writes −250 with a floor (pensar_em_demitir_o_tecnico)'),
          ('não jogador do presidente (+0xbf, +0x69) +0x1d: a paciência; valor; lê (pensar_em_demitir_o_tecnico)',
           'chairman non-player (+0xbf, +0x69) +0x1d: patience; value; reads (pensar_em_demitir_o_tecnico)'),
          ('linha da tabela da divisão (0x4a4850) +4 posição, +0x14..+0x18 últimos resultados, +0x19/+0x41/+0x55/+0x2d séries, +0x74; valores; lê (pensar_em_demitir_o_tecnico)',
           'division table row (0x4a4850) +4 position, +0x14..+0x18 last results, +0x19/+0x41/+0x55/+0x2d runs, +0x74; values; reads (pensar_em_demitir_o_tecnico)'),
          ('staff +0x29 / +0x3e: a data no cargo da seleção / do clube; valor; grava a de hoje (tirar_o_tecnico_do_cargo)',
           'staff +0x29 / +0x3e: national team / club job date; value; writes today (tirar_o_tecnico_do_cargo)'),
          ('contrato +0x40 (marcas 0x80, 0x100) e +0x45 (queixas 0x8000000, 8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000, 0x10000000); valores; apaga (tirar_o_tecnico_do_cargo)',
           'contract +0x40 (flags 0x80, 0x100) and +0x45 (grievances 0x8000000, 8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000, 0x10000000); values; clears (tirar_o_tecnico_do_cargo)'),
          ('[0xdd6ec8] data a partir da qual a diretoria pensa em demitir; valor; lê (pensar_em_demitir_o_tecnico)',
           '[0xdd6ec8] date from which boards consider sackings; value; reads (pensar_em_demitir_o_tecnico)')]
INTERESSA = ['CMRTE']
