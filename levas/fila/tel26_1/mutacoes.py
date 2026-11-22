# Mutations (tel26_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): any screen adds items, and the add_item comparer catches both.
# - the free widget order written one slot further on;
# - the area order count not advanced.
MUTACOES = [
    ('tel26_1_free_order_slot', 'write_i16(d + display_list::ITEM_ORDER + n as usize * 2, index);',
     'write_i16(d + display_list::ITEM_ORDER + n as usize * 2 + 2, index);'),
    ('tel26_1_area_order_count', 'write_i16(d + display_list::AREA_ORDER_COUNT, n + 1);',
     'write_i16(d + display_list::AREA_ORDER_COUNT, n);'),
]
