# Bloco das competições, os amistosos (friendly.cpp): o adversário do amistoso (Friendly::find_friendly_opponent
# 0x5c4cb0, 356 chamadas na carreira; chamada pela 0x5c3040 e pela 0x5c4770) e o da excursão (find_tour_opponent
# 0x5c5040, 174; pela 0x5c3040). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real
# comparando. Sem SEH, sem caixa, locais todas gravadas antes de lidas (a chave do bsearch: 0x16 bytes zerados com a
# faixa, ou 0xff e a nação). As duas sorteiam (0x90cfc0): comparador lado a lado pelo estado do sorteio
# (STATE_REGIONS), a cópia em without_counting, e pelo cursor (+0xa) da lista de cada candidato (a única escrita: as duas
# voltam o cursor ao começo e percorrem com a 0x68d1b0). Nenhuma em laço quente.
SAVE = False
ROTINAS = [('adversario_do_amistoso', 0x5c4cb0, [0x83, 0xec, 0x1c, 0x89, 0x4c, 0x24, 0x00]),
           ('adversario_da_excursao', 0x5c5040, [0x83, 0xec, 0x20, 0x8b, 0x44, 0x24, 0x24])]
DOCS = ('os amistosos: o adversário do amistoso e o da excursão',
        'friendlies: the friendly opponent and the tour opponent',
        [('adversario_do_amistoso', 'Friendly::find_friendly_opponent', '0x5c4cb0', 'friendly.cpp', 'thiscall, ret 0x14',
          'busca (bsearch) os candidatos da faixa (1 a 20, subindo até achar) e, dali, o primeiro que não é o clube, nem excluído, nem já adversário, nem fora pela data, nem de faixa cruzada, rival, de nação rival (meio a meio), da mesma cidade ou acima da faixa sorteada, que passa pela nação conforme a reputação do clube e é sorteado (1 em 5); senão 0',
          'searches (bsearch) the candidates of the band (1 to 20, going up until found) and, from there, the first one that is not the club, excluded, already an opponent, out by date, of crossed bands, a rival, of a rival nation (half the time), from the same city or above the drawn band, that passes the nation check by the club reputation and is drawn (1 in 5); else 0'),
         ('adversario_da_excursao', 'Friendly::find_tour_opponent', '0x5c5040', 'friendly.cpp', 'thiscall, ret 0x14',
          'busca (bsearch) os candidatos da nação da excursão e, do primeiro dela, o que não passa da faixa máxima nem é fora pela data, o próprio clube, já adversário, rival, de nação rival (meio a meio), da mesma cidade ou acima da faixa sorteada, sorteado meio a meio; para no primeiro de outra nação; senão 0',
          'searches (bsearch) the candidates of the tour nation and, from its first one, the one not above the maximum band nor out by date, the club itself, already an opponent, a rival, of a rival nation (half the time), from the same city or above the drawn band, drawn half the time; stops at the first one of another nation; else 0')])
CAMPOS = [('candidato do amistoso (0x16 bytes): +0 id do clube, +4 faixa sorteada, +5 faixa, +6 nação, +0xa lista de adversários, +0xe/+0x10 faixas (curtos); lê e grava o cursor (+0xa) da lista (adversario_do_amistoso, adversario_da_excursao)',
           'friendly candidate (0x16 bytes): +0 club id, +4 drawn band, +5 band, +6 nation, +0xa opponent list, +0xe/+0x10 bands (shorts); reads and writes the list cursor (+0xa) (adversario_do_amistoso, adversario_da_excursao)'),
          ('clube (0x245 bytes): +0x53 nação, +0x57 cidade, +0x80 reputação, +0xb3/+0xb7/+0xbb rivais, +0x1db; nação +0x71, +0x75, +0x110/+0x114/+0x118 rivais; Friendly +0x34e8 ano; lê (as duas)',
           'club (0x245 bytes): +0x53 nation, +0x57 city, +0x80 reputation, +0xb3/+0xb7/+0xbb rivals, +0x1db; nation +0x71, +0x75, +0x110/+0x114/+0x118 rivals; Friendly +0x34e8 year; reads (both)')]
INTERESSA = ['SGE', 'CMRTE']
