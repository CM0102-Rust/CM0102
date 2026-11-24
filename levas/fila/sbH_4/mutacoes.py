# Mutacoes (sbH_4) pela prova do save (prova_save.py --leva N: o .qst gravado pela original e pela Rust, item a item, e
# o save de ferias carregado com e sem as rotinas), no que todo save tem:
# - a capacidade do plano gravada com a contagem: o Friendly.tmp muda (e a carga refaz o plano menor);
# - a data base da pre-temporada sem gravar: o arquivo encurta e a carga le tudo deslocado;
# - a carga sem as listas por nacao (nenhum item lido): as listas de amistosos internacionais ficam vazias e a impressao
#   da carga muda.
MUTACOES = [
    ('sbH_4_capacidade_pela_contagem',
     '    for field in [friendly::PLANNED_NEXT_INDEX, friendly::PLANNED_COUNT, friendly::PLANNED_CAPACITY] {\n        exe::crt_fwrite(a + field, 4, 1, f);',
     '    for field in [friendly::PLANNED_NEXT_INDEX, friendly::PLANNED_COUNT, friendly::PLANNED_COUNT] { // mutation\n        exe::crt_fwrite(a + field, 4, 1, f);',
     'sbH_4 friendlies saved'),
    ('sbH_4_sem_data_base',
     '    exe::crt_fwrite(a + friendly::PRESEASON_BASE_DATE, 8, 1, f);',
     '    // mutation: the pre-season base date is not written',
     'sbH_4 friendlies saved'),
    ('sbH_4_carga_sem_itens_por_nacao',
     '            exe::llist_add_tail(read_usize(this.nation_lists() + 4 * i), &mut item as *mut u32 as usize, 4);',
     '            let _ = item; // mutation: the item is not added',
     'sbH_4 friendlies loaded'),
]
