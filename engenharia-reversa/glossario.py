r"""Gera docs/GLOSSARIO.md e docs/GLOSSARIO.en.md: nomes em portugues do codigo e o significado em ingles.

Uso: python glossario.py   (depois de cada leva: rotina nova sem traducao no dicionario EN faz o script parar)
"""
import os
import re
RAIZ = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
s = open(os.path.join(RAIZ, 'rust', 'src', 'motor.rs'), encoding='utf-8').read()
i = s.index('let rust: [usize; N] = [')
rust = re.findall(r'(\w+) as \*const', s[i:s.index('];', i)])
pro = re.findall(r'Prologo \{ endereco: (0x[0-9a-f]+)', s[:s.index('const N')])

EN = {
 'tabela_a': 'table_a', 'tabela_b': 'table_b', 'diferenca_lado': 'side_difference', 'faixa': 'band',
 'por_diferenca': 'by_difference', 'limite': 'limit', 'condicao_jogador': 'player_condition',
 'tabela_do_jogador': 'player_table', 'diferenca_jogador': 'player_difference', 'byte_do_lado': 'side_byte',
 'limite_jogador': 'player_limit', 'faixa_jogador': 'player_band', 'byte_da_posicao': 'position_byte',
 'campo_pela_posicao': 'field_by_position', 'pode_escolher': 'can_choose', 'apontar': 'aim (set facing direction)',
 'desgastar': 'tire (fatigue)', 'virado_para': 'facing', 'procurar_no_lado': 'search_on_side', 'alcanca': 'reaches',
 'classificar_lance': 'classify_move', 'distancia_ao_ponto': 'distance_to_point', 'distancia_ao_gol': 'distance_to_goal',
 'mais_perto_do_gol': 'closest_to_goal', 'jogadores_na_linha': 'players_in_line', 'consegue_jogar': 'can_play',
 'na_area': 'in_the_box', 'marcas_da_posicao': 'position_marks', 'papel_no_ataque': 'attacking_role',
 'papel_na_defesa': 'defending_role', 'posicionar': 'position (place players)', 'linhas_defensivas': 'defensive_lines',
 'atualizar_jogador': 'update_player', 'sortear_momento': 'draw_moment', 'manter_posicao_um': 'hold_position_one',
 'caminho_livre': 'clear_path', 'espaco_a_frente': 'space_ahead', 'fora_do_meio': 'off_centre',
 'alem_da_linha': 'beyond_the_line', 'distancia_entre': 'distance_between', 'ler_8ea7': 'read_8ea7',
 'ler_8ea8': 'read_8ea8', 'ler_8ea9': 'read_8ea9', 'ler_8eaa': 'read_8eaa', 'ler_8eb2': 'read_8eb2',
 'gravar_8ea9': 'write_8ea9', 'gravar_8ea7': 'write_8ea7', 'gravar_8ea8': 'write_8ea8', 'gravar_8eaa': 'write_8eaa',
 'gravar_8ed8': 'write_8ed8', 'ler_8f7a': 'read_8f7a', 'registrar_posse': 'record_possession', 'na_lista': 'in_list',
 'guardar_extremos': 'store_extremes', 'contar_ativos': 'count_active', 'codigo_especial': 'special_code',
 'atras_do_extremo': 'behind_the_extreme', 'sem_partida': 'no_match', 'pressao_na_area': 'pressure_in_box',
 'mais_proximo_do_ponto': 'nearest_to_point', 'pressao_no_ponto': 'pressure_at_point', 'menor_distancia': 'smallest_distance',
 'repartir_a': 'split_a', 'repartir_b': 'split_b', 'repartir_c': 'split_c', 'repartir_d': 'split_d',
 'repartir_e': 'split_e', 'repartir_f': 'split_f', 'escolher_por_papel': 'choose_by_role',
 'escolher_por_papel_b': 'choose_by_role_b', 'escolher_por_papel_c': 'choose_by_role_c',
 'escolher_por_papel_e': 'choose_by_role_e', 'direcao': 'direction', 'zerar_guardado': 'clear_stored',
 'x_fora_do_meio': 'x_off_centre', 'e_o_guardado': 'is_the_stored', 'lado_do_lance': 'side_of_move',
 'jogador_no_bloco': 'player_in_block', 'tabela_a_1a': 'table_a_1a', 'momento_marcado': 'marked_moment',
 'estado_do_lado': 'side_state', 'estado_do_adversario': 'opponent_state', 'gravar_19c': 'write_19c',
 'codigo_de_bola_parada': 'set_piece_code', 'perto_do_gol': 'near_goal', 'limpar_lance': 'clear_move',
 'somar_disposicao': 'add_disposition', 'registrar_lance': 'record_move', 'remover_da_lista': 'remove_from_list',
 'entrar_em_campo': 'take_the_field', 'em_perigo': 'in_danger', 'conferir_lance': 'check_move',
 'caminho_livre_ate': 'clear_path_to', 'alcanca_o_ponto': 'reaches_the_point', 'escolher_destaque': 'choose_standout',
 'casa_da_formacao': 'formation_square', 'calcular_percentuais': 'compute_percentages', 'aplicar_fator': 'apply_factor',
 'valor_pela_condicao': 'value_by_condition', 'aplicar_ao_time': 'apply_to_team', 'aplicar_ao_lado': 'apply_to_side',
 'escolher_pela_condicao': 'choose_by_condition', 'ler_1bc': 'read_1bc', 'somar_2b': 'add_2b', 'gravar_2b': 'write_2b',
 'tabela_a_1a_do_jogador': 'player_table_a_1a', 'tabela_b_do_jogador': 'player_table_b',
 'diferenca_do_jogador': 'difference_of_player', 'marcas_do_jogador_rot': 'player_marks_rot', 'marca_800000': 'mark_800000',
 'perto_do_proprio_gol': 'near_own_goal', 'papel_880': 'role_880', 'soltar_bola': 'release_ball',
 'jogador_pela_ficha': 'player_by_record', 'byte_do_codigo': 'code_byte', 'nota_ajustada': 'adjusted_rating',
 'ajustar_17': 'adjust_17', 'pegar_bola': 'take_ball', 'iniciar_rigor': 'init_strictness', 'lance_valendo': 'move_counts',
 'decidir_falta': 'decide_foul', 'punir_falta': 'punish_foul', 'punir_lance': 'punish_move',
 'motor_decidir_falta': 'engine_decide_foul', 'motor_punir_lance': 'engine_punish_move', 'motor_punir_falta': 'engine_punish_foul',
 'procurar_impedimento': 'look_for_offside', 'motor_procurar_impedimento': 'engine_look_for_offside',
 'cartao_pela_gravidade': 'card_by_severity', 'motor_cartao_pela_gravidade': 'engine_card_by_severity',
 'estatistica_de_posse': 'possession_statistic', 'resumo_da_partida': 'match_summary', 'mais_perto_no_lado': 'closest_on_side',
 'defesa_do_goleiro': 'goalkeeper_save', 'mover_para': 'move_to', 'escolher_casa': 'choose_square',
 'dominio_do_lance': 'control_of_move', 'tracar_lance': 'trace_move', 'tem_jogo': 'has_match', 'venceu_o_jogo': 'won_the_match',
 'comentario_a': 'commentary_a', 'comentario_b': 'commentary_b', 'bola_fora': 'ball_out', 'valor_do_chute': 'shot_value',
 'forca_do_time': 'team_strength', 'pontape_inicial': 'kick_off', 'rolar_bola': 'roll_ball',
 'teste_de_condicao': 'condition_test',
 'gosta_de': 'likes', 'nao_gosta_de': 'dislikes', 'marcar_posicao': 'mark_position', 'posicoes_finas': 'fine_positions',
 'unidades': 'units_digit', 'e_amistoso': 'is_friendly', 'contrato_de': 'contract_of', 'e_selecao': 'is_national_team',
 'tecnico_9a01': 'manager_9a01', 'tecnico_9a05': 'manager_9a05', 'bit_da_casa': 'square_bit', 'separar_mascara': 'split_mask',
 'e_segunda_selecao': 'is_second_national_team', 'placar_agregado': 'aggregate_score', 'marcador_direto': 'direct_marker', 'tirar_de_campo': 'take_off_pitch',
 'marcacao_individual': 'individual_marking', 'nova_reputacao': 'new_reputation', 'razao_global': 'global_ratio',
 'reputacao_do_jogador': 'player_reputation', 'escolher_cobrador': 'choose_taker', 'erro_fatal': 'fatal_error',
 'lateral': 'throw_in', 'escanteio': 'corner', 'penalti': 'penalty', 'cobrador': 'taker', 'mover': 'move', 'marcar_2b': 'mark_2b', 'pega_bola': 'take_ball', 'comparar_reinicio': 'compare_restart',
 'gravar_casa': 'set_square', 'marcar_craques': 'mark_stars',
 'designado_da_posicao': 'designated_for_position', 'marcado_designado': 'designated_marked',
 'selecao_principal': 'main_national_team', 'selecao_b': 'b_national_team',
 'quem_marcar': 'who_to_mark', 'distancia_do_gol': 'distance_to_goal', 'cerco': 'surrounding_count', 'bonus_do_gol': 'goal_bonus', 'reputacao_do_alvo': 'target_reputation',
 'tecnico_9ace': 'manager_9ace', 'tecnico_9a06': 'manager_9a06', 'pessoa_e_registro': 'person_and_record',
 'clube_humano': 'human_club',
 'lado_sob_controle': 'side_under_control',
 'linha_tatica': 'tactics_row', 'byte_tatico': 'tactics_byte',
 'ordem_dos_penaltis': 'penalty_order',
 'ordem_dos_jogadores': 'player_order',
 'vaga_marcada': 'marked_slot', 'marcar_vaga': 'mark_slot', 'pessoa_da_linha': 'row_person', 'linha_na_tabela': 'row_in_table',
 'clube_na_competicao': 'club_in_competition', 'clube_especial': 'special_club',
 'decisao_da_falta': 'foul_decision',
 'escolher_companheiro': 'choose_teammate', 'menor_distancia_livre': 'nearest_free_distance',
 'falta_no_motor': 'foul_in_engine', 'tecnico_9b23': 'manager_9b23', 'gosta_do_clube': 'likes_club', 'nao_gosta_do_clube': 'dislikes_club',
 'clube_parceiro': 'partner_club',
 'contrato_com_clube': 'contract_with_club', 'nivel_do_contrato': 'contract_level',
 'chance_de_conhecer': 'chance_to_know', 'conhecer_pessoa': 'know_person', 'nevoa_apos_partida': 'fog_after_match',
 'entrada_da_lista': 'shortlist_entry', 'bandeiras_do_contrato': 'contract_flags',
 'proximo_da_lista': 'next_list_item', 'palavra_0c': 'word_0c', 'arredondar_abaixo_de_1000': 'round_below_1000', 'nacao_do_grupo': 'group_nation', 'quantos_marcam': 'how_many_mark', 'quem_marca': 'who_marks',
 'competicao_conta': 'competition_counts', 'ordem_por_5f': 'order_by_5f', 'ordem_por_80': 'order_by_80', 'ordem_por_80_desc': 'order_by_80_desc', 'ordem_por_80_cresc': 'order_by_80_asc',
 'nota_do_jogador': 'player_rating',
 'esquecer_pessoa': 'forget_person', 'quadro_do_tecnico': 'manager_table', 'casa_do_setor_ocupada': 'sector_slot_taken', 'vaga_da_casa': 'slot_of_code', 'limpar_designados': 'clear_assigned', 'mascara_da_formacao': 'formation_mask',
 'casa_da_vaga': 'slot_code', 'criterio_da_competicao': 'competition_criterion',
 'tipo_nao_11': 'type_not_11', 'tipo_11_a_15': 'type_11_to_15', 'classe_da_selecao': 'national_side_class', 'ler_marca_da_pessoa': 'read_person_mark', 'zerar_marca_da_pessoa': 'clear_person_mark', 'registro_livre': 'free_record',
 'ligar_44_da_pessoa': 'set_person_44', 'byte_44_da_pessoa': 'person_byte_44', 'campo_3a_da_pessoa': 'person_field_3a', 'classe_pela_selecao': 'class_by_national_side', 'global_9b5984': 'global_9b5984',
 'competicao_da_pessoa': 'person_competition', 'byte_42_da_selecao': 'national_side_byte_42', 'byte_43_da_selecao': 'national_side_byte_43', 'selecao_ou_especial': 'national_or_special', 'nao_gosta_da_selecao': 'dislikes_national_side', 'copiar_data_e_marca': 'copy_date_and_mark',
 'atualizar_atributos': 'update_attributes',
 'desempenho_na_partida': 'match_performance',
 'forca_do_lado': 'side_strength',
 'evoluir_momento': 'evolve_moment', 'bola_no_ataque': 'ball_in_attack', 'partida_de_interesse': 'match_of_interest',
 'grupo_decidido': 'group_decided', 'narrar_classificacao': 'narrate_qualification',
 'procurar_jogada': 'look_for_play', 'jogadores_no_trajeto': 'players_on_path', 'percorrer_linha': 'walk_line',
 'interceptar': 'intercept',
 'avancar_linha': 'advance_row', 'comparar_com_fita': 'compare_with_tape', 'chamar_gravando': 'call_recording',
 'arrumar_saida': 'line_up_kickoff',
}
faltam = [n for n in rust if n not in EN]
if faltam:
    raise SystemExit('rotinas sem traducao no dicionario EN: %s' % faltam)

