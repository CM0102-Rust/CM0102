# Mutacoes (cqz_12). As campanhas sul-americana e asiatica de 2004 so comecam depois de 2002-06-01 e o dia da liga de uma
# secao inativa devolve 0 sem efeito; as duas primeiras mutacoes antecipam o comeco para o primeiro dia completo (jogos,
# tabela e selecoes ja em 2001: muda a memoria do jogo, o banco). A terceira deixa 41 em vez de 40 na lista asiatica.
MUTACOES = [
    ('cqz12_america_do_sul_ja',
     '    if before {\n        return 0;\n    }\n    if draw_day && league.active() == 0 && full != 0 {\n        covered!("cqz_12 south american start");',
     '    if before && false {\n        return 0;\n    }\n    if (draw_day || true) && league.active() == 0 && full != 0 { // mutation: the start at once\n        covered!("cqz_12 south american start");',
     'cqz_12 south american day'),
    ('cqz12_asia_ja',
     '    if before {\n        return 0;\n    }\n    if draw_day && league.active() == 0 && full != 0 {\n        covered!("cqz_12 asian draw");',
     '    if before && false {\n        return 0;\n    }\n    if (draw_day || true) && league.active() == 0 && full != 0 { // mutation: the draw at once\n        covered!("cqz_12 asian draw");',
     'cqz_12 asian day'),
    ('cqz12_asia_41',
     'let capped = (read_u32(this + 0xf3) as i32).min(0x28) as i16; // cqz12: at most 40 go on',
     'let capped = (read_u32(this + 0xf3) as i32).min(0x29) as i16; // mutation: 41',
     'cqz_12 asian entrants'),
]
