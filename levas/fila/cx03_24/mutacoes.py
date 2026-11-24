# Mutacoes (cx03_24), carreira com as ligas da Suecia ate 2002-08-01. Nenhuma abre caixa.
# - o texto da secao vencida com o nome inteiro da competicao (+4): a comparacao acusa (modo CM_TEXTO_NOTICIAS);
# - o vencedor da secao com o peso 0x50 no registro: o historico dos tecnicos muda (a reputacao dos tecnicos, que pesa
#   nas ofertas de emprego: a carreira).
MUTACOES = [
    ('cx03_24_texto_com_nome_inteiro',
     'european_cup_fate_text(text, club_id, 0xa8b4d4, comp.short_gender_name(), comp.addr() + competition::SHORT_NAME, comp.addr());',
     'european_cup_fate_text(text, club_id, 0xa8b4d4, comp.short_gender_name(), comp.addr() + competition::NAME, comp.addr()); // mutation'),
    ('cx03_24_vencedor_da_secao_peso_0x50',
     '            1 => european_cup_add_qualified_record(club, comp, 0, 0x1e, 0x5a), // through to the main draw',
     '            1 => european_cup_add_qualified_record(club, comp, 0, 0x1e, 0x50), // mutation'),
]
