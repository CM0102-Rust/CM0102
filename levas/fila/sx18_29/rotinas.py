# A marca de mando de um registro de partida e os destrutores da competicao reserva (record_utils.cpp; leitor sx18,
# rodada 5, 24/11/2026). Usa RECORD_UTILS_CPP do sx18_4: integrar depois dele. Item de TELAS:
# - format_venue_marker (cdecl(lado, buf, tamanho), devolve 0/1; chamada pelas telas de recordes e historico do clube,
#   0x44a11c..0x44bcd7, 0x4a6f6a..0x4a7c59, 0x80e2da, 0x87c5fd): caminho no jogo: o historico de partidas/recordes de um
#   clube (a coluna H/A/N);
# - os destrutores (~ReserveComp thiscall; ~Under21IntComp e ~BIntComp thiscall, a tabela virtual de volta e o salto
#   para Comp::~Comp 0x48c650): rodam quando o jogo e desmontado (carregar outro jogo ou sair da carreira); caminho: com
#   um jogo aberto, carregar outro save.
# Ficam de fora os tres destrutores escalares (0x7e6130, 0x7e6320, 0x7e6350): o prologo tem 3 bytes antes de um call
# rel32 (push esi; mov esi, ecx; call ~X); entrariam pelas RELATIVAS do detours.rs.
# Comparadores: a marca lado a lado num buffer local; os destrutores so a original (liberam memoria).
SAVE = False
TELAS = True
ROTINAS = [('format_venue_marker', 0x7e4ca0, [0x8b, 0x4c, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('reserve_comp_destructor', 0x7e6150, [0x6a, 0xff, 0x68, 0x18, 0x07, 0x96, 0x00]),
           ('under21_int_comp_destructor', 0x7e6340, [0xc7, 0x01, 0xe4, 0xef, 0x96, 0x00]),
           ('b_int_comp_destructor', 0x7e6370, [0xc7, 0x01, 0x74, 0xf0, 0x96, 0x00])]
DOCS = ('a marca de mando de um registro de partida e os destrutores da competição reserva',
        'a match record\'s venue marker and the reserve competition\'s destructors',
        [('format_venue_marker', 'format_venue_marker', '0x7e4ca0', 'record_utils.cpp', 'cdecl',
          'escreve a marca de mando (H em casa, A fora, N neutro) do lado 1, 2 ou 3 no buffer; buffer nulo ou de menos de 5 bytes, ou outro lado: a caixa de erro e 0',
          'writes the venue marker (H home, A away, N neutral) of venue 1, 2 or 3 into the buffer; a null buffer or one under 5 bytes, or another venue: the error box and 0'),
         ('reserve_comp_destructor', 'ReserveComp::~ReserveComp', '0x7e6150', 'record_utils.cpp', 'thiscall',
          'desmonta a competição reserva: apaga as competições B e sub-21 e roda o destrutor da base',
          'destroys the reserve competition: deletes the B and Under 21 competitions and runs the base destructor'),
         ('under21_int_comp_destructor', 'Under21IntComp::~Under21IntComp', '0x7e6340', 'record_utils.cpp', 'thiscall',
          'desmonta a competição sub-21: a tabela virtual de volta e o destrutor da base',
          'destroys the Under 21 competition: its vtable back and the base destructor'),
         ('b_int_comp_destructor', 'BIntComp::~BIntComp', '0x7e6370', 'record_utils.cpp', 'thiscall',
          'desmonta a competição B: a tabela virtual de volta e o destrutor da base',
          'destroys the B competition: its vtable back and the base destructor')])
CAMPOS = []
INTERESSA = []
