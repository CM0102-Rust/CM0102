# Mutacoes do nome da regiao e do adjetivo do continente. As duas so rodam nas telas (busca de jogadores,
# find_screens, Menubar, jogos do dia) e a ordem das regioes (0x545dc0) so no menu da busca: a carreira oculta nao
# passa por elas e ESCAPAM (lacuna); os comparadores rodam no jogo real comparando, nas telas (ver o LEIA da leva):
# a da regiao (adjetivo e substantivo trocados fora dos idiomas 2, 4, 5 e 9, o ingles do jogo) e a do continente
# (o adjetivo do registro sem a primeira letra, forma 0 ou idioma fora de 9, 5 e 4).
MUTACOES = [
    ('dbu_regiao', 'let usa_adjetivo = if grupo { fb == 0xff } else { fb != 0xff }; // (dbu)',
     'let usa_adjetivo = if grupo { fb == 0xff } else { fb == 0xff }; // mutacao: adjetivo e substantivo trocados'),
    ('dbu_continente', '    let proprio = continente + 0x23; // o adjetivo do registro (dbu)',
     '    let proprio = continente + 0x24; // mutacao: sem a primeira letra'),
]
