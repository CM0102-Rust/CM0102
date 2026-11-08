# Mutacoes. Todas no indice da nacao-chave da competicao (0x669470), que o alcance das regras da partida 0x8d2fa0 (cpk_4)
# usa para escolher as regras de escalacao: indice 0 e 1, tipo 4 (sem regras); 3 a 7, tipo 2 (as do continente); os
# outros, tipo 1 (as da nacao da linha). A liga na linha seguinte usa as regras de outra nacao, e a liga fora da tabela
# (4) as do continente: jogadores elegiveis ou nao, escalacoes e partidas mudam; a liga do dia 0x6847c0 tambem le o dia
# da linha (+0x15). Sem a eliminatoria europeia na lista da Copa, os jogos de setembro a novembro de 2001 caem na linha
# do continente (tipo 2, as regras da Europa no lugar de nenhuma): pega se alguma regra do continente barrar jogador de
# selecao (a suspensao na competicao, por exemplo) - pode escapar. TapeCall a cada partida: sem contador.
MUTACOES = [
    ('cpt4_linha_seguinte', 'Some(k) => k as u8, // cpt4: a linha da nacao',
     'Some(k) => k as u8 + 1, // mutacao: a linha seguinte'),
    ('cpt4_nacao_fora_da_tabela', 'let linha = (0..0x23usize).find(|&k| cpt4_d(0xb63d60 + 72 * k) == nacao); // cpt4: a busca da nacao',
     'let linha: Option<usize> = None; // mutacao: a nacao fora da tabela'),
    ('cpt4_sem_a_eliminatoria_europeia', 'let copa = [0x9cf79c, 0x9cf77c, 0x9cf774, 0x9cf770, 0x9cf780, 0x9cf76c, 0x9cf778]; // cpt4: a Copa e as eliminatorias',
     'let copa = [0x9cf79c, 0x9cf77c, 0x9cf774, 0x9cf770, 0x9cf76c, 0x9cf778]; // mutacao: sem a eliminatoria europeia'),
]
