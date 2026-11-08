# Mutacoes. O candidato a tecnico com a nota minima meio ponto acima (os candidatos no limit deixam de servir: as
# contratacoes da maquina mudam; sem ramo, a 0x69f810 roda para cada candidato de cada vaga). O cargo com a pessoa nula
# valendo 1 (o regen de nao jogador 0x7ab9f2 pergunta com nula: passa a tirar o contrato 0x4d0b50; se nao houver
# regen de nao jogador ate 2002-06-01 PODE ESCAPAR, ver o ramo). A queda da moral sem o arredondamento (+0,5): a
# moral depois de cada partida ruim fica um ponto acima em parte dos casos (sem ramo, quente).
MUTACOES = [
    ('cpp_nota_do_candidato', '    if nota >= nota_minima { 1 } else { 0 }',
     '    if nota >= nota_minima + 0.5 { 1 } else { 0 } // mutacao: a nota minima mais alta'),
    ('cpp_cargo_pessoa_nula', '        return 0; // pessoa nula',
     '        return 1; // mutacao: a pessoa nula pode',
     'cpp_4 null person'),
    ('cpp_queda_sem_arredondar', '    Some(match_pl_ftol(d / y + read_f64_at(0x968968)) as u8 as i8)',
     '    Some(match_pl_ftol(d / y) as u8 as i8) // mutacao: sem o arredondamento'),
]
