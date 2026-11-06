# O fim da temporada das ligas e duas notícias das competições (league.cpp, comp.cpp; família do Agente A: a base).
# - League::process_promotion_relegation_chain (0x689c20): 1 chamada na carreira de 8 ligas até 2002-06-01 (na de
#   uma liga só, a da Inglaterra, ela só roda depois de junho: por isso saiu da cqh_5). É um slot da vtable das ligas
#   (o endereço está em ~100 vtables de 0x967320 a 0x9713a0; nenhuma chamada direta).
# - Comp::send_prize_money_news (0x48e3e0, ~1.200 chamadas: os prêmios das ligas e copas) e Comp::send_tv_rights_news
#   (0x48ea40, ~40: os direitos de TV), passageiras (a notícia só chega ao técnico humano).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258ª):
# - 0x689c20: o slot da vtable; a rotina usa o ecx (mov ebx, ecx) e sai com ret 4 -> thiscall(this, aos_pares).
# - 0x48e3e0: pelos pagadores de prêmio (ex. 0x522240, 0x682e10, Rust): 7 push (tipo, partida, rodada, posição, valor
#   alto, valor baixo, clube); mov ecx, competição -> thiscall, ret 0x1c (o valor é um i64).
# - 0x48ea40: pela 0x683010 (0x683093): push valor; push clube; mov ecx, competição -> thiscall, ret 8.
# - dentro: o slot 0xa4 da liga (mov ecx, liga; call [vtable+0xa4], sem push -> thiscall, ret 0); 0x689c80 6 push (−1,
#   −1, aos_pares, 1, abaixo, acima); mov ecx, this -> thiscall, ret 0x18 (Rust); 0x7875e0 push clube (o ecx =
#   [0xb7431c] não conta) -> stdcall de 1 (Rust); 0x77bbe0 push 0; push tipo; lea ecx, notícia -> thiscall, ret 8
#   (Rust); 0x785790 push valor; push índice; lea ecx -> thiscall, ret 8 (Rust); 0x7861e0 push clube; push &notícia;
#   mov ecx, [0xb7431c] -> thiscall, ret 8 (Rust).
# Comparadores: só a original nas três (a cadeia move clubes entre ligas, com contratos, técnicos e notícias; as
# notícias são entregues, o que aloca); as cópias levam os ramos.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('cadeia_de_acessos', 0x689c20, [0x53, 0x8b, 0xd9, 0x57, 0x8b, 0x43, 0x1c]),
           ('noticia_do_premio', 0x48e3e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_da_tv', 0x48ea40, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o fim da temporada das ligas: a cadeia de acessos e rebaixamentos; as notícias do prêmio e da TV de uma competição',
        'the league season end: the promotion and relegation chain; a competition\'s prize money and TV rights news',
        [('cadeia_de_acessos', 'League::process_promotion_relegation_chain', '0x689c20', 'league.cpp', 'thiscall, ret 4',
          'no fim da temporada, só a liga do topo da pirâmide (sem liga acima) fecha a classificação e desce a cadeia das divisões: em cada par (a de cima e a de baixo), fecha a de baixo e troca os que sobem e os que descem (0x689c80, com as cláusulas de contrato, aos pares ou em bloco, sem conferir quantos)',
          'at the season end, only the top league of the pyramid (no league above) closes its table and walks down the chain of divisions: for each pair (upper and lower), closes the lower one and swaps the promoted and relegated clubs (0x689c80, with contract clauses, pairwise or as a block, without checking counts)'),
         ('noticia_do_premio', 'Comp::send_prize_money_news', '0x48e3e0', 'comp.cpp', 'thiscall, ret 0x1c',
          'a notícia do prêmio em dinheiro ao técnico humano do clube, só quando o prêmio passa de 50 × a reputação do clube: a competição, o valor, a posição, a rodada, o tipo e, com a partida, o adversário e se o clube venceu',
          'the prize money news to the club\'s human manager, only when the prize exceeds 50 × the club reputation: the competition, the amount, the position, the round, the kind and, with the fixture, the opponent and whether the club won'),
         ('noticia_da_tv', 'Comp::send_tv_rights_news', '0x48ea40', 'comp.cpp', 'thiscall, ret 8',
          'a notícia dos direitos de TV ao técnico humano do clube: o clube, a competição, o valor e o ano',
          'the TV rights news to the club\'s human manager: the club, the competition, the amount and the year')])
CAMPOS = [('liga +0x1c a liga de cima (índice; negativo no topo); +0x20 a liga de baixo (índice em [0xadadfc]; negativo na última); lê (cadeia_de_acessos)', 'league +0x1c the league above (index; negative at the top); +0x20 the league below (index in [0xadadfc]; negative at the bottom); reads (cadeia_de_acessos)'),
          ('competição +4 o registro (+0 id); +0x40 o ano; clube +0x80 reputação; partida +0x1c/+0x20 os clubes, +0x3d/+0x3e [inferência: vencedor nos pênaltis], +0x43/+0x44 e +0x4b/+0x4c os placares; lê (notícias)', 'competition +4 the record (+0 id); +0x40 year; club +0x80 reputation; fixture +0x1c/+0x20 the clubs, +0x3d/+0x3e [inference: penalty winner], +0x43/+0x44 and +0x4b/+0x4c the scores; reads (news)')]
INTERESSA = ['CMRTE']
