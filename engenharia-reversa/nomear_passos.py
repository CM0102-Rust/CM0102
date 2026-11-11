r"""Raw array strides by name (11/11/2026, the outside reader's round 12, obstacle 3: "0x3b, 0x49, 0x625, 0x4f... each
hides 'record k of table X'"). A literal in a multiplication (`k * 0x3b`, `0x49 * j`, `.wrapping_mul(0x3b)`) becomes the
record's size constant (`league_table_row::SIZE`) only when the same line already names that record (its module, its
type or its table global), so a 0x3b that is not a table row stays a number; a line that names two candidates for the
same value is left alone. Missing SIZE constants are added to fields.rs. Run ajustar_constantes.py afterwards.
Usage: python nomear_passos.py [--gravar]
[pt] Os passos de vetor crus viram o tamanho do registro com nome, só na linha que já cita o registro.
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
FIELDS = os.path.join(SRC, 'motor', 'fields.rs')

# value -> [(constant, cue regex, size comment)]
PASSOS = {
    0x3b: [('league_table_row::SIZE', r'league_table_row::|LeagueTableRow|\bTABLE\b|\.table\(\)', 'the table row size (59 bytes)')],
    0x6e: [('staff::SIZE', r'\bstaff::|\bStaff\(|G_STAFF_TABLE', None)],
    0x4f: [('staff_state::SIZE', r'staff_state::|StaffState|G_STAFF_STATE_TABLE', 'the record size (79 bytes)'),
           ('fixture::SIZE', r'\bfixture::|\bFixture\(', 'the fixture record size (79 bytes)')],
    0x49: [('match_record_row::SIZE', r'match_record_row::|MatchRecordRow|ROW_STAFF_ID|HOME_LINEUP|lineup_row::', 'the row size (73 bytes)')],
    0x68: [('cup_round::SIZE', r'cup_round::|CupRound', 'the round record size (104 bytes)')],
    0x36: [('side_stats::SIZE', r'side_stats::|SIDE_STATS', 'the per-side block size (54 bytes)')],
    0x41: [('league_round_date::SIZE', r'league_round_date::|LeagueRoundDate', 'the record size (65 bytes)')],
    0x625: [('match_record::SIDE_SIZE', r'match_record::|MatchRecord\b|MatchRecord\(|match_stats_record::|HOME_TEAM', None)],
    0x245: [('club::SIZE', r'\bclub::|G_CLUB_TABLE|\bClub\(', None)],
    0x18e3: [('formation_fields::SIZE', r'formation_fields::|tactics_fields::|TACTICS', 'the per-side tactics block size (6371 bytes)')],
}
MUL = r'(?:(?<=\*)(?P<sp>\s*)(?P<a>%s)\b|\b(?P<b>%s)(?=\s*\*)|(?<=wrapping_mul\()(?P<c>%s)(?=\)))'


def garantir_size(s, const, coment):
    mod, nome = const.split('::')
    m = re.search(r'pub\(crate\) mod %s \{\n' % mod, s)
    if not m:
        return s, False
    fim = s.index('\n}', m.end() - 1)
    if re.search(r'const %s: usize' % nome, s[m.end():fim]):
        return s, True
    if coment is None:
        return s, False
    val = [v for v, l in PASSOS.items() if any(c == const for c, _, _ in l)][0]
    linha = '    pub(crate) const %s: usize = %s; // %s\n' % (nome, hex(val), coment)
    return s[:m.end()] + linha + s[m.end():], True


def main():
    grava = '--gravar' in sys.argv
    s = open(FIELDS, encoding='utf-8').read()
    usaveis = {}
    for v, lista in PASSOS.items():
        for const, cue, coment in lista:
            s, ok = garantir_size(s, const, coment)
            if ok:
                usaveis.setdefault(v, []).append((const, re.compile(cue)))
    n = 0
    for raiz, _, fs in os.walk(SRC):
        for f in fs:
            if not f.endswith('.rs') or f in ('fields.rs', 'records.rs', 'bits.rs', 'enums.rs', 'match_codes.rs'):
                continue
            p = os.path.join(raiz, f)
            linhas = open(p, encoding='utf-8', newline='').read().split('\n')
            mudou = False
            for i, l in enumerate(linhas):
                cod, sep, com = l.partition('//')
                if cod.lstrip().startswith('///') or not re.search(r'\*|wrapping_mul', cod):
                    continue
                partes = re.split(r'("(?:[^"\\]|\\.)*")', cod)
                novo_partes = []
                for parte in partes:
                    if parte.startswith('"'):
                        novo_partes.append(parte)
                        continue
                    for v, cands in usaveis.items():
                        hits = [c for c, cue in cands if cue.search(cod)]
                        if len(hits) != 1:
                            continue
                        lits = r'0x%x|%d' % (v, v)
                        parte = re.sub(MUL % (lits, lits, lits), lambda m, h=hits[0]: (m.group('sp') or '') + h, parte)
                    novo_partes.append(parte)
                novo = ''.join(novo_partes)
                if novo != cod:
                    n += 1
                    linhas[i] = novo + sep + com
                    mudou = True
            if mudou and grava:
                open(p, 'w', encoding='utf-8', newline='').write('\n'.join(linhas))
    print('%d linhas com passo nomeado' % n)
    if grava:
        open(FIELDS, 'w', encoding='utf-8').write(s)


if __name__ == '__main__':
    main()
