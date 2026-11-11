# Mutacoes (cqw_12), nas copas entre continentes montadas no comeco do jogo: cada uma um ano depois (a da CONCACAF de
# agosto de 2001, a Interamericana de janeiro de 2002 e a Intercontinental de dezembro de 2001 saem da janela; os
# clubes delas, inclusive os da MLS, nao jogam).
MUTACOES = [
    ('cqw12_concacaf_um_ano_depois',
     'this.set_year(year as i16); // cqw12: the CONCACAF cup of the game\'s year',
     'this.set_year((year as i16).wrapping_add(1)); // mutation: one year later',
     'cqw_12 concacaf champions cup'),
    ('cqw12_intercontinental_um_ano_depois',
     'let year = year; // cqw12: the Intercontinental Cup of the game\'s year',
     'let year = year + 1; // mutation: one year later',
     'cqw_12 intercontinental cup'),
]
