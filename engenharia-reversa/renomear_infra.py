r"""Nomes em inglês para a infraestrutura de comparação que o motor usa (etapa 4 da reorganização, 08/11/2026):
`crate::comparacao::lado_a_lado` -> `crate::comparison::side_by_side`, `PLACAR.registrar` -> `PLACAR.record`... Os
módulos `comparacao.rs` e `aleatorio.rs` passam a `comparison.rs` e `rng.rs`. Troca só no código (não em comentário nem
em texto entre aspas: os nomes do registro, "motor", "data", "aleatorio", não mudam), nos .rs do crate, nas listas de
mutação, nas levas da fila e nas ferramentas que geram ou conferem código. Os nomes genéricos (`ramo`, `registrar`,
`ler`...) só trocam na forma qualificada ou na definição.

Uso: python renomear_infra.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import traduzir_locais  # noqa: E402

# (padrão, troca), na ordem
TROCAS = [
    (r'\blado_a_lado_leve\b', 'side_by_side_light'),
    (r'\blado_a_lado\b', 'side_by_side'),
    (r'\bREGIOES_DO_ESTADO\b', 'STATE_REGIONS'),
    (r'\bsem_contar\b', 'without_counting'),
    (r'\bpausar_rastro\b', 'pause_trace'),
    (r'\bMemoria\b', 'MemorySnapshot'),
    (r'\bPlacar\b', 'Scoreboard'),
    (r'\bSorteio\b', 'TestRng'),
    (r'\bcomparacao::ramo\b', 'comparacao::branch'),
    (r'\baleatorio::rastrear\b', 'aleatorio::trace'),
    (r'\bMemorySnapshot::ler\b', 'MemorySnapshot::read'),
    (r'\bScoreboard::novo\b', 'Scoreboard::new'),
    (r'\bPLACAR\.registrar\b', 'PLACAR.record'),
    (r'\bPLACAR\.contagem\b', 'PLACAR.counts'),
    (r'\.escrever\(\)', '.write_back()'),
    (r'\bcomparacao::', 'comparison::'),
    (r'\baleatorio::', 'rng::'),
    (r'\bmod comparacao\b', 'mod comparison'),
    (r'\bmod aleatorio\b', 'mod rng'),
]
# só nos arquivos que definem (e nos `use` sem qualificar)
DEFS = {
    'comparacao.rs': [(r'\bfn ramo\(', 'fn branch('), (r'\bfn registrar\(', 'fn record('), (r'\bfn contagem\(', 'fn counts('),
                      (r'\bfn ler\(', 'fn read('), (r'\bfn escrever\(', 'fn write_back('), (r'\bfn novo\(', 'fn new('),
                      (r'\bfn proximo\(', 'fn next_u64('), (r'\bself\.registrar\(', 'self.record('),
                      (r'\bMemorySnapshot::ler\b', 'MemorySnapshot::read')],
    'aleatorio.rs': [(r'\bfn rastrear\b', 'fn trace')],
}


def trocar(s, defs=()):
    for a, b in list(TROCAS) + list(defs):
        s = re.sub(a, b, s)
    return s


def arquivos():
    for p in glob.glob(os.path.join(R, 'rust', 'src', '**', '*.rs'), recursive=True):
        yield p, 'rs'
    for p in glob.glob(os.path.join(R, 'engenharia-reversa', 'mutacoes', '*.py')):
        yield p, 'txt'
    for p in glob.glob(os.path.join(R, 'levas', 'fila', '**', '*'), recursive=True):
        if p.endswith('.rs'):
            yield p, 'rs'
        elif p.endswith('.py'):
            yield p, 'txt'
    for n in ('fila.py',):
        yield os.path.join(R, 'engenharia-reversa', n), 'txt'


def main():
    n = 0
    for p, tipo in arquivos():
        t = open(p, encoding='utf-8').read()
        defs = DEFS.get(os.path.basename(p), ()) if os.sep + 'src' + os.sep in p else ()
        if tipo == 'rs':
            novo = traduzir_locais.so_codigo(t, lambda s, defs=defs: trocar(s, defs))
        else:
            novo = trocar(t)
        # os usos de Sorteio(...).proximo() (o gerador dos autotestes)
        novo = re.sub(r'(\b(?:rng|sorteio|s|g|gerador)\w*)\.proximo\(\)', r'\1.next_u64()', novo) if tipo == 'rs' else novo
        if novo != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8').write(novo)
    print('%d arquivos' % n)
    if '--gravar' in sys.argv:
        src = os.path.join(R, 'rust', 'src')
        for a, b in (('comparacao.rs', 'comparison.rs'), ('aleatorio.rs', 'rng.rs')):
            if os.path.exists(os.path.join(src, a)):
                os.system('git -C "%s" mv rust/src/%s rust/src/%s' % (R, a, b))
        print('gravado')


if __name__ == '__main__':
    main()
