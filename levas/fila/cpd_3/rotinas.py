# Seleções (national_teams.cpp), 3º item: limpar a competição atual da nação (clear_current_competition 0x775000, 9
# chamadas na carreira até 2002-06-01; 0x57b2b5, 0x90fd1a...), liberar as competições do continente
# (release_competitions_of_continent 0x775070, 4; 0x57cad0, 0x9112e9...) e convocar o substituto (call_up_replacement
# 0x775f90, 140; 0x775f4e, 0x779835, 0x784711). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no
# jogo real comparando. Sem SEH, sem caixa no caminho comum, sem local lida sem valor (a 0x775f90 passa à 0x7762f0 um
# dword com só o byte baixo gravado; a 0x7762f0 só lê o byte, a cópia passa zero em cima). Comparadores: a 0x775000
# lado a lado quando não há competição seguinte (o caminho sem aviso) e só a original com ela (o aviso 0x6a3770 sorteia
# e mexe nos técnicos); a 0x775070 só a original (mesmo aviso); a 0x775f90 só a original (a notícia 0x76fd10, com SEH,
# e a 0x600a50, que alcança a 0x68d0d0); as 0x776250/0x7762f0 que ela chama passam pelos comparadores delas. Nenhuma em
# laço quente.
SAVE = False
ROTINAS = [('limpar_competicao_atual', 0x775000, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('liberar_competicoes_do_continente', 0x775070, [0x51, 0xa1, 0x50, 0x23, 0xae, 0x00]),
           ('convocar_substituto', 0x775f90, [0x83, 0xec, 0x18, 0x8b, 0x44, 0x24, 0x28])]
DOCS = ('as seleções: a competição atual de cada nação e a convocação do substituto',
        'national teams: the current competition of each nation and calling up a replacement',
        [('limpar_competicao_atual', 'clear_current_competition', '0x775000', 'national_teams.cpp', 'thiscall, ret 4',
          'a competição atual da nação (registro +4) vira −1 e, havendo a seguinte ([this+8][id]), passa a ser ela, com o aviso aos técnicos (0x6a3770)',
          'the nation current competition (record +4) becomes −1 and, if there is a next one ([this+8][id]), that one takes its place, notifying the managers (0x6a3770)'),
         ('liberar_competicoes_do_continente', 'release_competitions_of_continent', '0x775070', 'national_teams.cpp', 'thiscall, ret 8',
          'nas nações do continente (+0x71), a atual e a seguinte que acabaram (+0x59 da competição nulo, ou a fase [0x9cfa10]) viram −1; atual vazia e seguinte: a seguinte sobe, com o aviso (0x6a3770)',
          'in the nations of the continent (+0x71), the current and next competitions that ended (competition +0x59 null, or stage [0x9cfa10]) become −1; empty current and a next one: the next moves up, with the notice (0x6a3770)'),
         ('convocar_substituto', 'call_up_replacement', '0x775f90', 'national_teams.cpp', 'thiscall, ret 0x10',
          'na reserva do nível, o primeiro apto da nação que joga numa posição de quem sai (0x5b7d20) é convocado (0x776250, saindo antes de outro elenco pela 0x7762f0) com a notícia (0x76fd10); ninguém: zera a vaga do tipo (+0x45)',
          'in the pool of the level, the first eligible player of the nation who plays a position of the one leaving (0x5b7d20) is called up (0x776250, leaving another squad first through 0x7762f0) with the news (0x76fd10); nobody: clears the type slot count (+0x45)')])
CAMPOS = [('registro da nação no gerenciador das seleções +4 competição atual, +0x3a datas e +0x45 vagas por tipo (20 bytes); [this+8] competição seguinte por nação; lê e grava',
           'nation record in the national team manager +4 current competition, +0x3a dates and +0x45 slots per type (20 bytes); [this+8] next competition per nation; reads and writes'),
          ('TNation (0x122 bytes) +0x71 continente; competição ([0xae23d0], 107 bytes) +0x59; lê (liberar_competicoes_do_continente)',
           'TNation (0x122 bytes) +0x71 continent; competition ([0xae23d0], 107 bytes) +0x59; reads (liberar_competicoes_do_continente)')]
INTERESSA = ['SGE', 'CMRTE']
