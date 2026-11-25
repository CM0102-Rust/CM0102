# Mutacoes (rx08_4), ENG ate 2005-12-31 (a campanha europeia de 2004, ate novembro de 2005):
# - o vencedor de grupo nao entra na Copa do Mundo: a Copa de 2006 fica com uma selecao europeia a menos no sorteio de
#   dezembro de 2005 (o comparador dos registros so roda a original: a carreira acusa, como a cx17_2_classificada_fora_da_copa);
# - o tamanho do grupo pelo proprio indice (sem o - 1): outros grupos no sorteio (a carreira acusa nos jogos e o
#   comparador do sorteio nas vagas);
# - o eliminado de grupo com a frase de classificado (7): acusada pelo comparador do texto.
MUTACOES = [
    ('rx08_4_vencedor_fora_da_copa',
     'exe::worldcup_add_qualified_team(world_cup, club); // rx08_4: the nation enters the World Cup',
     'let _ = world_cup; // rx08_4 mutation: the nation stays out of the World Cup',
     'rx08_4 european group winner to the world cup'),
    ('rx08_4_tamanho_do_grupo',
     "if (had as i16) < size(this, g.wrapping_sub(1) as u8 as u32) { // rx08_4: the group's size by its index less one",
     "if (had as i16) < size(this, g as u8 as u32) { // rx08_4 mutation: the size by the index itself",
     'rx08_4 european group draw'),
    ('rx08_4_eliminado_classificado',
     'exe::comp_util_write_group_qualify_news(club_index, a3, comp, -2, full_sentence, text); // rx08_4: the group exit sentence',
     'exe::comp_util_write_group_qualify_news(club_index, a3, comp, 7, full_sentence, text); // rx08_4 mutation',
     'rx08_4 european group out'),
]
