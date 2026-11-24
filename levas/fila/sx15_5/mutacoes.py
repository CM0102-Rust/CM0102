# Mutacoes (sx15_5).
# - o aviso sem o sorteio da redacao do corpo (slot 3 = 0): o gerador anda um passo a menos a cada final com
#   classificados, e a carreira ENG ate 2002-06-01 diverge depois do primeiro aviso (classificados da Copa de 2002, out/nov
#   de 2001);
# - o vinculo do nome longo da competicao como clube (0x7d5): acusada pela comparacao do texto (compare_text_writer),
#   como as de texto aceitas em 12/11/2026.
MUTACOES = [
    ('sx15_5_sem_sorteio',
     '        exe::newsitem_set_slot(nt, 3, exe::utils_random(2) as u32); // sx15_5: the body wording',
     '        exe::newsitem_set_slot(nt, 3, 0); // mutation: no draw for the body wording',
     'sx15_5 qualified nations news'),
    ('sx15_5_vinculo_competicao',
     '    exe::text_add_link(out, q.comp.name_addr() as *const u8, 2000, q.comp); // sx15_5: the competition links',
     '    exe::text_add_link(out, q.comp.name_addr() as *const u8, 0x7d5, q.comp); // mutation: linked as a club',
     'sx15_5 qualified nations headline'),
]
