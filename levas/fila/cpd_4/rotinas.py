# Seleções (national_teams.cpp), 4º item: montar as reservas de convocáveis (build_selection_pools 0x76abf0, 6 chamadas
# na carreira até 2002-06-01; 0x76ab52, 0x76b1e8; usa qsort) e as duas ordens dos qsort das seleções, fora do mapa de
# funções e da cobertura: pela nota (0x778ed0, só pela 0x76abf0, a cada trecho cheio) e pelo clube (0x778ef0, pelos
# qsort de 0x76c120 (1.007 chamadas), 0x7706c0 (800) e 0x7771c0 (684)). Simulação: as mutações pela carreira até
# 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio; na 0x76abf0 as locais são
# gravadas antes de ler (a nota é guardada em 32 bits com a metade alta do registro, só o curto é lido). Comparadores:
# a 0x76abf0 lado a lado pelos registros das nações e pelos trechos das duas reservas; as ordens pelo valor. Em laço
# quente: as duas ordens (dentro dos qsort; a 0x778ef0 nas três rotinas das seleções que rodam a cada dia de jogos).
SAVE = False
ROTINAS = [('montar_reservas_das_selecoes', 0x76abf0, [0x83, 0xec, 0x28, 0xa1, 0x50, 0x23, 0xae, 0x00]),
           ('ordem_das_reservas', 0x778ed0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08]),
           ('ordem_pelo_clube', 0x778ef0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('as seleções: as reservas de convocáveis de cada nação e as duas ordens delas',
        'national teams: the call-up pools of each nation and their two orderings',
        [('montar_reservas_das_selecoes', 'build_selection_pools', '0x76abf0', 'national_teams.cpp', 'thiscall',
          'zera os 4 níveis de cada registro de nação e põe cada pessoa apta na reserva da nação (e do sub-21, até 21 anos e menos de 5 jogos), também pela segunda nacionalidade; trecho cheio: entra pela nota, e ordena (qsort com 0x778ed0)',
          'clears the 4 levels of each nation record and puts every eligible person in the nation pool (and the under-21 one, up to 21 years and under 5 caps), also by the second nationality; full section: enters by rating, and sorts (qsort with 0x778ed0)'),
         ('ordem_das_reservas', 'compare_by_rating', '0x778ed0', 'national_teams.cpp', 'cdecl',
          'a ordem do qsort das reservas: nota (+9 da ficha) decrescente',
          'the pool qsort order: rating (+9 of the player data) descending'),
         ('ordem_pelo_clube', 'compare_by_club', '0x778ef0', 'national_teams.cpp', 'cdecl',
          'a ordem pelo clube (+0x39): sem clube depois, clubes pelo id, no mesmo clube o elenco 4 antes do 8 (+0x5e)',
          'the order by club (+0x39): no club last, clubs by id, within a club squad bit 4 before 8 (+0x5e)')])
CAMPOS = [('registro da nação ([this+4] + 0x6e × id): +8/+0x1d começo das reservas, +0xd/+0x22 quantos, +0xe/+0x23 vagas, +0xf/+0x24 menor nota (4 níveis de 4 bytes); [this+0xc]/[this+0x10] reservas; lê e grava (montar_reservas_das_selecoes)',
           'nation record ([this+4] + 0x6e × id): +8/+0x1d pool start, +0xd/+0x22 count, +0xe/+0x23 slots, +0xf/+0x24 lowest rating (4 levels of 4 bytes); [this+0xc]/[this+0x10] pools; reads and writes (montar_reservas_das_selecoes)'),
          ('pessoa (0x6e bytes) +0x18, +0x1a/+0x1e nacionalidades, +0x22 jogos, +0x24, +0x28, +0x39 clube, +0x3d, +0x5e, +0x61 ficha (+9 nota); lê',
           'person (0x6e bytes) +0x18, +0x1a/+0x1e nationalities, +0x22 caps, +0x24, +0x28, +0x39 club, +0x3d, +0x5e, +0x61 player data (+9 rating); reads')]
INTERESSA = ['SGE', 'CMRTE']
