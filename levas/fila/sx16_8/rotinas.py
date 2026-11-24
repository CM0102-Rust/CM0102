# notes.cpp (leitor sx16, rodada 5, 24/11/2026): o objeto das notas: construtor, destrutor e a carga do notes.dat.
# - 0x794ad0 Notes::Notes thiscall(notas, save) ret 4 -> notas: 32 listas (16 vagas x 2): 0x944cf1 operator new
#   cdecl(0xe) e 0x68d020 LLIST::LLIST thiscall(lista) (0 se o new falhar); com save e |[0xdd7110]| > 0x3e032,
#   0x794be0 (pelo desvio; a resposta nao e usada). Quadro SEH das alocacoes (nada lancado). Chamado pela criacao do jogo
#   (0x829298, sem save; G_NOTES, 0x1070 bytes) e pela carga (0x82c2ce, com o save).
# - 0x794ba0 Notes::~Notes thiscall(notas) ret: 0x68d040 LLIST::~LLIST e 0x944c94 operator delete em cada lista nao
#   nula (os ponteiros ficam). Chamado pela limpeza do jogo (0x830174).
# - 0x794be0 Notes::load_notes thiscall(notas, save) ret 4 -> al: sem save ou sem o item, 0 sem caixa; 0x9326e0 zopen
#   (save, "notes.dat", 0, 0x16, 4); por vaga: zread da contagem e das notas de 0x114 bytes, 0x68d0d0 add_tail
#   thiscall(lista, &rec, 0x114) ret 8; a segunda lista com 0x10c; caixas 0x73, 0x7d, 0x88, 0x91 + zclose e 0.
# Comparadores: so a original (aloca e le o save). Na carreira automatica as listas ficam vazias (so a tela poe notas):
# o notes.dat do save de ferias tem 16 x (0, 0).
SAVE = True
CARGA = True
ROTINAS = [('new_notes', 0x794ad0, [0x6a, 0xff, 0x68, 0x8b, 0xe9, 0x95, 0x00]),
           ('drop_notes', 0x794ba0, [0x53, 0x55, 0x56, 0x57, 0xbd, 0x10, 0x00, 0x00, 0x00]),
           ('notes_load_notes', 0x794be0, [0x6a, 0xff, 0x68, 0xc1, 0xe9, 0x95, 0x00])]
DOCS = ('o objeto das notas e a carga do notes.dat',
        'the notes manager and the loading of notes.dat',
        [('new_notes', 'Notes::Notes', '0x794ad0', 'notes.cpp', 'thiscall, ret 4',
          'monta as duas listas vazias de cada uma das 16 vagas de técnico humano; com um save de versão acima de 0x3e032, lê as notas dele',
          "builds the two empty lists of each of the 16 human manager slots; with a save of a version above 0x3e032, reads its notes"),
         ('drop_notes', 'Notes::~Notes', '0x794ba0', 'notes.cpp', 'thiscall',
          'esvazia e libera as 32 listas de notas',
          'empties and frees the 32 note lists'),
         ('notes_load_notes', 'Notes::load_notes', '0x794be0', 'notes.cpp', 'thiscall, ret 4',
          'lê o notes.dat do save: para cada vaga, a contagem e as notas (0x114 bytes) da primeira lista e a contagem e os registros (0x10c bytes) da segunda; sem save ou sem o item, 0; leitura curta, a caixa de erro, o item fechado e 0 (o que já foi lido fica)',
          "reads notes.dat from the save: for each slot, the count and the notes (0x114 bytes) of the first list and the count and the records (0x10c bytes) of the second; without a save or the item, 0; a short read, the error box, the item closed and 0 (what was read stays)")])
