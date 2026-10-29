# Mutacoes. Ninguem sob oferta sem a visao do humano (~4,9 milhoes de chamadas, quente: sem contador; o comparando
# acusa na primeira pessoa marcada; a carreira, porque os clubes passam a disputar quem ja esta em negociacao). A lei
# Bosman invertida (~31 milhoes, quente: sem contador; os contratos que vencem mudam de destino na carreira). O veto da
# diretoria sem cortar os valores do jogador (so o modo Rust e a carreira: o comparando roda a original; os valores
# +9/+0xb/+0xd entram nas negociacoes seguintes; se o bit 0x2000 nao vier na carreira, lacuna).
MUTACOES = [
    ('cpl_sob_oferta_marca', 'return marca(); // a marca "sob oferta"',
     'return 0; // mutacao: ninguem sob oferta'),
    ('cpl_bosman_invertida', 'livre(cpl2_d(clube + 0x53)) // a nação do clube',
     '(livre(cpl2_d(clube + 0x53)) == 0) as u32 // mutacao: invertida'),
    ('cpl_veto_valores', 'core::ptr::write_unaligned(p, ftol(core::ptr::read_unaligned(p) as f64 * f64_em(0x9689d8)) as i16); // 90%',
     'let _ = p; // mutacao: sem cortar', 'cpl_2 veto da diretoria valores'),
]
