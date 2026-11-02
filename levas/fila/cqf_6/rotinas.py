# A Copa do Mundo: a tabela das rodadas de cada fase, a nação classificada e as cabeças de chave (world_cup.cpp):
# - 0x92c290 (WorldCup::create_schedule, 2624 bytes até a tabela de saltos 0x92ccd0, método +0x3c): nas fases de
#   grupos (fase < 7) a tabela de 3 rodadas (12, 19 e 24 de junho do ano da Copa, levadas ao dia da semana) com os
#   dois jogos de cada rodada da fase; o mata-mata (fase 7, com as rotinas da copa 0x521e60/0x521eb0/0x521f10) fica com
#   a original (a cópia a chama pelo trampolim). Roda na carga do jogo (a fase -1, pela criação das regras 0x92d200) e
#   sete vezes no sorteio de 27/12/2001 (0x92d540) na carreira automática até 2002-06-01.
# - 0x92e740 (WorldCup::add_qualified_team, 173 bytes): a nação classificada nas eliminatórias entra na tabela das
#   nações (situação 6), ou a caixa com a lista cheia. Pelas seis eliminatórias (0x90f4f0, 0x911e10, 0x91538e,
#   0x9184c0, 0x91e200, 0x920500): passageira na carreira de 2001 [inferência: as eliminatórias de 2002 já vêm
#   decididas no banco de dados].
# - 0x92f230 (WorldCup::seed_teams, 1072 bytes): as cabeças de chave e os potes pelos continentes; na Copa de 2002 volta
#   logo no começo (é por isso que as vagas do sorteio de 2002 precisam vir do banco de dados: ver o laço da 0x92e940 na
#   cqf_4). Roda em 15/12/2001 só com essa volta: passageira.
# Iguais no cm0102-1080.exe (as três inteiras). Chamam originais não portadas: 0x944e46 (malloc), 0x5fa730
# (HostCountry::find_hosts), 0x92f660 (a comparação dos potes, passada à qsort) e a caixa; o resto já em Rust (0x68a160,
# 0x68a1c0, 0x53b3d0, 0x945333, 0x4b07b0, 0x4afd80).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x92c290: 0x92d616 (push 0; push &id; push &rodadas; push g; mov ecx, esi; call [vt+0x3c]) e 0x92d2eb
#   (push 0; push edi; push &+0xa9; push -1; mov ecx, esi; call [edx+0x3c]) -> thiscall, ret 0x10; devolve a tabela.
# - quem chama a 0x92e740: as eliminatórias (push clube; mov ecx, copa) -> thiscall, ret 4.
# - quem chama a 0x92f230: 0x92f1d6 (mov ecx, esi; sem push) -> thiscall, ret 0.
# - dentro da 0x92c290: 0x944e46 push 0xc3; add esp, 4 -> cdecl; 0x68a160 9 push (a tabela, i, dia, mês, 0, semana, 1,
#   o ano, 1); add esp (acumulado) -> cdecl: o ano vai no cx com lixo nos bytes altos (o ecx de antes), e a 0x68a160 só
#   usa os 16 bits de baixo de ano + base (a 0x549f70 lê word; +2 guarda ano − base em word): a cópia passa o ano limpo;
#   0x68a1c0 7 push (a tabela, rodada, jogo, a, b, c, valor); add esp -> cdecl; a caixa (aviso_em).
# - dentro da 0x92f230: 0x53b3d0 push nação; add esp -> cdecl; 0x945333 push comparação; push tamanho; push n; push
#   base; add esp, 0x10 -> cdecl; 0x5fa730 push 1; push &b; push &a; push ano; push id; mov ecx, [0xb5e134] -> thiscall,
#   ret 0x14 (o ano no ax com lixo nos bytes altos; ela lê word ptr [esp + 0x18]: a cópia passa o ano limpo); 0x4afd80
#   push 0; push competição; add esp, 8 -> cdecl.
# Comparadores: a 0x92c290 lado a lado nas fases de grupos (a original com as saídas de quem chama, a cópia com saídas
# próprias; as saídas e os bytes gravados das duas tabelas iguais, a da cópia liberada; o resto da memória alocada não
# é escrito por nenhuma das duas); a 0x92e740 lado a lado (+0x36 e a vaga), só a original com a lista cheia (a caixa);
# a 0x92f230 lado a lado na Copa de 2002 (nada muda), só a original nas outras (caixas).
# Determinismo: as duas saídas da 0x5fa730 começam em −1 (sem sede) na cópia; a original as deixa com o resto da pilha
# quando a Copa não está na lista das sedes (não acontece nas Copas da carreira: só a de 2002 roda, e volta antes).
SAVE = False
ROTINAS = [('mundial_rodadas_da_fase', 0x92c290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('mundial_nacao_classificada', 0x92e740, [0x66, 0x8b, 0x41, 0x36, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('mundial_cabecas_de_chave', 0x92f230, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00])]
DOCS = ('a Copa do Mundo: a tabela das rodadas de cada fase de grupos, a nação classificada e as cabeças de chave',
        'the World Cup: the round schedule of each group stage, the qualified nation and the seeding',
        [('mundial_rodadas_da_fase', 'WorldCup::create_schedule', '0x92c290', 'world_cup.cpp', 'thiscall, ret 0x10',
          'a tabela de três rodadas de uma fase de grupos (12, 19 e 24 de junho, levadas ao dia da semana) com os dois jogos de cada rodada; o mata-mata fica com a original',
          'the three-round schedule of a group stage (12, 19 and 24 June, moved to the weekday) with the two fixtures of each round; the knockout stays with the original'),
         ('mundial_nacao_classificada', 'WorldCup::add_qualified_team', '0x92e740', 'world_cup.cpp', 'thiscall, ret 4',
          'a nação classificada entra na tabela das nações da Copa (situação 6), ou a caixa com a lista cheia',
          'the qualified nation enters the World Cup nation table (status 6), or the box when the list is full'),
         ('mundial_cabecas_de_chave', 'WorldCup::seed_teams', '0x92f230', 'world_cup.cpp', 'thiscall, ret 0',
          'as cabeças de chave (detentora, sedes, reputação) e os potes pelo continente; na Copa de 2002 não faz nada',
          'the seeds (holder, hosts, reputation) and the pots by continent; does nothing for the 2002 World Cup')])
CAMPOS = [('copa do mundo +0x14 tabela das nações (6 bytes: +0 clube, +4 situação: 3 cabeça de chave, 6 classificada, 0xa/0xb/0xc potes), +0x36 quantas (word), +0x38 o máximo (word), +0x40 ano, +0x4b, +0xee as 8 vagas das cabeças de chave e +0x116 as 24 outras (5 bytes: +0 id do clube, +4 pote); valor, id e ponteiro; lê e grava (mundial_nacao_classificada, mundial_cabecas_de_chave), lê (mundial_rodadas_da_fase)',
           'world cup +0x14 nation table (6 bytes: +0 club, +4 status: 3 seed, 6 qualified, 0xa/0xb/0xc pots), +0x36 count (word), +0x38 maximum (word), +0x40 year, +0x4b, +0xee the 8 seed slots and +0x116 the other 24 (5 bytes: +0 club id, +4 pot); value, id and pointer; reads and writes (mundial_nacao_classificada, mundial_cabecas_de_chave), reads (mundial_rodadas_da_fase)'),
          ('tabela das rodadas (0x41 bytes por rodada): +0 dia do ano, +2 ano − base, +4 código, +0x3d valor; jogos de 7 bytes a partir de +5 (+5 valor, +9 a, +0xa b, +0xb c); valor; grava pela 0x68a160 e 0x68a1c0 (mundial_rodadas_da_fase)',
           'round schedule (0x41 bytes per round): +0 day of year, +2 year − base, +4 code, +0x3d value; 7-byte fixtures from +5 (+5 value, +9 a, +0xa b, +0xb c); value; writes through 0x68a160 and 0x68a1c0 (mundial_rodadas_da_fase)'),
          ('clube +0x53 nação; nação +0x71 continente; [0x9cfa08..0x9cfa1c] chaves dos continentes; [0x9cf79c] id da Copa do Mundo; [0xae23d0] competições (0x6b bytes); [0xb5e134] sedes; ponteiro e id; lê (mundial_cabecas_de_chave)',
           'club +0x53 nation; nation +0x71 continent; [0x9cfa08..0x9cfa1c] continent keys; [0x9cf79c] World Cup id; [0xae23d0] competitions (0x6b bytes); [0xb5e134] hosts; pointer and id; reads (mundial_cabecas_de_chave)')]
INTERESSA = ['CMRTE']
