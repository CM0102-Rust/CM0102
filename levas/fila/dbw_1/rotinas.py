# set_key_ids (0x601a40, index.cpp), na carga da base (0x528540; uma vez na criação do .qst e outra ao carregar o save):
# por isso SAVE e CARGA. Decisão do Lucas (opção A): duas entradas, cada uma com o seu -1 exato. 0x601a40 = a do
# cm0102.exe (e dos exes de terceiros cujo código bata): o -1 nas 1.282 globais da original. 0x603834 = a do
# cm0102-1080.exe (e do original com CM_RESOLUCAO=1080, cujas trocas vêm antes dos desvios): sub esp,0x220 / push ebx,
# ebp, esi, edi, ecx / rep stosd nas faixas inteiras 0x9cf178..0x9d0527 e 0xb63c9c..0xb63d03 (também 0x9d0260, 0x9d0300,
# 0x9d034c e 0x9d0474) e emenda em 0x603859; a chamada de 0x52d8fa vai para ela. Cada uma vale só no seu exe: o prólogo
# de 0x603834 só existe no 1080 (a conferência de bytes recusa no original) e o assinaturas.py dá à 0x601a40 só o hash do
# original (o 1080 não a chama mais). Listas geradas do exe (engenharia-reversa/chaves_fixas.py → rust/src/chaves_fixas.rs).
# Os casadores (0x6146b0, 0x60d810, 0x60efd0, 0x6115e0, já em Rust na 227ª) e as rotinas da CRT e da caixa vão pelo
# endereço do jogo. Sem SEH, sem sorteio. As locais da original (as 8 contagens e os rascunhos de 0x100 da caixa) não
# são lidas sem valor (a não ser com o fread curto do eng.lng). Comparando, a cópia roda sem o relato (a caixa não
# aparece duas vezes).
SAVE = True
CARGA = True
ROTINAS = [('definir_chaves_fixas', 0x601a40, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('definir_chaves_fixas_1080', 0x603834, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
EXE = {0x601a40: 'original', 0x603834: '1080'}
DOCS = ('as chaves fixas do jogo: os ids pela base, as cores e os continentes, e o relato das que faltam',
        "the game's fixed keys: ids from the database, colours and continents, and the report of the missing ones",
        [('definir_chaves_fixas', 'set_key_ids', '0x601a40', 'index.cpp', 'cdecl',
          'as chaves fixas (cores, continentes, nações, competições, prêmios, clubes) em -1 e depois a linha da base pelo nome (pelo eng.lng ou pela base); as que ficam sem valor vão à caixa "Unable to find the %s index" (7 fatais pela nação); no cm0102-1080.exe pela entrada 0x603834, com o -1 nas faixas inteiras',
          'the fixed keys (colours, continents, nations, competitions, awards, clubs) set to -1 and then to the database row by name (from eng.lng or from the database); the ones left without a value go to the "Unable to find the %s index" box (7 fatal by nation); in cm0102-1080.exe through the 0x603834 entry, with -1 over the whole ranges')])
CAMPOS = [('[0x9cf178..0x9d0527], [0xb63c9c..0xb63d03]: as chaves fixas (-1 e a linha da base); grava (definir_chaves_fixas)',
           '[0x9cf178..0x9d0527], [0xb63c9c..0xb63d03]: the fixed keys (-1 and the database row); writes (definir_chaves_fixas)'),
          ('[0x9b5984] idioma; CM3_TEMP\\eng.lng (8 contagens e as tabelas); [0xae23fc] lista de clubes; lê (definir_chaves_fixas)',
           '[0x9b5984] language; CM3_TEMP\\eng.lng (8 counts and the tables); [0xae23fc] club list; reads (definir_chaves_fixas)')]
INTERESSA = ['SGE', 'CMRTE', 'CMScout']
