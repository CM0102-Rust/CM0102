# Os idiomas do jogador e o regen (player_stats.cpp): acrescentar um idioma (PlayerStats::add_language 0x7c6a30, 4071
# chamadas na carreira até 2002-06-01: os regens 0x7ab4a9/0x7abe55, o jogo 0x5d3020/0x5d33e9, a 0x7bfd50
# e a 0x7c6ca0), melhorar um idioma (PlayerStats::improve_language 0x7c6ca0, 1443, do jogo 0x5d3044/0x5d340a) e zerar as
# estatísticas do jogador regenerado (PlayerStats::reset_player 0x7c6650, 249, do regen 0x7a59e0). A fluência é lida
# pela escalação (0x7c6830 em 0x6e4e76/0x6e72ea, já em Rust). malloc/realloc/qsort/bsearch originais, a comparação
# 0x7c6d40 (id, idioma; 0xff vale qualquer) e a 0x58f110 (só grava −1 no segundo argumento). Sem SEH, sem sorteio, sem
# leitura de lixo (o idioma vem num registro inteiro com o alto de lixo do chamador: a original e a cópia só leem o
# byte). Caixas fatais (sem memória; pessoa nula ou não jogadora na 0x7c6650) como a original. Iguais no
# cm0102-1080.exe. Atenção: a 0x7c6bf0 (tirar o idioma, da mesma tabela) está DESLIGADA pelo resíduo de pilha do
# bsearch dela; a 0x7c6ca0 também usa o bsearch (conferir na carreira ampla). Simulação: as mutações pela carreira e
# a confirmação no jogo real comparando. Comparadores: a 0x7c6a30 pelo plano (realoca); a 0x7c6ca0 lado a lado no
# registro achado (sem ele, só a original); a 0x7c6650 lado a lado.
SAVE = False
ROTINAS = [('acrescentar_idioma', 0x7c6a30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('melhorar_idioma', 0x7c6ca0, [0x83, 0xec, 0x08, 0x53, 0x56]),
           ('zerar_estatisticas_do_jogador', 0x7c6650, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os idiomas do jogador (acrescentar e melhorar) e zerar as estatísticas do jogador regenerado',
        "the player's languages (adding and improving) and resetting a regenerated player's statistics",
        [('acrescentar_idioma', 'PlayerStats::add_language', '0x7c6a30', 'player_stats.cpp', 'thiscall, ret 0x10',
          'põe no fim da tabela dos idiomas (6 bytes: id, idioma, fluência até 10; nasce com 100, cresce 100) o registro novo e, se pedido, ordena (qsort)',
          'appends the new entry to the language table (6 bytes: id, language, fluency up to 10; starts at 100, grows by 100) and sorts it when asked (qsort)'),
         ('melhorar_idioma', 'PlayerStats::improve_language', '0x7c6ca0', 'player_stats.cpp', 'thiscall, ret 0x10',
          'acha o registro da pessoa no idioma (bsearch) e soma o aumento à fluência, até 10; sem ele, acrescenta o idioma com a fluência do aumento',
          "finds the person's entry for the language (bsearch) and adds the increase to the fluency, up to 10; without one, adds the language with the increase as fluency"),
         ('zerar_estatisticas_do_jogador', 'PlayerStats::reset_player', '0x7c6650', 'player_stats.cpp', 'thiscall, ret 4',
          'libera os 6 registros de categoria do jogador (+0xe = −1, +1 nos livres), zera os totais da liga e as notas recentes e esvazia o histórico',
          "frees the player's 6 category records (+0xe = −1, +1 to the free count), clears the league totals and recent ratings and empties the history")])
CAMPOS = [('estatísticas +0x3c/+0x40/+0x44: a tabela dos idiomas (6 bytes: +0 id da pessoa, +4 idioma, +5 fluência 0..10), capacidade e conta; ponteiro e valor; lê e grava (acrescentar_idioma, melhorar_idioma)',
           'stats +0x3c/+0x40/+0x44: the language table (6 bytes: +0 person id, +4 language, +5 fluency 0..10), capacity and count; pointer and value; reads and writes (acrescentar_idioma, melhorar_idioma)'),
          ('estatísticas +0x30: registro de 39 bytes por pessoa (+0..+0x14 os 6 índices de categoria, +0x18/+0x1c histórico, +0x20/+0x22 totais da liga, +0x24..+0x26 notas recentes); lê e grava (zerar_estatisticas_do_jogador)',
           'stats +0x30: 39-byte record per person (+0..+0x14 the 6 category indices, +0x18/+0x1c history, +0x20/+0x22 league totals, +0x24..+0x26 recent ratings); reads and writes (zerar_estatisticas_do_jogador)'),
          ('estatísticas +4: registros de categoria de 32 bytes (+0xe = −1 livre); +0x38 quantos livres; grava (zerar_estatisticas_do_jogador)',
           'stats +4: 32-byte category records (+0xe = −1 free); +0x38 free count; writes (zerar_estatisticas_do_jogador)'),
          ('pessoa +0 id, +0x61 jogador; lê', 'person +0 id, +0x61 player; reads')]
INTERESSA = ['CMRTE', 'CMScout', 'GPF2']
