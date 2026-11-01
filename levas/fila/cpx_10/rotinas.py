# As notícias do técnico (manager_manager.cpp): MANAGER_MANAGER::post_manager_under_pressure_news 0x6a0e70 (por pensar
# em demitir o técnico 0x69aba0, já em Rust: sorteia o motivo e o boato e grava a data em +0x18 da linha da diretoria,
# que a 0x69aba0 lê nos prazos de 14 e 30 dias), MANAGER_MANAGER::post_manager_departure_news 0x6a1b00 (por tirar o
# técnico do cargo 0x69b790; a 257ª contou "cpq_1 o tecnico sai") e MANAGER_MANAGER::post_manager_appointment_news
# 0x6a2e00 (pelo mercado de técnicos 0x68f9c0 e pela nomeação 0x69ebc0). Iguais no cm0102-1080.exe; quadro SEH só pela
# notícia local (0x77bbe0, já em Rust; sem destrutor no caminho normal); sem x87. Comparadores em ensaio: a cópia roda
# sem entregar a notícia (as entregas 0x786780/0x7863f0/0x7862d0 copiam a notícia nas caixas dos humanos, alocam e dão a
# resposta automática) e guarda o alcance e a notícia montada; a original roda a partir do mesmo estado. Conferem a
# linha da diretoria (a 0x6a0e70), os sorteios da cópia como começo dos da original e, em cada caixa de humano que andou,
# a notícia entregue (tipo, dados, data e alcance) igual à da cópia; sem entrega planejada, nenhuma caixa anda.
SAVE = False
ROTINAS = [('noticia_da_pressao', 0x6a0e70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_da_saida_do_tecnico', 0x6a1b00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_tecnico_novo', 0x6a2e00, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as notícias do técnico: a diretoria pensa no cargo do técnico, a saída do técnico e o técnico novo',
        'the manager news: the board considering the manager position, the manager departure and the new manager',
        [('noticia_da_pressao', 'MANAGER_MANAGER::post_manager_under_pressure_news', '0x6a0e70', 'manager_manager.cpp', 'thiscall, ret 8',
          'a notícia 0x1777 com o técnico, o clube e o presidente; o motivo pelas avaliações da diretoria abaixo da confiança − 500 (sorteios de 3 e 5) e o boato (sorteio de 10, ou 1 se o técnico conhece o clube); entrega a todos, aos do país ou aos da divisão; a data de hoje em +0x18 da linha da diretoria',
          'the 0x1777 news with the manager, the club and the chairman; the reason by the board ratings below confidence − 500 (draws of 3 and 5) and the rumour (draw of 10, or 1 when the manager knows the club); delivered to all, to the nation or to the division managers; today in +0x18 of the board row'),
         ('noticia_da_saida_do_tecnico', 'MANAGER_MANAGER::post_manager_departure_news', '0x6a1b00', 'manager_manager.cpp', 'thiscall, ret 0x14',
          'a notícia 6000 com o técnico, o tipo, o clube, o cargo, o presidente e o motivo; técnico humano com os nomes e a todos; senão pelo clube (seleção ou reputação acima de 7250 a todos, senão aos do país)',
          'the 6000 news with the manager, the type, the club, the job, the chairman and the reason; a human manager with the names and to all; otherwise by the club (national team or reputation above 7250 to all, otherwise to the nation)'),
         ('noticia_do_tecnico_novo', 'MANAGER_MANAGER::post_manager_appointment_news', '0x6a2e00', 'manager_manager.cpp', 'thiscall, ret 0x10',
          'só clube de nação ativa com a divisão ativa, ou seleção de nação ativa: a notícia 0x1787 com o técnico, o clube anterior, o clube, o cargo e a função anterior; entrega como a da saída',
          'only a club of an active nation with an active division, or a national team of an active nation: the 0x1787 news with the manager, the previous club, the club, the job and the previous role; delivered like the departure one')])
CAMPOS = [('linha da diretoria (0x49 bytes por id em [MANAGER_MANAGER+0]) +6 confiança, +8/+0xa/+0xc avaliações (palavras com sinal), +0x18 data da última notícia de pressão; valores; lê, grava +0x18 (noticia_da_pressao)',
           'board row (0x49 bytes per id at [MANAGER_MANAGER+0]) +6 confidence, +8/+0xa/+0xc ratings (signed words), +0x18 date of the last pressure news; values; reads, writes +0x18 (noticia_da_pressao)'),
          ('clube +0 id, +0x53 nação, +0x57 divisão, +0x80 reputação, +0xbf presidente, +0xcf técnico; pessoa +0 id, +4/+8/+0xc nomes (+0x33 chave), +0x28 e +0x3d função, +0x69 não jogador (+0x1b); ponteiros/ids/valores; lê',
           'club +0 id, +0x53 nation, +0x57 division, +0x80 reputation, +0xbf chairman, +0xcf manager; staff +0 id, +4/+8/+0xc names (+0x33 key), +0x28 and +0x3d job, +0x69 non-player (+0x1b); pointers/ids/values; reads'),
          ('[0xdd6ec8] data a partir da qual saem as notícias de técnico; nação +0x11c ativa; [0xadadfc] divisões ativas; [0x9cf764] divisão sem notícia; datas/ponteiros/ids; lê',
           '[0xdd6ec8] date from which manager news are posted; nation +0x11c active; [0xadadfc] active divisions; [0x9cf764] division without news; dates/pointers/ids; reads')]
INTERESSA = ['CMRTE']
