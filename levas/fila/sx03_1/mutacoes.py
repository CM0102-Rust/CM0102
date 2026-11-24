# Mutacoes (sx03_1). Os premios dao reputacao ao vencedor (banco de dados): a carreira ENG ate 2002-06-01 acusa.
# - a partida nao chega aos premios da nacao: sem notas do mes, o jogador do mes muda;
# - a atualizacao diaria dos premios das nacoes nao roda: nenhum premio do mes nem do ano;
# - as notas do ano mundiais sem as partidas: o jogador do ano internacional de 6 de maio muda;
# - a reputacao mundial no lugar da nacional: o comparador lado a lado acusa, e a carreira pela reputacao dada.
MUTACOES = [
    ('sx03_1_sem_premios_da_nacao',
     "record_match(awards, fixture, record); // sx03_1: the nation's awards",
     'let _ = (record_match, awards); // mutation: the nation awards miss the match',
     'sx03_1 nation awards take the match'),
    ('sx03_1_sem_diaria',
     "daily(awards, today); // sx03_1: each nation's awards",
     'let _ = daily; // mutation: no daily update of the nation awards',
     'sx03_1 daily slot'),
    ('sx03_1_sem_notas_do_ano',
     'year_ratings(world, fixture, record, AWARD_FILTER_REPUTATION_OVER_2000); // sx03_1: the world year ratings',
     '{} // mutation: the world year ratings miss the match',
     'sx03_1 world year ratings'),
    ('sx03_1_reputacao_trocada',
     'write_u16(this + award_fields::NATIONAL_REPUTATION, national_reputation as u16); // sx03_1: the national reputation',
     'write_u16(this + award_fields::NATIONAL_REPUTATION, world_reputation as u16); // mutation: the world reputation',
     'sx03_1 award built'),
]
