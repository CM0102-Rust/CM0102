# O premio do mes no save e a carga das notas do mes (month_award.cpp, month_ratings.cpp; leitor sx04b, rodada 5,
# 23/11/2026). Rodam na gravacao do jogo (NationAwards::save 0x76a380 chama a virtual +4 de cada premio: a do premio
# do mes e a 0x767070) e na carga (o construtor do premio do mes 0x766e10 com arquivo chama a 0x766f50; o das notas do
# mes 0x767af0 com arquivo, pelo NationAwards::load 0x76a280, a 0x767b20).
# Convencoes conferidas na desmontagem: as quatro thiscall(arquivo), ret 4; zread 0x932c30 e zwrite 0x932e10
# cdecl(destino, tamanho, quantidade, arquivo) devolvendo a quantidade; malloc 0x944e46 cdecl(n * 12). As caixas: na
# carga, "v%s %s.%s %d (%s)" (0x9a8120) com a mensagem, o titulo "Load Game Error" (0x9af3e0) e o corpo 0x9af334
# traduzidos (0x66f4e0) e a caixa 0x5e8290, devolvendo 0 (na falta de memoria, o titulo 0x9a8108 e o corpo 0x9a7ff8);
# na gravacao, a caixa "Error" (tel26_error_box) e o fim. A 0x767b20 grava a capacidade antes do malloc e relê a
# contagem para o zread e para a comparacao.
# Prologos sem chamada relativa; iguais no cm0102-1080.exe.
# Depende do sx04_1 (MONTH_AWARD_SLOT_COUNT, MONTH_AWARD_SLOT_SIZE).
# Comparadores: so a original (leem ou gravam o arquivo uma vez); nas cargas, o estado lido conferido (ate 12 vagas;
# a capacidade igual a contagem).
SAVE = True
CARGA = True  # a 0x766f50 e a 0x767b20 so rodam na carga: a prova tambem carrega o save de ferias com e sem a leva
ROTINAS = [('month_award_load_from_disk', 0x766f50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('month_award_save_to_disk', 0x767070, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('month_ratings_load_from_disk', 0x767b20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o prêmio do mês no save e a carga das notas do mês',
        "the month award in the saved game and the month ratings' load",
        [('month_award_load_from_disk', 'MONTH_AWARD::load_from_disk', '0x766f50', 'month_award.cpp', 'thiscall, ret 4',
          'regra: lê do save as doze vagas do mês (11 bytes cada) e a contagem de vagas; devolve 1, ou 0 depois da caixa "Load Game Error" da primeira leitura curta',
          'rule: reads the twelve month slots (11 bytes each) and the slot count from the save; returns 1, or 0 after the "Load Game Error" box of the first short read'),
         ('month_award_save_to_disk', 'MONTH_AWARD::save_to_disk', '0x767070', 'month_award.cpp', 'thiscall, ret 4',
          'regra: grava no save as doze vagas do mês e a contagem de vagas; uma gravação curta mostra a caixa "Error" e para',
          'rule: writes the twelve month slots and the slot count into the save; a short write shows the "Error" box and stops'),
         ('month_ratings_load_from_disk', 'MONTH_RATINGS::load_from_disk', '0x767b20', 'month_ratings.cpp', 'thiscall, ret 4',
          'regra: lê do save a contagem das notas; com notas, a capacidade vira a contagem, aloca 12 bytes por nota e as lê; depois o mês e o ano; devolve 1, ou 0 depois da caixa da primeira falha ("Load Game Error", ou "Memory Allocation Error" sem memória)',
          'rule: reads the ratings count from the save; with entries, the capacity becomes the count, 12 bytes an entry are allocated and read; then the month and the year; returns 1, or 0 after the box of the first failure ("Load Game Error", or "Memory Allocation Error" out of memory)')])
CAMPOS = [('notas do mês (MONTH_RATINGS) +0 entradas (12 bytes cada), +4 contagem, +8 capacidade; ponteiro e valores; grava (month_ratings_load_from_disk)',
           'month ratings (MONTH_RATINGS) +0 entries (12 bytes each), +4 count, +8 capacity; pointer and values; writes (month_ratings_load_from_disk)')]
INTERESSA = []
