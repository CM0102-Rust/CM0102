# Mutacoes (mwg_16):
# - o candidato novo ja entra com um voto (o comparador lado a lado de add_entry pega; na carreira, a votacao do
#   premio de fim de temporada);
# - a lista cabe um candidato a mais que o escopo manda (o construtor so roda o original: pega pela carreira, outro
#   candidato na votacao do premio de maio/junho).
# announce fica sem mutacao propria: so manda noticias aos humanos e registra a lista (o comparador so roda o original).
MUTACOES = [
    ('mwg16_candidato_com_voto',
     "write_u16(e + asl::ENTRY_VOTES, 0); // mwg16: a new candidate without votes",
     "write_u16(e + asl::ENTRY_VOTES, 1); // mutation: a new candidate with one vote",
     'mwg_16 shortlist candidate'),
    ('mwg16_lista_maior',
     "let capacity = read_i8(scope + 0xd) as i16; // mwg16: the shortlist's size from its scope",
     "let capacity = read_i8(scope + 0xd) as i16 + 1; // mutation: one more candidate",
     'mwg_16 shortlist built'),
]