OUTROS = [  # (pt, en, endereco ou arquivo)
 ('aleatorio(n)', 'random(n)', '0x90cfc0'),
 ('Data()', 'Date()', '0x54a110'), ('Data(dia, ano)', 'Date(day, year)', '0x549ef0'),
 ('Data(dia_do_mes, mes, ano, dia_da_semana)', 'Date(day_of_month, month, year, weekday)', '0x549f70'),
 ('dia_da_semana()', 'weekday()', '0x54a3f0'), ('ajustar_semana(alvo, dest)', 'adjust_week(target, dest)', '0x54a2d0'),
 ('mais_semanas(n)', 'plus_weeks(n)', '0x54c770'),
]
AUX = [  # auxiliares do motor.rs
 ('jogador_do_bloco', 'block_player', 'jogador k do bloco de 20 do lado', 'player k of the side\'s 20-player block'),
 ('casa_da_grade', 'grid_square', 'casa (x, y) da grade do campo', 'pitch grid square (x, y)'),
 ('sortear', 'draw', 'chama o sorteio do jogo (0x90cfc0)', 'calls the game\'s random draw (0x90cfc0)'),
 ('ftol', 'ftol', 'truncamento float → inteiro (_ftol, 0x945640)', 'float → integer truncation (_ftol, 0x945640)'),
 ('f32_em / f64_em', 'f32_at / f64_at', 'constante float/double lida do .rdata', 'float/double constant read from .rdata'),
 ('byte', 'byte', 'byte com sinal num endereço + deslocamento', 'signed byte at address + offset'),
 ('lado', 'side', 'lado do jogador (byte 0x27)', 'player\'s side (byte 0x27)'),
 ('degrau', 'step', 'escada de multiplicadores pelo atributo', 'ladder of multipliers by attribute'),
 ('pow_do_jogo', 'game_pow', 'potência pela CRT do jogo (0x945ff0)', 'power via the game\'s CRT (0x945ff0)'),
 ('comparar_*', 'compare_*', 'roda original e Rust lado a lado e registra divergências', 'runs original and Rust side by side and records divergences'),
 ('comentar / porta_6d6f70', 'comment / gate_6d6f70', 'narração e a porta que grava as chamadas', 'commentary and the gate that records the calls'),
 ('PLACAR', 'SCOREBOARD', 'contador de comparações e divergências', 'counter of comparisons and divergences'),
]
MODULOS = [
 ('video.rs', 'video', 'DirectDraw próprio', 'own DirectDraw'), ('apresentacao.rs', 'presentation', 'quadro → janela, mouse', 'frame → window, mouse'),
 ('ganchos.rs', 'hooks (IAT)', 'ganchos na tabela de importação', 'import table hooks'),
 ('desvios.rs', 'detours', 'desvios no exe, pilha particular', 'detours in the exe, private stack'),
 ('simulacao.rs', 'simulation', 'simulação reproduzível', 'reproducible simulation'), ('alocador.rs', 'allocator', 'alocador fixo', 'fixed allocator'),
 ('aleatorio.rs', 'random', 'sorteio do jogo', 'the game\'s random generator'), ('data.rs', 'date', 'datas (Date.cpp)', 'dates (Date.cpp)'),
 ('motor.rs', 'engine', 'motor de partidas', 'match engine'), ('espiao.rs', 'spy', 'diagnóstico (CM_ESPIAO=1)', 'diagnostics (CM_ESPIAO=1)'),
 ('comparacao.rs', 'comparison', 'comparação lado a lado', 'side-by-side comparison'), ('registro.rs', 'log', 'registro em ddraw.log', 'logging to ddraw.log'),
 ('simular.py', 'simulate', 'carreira automática sem tela', 'headless automatic career'),
 ('laboratorio.py', 'laboratory', 'trava entre sessões e vigia de memória', 'cross-session lock and memory watchdog'),
 ('desmonta.py', 'disassemble', 'desmontagem', 'disassembly'), ('desvios.py', 'detours', 'onde dá para desviar', 'where a hook can go'),
 ('escuta_tela.py', 'screen listener', 'escuta o que a tela desenha', 'listens to what the screen draws'),
 ('patch_portatil.py', 'portable patch', 'gera o exe sem CD', 'generates the no-CD exe'),
 ('resolucao.py', 'resolution', 'gera o exe 1920×1080', 'generates the 1920×1080 exe'),
 ('cobertura.py', 'coverage', 'contador de chamadas por função', 'per-function call counter'),
 ('candidatos.py', 'candidates', 'próximas rotinas', 'next routines'), ('mutacoes.py', 'mutations', 'autoteste com mutações', 'self-test with mutations'),
 ('prologos.py', 'prologues', 'confere os bytes desviados', 'checks the hooked bytes'),
 ('textos.py / refs.py', 'strings / refs', 'textos do exe e quem os usa', 'exe strings and who uses them'),
]
PALAVRAS = [
 ('motor', 'engine (the match engine object)'), ('jogador', 'player'), ('pessoa', 'person (the player\'s database record)'),
 ('ficha', 'record / sheet'), ('lado', 'side (0 or 1)'), ('bloco', 'block (20 players of a side)'), ('casa', 'square (grid cell)'),
 ('grade', 'grid (pitch, 9 × 12)'), ('papel', 'role (tactical)'), ('lance', 'move / play'), ('bola', 'ball'), ('chute', 'shot'),
 ('passe', 'pass'), ('falta', 'foul'), ('cartão', 'card'), ('impedimento', 'offside'), ('goleiro', 'goalkeeper'), ('dono', 'owner (of the ball)'),
 ('posse', 'possession'), ('desgaste', 'fatigue'), ('condição', 'condition'), ('formação', 'formation'), ('narração / comentário', 'commentary'),
 ('sorteio', 'random draw'), ('leva', 'batch'), ('rotina', 'routine'), ('desvio', 'hook / detour'), ('autoteste', 'self-test'),
 ('mutação', 'mutation'), ('divergência', 'divergence'), ('impressão digital', 'fingerprint'), ('carreira', 'career'),
 ('ler / gravar', 'read / write'), ('escolher', 'choose'), ('somar', 'add'), ('zerar', 'clear (set to zero)'), ('registrar', 'record'),
 ('guardado', 'stored'), ('tabela', 'table'), ('faixa', 'band / range'), ('limite / limiar', 'limit / threshold'),
]


