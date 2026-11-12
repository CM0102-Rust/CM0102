# Mutacoes (cqz_8), nas eliminatorias sul-americanas da campanha de 2004 (montadas na criacao: a tabela e as rodadas ficam
# na memoria do jogo, o banco): a tabela de oito selecoes mesmo sem isentos; um isento a mais desde o comeco (a tabela
# fica com nove vagas); nove vagas em vez de dez menos os isentos.
MUTACOES = [
    ('cqz8_rodadas_de_oito',
     'if (read_u8(this.addr() + wc_south_american_league_fields::EXEMPT_COUNT) as i8) >= 2 { // cqz8: eight nations with two exempt',
     'if (read_u8(this.addr() + wc_south_american_league_fields::EXEMPT_COUNT) as i8) >= 0 { // mutation: always eight',
     'cqz_8 south american qualifying round table'),
    ('cqz8_um_isento_a_mais',
     'write_u8(south, 0); // cqz8: no South American nation exempt yet',
     'write_u8(south, 1); // mutation: one exempt from the start',
     'cqz_8 exempt nations'),
    ('cqz8_nove_vagas',
     'let places = 10 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i16; // cqz8: ten places minus the exempt',
     'let places = 9 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i16; // mutation: nine places',
     'cqz_8 south american slot'),
]
