# Os construtores das ligas nacionais de fora da Europa: a Série A, a Série B, a J1 e a NSL (bra_nat_first.cpp,
# bra_nat_second.cpp, jap_j1.cpp, aus_nsl.cpp; agente C da frente das competições, 06/11/2026). Rodam uma vez cada, na
# criação da carreira de 8 ligas (docs/cobertura_competicoes.txt). O mesmo molde de 544 bytes dos doze estaduais (cqt_1..4),
# conferido instrução a instrução: só mudam a vtable, a regra de disciplina, a importância (0xa nas Séries A e B e na
# NSL, 0xf na J1; na NSL gravada antes de carregar o estado salvo), as fases além da liga, as regras, a tabela, o fonte e
# as linhas das caixas; as posições previstas são as da League (0x68a850), não uma rotina própria.
# - 0x429850 BraNatFirst::BraNatFirst: vtable 0x967d98, disciplina 6, importância 0xa, 1 fase além da liga, regras 0x42a5a0, tabela 0x42a480.
# - 0x42be40 BraNatSecond::BraNatSecond: vtable 0x967e4c, disciplina 6, importância 0xa, 1 fase, regras 0x42cb70, tabela 0x42ca70.
# - 0x662b30 JapJ1::JapJ1: vtable 0x96ca40, disciplina 0x13, importância 0xf, 3 fases, regras 0x663620, tabela 0x6634e0.
# - 0x410e90 AusNsl::AusNsl: vtable 0x967768, disciplina 0x22, importância 0xa (antes da carga), 2 fases, regras 0x411ce0, tabela 0x411b70.
# Iguais no cm0102-1080.exe (inteiros). Convenções conferidas nos pontos de chamada da original:
# - os construtores, da criação das competições (0x832b3d, 0x832b8d, 0x835add, 0x832567): push registro; push ano; mov
#   ecx, novo -> thiscall, ret 8 (a rotina termina em ret 8 e lê o ano como word);
# - 0x682200 (League::League): mov ecx, esi; call -> thiscall, ret 0;
# - 0x687b10 (League::load_from_temp_file): push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x944e46 (malloc): push 4 × fases; call; add esp, 4 -> cdecl;
# - as regras e a tabela (call direto), 0x6835c0 (create_fixtures) e 0x68a850 (set_predicted_positions): mov ecx, esi;
#   call -> thiscall, ret 0; 0x6827d0 (sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x944cf1 (operator new): push 0x5ce; call; add esp, 4 -> cdecl; 0x49ee70 (Comp_Stats::Comp_Stats): push esi; mov
#   ecx, novo -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparador: só a original (aloca e sorteia os jogos), conferindo a devolução, a vtable, o ano, o registro, a
# disciplina e (na NSL) a importância; na liga nova, as fases, o vetor delas e a importância.
# Determinismo: só constantes e os argumentos.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_bra_nat_first', 0x429850, [0x6a, 0xff, 0x68, 0x29, 0x44, 0x95, 0x00]),
           ('construct_bra_nat_second', 0x42be40, [0x6a, 0xff, 0x68, 0x99, 0x44, 0x95, 0x00]),
           ('construct_jap_j1', 0x662b30, [0x6a, 0xff, 0x68, 0xf9, 0xc0, 0x95, 0x00]),
           ('construct_aus_nsl', 0x410e90, [0x6a, 0xff, 0x68, 0xd9, 0x39, 0x95, 0x00])]
_C = ('{liga} na criação da carreira: o ano e a competição do banco, a regra de disciplina {disc}{antes} e, sem estado salvo,{imp} {formato}, as regras ({regras}), a tabela ({tabela}), os jogos, a ordem, as estatísticas e as posições previstas pela força dos clubes',
      '{liga_en} at career creation: the year and the database competition, disciplinary rule {disc}{antes_en} and, with no saved state,{imp_en} {formato_en}, the rules ({regras_en}), the table ({tabela_en}), the fixtures, the order, the statistics and the predicted positions by club strength')


