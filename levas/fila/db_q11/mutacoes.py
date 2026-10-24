# Mutacoes dos vetores de volta ao nao jogador e ao jogador e do reequilibrio do nao jogador. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq11_nao_jogador', "        ultimo = *((vetor + 3 * k) as *const u8);", "        ultimo = *((vetor + 3 * k + 1) as *const u8);"),
    ('dq11_jogador', "*((jogador + d as usize) as *mut u8) = *((vetor + 3 * i) as *const u8);", "*((jogador + d as usize) as *mut u8) = *((vetor + 3 * i + 1) as *const u8);"),
    ('dq11_normalizar', "    let delta = (-5i32).wrapping_sub", "    let delta = (-4i32).wrapping_sub"),
]
