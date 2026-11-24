# O discipline.dat do save: a carga e a gravacao da disciplina (Discipline::load 0x555860, Discipline::save 0x555dd0,
# discipline.cpp; leitor sx09c, rodada 5, 23/11/2026). As duas rotinas inteiras: 0x555860..0x555dbe (ret 4) e
# 0x555dd0..0x556207 (ret 4; o resto ate 0x556210 e enchimento).
# Chamadas: a carga so pelo construtor (0x55568a, com o save; o construtor testa eax inteiro); a gravacao pela gravacao
# do jogo (0x82ee1c; o retorno nao e usado: a original nao poe nada em eax).
# Convencoes conferidas na desmontagem:
# - 0x555860 thiscall(disciplina, save) ret 4 -> eax (0 ou 1); 0x9326e0 zopen cdecl(save, "discipline.dat", 0, 0x16,
#   4); 0x932c30 zread cdecl; 0x944e46 malloc; 0x55d420 Discipline::create_rule_sets thiscall(disciplina, &z) ret 4 (ja
#   Rust, discipline.rs; cada regra le as entradas e os pendentes pelo &z); 0x68d0d0 LLIST::add_tail thiscall(+0x1c,
#   dado, 0xa) ret 8; 0x932b00 zclose cdecl(&z); a caixa de erro como no show_error_box (transfer_manager.rs). Sem save
#   ou sem o item, a caixa sem zclose; os erros das tabelas, a caixa e o zclose; a quantidade de guardadas curta (0x139)
#   so a caixa e a quantidade 0 (o dword local inteiro); a guardada curta (0x143) so a caixa, e o laco segue;
# - 0x555dd0 thiscall(disciplina, save) ret 4, sem retorno; zopen com 1; 0x932e10 zwrite cdecl; 0x7e7b80
#   RulingBody::save thiscall(regra, &z) ret 4 (original, pelo endereco) para cada uma das 43 nao nulas; a quantidade de
#   guardadas e o i16 da lista copiado num dword (mov cx, os 16 bits altos de ecx ficam) e 2 bytes gravados; as
#   guardadas pelo get_next ate acabar (a quantidade nao limita).
# O discipline.dat: os registros por pessoa (29 bytes x [0xae2364]); cada regra existente (entradas e pendentes); i16 e
# as suspensoes (20 bytes); i16 e o indice por nacao (8 bytes); i16 e as guardadas (10 bytes). Ver docs/MAPA_DISCIPLINA.md.
# Prologos sem salto: 0x555860 mov eax,[esp+4]; sub esp,0x214 (10); 0x555dd0 mov eax,[esp+4]; sub esp,0x208 (10).
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('discipline_load', 0x555860, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('discipline_save', 0x555dd0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o discipline.dat do save: a carga e a gravacao da disciplina',
        'discipline.dat in the save: loading and saving the discipline',
        [('discipline_load', 'Discipline::load', '0x555860', 'discipline.cpp', 'thiscall, ret 4',
          'le o discipline.dat do save: sem save ou sem o item, a caixa "Error" do discipline.cpp (linhas 0xda, 0xe2) e 0; os registros de 29 bytes por pessoa, as regras (create_rule_sets com o item aberto: cada regra le as entradas e os pendentes), as suspensoes (i16 e 20 bytes cada) e o indice por nacao (i16 e 8 bytes), alocados so com quantidade (falhas: a caixa, o item fechado e 0; linhas 0xea, 0xf2, 0xfd, 0x10a, 0x112, 0x11b, 0x128, 0x130); a quantidade de suspensoes guardadas (curta: a caixa 0x139 e 0) e as guardadas de 10 bytes na lista (curta: a caixa 0x143 e segue); fecha o item e devolve 1',
          'reads discipline.dat from the save: without a save or the item, the discipline.cpp "Error" box (lines 0xda, 0xe2) and 0; the 29-byte records per person, the rule sets (create_rule_sets with the open item: each one reads its entries and pending bans), the bans (i16 and 20 bytes each) and the nation index (i16 and 8 bytes), allocated only with a count (failures: the box, the item closed and 0; lines 0xea, 0xf2, 0xfd, 0x10a, 0x112, 0x11b, 0x128, 0x130); the held-ban count (short: the box 0x139 and 0) and the 10-byte held bans into the list (short: the box 0x143 and it goes on); closes the item and returns 1'),
         ('discipline_save', 'Discipline::save', '0x555dd0', 'discipline.cpp', 'thiscall, ret 4',
          'grava o discipline.dat: os registros por pessoa, cada regra existente (RulingBody::save), as suspensoes, o indice por nacao, a quantidade de guardadas e as guardadas ate a lista acabar; fecha o item. Sem save ou sem o item, so a caixa (0x163, 0x16b); gravacao curta dos registros e tabelas, a caixa e o item fechado (0x172, 0x185, 0x18f, 0x198, 0x1a2); a quantidade de guardadas curta so a caixa (0x1ac); a guardada curta, a caixa e para (0x1b7). Nao devolve nada',
          'writes discipline.dat: the per-person records, each existing rule set (RulingBody::save), the bans, the nation index, the held-ban count and the held bans until the list runs out; closes the item. Without a save or the item, only the box (0x163, 0x16b); a short write of the records and tables, the box and the item closed (0x172, 0x185, 0x18f, 0x198, 0x1a2); a short held-ban count only the box (0x1ac); a short held ban, the box and it stops (0x1b7). Returns nothing')])
