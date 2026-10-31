# A projeção da liga para a diretoria (league.cpp): League::update_projected_positions (0x68ac40, a virtual +0x84 das
# 69 ligas, chamada só por Comp_Stats::get_club_projected_positions 0x4ae300, que a confiança semanal da diretoria
# 0x697c40 e o motivo da saída do técnico 0x69b350 usam) e as duas ordens do qsort dela, pela reputação (0x68afb0) e
# pelos pontos projetados (0x68afd0). Sem SEH; o bloco da projeção vem da 0x944cf1 e volta à 0x944c94 dentro da
# rotina; x87 em precisão dupla (constantes do .rdata: 0,83, 0,23, 1,5 e 0,444). Os três locais da original que viram
# argumentos da virtual +0x68 têm bytes altos de sobra de pilha fora da escala pela reputação: as 69 versões da +0x68
# só leem o byte baixo (ou repassam os dwords a uma que lê). Iguais no cm0102-1080.exe. league.cpp não entrou na
# cobertura de 17/10: o ramo diz se a escala pela reputação rodou. Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Comparadores: as duas ordens pelo valor; a projeção lado a lado nos bytes
# +0x73..+0x75 do registro do clube no Comp_Stats e na devolução.
SAVE = False
ROTINAS = [('projetar_posicoes_da_liga', 0x68ac40, [0x83, 0xec, 0x0c, 0x53, 0x55, 0x56]),
           ('ordem_da_projecao_pela_reputacao', 0x68afb0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08]),
           ('ordem_da_projecao_pelos_pontos', 0x68afd0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08])]
DOCS = ('a projeção da liga para a diretoria: as posições projetadas do clube e as duas ordens da projeção',
        "the league projection for the board: the club's projected positions and the two projection orders",
        [('projetar_posicoes_da_liga', 'League::update_projected_positions', '0x68ac40', 'league.cpp', 'thiscall, ret 4',
          'projeta os pontos de cada clube da tabela no fim da temporada (no primeiro terço, por uma escala pela reputação), ordena e entrega à virtual +0x68 as posições do clube na forma normal, melhor (× 1,5) e pior (× 0,444)',
          "projects each table club's end-of-season points (in the first third, by a reputation scale), sorts them and hands the club's positions in normal, better (× 1.5) and worse (× 0.444) form to virtual +0x68"),
         ('ordem_da_projecao_pela_reputacao', 'compare_projection_by_reputation_desc', '0x68afb0', 'league.cpp', 'cdecl',
          'comparador do qsort das entradas da projeção pela reputação do clube (+0x80), a maior primeiro',
          'qsort comparator of projection entries by club reputation (+0x80), highest first'),
         ('ordem_da_projecao_pelos_pontos', 'compare_projection_by_points_desc', '0x68afd0', 'league.cpp', 'cdecl',
          'comparador do qsort das entradas da projeção pelos pontos projetados (+0xa), o maior primeiro',
          'qsort comparator of projection entries by projected points (+0xa), highest first')])
CAMPOS = [('liga +0x3e: a conta das linhas da tabela; +0xb1 a tabela (0x3b bytes: +0 o clube, +0x11 os jogos, +0x19 os pontos); valor e ponteiro; lê (projetar_posicoes_da_liga)',
           'league +0x3e: the table row count; +0xb1 the table (0x3b bytes: +0 the club, +0x11 games played, +0x19 points); value and pointer; reads (projetar_posicoes_da_liga)'),
          ('liga +0xa7: os jogos da temporada; +0xc2 os pontos por jogo da escala (byte); +0x44 a competição; valor; lê (projetar_posicoes_da_liga)',
           'league +0xa7: season games; +0xc2 the scale points per game (byte); +0x44 the competition; value; reads (projetar_posicoes_da_liga)'),
          ('liga +8: o Comp_Stats; ponteiro; lê (projetar_posicoes_da_liga, comparador)',
           'league +8: the Comp_Stats; pointer; reads (projetar_posicoes_da_liga, comparator)'),
          ('clube +0x80: a reputação; valor; lê (ordem_da_projecao_pela_reputacao)',
           'club +0x80: the reputation; value; reads (ordem_da_projecao_pela_reputacao)'),
          ('registro do clube no Comp_Stats +0x73..+0x75: as posições projetadas (normal, melhor, pior); valor; grava (pela virtual +0x68 da projetar_posicoes_da_liga)',
           "club Comp_Stats record +0x73..+0x75: the projected positions (normal, better, worse); value; writes (through virtual +0x68 of projetar_posicoes_da_liga)")]
INTERESSA = ['CMRTE']
