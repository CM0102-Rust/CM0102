# Mutacoes (com o ramo de cada uma). O prazo sorteado ate estatuto/10 + 6 (as respostas da IA as ofertas chegam em
# outros dias: as transferencias da carreira mudam; o comparando acusa pelas saidas). As prestacoes acabadas sem sair
# da lista (o pagador segue pagando: as financas e os orcamentos de transferencia mudam). Tirar a oferta sem baixar as
# ofertas pelo jogador (+1 do registro de 8 bytes: o jogador segue "sob oferta" e as ofertas seguintes mudam). As duas
# ultimas so a carreira acusa (os comparadores rodam so a original).
MUTACOES = [
    ('cpo_prazo_sorteio', 'let s = sortear(estatuto as i32 / 10 + 0xc) as i8;',
     'let s = sortear(estatuto as i32 / 10 + 6) as i8; // mutacao: ate +6',
     'cpo_10 drawn deadline'),
    ('cpo_prestacao_acabada', 'tirar(lista, e);',
     'let _ = (tirar, lista); // mutacao: a acabada fica',
     'cpo_10 instalment finished'),
    ('cpo_tirar_ofertas_pessoa', '*q = (*q).wrapping_sub(1); // as ofertas pela pessoa',
     '// mutacao: sem baixar as ofertas pela pessoa',
     'cpo_10 remove offer'),
]