def gerar(en):
    L = []
    if en:
        L += ['*English version of [GLOSSARIO.md](GLOSSARIO.md). Both are kept in sync; the Portuguese file is the original.*', '',
              '# Glossary: Portuguese names in the code', '',
              'The code (Rust and Python) is written with Portuguese names. This glossary gives the English meaning of each name, so the code can be read alongside the English documentation. The names in the code do not change; this is only a reading aid. Addresses are VAs in `cm0102.exe` 3.9.68.', '']
    else:
        L += ['*[English version](GLOSSARIO.en.md).*', '',
              '# Glossário: nomes em português no código', '',
              'O código (Rust e Python) usa nomes em português. Este glossário dá o significado em inglês de cada nome, para ler o código junto com a documentação em inglês. Os nomes no código não mudam; é só uma ajuda de leitura. Endereços são VA no `cm0102.exe` 3.9.68.', '']
    L += ['## ' + ('Common words' if en else 'Palavras comuns'), '', '| Português | English |', '|---|---|']
    L += ['| %s | %s |' % (p, e) for p, e in PALAVRAS]
    L += ['', '## ' + ('Match engine routines (`rust/src/motor.rs`)' if en else 'Rotinas do motor de partidas (`rust/src/motor.rs`)'), '',
          '| %s | Português | English |' % ('Address' if en else 'Endereço'), '|---|---|---|']
    L += ['| %s | `%s` | %s |' % (a, n, EN[n]) for a, n in sorted(zip(pro, rust), key=lambda x: x[1])]
    L += ['', '## ' + ('Random generator and dates' if en else 'Sorteio e datas'), '',
          '| %s | Português | English |' % ('Address' if en else 'Endereço'), '|---|---|---|']
    L += ['| %s | `%s` | %s |' % (a, p, e) for p, e, a in OUTROS]
    L += ['', '## ' + ('Helpers in `motor.rs`' if en else 'Auxiliares do `motor.rs`'), '',
          '| Português | English | %s |' % ('What it is' if en else 'O que é'), '|---|---|---|']
    L += ['| `%s` | %s | %s |' % (p, e, de if en else dp) for p, e, dp, de in AUX]
    L += ['', '## ' + ('Modules and tools' if en else 'Módulos e ferramentas'), '',
          '| %s | English | %s |' % ('File' if en else 'Arquivo', 'What it is' if en else 'O que é'), '|---|---|---|']
    L += ['| `%s` | %s | %s |' % (p, e, de if en else dp) for p, e, dp, de in MODULOS]
    return '\n'.join(L) + '\n'


open(os.path.join(RAIZ, 'docs', 'GLOSSARIO.md'), 'w', encoding='utf-8').write(gerar(False))
open(os.path.join(RAIZ, 'docs', 'GLOSSARIO.en.md'), 'w', encoding='utf-8').write(gerar(True))
print(len(rust), 'rotinas')
