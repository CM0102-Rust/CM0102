# Mutacoes (sx19b_4), pela prova do save (prova_save.py --leva N, mutacoes.py --prova-save N); o construtor roda na
# criacao do jogo que a prova grava:
# - a lista curta (vaga 0) com o tipo 2 (o scout_manager.dat do .qst muda);
# - a lista escolhida de cada tecnico humano 2 em vez de 1 (os 16 bytes gravados mudam).
MUTACOES = [
    ('sx19b_4_tipo_trocado',
     'set_list_type(s, match k { 0 => 1, 1 => 2, 2 => 3, _ => 4 }); // sx19b_4: the list types',
     'set_list_type(s, match k { 0 => 2, 1 => 2, 2 => 3, _ => 4 }); // mutation: no short list',
     'sx19b_4 searches built'),
    ('sx19b_4_lista_escolhida',
     'write_u8(this + scout_manager_fields::SELECTED_LISTS + m, 1);',
     'write_u8(this + scout_manager_fields::SELECTED_LISTS + m, 2); // mutation: another selected list',
     'sx19b_4 searches built'),
]
