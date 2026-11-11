# Mutacoes (mka_1), cada uma com o ramo:
# - o presidente: m = Negocios + 1 no lugar de + 6 (a reserva da diretoria cai: verbas maiores, outros lances e outras
#   contratacoes da IA);
# - o teto suave: acima de Z fica a metade do excesso, nao 10% (verbas dos clubes ricos maiores: outros lances);
# Sem mutacao da 0x4d42d0 (preco do clube humano): na bancada 2 o ramo "mka_1 human asking price" (logo depois do
# contrato tipo 2, por onde passa quase toda chamada) nao rodou na base; os chamadores so a chamam com vendedor humano
# e, na carreira de ferias, ela nao e alcancada. Nenhum caminho dela roda sem clube humano conduzido: fica coberta so
# pelo comparador no jogo real (Lucas jogando com JOGAR_COMPARANDO).
MUTACOES = [
    ('mka1_presidente',
     'let mut v = (chairman.non_player().business() as i8).wrapping_add(6);',
     'let mut v = (chairman.non_player().business() as i8).wrapping_add(1); // mutation: Business + 1',
     'mka_1 board reserve'),
    ('mka1_teto_suave',
     'budget = ftol_i64(budget.wrapping_sub(cap as i64) as f64 * read_f64(rdata::F64_0_1) + cap as f64);',
     'budget = ftol_i64(budget.wrapping_sub(cap as i64) as f64 * read_f64(rdata::F64_0_5) + cap as f64); // mutation: half the excess',
     'mka_1 soft cap'),
]
