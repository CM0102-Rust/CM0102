# Mutacoes (sx03_3). As regras europeias rodam na carreira ENG: o construtor na montagem das regras, as outras duas
# nas partidas das copas europeias (Liga dos Campeoes, Copa UEFA, Intertoto).
# - a regra do jogador inscrito por outro clube sempre 0: escala jogadores que o original barra (comparador e carreira);
# - o limite de idade 21 em vez de 20 (comparador);
# - o registro de limites com 40 bytes em vez de 42 (comparador lado a lado);
# - a contagem de janelas 1 em vez de 0 (comparador dos campos do construtor).
MUTACOES = [
    ('sx03_3_sem_regra_europeia',
     'if person.squad_selected_for() != 2 { 8 } else { 0 } // sx03_3: cup-tied',
     '0 // mutation: never cup-tied',
     'sx03_3 cup-tied rule'),
    ('sx03_3_idade_21',
     'if (person.age() as i8) < 0x14 { // sx03_3: players under 20',
     'if (person.age() as i8) < 0x15 { // mutation: players under 21',
     'sx03_3 cup-tied rule'),
    ('sx03_3_limites_40',
     'for k in 0..42 { // sx03_3: the 42-byte record',
     'for k in 0..40 { // mutation: 40 bytes',
     'sx03_3 default squad restrictions'),
    ('sx03_3_janelas_1',
     "write_u32(this, 0x96a188); // sx03_3: the Europe vtable\n    write_u8(this + 8, 0);",
     "write_u32(this, 0x96a188); // mutation\n    write_u8(this + 8, 1);",
     'sx03_3 europe rules'),
]
