# Os construtores dos estaduais: Goiano, Mineiro e Norte (bra_reg_goias.cpp, bra_reg_minas_gerais.cpp, bra_reg_north.cpp; agente C da frente das competições, 06/11/2026).
# Rodam uma vez cada, na criação da carreira de 8 ligas (docs/cobertura_competicoes.txt). O mesmo molde de 544 bytes
# nos doze estaduais (conferido instrução a instrução: só mudam a vtable, as fases além da liga, as rotinas das regras,
# da tabela e das posições previstas, o fonte e as linhas das caixas); os quatro itens cqt_1..4 cobrem os doze.
# - 0x432b50 BraRegGoias::BraRegGoias: vtable 0x9681d0, 1 fase(s) além da liga, regras 0x433640, tabela 0x433540, posições previstas 0x433b40.
# - 0x433d50 BraRegMinasGerais::BraRegMinasGerais: vtable 0x968284, 3 fase(s) além da liga, regras 0x43a600, tabela 0x434810, posições previstas 0x434f80.
# - 0x435340 BraRegNorth::BraRegNorth: vtable 0x968338, 1 fase(s) além da liga, regras 0x439430, tabela 0x435d80, posições previstas 0x436280.
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
ROTINAS = [('cqt2_construir_goiano', 0x432b50, [0x6a, 0xff, 0x68, 0x39, 0x47, 0x95, 0x00]),
           ('cqt2_construir_mineiro', 0x433d50, [0x6a, 0xff, 0x68, 0xa9, 0x47, 0x95, 0x00]),
           ('cqt2_construir_norte', 0x435340, [0x6a, 0xff, 0x68, 0x39, 0x48, 0x95, 0x00])]
DOCS = ('os construtores dos estaduais: Goiano, Mineiro e Norte',
        'the state league constructors: Goias, Minas Gerais and North',
        [('cqt2_construir_goiano', 'BraRegGoias::BraRegGoias', '0x432b50', 'bra_reg_goias.cpp', 'thiscall, ret 8',
          'O Goiano na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas',
          'The Goias state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions'),
         ('cqt2_construir_mineiro', 'BraRegMinasGerais::BraRegMinasGerais', '0x433d50', 'bra_reg_minas_gerais.cpp', 'thiscall, ret 8',
          'O Mineiro na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, dois turnos e a final (três fases além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas',
          'The Minas Gerais state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions'),
         ('cqt2_construir_norte', 'BraRegNorth::BraRegNorth', '0x435340', 'bra_reg_north.cpp', 'thiscall, ret 8',
          'O Norte na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas',
          'The North state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions')])
CAMPOS = [('liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 7', 'league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 7'),
          ('liga +0x51: [inferência] a importância; valor; grava 0xf', 'league +0x51: [inference] the importance; value; writes 0xf'),
          ('liga +0x40: o ano da temporada; valor; grava', 'league +0x40: the season year; value; writes'),
          ('liga +4: o registro da competição no banco; ponteiro; grava', 'league +4: the competition record in the database; pointer; writes'),
          ('liga +0x44: [inferência] marca da liga nova; valor; grava 0xff', 'league +0x44: [inference] new league flag; value; writes 0xff'),
          ('liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava', 'league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava', 'league +8: the statistics block (Comp_Stats); pointer; writes')]
INTERESSA = ['CMRTE']
