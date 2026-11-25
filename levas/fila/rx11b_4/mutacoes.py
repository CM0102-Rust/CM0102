# Mutacoes (rx11b_4), item de telas: acusadas pelo comparador do texto lado a lado quando o Lucas abre um historico.
MUTACOES = [
    ('rx11b_4_temporada_do_premio',
     '            history_season_text(sp, span != 0, year - span, year, year);\n',
     '            history_season_text(sp, span != 0, year, year + span, year); // mutation\n',
     'rx11b_4 named in a team of the season'),
    ('rx11b_4_venda_sem_troca',
     "fmt(NEWS_TEXT_BUFFER, if part_exchange { 0xa83f98 } else { 0xa83f1c }, g, g, np, short, mp); // \"Sold .. to .. for ..\"",
     'fmt(NEWS_TEXT_BUFFER, 0xa83f1c, g, g, np, short, mp); // mutation',
     'rx11b_4 sold for a fee'),
]
