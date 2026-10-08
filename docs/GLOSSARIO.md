*[English version](GLOSSARIO.en.md).*

# Glossário: nomes em português no código

O código (Rust e Python) usa nomes em português. Este glossário dá o significado em inglês de cada nome, para ler o código junto com a documentação em inglês. Os nomes no código não mudam; é só uma ajuda de leitura. Endereços são VA no `cm0102.exe` 3.9.68.

## Palavras comuns

| Português | English |
|---|---|
| motor | engine (the match engine object) |
| jogador | player |
| pessoa | person (the player's database record) |
| ficha | record / sheet |
| lado | side (0 or 1) |
| bloco | block (20 players of a side) |
| casa | square (grid cell) |
| grade | grid (pitch, 9 × 12) |
| papel | role (tactical) |
| lance | move / play |
| bola | ball |
| chute | shot |
| passe | pass |
| falta | foul |
| cartão | card |
| impedimento | offside |
| goleiro | goalkeeper |
| dono | owner (of the ball) |
| posse | possession |
| desgaste | fatigue |
| condição | condition |
| formação | formation |
| narração / comentário | commentary |
| sorteio | random draw |
| leva | batch |
| rotina | routine |
| desvio | hook / detour |
| autoteste | self-test |
| mutação | mutation |
| divergência | divergence |
| impressão digital | fingerprint |
| carreira | career |
| ler / gravar | read / write |
| escolher | choose |
| somar | add |
| zerar | clear (set to zero) |
| registrar | record |
| guardado | stored |
| tabela | table |
| faixa | band / range |
| limite / limiar | limit / threshold |

## Rotinas do motor de partidas (`rust/src/motor.rs`)

| Endereço | Português | English |
|---|---|---|
| 0x7172d0 | `ajustar_17` | adjust_17 |
| 0x6cb9b0 | `alcanca` | reaches |
| 0x713d90 | `alcanca_o_ponto` | reaches_the_point |
| 0x6cb700 | `alem_da_linha` | beyond_the_line |
| 0x6bb950 | `aplicar_ao_lado` | apply_to_side |
| 0x6bb8d0 | `aplicar_ao_time` | apply_to_team |
| 0x716450 | `aplicar_fator` | apply_factor |
| 0x6f4920 | `apontar` | aim (set facing direction) |
| 0x6cd5f0 | `atras_do_extremo` | behind_the_extreme |
| 0x6f4aa0 | `atualizar_jogador` | update_player |
| 0x5b5a60 | `bit_da_casa` | square_bit |
| 0x6cd490 | `bola_fora` | ball_out |
| 0x5b57c0 | `byte_da_posicao` | position_byte |
| 0x6d5cd0 | `byte_do_codigo` | code_byte |
| 0x6cfda0 | `byte_do_lado` | side_byte |
| 0x6ce9c0 | `calcular_percentuais` | compute_percentages |
| 0x6bd5b0 | `caminho_livre` | clear_path |
| 0x6c41b0 | `caminho_livre_ate` | clear_path_to |
| 0x6f4600 | `campo_pela_posicao` | field_by_position |
| 0x6eab60 | `cartao_pela_gravidade` | card_by_severity |
| 0x6c2a40 | `casa_da_formacao` | formation_square |
| 0x713e90 | `classificar_lance` | classify_move |
| 0x70a8c0 | `codigo_de_bola_parada` | set_piece_code |
| 0x6cb650 | `codigo_especial` | special_code |
| 0x7002f0 | `comentario_a` | commentary_a |
| 0x70a860 | `comentario_b` | commentary_b |
| 0x70dfa0 | `condicao_jogador` | player_condition |
| 0x6bba80 | `conferir_lance` | check_move |
| 0x6f9f90 | `consegue_jogar` | can_play |
| 0x6cbbb0 | `contar_ativos` | count_active |
| 0x4cf5a0 | `contrato_de` | contract_of |
| 0x6e9540 | `decidir_falta` | decide_foul |
| 0x700420 | `defesa_do_goleiro` | goalkeeper_save |
| 0x6f56c0 | `desgastar` | tire (fatigue) |
| 0x715f30 | `diferenca_do_jogador` | difference_of_player |
| 0x6c4130 | `diferenca_jogador` | player_difference |
| 0x6c4180 | `diferenca_lado` | side_difference |
| 0x6f4a30 | `direcao` | direction |
| 0x6fa1e0 | `distancia_ao_gol` | distance_to_goal |
| 0x6f09b0 | `distancia_ao_ponto` | distance_to_point |
| 0x6cc4e0 | `distancia_entre` | distance_between |
| 0x70d690 | `dominio_do_lance` | control_of_move |
| 0x5c6710 | `e_amistoso` | is_friendly |
| 0x7103c0 | `e_o_guardado` | is_the_stored |
| 0x5409b0 | `e_segunda_selecao` | is_second_national_team |
| 0x53b900 | `e_selecao` | is_national_team |
| 0x70d630 | `em_perigo` | in_danger |
| 0x6cdd00 | `entrar_em_campo` | take_the_field |
| 0x6bce80 | `escolher_casa` | choose_square |
| 0x6d0f80 | `escolher_destaque` | choose_standout |
| 0x6c2310 | `escolher_pela_condicao` | choose_by_condition |
| 0x6c2750 | `escolher_por_papel` | choose_by_role |
| 0x6c2880 | `escolher_por_papel_b` | choose_by_role_b |
| 0x6c2050 | `escolher_por_papel_c` | choose_by_role_c |
| 0x6c23f0 | `escolher_por_papel_e` | choose_by_role_e |
| 0x6cbad0 | `espaco_a_frente` | space_ahead |
| 0x6f4a70 | `estado_do_adversario` | opponent_state |
| 0x6f4a40 | `estado_do_lado` | side_state |
| 0x6cf750 | `estatistica_de_posse` | possession_statistic |
| 0x6cb960 | `faixa` | band |
| 0x6f5b20 | `faixa_jogador` | player_band |
| 0x6cb910 | `fora_do_meio` | off_centre |
| 0x6c0c60 | `forca_do_time` | team_strength |
| 0x547810 | `gosta_de` | likes |
| 0x70a890 | `gravar_19c` | write_19c |
| 0x7160e0 | `gravar_2b` | write_2b |
| 0x700360 | `gravar_8ea7` | write_8ea7 |
| 0x700370 | `gravar_8ea8` | write_8ea8 |
| 0x700350 | `gravar_8ea9` | write_8ea9 |
| 0x700390 | `gravar_8eaa` | write_8eaa |
| 0x7003a0 | `gravar_8ed8` | write_8ed8 |
| 0x6cd660 | `guardar_extremos` | store_extremes |
| 0x6e9470 | `iniciar_rigor` | init_strictness |
| 0x6c2fe0 | `jogador_no_bloco` | player_in_block |
| 0x6e1cd0 | `jogador_pela_ficha` | player_by_record |
| 0x6cbc00 | `jogadores_na_linha` | players_in_line |
| 0x6bba50 | `lado_do_lance` | side_of_move |
| 0x6bab90 | `lance_valendo` | move_counts |
| 0x717400 | `ler_1bc` | read_1bc |
| 0x700320 | `ler_8ea7` | read_8ea7 |
| 0x700330 | `ler_8ea8` | read_8ea8 |
| 0x700340 | `ler_8ea9` | read_8ea9 |
| 0x700380 | `ler_8eaa` | read_8eaa |
| 0x700400 | `ler_8eb2` | read_8eb2 |
| 0x700410 | `ler_8f7a` | read_8f7a |
| 0x6cb930 | `limite` | limit |
| 0x70df40 | `limite_jogador` | player_limit |
| 0x6bba10 | `limpar_lance` | clear_move |
| 0x6c3aa0 | `linhas_defensivas` | defensive_lines |
| 0x6cb780 | `mais_perto_do_gol` | closest_to_goal |
| 0x6c30e0 | `mais_perto_no_lado` | closest_on_side |
| 0x6c3010 | `mais_proximo_do_ponto` | nearest_to_point |
| 0x6ce110 | `manter_posicao_um` | hold_position_one |
| 0x7163f0 | `marca_800000` | mark_800000 |
| 0x6d4330 | `marcador_direto` | direct_marker |
| 0x5b5910 | `marcar_posicao` | mark_position |
| 0x5b5270 | `marcas_da_posicao` | position_marks |
| 0x7163c0 | `marcas_do_jogador_rot` | player_marks_rot |
| 0x6c3950 | `menor_distancia` | smallest_distance |
| 0x6d4250 | `momento_marcado` | marked_moment |
| 0x6cfd40 | `motor_cartao_pela_gravidade` | engine_card_by_severity |
| 0x6cfca0 | `motor_decidir_falta` | engine_decide_foul |
| 0x6cfd70 | `motor_procurar_impedimento` | engine_look_for_offside |
| 0x6cfd10 | `motor_punir_falta` | engine_punish_foul |
| 0x6cfcd0 | `motor_punir_lance` | engine_punish_move |
| 0x6f46b0 | `mover_para` | move_to |
| 0x6f4650 | `na_area` | in_the_box |
| 0x714c60 | `na_lista` | in_list |
| 0x547840 | `nao_gosta_de` | dislikes |
| 0x715f50 | `nota_ajustada` | adjusted_rating |
| 0x717c70 | `papel_880` | role_880 |
| 0x716310 | `papel_na_defesa` | defending_role |
| 0x716270 | `papel_no_ataque` | attacking_role |
| 0x6f0630 | `pegar_bola` | take_ball |
| 0x70df70 | `perto_do_gol` | near_goal |
| 0x70def0 | `perto_do_proprio_gol` | near_own_goal |
| 0x6ce310 | `placar_agregado` | aggregate_score |
| 0x6bce20 | `pode_escolher` | can_choose |
| 0x6c51e0 | `pontape_inicial` | kick_off |
| 0x6cb6c0 | `por_diferenca` | by_difference |
| 0x6c2be0 | `posicionar` | position (place players) |
| 0x6d1770 | `posicoes_finas` | fine_positions |
| 0x6bd6b0 | `pressao_na_area` | pressure_in_box |
| 0x6c3a00 | `pressao_no_ponto` | pressure_at_point |
| 0x6ea760 | `procurar_impedimento` | look_for_offside |
| 0x6cd330 | `procurar_no_lado` | search_on_side |
| 0x6e9630 | `punir_falta` | punish_foul |
| 0x6ea4e0 | `punir_lance` | punish_move |
| 0x6bb9a0 | `registrar_lance` | record_move |
| 0x7003b0 | `registrar_posse` | record_possession |
| 0x714ca0 | `remover_da_lista` | remove_from_list |
| 0x6c06b0 | `repartir_a` | split_a |
| 0x6c07a0 | `repartir_b` | split_b |
| 0x6c0a70 | `repartir_c` | split_c |
| 0x6c0890 | `repartir_d` | split_d |
| 0x6c0980 | `repartir_e` | split_e |
| 0x6c0b70 | `repartir_f` | split_f |
| 0x6ce460 | `resumo_da_partida` | match_summary |
| 0x6bbe40 | `rolar_bola` | roll_ball |
| 0x6d4280 | `sem_partida` | no_match |
| 0x5b73a0 | `separar_mascara` | split_mask |
| 0x6f0970 | `soltar_bola` | release_ball |
| 0x7160d0 | `somar_2b` | add_2b |
| 0x6cf710 | `somar_disposicao` | add_disposition |
| 0x6f44a0 | `sortear_momento` | draw_moment |
| 0x6c38c0 | `tabela_a` | table_a |
| 0x6c3920 | `tabela_a_1a` | table_a_1a |
| 0x717430 | `tabela_a_1a_do_jogador` | player_table_a_1a |
| 0x6c38f0 | `tabela_b` | table_b |
| 0x717450 | `tabela_b_do_jogador` | player_table_b |
| 0x717410 | `tabela_do_jogador` | player_table |
| 0x5fd210 | `tecnico_9a01` | manager_9a01 |
| 0x5fd480 | `tecnico_9a05` | manager_9a05 |
| 0x6b59a0 | `tem_jogo` | has_match |
| 0x7164b0 | `teste_de_condicao` | condition_test |
| 0x6cdd50 | `tirar_de_campo` | take_off_pitch |
| 0x6ccb60 | `tracar_lance` | trace_move |
| 0x4b0850 | `unidades` | units_digit |
| 0x700bd0 | `valor_do_chute` | shot_value |
| 0x7165b0 | `valor_pela_condicao` | value_by_condition |
| 0x6b7cf0 | `venceu_o_jogo` | won_the_match |
| 0x6fa140 | `virado_para` | facing |
| 0x70df20 | `x_fora_do_meio` | x_off_centre |
| 0x6ce0f0 | `zerar_guardado` | clear_stored |

## Sorteio e datas

| Endereço | Português | English |
|---|---|---|
| 0x90cfc0 | `aleatorio(n)` | random(n) |
| 0x54a110 | `Data()` | Date() |
| 0x549ef0 | `Data(dia, ano)` | Date(day, year) |
| 0x549f70 | `Data(dia_do_mes, mes, ano, dia_da_semana)` | Date(day_of_month, month, year, weekday) |
| 0x54a3f0 | `dia_da_semana()` | weekday() |
| 0x54a2d0 | `ajustar_semana(alvo, dest)` | adjust_week(target, dest) |
| 0x54c770 | `mais_semanas(n)` | plus_weeks(n) |

## Auxiliares do `motor.rs`

| Português | English | O que é |
|---|---|---|
| `jogador_do_bloco` | block_player | jogador k do bloco de 20 do lado |
| `casa_da_grade` | grid_square | casa (x, y) da grade do campo |
| `sortear` | draw | chama o sorteio do jogo (0x90cfc0) |
| `ftol` | ftol | truncamento float → inteiro (_ftol, 0x945640) |
| `f32_em / f64_em` | f32_at / f64_at | constante float/double lida do .rdata |
| `byte` | byte | byte com sinal num endereço + deslocamento |
| `lado` | side | lado do jogador (byte 0x27) |
| `degrau` | step | escada de multiplicadores pelo atributo |
| `pow_do_jogo` | game_pow | potência pela CRT do jogo (0x945ff0) |
| `comparar_*` | compare_* | roda original e Rust lado a lado e registra divergências |
| `comentar / porta_6d6f70` | comment / gate_6d6f70 | narração e a porta que grava as chamadas |
| `PLACAR` | SCOREBOARD | contador de comparações e divergências |

## Módulos e ferramentas

| Arquivo | English | O que é |
|---|---|---|
| `video.rs` | video | DirectDraw próprio |
| `apresentacao.rs` | presentation | quadro → janela, mouse |
| `ganchos.rs` | hooks (IAT) | ganchos na tabela de importação |
| `desvios.rs` | detours | desvios no exe, pilha particular |
| `simulacao.rs` | simulation | simulação reproduzível |
| `alocador.rs` | allocator | alocador fixo |
| `aleatorio.rs` | random | sorteio do jogo |
| `data.rs` | date | datas (Date.cpp) |
| `motor.rs` | engine | motor de partidas |
| `espiao.rs` | spy | diagnóstico (CM_ESPIAO=1) |
| `comparacao.rs` | comparison | comparação lado a lado |
| `registro.rs` | log | registro em ddraw.log |
| `simular.py` | simulate | carreira automática sem tela |
| `laboratorio.py` | laboratory | trava entre sessões e vigia de memória |
| `desmonta.py` | disassemble | desmontagem |
| `desvios.py` | detours | onde dá para desviar |
| `escuta_tela.py` | screen listener | escuta o que a tela desenha |
| `patch_portatil.py` | portable patch | gera o exe sem CD |
| `resolucao.py` | resolution | gera o exe 1920×1080 |
| `cobertura.py` | coverage | contador de chamadas por função |
| `candidatos.py` | candidates | próximas rotinas |
| `mutacoes.py` | mutations | autoteste com mutações |
| `prologos.py` | prologues | confere os bytes desviados |
| `textos.py / refs.py` | strings / refs | textos do exe e quem os usa |