def _c(liga, liga_en, disc, imp, antes, formato, formato_en, regras, regras_en, tabela, tabela_en):
    a = ' e a importância %d' % imp if antes else ''
    a_en = ' and importance %d' % imp if antes else ''
    i = '' if antes else ' a importância %d,' % imp
    i_en = '' if antes else ' importance %d,' % imp
    return (_C[0].format(liga=liga, disc=disc, antes=a, imp=i, formato=formato, regras=regras, tabela=tabela),
            _C[1].format(liga_en=liga_en, disc=disc, antes_en=a_en, imp_en=i_en, formato_en=formato_en, regras_en=regras_en, tabela_en=tabela_en))


_A = _c('A Série A', 'The Brazilian first division', 6, 10, False, 'pontos corridos e o mata-mata (uma fase além da liga)', 'a round robin and the knockout (one stage besides the league)',
        'sem divisão de cima e com a Série B embaixo, oito rebaixados [inferência: mapa de maxevans60]', 'no league above and the second division below, eight relegated [inference: maxevans60 map]',
        'os 28 clubes da primeira divisão nacional, cada um recebendo 150.000 [inferência: mapa]', 'the 28 national first division clubs, each paid 150,000 [inference: map]')
_B = _c('A Série B', 'The Brazilian second division', 6, 10, False, 'pontos corridos e o mata-mata (uma fase além da liga)', 'a round robin and the knockout (one stage besides the league)',
        'a Série A em cima e a Série C embaixo, dois rebaixados [inferência: mapa]', 'the first division above and the third below, two relegated [inference: map]',
        'os clubes da segunda divisão nacional', 'the national second division clubs')
_J = _c('A J1', 'The J1 League', 0x13, 15, False, 'os dois turnos e a final entre os campeões dos turnos (três fases além da liga)', 'the two halves and the final between the half winners (three stages besides the league)',
        'pontos, desempates e regras dos jogos, com a J2 como divisão de rebaixamento [inferência: mapa]', 'points, tie-breaks and match rules, with J2 as the relegation division [inference: map]',
        'os 16 clubes da J1 e o prêmio [inferência: mapa]', 'the 16 J1 clubs and the prize fund [inference: map]')
_N = _c('A NSL australiana', 'The Australian NSL', 0x22, 10, True, 'a série final e a grande final (duas fases além da liga)', 'the finals series and the grand final (two stages besides the league)',
        'entre elas o número de substitutos [inferência: mapa]', 'among them the substitute counts [inference: map]',
        'os clubes da NSL', 'the NSL clubs')
DOCS = ('os construtores das ligas nacionais de fora da Europa: as Séries A e B, a J1 e a NSL',
        'the constructors of the national leagues outside Europe: the Brazilian first and second divisions, J1 and the NSL',
        [('construct_bra_nat_first', 'BraNatFirst::BraNatFirst', '0x429850', 'bra_nat_first.cpp', 'thiscall, ret 8', _A[0], _A[1]),
         ('construct_bra_nat_second', 'BraNatSecond::BraNatSecond', '0x42be40', 'bra_nat_second.cpp', 'thiscall, ret 8', _B[0], _B[1]),
         ('construct_jap_j1', 'JapJ1::JapJ1', '0x662b30', 'jap_j1.cpp', 'thiscall, ret 8', _J[0], _J[1]),
         ('construct_aus_nsl', 'AusNsl::AusNsl', '0x410e90', 'aus_nsl.cpp', 'thiscall, ret 8', _N[0], _N[1])])
CAMPOS = [('liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 6, 0x13 ou 0x22', 'league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 6, 0x13 or 0x22'),
          ('liga +0x51: [inferência] a importância; valor; grava 0xa ou 0xf', 'league +0x51: [inference] the importance; value; writes 0xa or 0xf'),
          ('liga +0x40: o ano da temporada; valor; grava', 'league +0x40: the season year; value; writes'),
          ('liga +4: o registro da competição no banco; ponteiro; grava', 'league +4: the competition record in the database; pointer; writes'),
          ('liga +0x44: [inferência] marca da liga nova; valor; grava 0xff', 'league +0x44: [inference] new league flag; value; writes 0xff'),
          ('liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava', 'league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava', 'league +8: the statistics block (Comp_Stats); pointer; writes')]
INTERESSA = ['CMRTE']
