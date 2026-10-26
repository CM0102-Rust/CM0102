# Três consultas do Database.cpp que o olheiro usa no relatório de um clube (PLAYER_SEARCH::run_scout_day 0x7b3e20,
# o único chamador; as respostas vão como parâmetros da notícia do relatório, 0x785790): faltam ao clube atacantes
# e zagueiros bons pelo alto e zagueiros rápidos? Leitura pura do elenco (+0xd7) com a lesão pela INJURY 0x62c400
# (chamada pelo endereço do jogo). Sem SEH, sem sorteio, sem caixa; a única local (o contador, o byte alto do ecx
# empurrado) é zerada na entrada.
SAVE = False
ROTINAS = [('faltam_atacantes_pelo_alto', 0x5494d0, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('faltam_zagueiros_pelo_alto', 0x5495b0, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('faltam_zagueiros_rapidos', 0x549690, [0x51, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('o olheiro: faltam ao clube atacantes e zagueiros bons pelo alto e zagueiros rápidos',
        'the scout: the club lacks aerial strikers, aerial defenders and fast defenders',
        [('faltam_atacantes_pelo_alto', 'club_lacks_aerial_strikers', '0x5494d0', 'Database.cpp', 'cdecl',
          '1 se no máximo 1 atacante (+0x15 = 20) sem lesão grave passa do limite de habilidade (reputação/75 + 25, de 40 a 135) com o atributo +0x2e acima de reputação/500 + 5 (no máximo 13)',
          '1 when at most 1 striker (+0x15 = 20) without serious injury is above the ability limit (reputation/75 + 25, 40 to 135) with attribute +0x2e above reputation/500 + 5 (at most 13)'),
         ('faltam_zagueiros_pelo_alto', 'club_lacks_aerial_defenders', '0x5495b0', 'Database.cpp', 'cdecl',
          'o mesmo com zagueiros (+0x11 = 20) e o atributo +0x2e (no máximo 13)',
          'the same with defenders (+0x11 = 20) and attribute +0x2e (at most 13)'),
         ('faltam_zagueiros_rapidos', 'club_lacks_fast_defenders', '0x549690', 'Database.cpp', 'cdecl',
          'o mesmo com zagueiros (+0x11 = 20) e o atributo +0x36 (no máximo 10)',
          'the same with defenders (+0x11 = 20) and attribute +0x36 (at most 10)')])
CAMPOS = [('TClub +0x80: reputação; curto com sinal; lê (as três)', 'TClub +0x80: reputation; signed short; reads (all three)'),
          ('TClub +0xd7 (50): elenco; ponteiros de pessoa; lê (as três)', 'TClub +0xd7 (50): squad; person pointers; reads (all three)'),
          ('TPlayer +0x11: zagueiro; +0x15: atacante; bytes (20 = natural); +0x2e, +0x36: atributos; bytes com sinal; lê', 'TPlayer +0x11: defender; +0x15: striker; bytes (20 = natural); +0x2e, +0x36: attributes; signed bytes; reads')]
INTERESSA = ['SGE', 'CMRTE']
