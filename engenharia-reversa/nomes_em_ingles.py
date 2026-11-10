r"""The functions and comparers left with Portuguese names get English ones (10/11/2026, Lucas: everything that can
move to English moves). A hand-made map, applied as whole words (each old name is unique in the code) to rust/src,
the mutation lists and docs/NOMES_ROTULOS.csv; a routine in the `routines!` table also has its IDX_ constant renamed
(build.rs makes it from the name). Methods and short names that are also common local names (`novo`, `alvo`, `bloco`,
`sorteio`) are renamed only where they are the method/function (`fn x(`, `.x(`, `::x(`).
Usage: python nomes_em_ingles.py [--gravar]
[pt] As funções e os comparadores que ficaram com nome em português ganham nome em inglês (mapa feito à mão).
"""
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

PALAVRA = {
    'posicao_no_conteiner': 'position_in_container',
    'compare_posicao_no_conteiner': 'compare_position_in_container',
    'compare_condicao': 'compare_player_condition',
    'compare_na_linha': 'compare_players_in_line',
    'compare_papel_ataque': 'compare_attacking_role',
    'compare_papel_defesa': 'compare_defending_role',
    'compare_momento': 'compare_draw_moment',
    'compare_posicao_um': 'compare_hold_position_one',
    'compare_mais_proximo': 'compare_nearest_to_point',
    'compare_normalizacao_do_jogador': 'compare_player_normalisation',
    'compare_montagem_da_partida': 'compare_match_screen_build',
    'compare_pela_lista_com': 'compare_by_display_list_with',
    'compare_valores_da_partida': 'compare_match_values',
    'compare_painel_pela_lista': 'compare_panel_by_display_list',
    'compare_lista_de_pagamentos': 'compare_payment_list',
    'instalar_passagem': 'install_passthrough',
    'ita_cup_create_group_stage_cqr8_fase_do_grupo': 'ita_cup_create_group_stage_cqr8_group_stage',
    'con_merc_cup_advance_stage_cqs11_fase_seguinte': 'con_merc_cup_advance_stage_cqs11_next_stage',
    'dois_de_nacoes_diferentes': 'two_from_different_nations',
    'bra_nat_third_create_group_stage_cqs3_grupo': 'bra_nat_third_create_group_stage_cqs3_group',
    'cqt4_atualizar_media_de_publico': 'cqt4_update_average_attendance',
    'compare_cqt4_atualizar_media_de_publico': 'compare_cqt4_update_average_attendance',
    'ita_ser_c1a_advance_stage_cql5_passar_de_fase': 'ita_ser_c1a_advance_stage_cql5_advance_stage',
}
# only as a function/method: `fn x(`, `.x(`, `::x(`
METODO = {'novo': 'new', 'bloco': 'block_at', 'sorteio': 'record_draw', 'alvo': 'target', 'usada': 'used'}


def trocar(s):
    for a, b in PALAVRA.items():
        s = re.sub(r'\b%s\b' % a, b, s)
        s = re.sub(r'\bIDX_%s\b' % a.upper(), 'IDX_' + b.upper(), s)
    for a, b in METODO.items():
        s = re.sub(r'(\bfn |\.|::)%s(\s*[(<])' % a, r'\g<1>%s\2' % b, s)
    return s


def main():
    grava = '--gravar' in sys.argv
    alvos = [os.path.join(dp, f) for dp, _, fs in os.walk(os.path.join(R, 'rust', 'src')) for f in fs if f.endswith('.rs')]
    alvos += [os.path.join(R, 'engenharia-reversa', 'mutacoes', f) for f in os.listdir(os.path.join(R, 'engenharia-reversa', 'mutacoes')) if f.endswith('.py')]
    alvos.append(os.path.join(R, 'docs', 'NOMES_ROTULOS.csv'))
    n = 0
    for p in alvos:
        t = open(p, encoding='utf-8').read()
        t2 = trocar(t)
        if t2 != t:
            n += 1
            if grava:
                open(p, 'w', encoding='utf-8').write(t2)
    print('%d arquivos' % n)


if __name__ == '__main__':
    main()
