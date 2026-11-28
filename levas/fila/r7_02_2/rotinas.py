# Liberar o banco e carregá-lo da pasta Data (Database.cpp; leitor r7_02, rodada 7, 28/11/2026).
# - 0x528080 database_free_all_tables: cdecl(), sem retorno (0x528080..0x528530, ret; enchimento até 0x52853f). Prólogo
#   mov eax, [0xae23a4] (5 bytes, endereço absoluto, sem relativo). Chamada pela load_database, pela load_game_data e
#   pela free_game_data. Chama free (0x9452ca), GlobalFree pela importação (0x9670f4, trocada pelo alocador da DLL),
#   eng_lng_free_tables (0x544f30, thiscall, já Rust, chamada direta) com operator delete, e utils_clear_temp_folder.
#   Por que estava fora (sobras.md, sx08): "efeito único" (dbn_1: "as mutações não aparecem em prova nenhuma"). Agora
#   há o comparador: a original libera e a cópia roda nos globais de antes sem liberar nada (o que solta vai para um
#   fecho que no comparador não faz nada); os globais das tabelas e contagens (0xae2340..0xae240b) e a língua dos nomes
#   têm de bater. Na comparação, com o r7_02_1, a carga da original libera as tabelas que a cópia acabou de carregar:
#   é aí que as mutações aparecem.
# - 0x524d50 load_database_from_files: cdecl(pasta, ligado) -> load_database(pasta, ligado, 0) (0x524d50..0x524d64, ret).
#   Prólogo mov eax, [esp+8]; mov ecx, [esp+4] (8 bytes). Por que estava fora (sobras.md e dbn_1): "desviada, a
#   load_database rodaria na pilha particular e a create_game lê resíduo de pilha, como na 0x827110 desligada".
#   Conferido em 28/11: não vale mais; quem a chama (0x81ce00, 0x827110, 0x828110) já é desviado e a 0x827110 está no
#   motor desde a rodada 10 da legibilidade; a leitora do resíduo (0x697c40) já é Rust. Comparador de passagem (a carga
#   tem o seu).
# Ambas iguais no cm0102-1080.exe (corpo e prólogo).
# Tipo: simulação. A carreira não as acusa (a única liberação na criação acha as tabelas vazias): as mutações escapam
# da carreira e são confirmadas no modo comparando, pelo comparador da liberação. Precisa do r7_02_1 integrado antes
# (INTEGRAR_ANTES.txt): sem a carga desviada não há liberação com tabelas cheias no jogo comparando.
SAVE = False
ROTINAS = [('database_free_all_tables', 0x528080, [0xa1, 0xa4, 0x23, 0xae, 0x00]),
           ('load_database_from_files', 0x524d50, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04])]
DOCS = ('liberar o banco de dados e carregá-lo da pasta Data',
        'freeing the database and loading it from the Data folder',
        [('database_free_all_tables', 'database_free_all_tables', '0x528080', 'Database.cpp', 'cdecl',
          'libera cada tabela do banco (GlobalFree no modo ligado e free no separado, ou sempre free conforme a tabela) e '
          'zera o ponteiro e a contagem; no modo ligado as tabelas de dentro dos blocos comuns só são zeradas. Depois o '
          'índice dos nomes ingleses dos clubes, o objeto dos nomes ingleses, a língua dos nomes (−1) e os .lng copiados '
          'para a CM3_TEMP',
          'frees each database table (GlobalFree in the linked layout and free in the separate one, or always free by the '
          'table) and zeroes the pointer and the count; in the linked layout the tables inside the shared blocks are only '
          'zeroed. Then the clubs\' English name index, the English names object, the names language (−1) and the .lng '
          'files copied into CM3_TEMP'),
         ('load_database_from_files', 'load_database_from_files', '0x524d50', 'Database.cpp', 'cdecl',
          'carrega o banco da pasta Data: load_database(pasta, ligado, 0)',
          'loads the database from the Data folder: load_database(folder, linked, 0)')])
INTERESSA = []
