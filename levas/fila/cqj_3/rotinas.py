# As sedes das competições (host_country.cpp): a sede de uma edição, a próxima sede e o calendário das candidaturas.
# - 0x5fa730 (HostCountry::find_hosts, 160 bytes): 522 chamadas até 2002-06-01 (docs/cobertura_competicoes.txt; o
#   histórico das competições, 0x684640).
# - 0x5fa7d0 (HostCountry::find_next_hosts, 121 bytes): 663 chamadas (notícias, estádios 0x85dfc0).
# - 0x5fa850 (HostCountry::daily_update, 790 bytes, quadro SEH pela lista local, destruída no caminho normal pela
#   0x68d040 chamada no fim, como a original): 516 chamadas (todo dia, pelo laço do jogo).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x5fa730: 0x684640 e outras (push padrão; push &b; push &a; push ano; push id; mov ecx, [0xb5e134]) ->
#   thiscall(this, comp, ano, &a, &b, padrão), ret 0x14 (cqa_4 também conferiu). A devolução é um char (o al; os bytes
#   altos da original são o que sobra do eax: a segunda sede ou o ponteiro da entrada); a cópia devolve o al com zeros.
# - quem chama a 0x5fa7d0: 0x85e065, 0x85e088, 0x85e0ab (push &b; push &a; push id; mov ecx, [0xb5e134]), 0x744bee,
#   0x74b649, 0x777b02 -> thiscall(this, comp, &a, &b), ret 0xc; lê o al (mov bl, al; add bl, al).
# - quem chama a 0x5fa850: 0x5ccaed e 0x5cda06 (mov ecx, [0xb5e134]; push 0xae2c90) -> thiscall(this, &hoje), ret 4.
# - dentro da 0x5fa850: 0x68d020 lea ecx, lista -> thiscall, ret 0; 0x549f70 push −1; push ano; push 6; push 1; lea
#   ecx, data -> thiscall, ret 0x10 (o ano no ax com o resto do eax nos bytes altos: a 0x549f70 lê o word; a cópia
#   passa limpo); 0x5fad20 mov ecx, edi -> thiscall, ret 0; 0x946464 push 0x5fb3f0; push 0x22; push n; push tabela;
#   push &chave; add esp, 0x14 -> cdecl (bsearch); 0x68d0d0 push 4; push &entrada; lea ecx, lista -> thiscall, ret 8;
#   0x90cfc0 push 3; add esp, 4 -> cdecl; 0x945333 4 push, add esp, 0x10 -> cdecl; 0x5faec0 push &lista; mov ecx, edi
#   -> thiscall, ret 4; 0x68d040 lea ecx, lista -> thiscall, ret 0.
# Comparadores: 0x5fa730 e 0x5fa7d0 a devolução (o al) e as duas saídas (só leem; a cópia parte dos mesmos valores de
# entrada nas saídas, que ficam sem gravar quando não há entrada); 0x5fa850 só a original (sorteia, monta a lista e
# entrega a notícia pela 0x5faec0); a cópia leva os contadores.
# Determinismo: a chave da busca binária da original só tem o ano (+4) e o dia (+0x1f) gravados; o comparador do jogo
# (0x5fb3f0) só lê esses dois; a cópia zera o resto. A lista local começa zerada e o construtor (0x68d020) a preenche.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('sede_da_edicao', 0x5fa730, [0x8b, 0x01, 0x53, 0x55, 0x56]),
           ('proxima_sede', 0x5fa7d0, [0x8b, 0x01, 0x56, 0x66, 0x8b, 0x71, 0x04]),
           ('calendario_das_sedes', 0x5fa850, [0x6a, 0xff, 0x68, 0x78, 0xaf, 0x95, 0x00])]
DOCS = ('as sedes das competições: a sede de uma edição, a próxima sede e o calendário das candidaturas (votação aberta 8 anos antes, sede sorteada 5 anos antes)',
        'the competitions\' hosts: the host of an edition, the next host and the bidding calendar (bidding opens 8 years ahead, host drawn 5 years ahead)',
        [('sede_da_edicao', 'HostCountry::find_hosts', '0x5fa730', 'host_country.cpp', 'thiscall, ret 0x14',
          'a sede (e a segunda sede, nas edições com duas) da competição num ano: a candidatura vencedora, ou a sede padrão quando a sede é fixa ou a votação está aberta (ou fechada, se pedido)',
          'the host (and the second host, for co-hosted editions) of a competition in a year: the winning bid, or the default host when the host is fixed or the bidding is open (or closed, on request)'),
         ('proxima_sede', 'HostCountry::find_next_hosts', '0x5fa7d0', 'host_country.cpp', 'thiscall, ret 0xc',
          'as sedes da próxima edição da competição já decidida (a candidatura vencedora ou a sede fixa)',
          'the hosts of the competition\'s next already decided edition (the winning bid or the fixed host)'),
         ('calendario_das_sedes', 'HostCountry::daily_update', '0x5fa850', 'host_country.cpp', 'thiscall, ret 4',
          'a regra das sedes: cada edição (Copa do Mundo, Eurocopa e as outras da tabela) tem uma data de decisão no ano; 8 anos antes a votação abre (notícia com as candidaturas); 5 anos antes a sede é sorteada entre as 3 candidaturas (notícia); no 1º de janeiro as edições passadas pulam para a próxima (o intervalo da competição, ex. 4 anos) com a votação fechada; fora de 2001, no 1º de julho, as sedes das finais europeias',
          'the hosting rule: each edition (World Cup, European Championship and the others in the table) has a decision date in the year; 8 years ahead the bidding opens (news with the bids); 5 years ahead the host is drawn among the 3 bids (news); on 1 January past editions jump to the next one (the competition\'s interval, e.g. 4 years) with the bidding closed; outside 2001, on 1 July, the European finals hosts')])
CAMPOS = [('sedes +0 tabela das edições (registros de 0x22: +0 id da competição, +4 ano, +6/+0xa a sede padrão e a segunda, +6 + 8k as 3 candidaturas, +0x1e a escolhida 0..2, −3 fixa, −1 votação aberta, −2 fechada, +0x1f dia do ano da decisão, +0x21 intervalo em anos), +4 número de edições; id e valor; lê (sede_da_edicao, proxima_sede), lê e grava +4/+0x1e e reordena (calendario_das_sedes)',
           'hosts +0 edition table (0x22-byte records: +0 competition id, +4 year, +6/+0xa default host and second host, +6 + 8k the 3 bids, +0x1e the chosen one 0..2, −3 fixed, −1 bidding open, −2 closed, +0x1f decision day of the year, +0x21 interval in years), +4 number of editions; id and value; reads (sede_da_edicao, proxima_sede), reads and writes +4/+0x1e and re-sorts (calendario_das_sedes)'),
          ('[0xae2c90] a data de hoje (+0 dia do ano, +2 ano); valor; lê (calendario_das_sedes)',
           '[0xae2c90] today\'s date (+0 day of the year, +2 year); value; reads (calendario_das_sedes)')]
INTERESSA = ['CMRTE']
