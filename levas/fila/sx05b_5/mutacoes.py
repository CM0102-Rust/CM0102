# Mutacoes (sx05b_5). Os construtores rodam na criacao do jogo; acusadas pelos comparadores (o objeto conferido
# contra o plano) e, as duas primeiras, tambem pela carreira (a equipe da semana e da temporada mudam).
# - o premio de equipe com quatro reservas;
# - o premio do ano sem o tipo do registro (o Staff_Records guarda outro);
# - a tabela das notas com o ano seguinte.
MUTACOES = [
    ('sx05b_5_quatro_reservas',
     '    write_u8(this + ta::SUBSTITUTES, 3);',
     '    write_u8(this + ta::SUBSTITUTES, 4); // mutation: four substitutes'),
    ('sx05b_5_sem_tipo',
     '    write_u8(this + ya::KIND, kind as u8);',
     '    write_u8(this + ya::KIND, 0); // mutation: no kind'),
    ('sx05b_5_ano_seguinte',
     '    write_u16(this + 0xc, year as u16);',
     '    write_u16(this + 0xc, year as u16 + 1); // mutation: next year'),
]
