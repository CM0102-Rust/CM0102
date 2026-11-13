# As escapadas da 290ª explicadas pelo código (para o ESCAPADAS do docs.py; formato da levas/284/docs.py).
ESCAPADAS = {
    'sma2_sem_marca_da_liga_humana': (
        'a marca só é posta quando a nação do novo clube tem o bit 1 (liga só de visita): o original testa '
        '`test byte [nação+0x11c], 1` em 0x69f513 e pula o bloco inteiro (com o `or 2` de 0x69f5ac) para 0x69f5b3; na carreira '
        'o técnico humano é nomeado uma vez só, no clube inglês, e o `create_game_from_setup_file` (0x827110) zera o byte de '
        'todas as nações (0x82745f) e dá às ligas do arquivo `((~x & 2) | 1) << 1`, 2 ou 6 (0x8274f4..0x8274fc, 0x82751e..0x827527), '
        'nunca o bit 1; então o bloco não roda e tirar o `| 2` não muda nada (provado pelo código)',
        'the mark is only set when the new club\'s nation has bit 1 (a view-only league): the original tests '
        '`test byte [nation+0x11c], 1` at 0x69f513 and skips the whole block (with the `or 2` at 0x69f5ac) to 0x69f5b3; in the '
        'career the human manager is appointed only once, at the English club, and `create_game_from_setup_file` (0x827110) '
        'clears every nation\'s byte (0x82745f) and gives the file\'s leagues `((~x & 2) | 1) << 1`, 2 or 6 (0x8274f4..0x8274fc, '
        '0x82751e..0x827527), never bit 1; so the block does not run and dropping the `| 2` changes nothing (proved from the code)'),
    'sma5_orgao_ingles_argumento_0': (
        'a carreira é um jogo novo: o `Discipline::Discipline` (0x5554a0) chama o `create_rule_sets` (0x55d420) com `push 0` '
        '(0x55575e), e só o `Discipline::load` (0x555860, a carga de um save) passa um arquivo (0x555a5e); com o argumento 0 o '
        'construtor do órgão inglês (0x7dcf90) chega ao construtor-base (0x7e7760), que pula a leitura (0x7e78a0) em 0x7e77cb, '
        'então trocar o argumento por 0 não muda nada fora da carga de um save (provado pelo código)',
        'the career is a new game: `Discipline::Discipline` (0x5554a0) calls `create_rule_sets` (0x55d420) with `push 0` '
        '(0x55575e), and only `Discipline::load` (0x555860, loading a save) passes a file (0x555a5e); with argument 0 the English '
        'ruling body constructor (0x7dcf90) reaches the base constructor (0x7e7760), which skips the read (0x7e78a0) at 0x7e77cb, '
        'so replacing the argument with 0 changes nothing outside loading a save (proved from the code)'),
}  # nome: (explicação pt, explicação en)
