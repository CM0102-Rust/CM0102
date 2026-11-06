# Os construtores dos estaduais: Carioca, Catarinense e Paulista (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp; agente C da frente das competições, 06/11/2026).
# Rodam uma vez cada, na criação da carreira de 8 ligas (docs/cobertura_competicoes.txt). O mesmo molde de 544 bytes
# nos doze estaduais (conferido instrução a instrução: só mudam a vtable, as fases além da liga, as rotinas das regras,
# da tabela e das posições previstas, o fonte e as linhas das caixas); os quatro itens cqt_1..4 cobrem os doze.
# - 0x439a40 BraRegRio::BraRegRio: vtable 0x968608, 3 fase(s) além da liga, regras 0x43a600, tabela 0x43a500, posições previstas 0x43add0.
# - 0x43b130 BraRegSanta::BraRegSanta: vtable 0x9686bc, 1 fase(s) além da liga, regras 0x43bc70, tabela 0x43bb70, posições previstas 0x43c110.
# - 0x43c280 BraRegSp::BraRegSp: vtable 0x968770, 1 fase(s) além da liga, regras 0x4322d0, tabela 0x43cc40, posições previstas 0x43d140.
# - 0x5d6280 (game.cpp, sem nome no mapa; pedido do Lucas): atualizar_media_de_publico, a média de público dos clubes
#   (+0x73, entre +0x77 e +0x7b) no fim da temporada de cada liga, do laço das temporadas 0x5d5ae0 (em 0x5d5cfb: push
#   liga; call; add esp, 4 -> cdecl). Chama 0x4a4b20 (Comp_Stats::get_sorted_team_table: push 1000; push &n; push 1;
#   mov ecx, [liga+8] -> thiscall, ret 0xc), as virtuais +0x98 e +0x9c (mov ecx, liga, sem push -> thiscall, ret 0),
#   0x90cfc0 (sorteio: push n; add esp, 4 -> cdecl), 0x945640 (_ftol: ftol do motor.rs) e 0x9452ca (free). x87
#   transcrito em f64 na ordem das contas. Comparador lado a lado sobre a tabela dos clubes e o estado do sorteador (a
#   cópia sem contar os sorteios).
# Iguais no cm0102-1080.exe (inteiros). Convenções conferidas nos pontos de chamada da original:
# - os construtores, da criação das competições: push registro; push ano; mov ecx, novo -> thiscall, ret 8 (a rotina
#   termina em ret 8 e lê o ano como word);
# - 0x682200 (League::League): mov ecx, esi; call -> thiscall, ret 0;
# - 0x687b10 (League::load_from_temp_file): push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x944e46 (malloc): push 4 ou 0xc; call; add esp, 4 -> cdecl;
# - as regras, a tabela e as posições previstas (call direto) e 0x6835c0 (create_fixtures): mov ecx, esi; call ->
#   thiscall, ret 0; 0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x944cf1 (operator new): push 0x5ce; call; add esp, 4 -> cdecl; 0x49ee70 (Comp_Stats::Comp_Stats): push esi; mov
#   ecx, novo -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca e sorteia os jogos), conferindo a devolução, a vtable, o ano, o registro e a
# disciplina 7; na liga nova, as fases, o vetor delas e a importância 0xf.
# Determinismo: só constantes e os argumentos.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqt4_construir_carioca', 0x439a40, [0x6a, 0xff, 0x68, 0x19, 0x4a, 0x95, 0x00]),
           ('cqt4_construir_catarinense', 0x43b130, [0x6a, 0xff, 0x68, 0xa9, 0x4a, 0x95, 0x00]),
           ('cqt4_construir_paulista', 0x43c280, [0x6a, 0xff, 0x68, 0x19, 0x4b, 0x95, 0x00]),
           ('cqt4_atualizar_media_de_publico', 0x5d6280, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('os construtores dos estaduais: Carioca, Catarinense e Paulista; e a média de público dos clubes no fim da temporada',
        'the state league constructors: Rio, Santa Catarina and Sao Paulo; and the clubs average attendance at the end of the season',
        [('cqt4_construir_carioca', 'BraRegRio::BraRegRio', '0x439a40', 'bra_reg_rio.cpp', 'thiscall, ret 8',
          'O Carioca na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, dois turnos e a final (três fases além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas',
          'The Rio state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions'),
         ('cqt4_construir_catarinense', 'BraRegSanta::BraRegSanta', '0x43b130', 'bra_reg_santa.cpp', 'thiscall, ret 8',
          'O Catarinense na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas',
          'The Santa Catarina state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions'),
         ('cqt4_construir_paulista', 'BraRegSp::BraRegSp', '0x43c280', 'bra_reg_sp.cpp', 'thiscall, ret 8',
          'O Paulista na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas',
          'The Sao Paulo state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions'),
         ('cqt4_atualizar_media_de_publico', 'atualizar_media_de_publico (sem nome no mapa)', '0x5d6280', 'game.cpp', 'cdecl',
          'A média de público dos clubes no fim da temporada de cada liga: anda com a posição final contra a prevista e com a reputação da competição, com um peso por nação (0,1, 0,25 ou 0,5 em algumas, 1 nas outras): o campeão e o vice sem acesso ganham (reputação + posições acima da prevista) × 200/100 ou 125/65 (dono do estádio ou não); quem sobe ganha pela reputação do clube e pela diferença de reputação para a liga de cima, com o piso da liga de cima; quem cai perde pela diferença para a de baixo, com o teto dela; os do play-off de acesso e os do meio da tabela ganham ou perdem por sorteio, com mistura com o público real da temporada (1/5 do público médio em casa + 4/5 da média) e os pisos e tetos pela reputação (2000, 3000 ou 5000 vezes); no fim, no mínimo 500 e entre o mínimo e o recorde do clube',
          'The clubs average attendance at the end of each league season: it moves with the final position against the predicted one and with the competition reputation, with a weight by nation (0.1, 0.25 or 0.5 in some, 1 in the others): the champion and runner-up without promotion gain (reputation + places above the prediction) × 200/100 or 125/65 (stadium owner or not); promoted clubs gain by club reputation and the reputation gap to the league above, with that league floor; relegated clubs lose by the gap to the league below, with its ceiling; promotion play-off and mid-table clubs gain or lose by a draw, mixed with the season real attendance (1/5 of the average home crowd + 4/5 of the average) and reputation floors and ceilings (2000, 3000 or 5000 times); at the end, at least 500 and between the club minimum and record')])
CAMPOS = [('liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 7', 'league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 7'),
          ('liga +0x51: [inferência] a importância; valor; grava 0xf', 'league +0x51: [inference] the importance; value; writes 0xf'),
          ('liga +0x40: o ano da temporada; valor; grava', 'league +0x40: the season year; value; writes'),
          ('liga +4: o registro da competição no banco; ponteiro; grava', 'league +4: the competition record in the database; pointer; writes'),
          ('liga +0x44: [inferência] marca da liga nova; valor; grava 0xff', 'league +0x44: [inference] new league flag; value; writes 0xff'),
          ('liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava', 'league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava', 'league +8: the statistics block (Comp_Stats); pointer; writes'),
          ('clube +0x73 / +0x77 / +0x7b: a média de público (Attendance) / o mínimo / o recorde; valor; lê e grava (média de público)', 'club +0x73 / +0x77 / +0x7b: the average attendance / the minimum / the record; value; reads and writes (attendance)'),
          ('clube +0x80: a reputação (word); +0x6d: dono do estádio; lê', 'club +0x80: the reputation (word); +0x6d: stadium owner; reads'),
          ('liga +0x1c / +0x20: a liga de cima / de baixo (id); +0xbe, +0xbf, +0xc1: vagas de acesso, play-off e rebaixamento; lê', 'league +0x1c / +0x20: the league above / below (id); +0xbe, +0xbf, +0xc1: promotion, play-off and relegation places; reads'),
          ('competição +0x69: a reputação (word); lê', 'competition +0x69: the reputation (word); reads')]
INTERESSA = ['CMRTE']
