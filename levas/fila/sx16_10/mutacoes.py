# Mutacoes (sx16_10) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa.
# - a dureza "rigoroso" a partir de 17 de disciplina (e nao 19): as linhas novas da criacao do jogo (o .qst gravado)
#   mudam;
# - a carga zera a ultima nota do primeiro arbitro: o estado depois da carga do save de ferias muda.
MUTACOES = [
    ('sx16_10_rigoroso_17',
     '        d if d > 0x12 => 1,',
     '        d if d > 0x10 => 1, // mutation'),
    ('sx16_10_ultima_nota_zerada',
     '        if exe::zread(rows, OFFICIAL_STATS_SIZE as u32, n, z) != n {\n            return fail(&mut z);\n        }',
     '        if exe::zread(rows, OFFICIAL_STATS_SIZE as u32, n, z) != n {\n            return fail(&mut z);\n        }\n        write_u8(rows + official_stats::LAST_GRADE, 0); // mutation'),
]
