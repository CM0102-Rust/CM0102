# As copas de clubes da Oceania e da Ásia: o calendário do Campeonato de Clubes da Oceania e os times da Liga dos
# Campeões e da Recopa asiáticas (oceania_club_champ.cpp, asia_club_champ.cpp, asia_cup_winner.cpp, em eurocomp\).
# Frente das competições (Agente D, 06/11/2026), carreira de 8 ligas (Austrália e Japão jogáveis; começa em janeiro de
# 2001).
# - 0x797230 (OceaniaClubChamp::create_schedule, 1146 bytes; 3 chamadas até 2002-06-01 com as 8 ligas: os dois grupos
#   e o mata-mata de janeiro de 2001). Uma mutação.
# - 0x40b620 (AsiaClubChamp::build_team_table, 221 bytes; 1 chamada): os 16 times. Uma mutação.
# - 0x40d4d0 (AsiaCupWinner::create_team_entries, 221 bytes; 1 chamada): os 16 times. Uma mutação.
# A tabela da Oceania é montada como as dos torneios de seleções (cqq12_tabela: a lista comum e a de cada grupo,
# extraídas da desmontagem, executadas na ordem); as duas asiáticas têm o mesmo corpo (cqq12_times).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): 0x797230 é a virtual +0x3c (push saida; push &extra; push &rodadas; push fase;
# mov ecx, copa -> thiscall, ret 0x10); 0x40b620 e 0x40d4d0 são thiscall, ret 0 (sem argumentos). Dentro: 0x944e46
# (push tamanho; add esp, 4 -> cdecl), 0x68a160 (9 pushes -> cdecl; o ano num registrador com lixo nos 16 bits altos,
# a 0x68a160 usa só a palavra: a cópia passa limpo), 0x68a1c0 (7 pushes -> cdecl), 0x521e60/0x521eb0 (7 e 9 pushes ->
# cdecl) e a caixa (aviso_em).
# Comparadores: lado a lado. A tabela: as saídas e os bytes gravados pelas operações nas duas tabelas (a da cópia
# liberada); fora das fases só a original. Os times: o +0x3e e o [0xb67a34]; a tabela da cópia comparada nas 16 vagas e
# liberada.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (05-06/11/2026, cópia fora do projeto: motor.rs da principal com a 277a publicada e a 278a integrada + os 12
# itens cqo_10..13, cqp_10..13, cqq_10..13, 39 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 (parou em 2001-273): 0 linhas DIVERGENCIA fora
#   da cqo11_copa_c_inscritos (8 dezenas embaralhadas, não 9: corrigida e conferida de novo até 2001-01-05, 0
#   divergências), 0 EXCECAO, nenhuma caixa;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a; os contadores das mutações rodaram;
# - jogo comparando estendido até 2002-06-01 (o mata-mata da Oceania de 2002 roda em janeiro de 2002): 0 DIVERGENCIA
#   nas três rotinas;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base f29c85195c2a534f): as três ACUSADAS (os grupos da
#   Oceania de 2002 em 2002-001, os times da Liga dos Campeões e da Recopa asiáticas de trás para a frente em 2001-233).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqq12_oceania_tabela', 0x797230, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqq12_asia_clubes_times', 0x40b620, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqq12_asia_recopa_times', 0x40d4d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as copas de clubes da Oceania e da Ásia: o calendário do Campeonato de Clubes da Oceania e os times da Liga dos Campeões e da Recopa asiáticas',
        'the Oceanian and Asian club cups: the Oceania Club Championship calendar and the Asian Club Championship and Cup Winners\' Cup teams',
        [('cqq12_oceania_tabela', 'OceaniaClubChamp::create_schedule', '0x797230', 'oceania_club_champ.cpp', 'thiscall, ret 0x10',
          'regra: o Campeonato de Clubes da Oceania tem dois grupos (fases −1 e 0, ids 0x41f e 0x420) de três rodadas em 10, 12 e 14 de janeiro do ano seguinte ao da copa, com os jogos fixos de cada grupo; o mata-mata (fase 1) em jogo único: semifinal em 17/1, terceiro lugar e final em 19/1 (mando 4 na final); categoria 8; desempate 0x283 na semifinal e 0x83 depois; sem prêmios',
          'rule: the Oceania Club Championship has two groups (stages −1 and 0, ids 0x41f and 0x420) of three rounds on 10, 12 and 14 January of the year after the cup year, with each group\'s fixed matches; the knockout (stage 1) in single matches: semi-final on 17/1, third place and final on 19/1 (venue 4 in the final); category 8; tie-break 0x283 in the semi-final and 0x83 after; no prizes'),
         ('cqq12_asia_clubes_times', 'AsiaClubChamp::build_team_table', '0x40b620', 'asia_club_champ.cpp', 'thiscall, ret 0',
          'regra: a Liga dos Campeões asiática tem 16 times, os classificados (+0xb2) na ordem em que foram escolhidos, todos com classe e situação 0',
          'rule: the Asian Club Championship has 16 teams, the qualified clubs (+0xb2) in the order they were picked, all with class and status 0'),
         ('cqq12_asia_recopa_times', 'AsiaCupWinner::create_team_entries', '0x40d4d0', 'asia_cup_winner.cpp', 'thiscall, ret 0',
          'regra: a Recopa asiática tem 16 times, os classificados (+0xb2) na ordem em que foram escolhidos, todos com classe e situação 0',
          'rule: the Asian Cup Winners\' Cup has 16 teams, the qualified clubs (+0xb2) in the order they were picked, all with class and status 0')])
CAMPOS = [('copa +0x40 ano; valor (palavra com sinal); lê (Oceania)', 'cup +0x40 year; value (signed word); reads (Oceania)'),
          ('rodada de grupo (0x41 bytes) e rodada de copa (0x68 bytes); valor; grava (Oceania)',
           'group round (0x41 bytes) and cup round (0x68 bytes); value; writes (Oceania)'),
          ('copa +0xb2 classificados (ponteiros de clube); ponteiro; lê (Ásia)', 'cup +0xb2 qualified clubs (club pointers); pointer; reads (Asia)'),
          ('copa +0x3e times, +0xa7 tabela (6 bytes: clube, +4 classe, +5 situação); valor/ponteiro; grava (Ásia)',
           'cup +0x3e teams, +0xa7 table (6 bytes: club, +4 class, +5 status); value/pointer; writes (Asia)')]
INTERESSA = ['CMRTE']
