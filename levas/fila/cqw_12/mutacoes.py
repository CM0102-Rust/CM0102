# Mutacoes (cqw_12), nas copas entre continentes montadas no comeco do jogo: cada uma um ano depois (a da CONCACAF de
# agosto de 2001, a Interamericana de janeiro de 2002 e a Intercontinental de dezembro de 2001 saem da janela; os
# clubes delas, inclusive os da MLS, nao jogam).
MUTACOES = [
    ('cqw12_concacaf_um_ano_depois',
     'ano as i16); // cqw12: a CONCACAF do ano do jogo',
     '(ano as i16).wrapping_add(1)); // mutacao: um ano depois',
     'cqw_12 copa da concacaf'),
    ('cqw12_interamericana_um_ano_depois',
     'let ano = ano; // cqw12: a Interamericana do ano do jogo',
     'let ano = ano + 1; // mutacao: um ano depois',
     'cqw_12 copa interamericana'),
    ('cqw12_intercontinental_um_ano_depois',
     'let ano = ano; // cqw12: a Intercontinental do ano do jogo',
     'let ano = ano + 1; // mutacao: um ano depois',
     'cqw_12 copa intercontinental'),
]
