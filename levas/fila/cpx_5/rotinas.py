# O save das finanças e do gerente de elencos (fin_prm.cpp, squad_manager.cpp): FinanceTable::save_finance_dat
# 0x59b3b0 (passo 0x1b do save_game_data, 0x82f1e5), FinanceTable::load_from_save 0x59b2a0 (do construtor 0x59a55c,
# na carga do save) e SQUAD_MANAGER::save 0x859cc0 (passo 0x24, 0x82f4cb). Database.cpp e index.cpp estão esgotados
# (sobram 0x5339a0, 8 KB com SEH, que só roda trocando o idioma dos nomes; 0x60d780, só com chave faltando, a caixa
# espera clique; 0x617780, começa com call, sem prólogo; e o inicializador estático 0x524cd0, que roda antes do
# jogo): este item e os cpx_6/cpx_7 vêm dos .dat/.tmp dos subsistemas que o save_game_data grava (MAPA_SAVE.md §2),
# de arquivos que nenhum outro agente usa. Iguais no cm0102-1080.exe; sem SEH; sem x87; só chamam o contêiner (em
# Rust desde a S1) e a caixa de erro. Comparadores: só a original (gravam/leem o contêiner); a prova é a do save
# (o .qst gravado com e sem a leva) e, pela 0x59b2a0, a carga do FeriasTesteSave (CARGA).
SAVE = True
CARGA = True  # a 0x59b2a0 só roda na carga: a prova do save também carrega o save de férias com e sem a leva
ROTINAS = [('gravar_financas', 0x59b3b0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('carregar_financas', 0x59b2a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gravar_gerente_de_elencos', 0x859cc0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o save das finanças e do gerente de elencos: gravar e carregar o finance.dat e gravar o squad_manager.dat',
        'saving the finances and the squad manager: writing and reading finance.dat and writing squad_manager.dat',
        [('gravar_financas', 'FinanceTable::save_finance_dat', '0x59b3b0', 'fin_prm.cpp', 'thiscall',
          'grava no item finance.dat as finanças de todos os clubes (0x167 bytes × [0xae235c] de [this]); sem contêiner, nada; erro: a caixa "Error" (finance.CPP 0x115e, 0x1166) sem fechar o item',
          'writes the finances of every club into the finance.dat item (0x167 bytes × [0xae235c] from [this]); no container, nothing; on error the "Error" box (finance.CPP 0x115e, 0x1166) without closing the item'),
         ('carregar_financas', 'FinanceTable::load_from_save', '0x59b2a0', 'fin_prm.cpp', 'thiscall',
          'lê do item finance.dat as finanças de todos os clubes em [this] e devolve 1; sem contêiner 0; erro: a caixa "Error" (0x1137, 0x113f) e 0 (o construtor refaz as finanças do zero)',
          'reads the finances of every club from the finance.dat item into [this] and returns 1; no container 0; on error the "Error" box (0x1137, 0x113f) and 0 (the constructor rebuilds the finances from scratch)'),
         ('gravar_gerente_de_elencos', 'SQUAD_MANAGER::save', '0x859cc0', 'squad_manager.cpp', 'thiscall',
          'grava no item squad_manager.dat o u16 em +0 estendido a 4 bytes e devolve 1; erro: a caixa "Error" (squad_manager.cpp 0x68, 0x6f) e 0',
          'writes the u16 at +0, widened to 4 bytes, into the squad_manager.dat item and returns 1; on error the "Error" box (squad_manager.cpp 0x68, 0x6f) and 0')])
CAMPOS = [('FinanceTable +0: as finanças dos clubes (0x167 bytes por clube, [0xae235c] clubes, nat_club inclusive); ponteiro; lê e grava no save (gravar_financas), grava do save (carregar_financas)',
           'FinanceTable +0: the club finances (0x167 bytes per club, [0xae235c] clubs, nat_club included); pointer; reads and writes into the save (gravar_financas), writes from the save (carregar_financas)'),
          ('SQUAD_MANAGER +0: o valor do gerente de elencos (u16; 0 no jogo novo); valor; lê e grava no save em 4 bytes (gravar_gerente_de_elencos)',
           'SQUAD_MANAGER +0: the squad manager value (u16; 0 in a new game); value; reads and writes into the save as 4 bytes (gravar_gerente_de_elencos)'),
          ('[0xae235c] quantidade de clubes; valor; lê (gravar_financas, carregar_financas)',
           '[0xae235c] club count; value; reads (gravar_financas, carregar_financas)')]
INTERESSA = ['SGE', 'CMRTE']
