# Mutacoes das regras por nacao. A do clube fora da Argentina inverte a resposta (disciplina, diario dos tecnicos e
# noticias, com clubes ingleses: a carreira ate 2002-06-01 deve acusar). A de Argentina/Franca/Croacia tira a Franca:
# so muda a resposta para clubes franceses no diario dos tecnicos 0x6ab800; na carreira da Inglaterra PODE ESCAPAR
# (o comparando acusa no primeiro clube frances). A da Melanesia tira a ultima chave: so roda nas copas da Oceania,
# que as carreiras nao jogam: ESCAPA (lacuna); o comparador roda no jogo real comparando se a base tiver a Oceania.
MUTACOES = [
    ('dbr_fora_da_argentina', '(id != *(0x9cf218 as *const u32)) as u32, // a Argentina', '(id == *(0x9cf218 as *const u32)) as u32, // mutacao: a resposta invertida'),
    ('dbr_fora_de_arg_fra_cro', 'let chaves = [0x9cf218usize, 0x9cf300, 0x9cf2ac]; // Argentina, França, Croácia', 'let chaves = [0x9cf218usize, 0x9cf218, 0x9cf2ac]; // mutacao: sem a Franca'),
    ('dbr_melanesia', '0x9cf2f8, 0x9cf500]) // melanésias', '0x9cf2f8, 0x9cf2f8]) // mutacao: sem a ultima melanesia'),
]
