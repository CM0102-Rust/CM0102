# Mutacoes (sx10_6), pela carreira ENG ate 2002-06-01 e pelo comparador (lado a lado sobre o registro inteiro: toda
# mutacao aqui e acusada pelo comparador na primeira partida da comparacao).
# - todos designados para a pessoa 0 (no lugar de -1): o Formation::load_from_disk nao regrava os designados, entao as
#   taticas dos clubes (0x893f38, carregadas dos .pct) ficam com 0 e a marcacao da partida pode mudar;
# - as instrucoes do time padrao com o bit 0 trocado (0x22a91 -> 0x22a90): so muda a partida se alguma formacao da
#   partida ficar com as instrucoes do construtor (as dos .pct as regravam).
MUTACOES = [
    ('sx10_6_designado_zero',
     'write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, -1); // sx10_6: nobody designated',
     'write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, 0); // mutation: person 0 designated',
     'sx10_6 default formation'),
    ('sx10_6_instrucoes_do_time',
     'write_u32(this + formation_fields::FLAGS, FORMATION_DEFAULT_TEAM_INSTRUCTIONS); // sx10_6: default team instructions',
     'write_u32(this + formation_fields::FLAGS, FORMATION_DEFAULT_TEAM_INSTRUCTIONS ^ 1); // mutation: another default',
     'sx10_6 default formation'),
]
