*English version of [GLOSSARIO.md](GLOSSARIO.md). Both are kept in sync; the Portuguese file is the original.*

# Glossary: Portuguese names in the code

The code (Rust and Python) is written with Portuguese names. This glossary gives the English meaning of each name, so the code can be read alongside the English documentation. The names in the code do not change; this is only a reading aid. Addresses are VAs in `cm0102.exe` 3.9.68.

## Common words

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

## Match engine routines (`rust/src/motor.rs`)

| Address | Português | English |
|---|---|---|
| 0x54d580 | `abrir_arquivo_do_banco` | db_files_open_for_read |
| 0x71b4a0 | `abrir_detalhes_da_partida` | open_match_details |
| 0x725f70 | `abrir_escalacao_da_partida` | match_screens_open_team_selection |
| 0x717cc0 | `abrir_jogos_do_dia` | open_day_fixtures_screen |
| 0x9326e0 | `abrir_no_conteiner` | open_in_container |
| 0x718aa0 | `abrir_tela_da_partida` | open_match_screen |
| 0x727f00 | `abrir_tela_de_espera` | open_waiting_screen |
| 0x71b1b0 | `abrir_ultimos_placares` | open_latest_scores |
| 0x547e70 | `acrescentar_clube_rejeitado` | staff_add_disliked_club |
| 0x547a10 | `acrescentar_pessoa_favorita` | staff_add_favoured_staff |
| 0x7172d0 | `ajustar_17` | adjust_17 |
| 0x544710 | `ajustar_moral_da_pessoa` | staff_adjust_morale |
| 0x6cb9b0 | `alcanca` | reaches |
| 0x713d90 | `alcanca_o_ponto` | reaches_the_point |
| 0x6cb700 | `alem_da_linha` | beyond_the_line |
| 0x6b5ce0 | `andar_partida_do_dia` | step_day_match |
| 0x540340 | `anos_entre_datas_em_texto` | database_format_years_between_dates |
| 0x76c060 | `apagar_repetidos_das_convocacoes` | national_teams_remove_duplicate_squad_entries |
| 0x6bb950 | `aplicar_ao_lado` | apply_to_side |
| 0x6bb8d0 | `aplicar_ao_time` | apply_to_team |
| 0x6bac40 | `aplicar_falta` | apply_foul |
| 0x716450 | `aplicar_fator` | apply_factor |
| 0x6d0b60 | `aplicar_trocas` | apply_substitutions |
| 0x6f4920 | `apontar` | aim (set facing direction) |
| 0x535930 | `arquivo_do_idioma_existe` | database_language_file_exists |
| 0x4b0870 | `arredondar_abaixo_de_1000` | round_below_1000 |
| 0x6c4820 | `arrumar_bola_parada` | line_up_set_piece |
| 0x6c4ec0 | `arrumar_saida` | line_up_kickoff |
| 0x6cd5f0 | `atras_do_extremo` | behind_the_extreme |
| 0x6d5950 | `atras_no_agregado` | behind_on_aggregate |
| 0x71dd70 | `atribuir_bloco_do_time` | match_team_block_assign |
| 0x71df00 | `atribuir_linha_do_jogador` | match_player_row_assign |
| 0x543540 | `atributo_calculado_do_jogador` | staff_compute_player_attribute |
| 0x543fd0 | `atributo_calculado_do_nao_jogador` | staff_compute_non_player_attribute |
| 0x5431e0 | `atributo_da_pessoa` | staff_get_player_attribute |
| 0x543f30 | `atributo_do_nao_jogador` | staff_get_non_player_attribute |
| 0x6ec010 | `atualizar_atributos` | update_attributes |
| 0x71d9e0 | `atualizar_barra_da_posse` | refresh_possession_bar |
| 0x6869a0 | `atualizar_confronto_direto` | League::update_head_to_head |
| 0x6f4aa0 | `atualizar_jogador` | update_player |
| 0x724520 | `atualizar_ultimos_placares` | refresh_latest_scores_rows |
| 0x53cb80 | `auxiliar_do_clube` | database_get_club_assistant |
| 0x6e1cf0 | `avaliar_jogador` | rate_player |
| 0x6f5b80 | `avancar_linha` | advance_row |
| 0x729bd0 | `balde_do_registro` | match_stats_get_record_bucket |
| 0x8676f0 | `bandeiras_do_contrato` | contract_flags |
| 0x897990 | `batedor_guardado` | stored_taker |
| 0x5b5a60 | `bit_da_casa` | square_bit |
| 0x7a7eb0 | `bits_do_registro` | record_bits |
| 0x6cd490 | `bola_fora` | ball_out |
| 0x6d58f0 | `bola_na_area` | ball_goal_end |
| 0x714d00 | `bola_no_ataque` | ball_in_attack |
| 0x933080 | `buscar_no_conteiner` | seek_in_container |
| 0x7754c0 | `byte_42_da_selecao` | national_side_byte_42 |
| 0x775520 | `byte_43_da_selecao` | national_side_byte_43 |
| 0x76e010 | `byte_44_da_pessoa` | person_byte_44 |
| 0x5b57c0 | `byte_da_posicao` | position_byte |
| 0x6d5cd0 | `byte_do_codigo` | code_byte |
| 0x6cfda0 | `byte_do_lado` | side_byte |
| 0x898040 | `byte_tatico` | tactics_byte |
| 0x6ce9c0 | `calcular_percentuais` | compute_percentages |
| 0x6bd5b0 | `caminho_livre` | clear_path |
| 0x6c41b0 | `caminho_livre_ate` | clear_path_to |
| 0x76e040 | `campo_3a_da_pessoa` | person_field_3a |
| 0x6f4600 | `campo_pela_posicao` | field_by_position |
| 0x524d10 | `carregar_banco_do_save` | load_database_from_stream |
| 0x72aa80 | `carregar_estatisticas_da_partida` | Match_Stats::load |
| 0x82a9c0 | `carregar_jogo` | load_game_data |
| 0x839760 | `carregar_regras_fifa` | load_fifa_transfer_rules |
| 0x6eab60 | `cartao_pela_gravidade` | card_by_severity |
| 0x6c2a40 | `casa_da_formacao` | formation_square |
| 0x5b2ae0 | `casa_da_vaga` | slot_code |
| 0x5b2e80 | `casa_do_setor_ocupada` | sector_slot_taken |
| 0x5b3c70 | `casa_guardada` | stored_cell |
| 0x5b7e60 | `casa_no_campo` | pitch_cell |
| 0x6146b0 | `casar_chave_de_clube` | match_club_key_id |
| 0x60efd0 | `casar_chave_de_competicao_de_clube` | match_club_comp_key_id |
| 0x6115e0 | `casar_chave_de_competicao_de_pessoa` | match_staff_comp_key_id |
| 0x60d810 | `casar_chave_de_nacao` | match_nation_key_id |
| 0x933a00 | `casar_nome_com_padrao` | zipdir_match_filename_pattern |
| 0x785670 | `categoria_da_noticia` | NewsItem::get_category |
| 0x4ca470 | `categoria_do_contrato` | contract_category |
| 0x544410 | `chance_de_conhecer` | chance_to_know |
| 0x519c90 | `chave_do_jogo_na_copa` | Cup::find_tree_slot_for_fixture |
| 0x702050 | `chutar` | shoot |
| 0x6f6c00 | `chute` | shot |
| 0x70a910 | `chute_a_gol` | shot_on_goal |
| 0x76d780 | `classe_da_selecao` | national_side_class |
| 0x76e070 | `classe_pela_selecao` | class_by_national_side |
| 0x713e90 | `classificar_lance` | classify_move |
| 0x547870 | `clube_acrescentar_pessoa_favorita` | club_add_favoured_staff |
| 0x547940 | `clube_acrescentar_pessoa_rejeitada` | club_add_disliked_staff |
| 0x53d7f0 | `clube_basco` | database_is_basque_club |
| 0x7875e0 | `clube_de_tecnico_humano` | news_club_has_human_manager |
| 0x8c5450 | `clube_do_contrato_acertado` | TRANSFER_MANAGER::get_agreed_contract_club |
| 0x6b79a0 | `clube_do_pos_jogo` | post_match_club |
| 0x549e50 | `clube_em_nacao_grande` | club_is_in_major_nation |
| 0x547770 | `clube_em_temporada` | club_is_in_season |
| 0x76d940 | `clube_especial` | special_club |
| 0x5487b0 | `clube_fora_da_argentina` | club_nation_is_not_argentina |
| 0x5487d0 | `clube_fora_de_argentina_franca_croacia` | club_nation_not_argentina_france_croatia |
| 0x600a50 | `clube_humano` | human_club |
| 0x48db80 | `clube_na_competicao` | club_in_competition |
| 0x540a50 | `clube_parceiro` | partner_club |
| 0x544820 | `clube_pode_ser_dirigido` | club_can_be_managed |
| 0x542740 | `clube_pode_ter_reservas` | database_club_has_reserve_squad |
| 0x549770 | `clube_tem_atacante_rapido` | club_has_fast_striker |
| 0x5492c0 | `clube_tem_atacantes` | club_has_enough_strikers |
| 0x549100 | `clube_tem_defensores` | club_has_enough_defenders |
| 0x5493c0 | `clube_tem_esforcados` | club_has_enough_hard_workers |
| 0x549470 | `clube_tem_goleiro` | club_has_fit_goalkeeper |
| 0x549200 | `clube_tem_meias` | club_has_enough_midfielders |
| 0x549940 | `clube_tem_meias_criativos` | club_has_two_creative_midfielders |
| 0x549850 | `clube_tem_meias_que_roubam` | club_has_ball_winning_midfielders |
| 0x53b9a0 | `clube_tem_vaga_para_cargo` | database_club_has_room_for_job |
| 0x547bd0 | `clube_tirar_pessoa_favorita` | club_remove_favoured_staff |
| 0x83bd00 | `codigo_da_posicao` | position_code |
| 0x70a8c0 | `codigo_de_bola_parada` | set_piece_code |
| 0x6cb650 | `codigo_especial` | special_code |
| 0x6e0d00 | `colocar_na_vaga` | put_in_place |
| 0x6d60b0 | `combinar_com_o_anterior` | combine_with_previous |
| 0x7002f0 | `comentario_a` | commentary_a |
| 0x70a860 | `comentario_b` | commentary_b |
| 0x6d4b70 | `comentario_de_contexto` | context_commentary |
| 0x6d5a70 | `companheiro_livre` | free_teammate |
| 0x4b0700 | `competicao_conta` | competition_counts |
| 0x7751e0 | `competicao_da_pessoa` | person_competition |
| 0x684790 | `competicao_marcada` | competition_flagged |
| 0x70dfa0 | `condicao_jogador` | player_condition |
| 0x7109f0 | `conduzir` | dribble |
| 0x6bba80 | `conferir_lance` | check_move |
| 0x6b5260 | `confirmar_partida_em_rede` | confirm_network_match |
| 0x5b09d0 | `conhecer_pessoa` | know_person |
| 0x6f9f90 | `consegue_jogar` | can_play |
| 0x6e9460 | `construir_arbitro` | construct_referee |
| 0x6d5bf0 | `construir_fila_da_narracao` | construct_commentary_queue |
| 0x6eada0 | `construir_jogador_da_partida` | construct_match_player |
| 0x6b7e50 | `construir_motor` | build_engine |
| 0x6cbbb0 | `contar_ativos` | count_active |
| 0x548150 | `contar_elenco_com_mascara` | club_count_squad_with_position_mask |
| 0x53c590 | `contar_elenco_do_clube` | database_count_club_squad |
| 0x547ce0 | `contar_favoritos_no_clube` | staff_count_favoured_staff_at_club |
| 0x728e90 | `contar_placar_ao_vivo` | match_screens_tally_live_score |
| 0x547c90 | `contar_rejeitados_no_clube` | staff_count_disliked_staff_at_club |
| 0x53c8d0 | `contar_treinadores_do_clube` | database_count_club_coaches |
| 0x4cf5f0 | `contrato_com_clube` | contract_with_club |
| 0x4cf5a0 | `contrato_de` | contract_of |
| 0x7a8780 | `copiar_aposentadoria_pendente` | retire_record_assign |
| 0x777590 | `copiar_data_e_marca` | copy_date_and_mark |
| 0x448490 | `copiar_recordista` | record_holder_assign |
| 0x448110 | `copiar_registro_de_partida` | match_record_assign |
| 0x448170 | `copiar_registro_de_sequencia` | run_record_assign |
| 0x839590 | `cor_do_titulo` | get_heading_colour |
| 0x53b7c0 | `cores_da_competicao` | database_get_comp_colours |
| 0x53b640 | `cores_do_uniforme_do_clube` | database_get_club_kit_colours |
| 0x540490 | `cores_se_confundem` | database_colours_clash |
| 0x541e60 | `cortar_elencos_de_a_e_b` | database_trim_senior_and_b_team_squads |
| 0x5267c0 | `cortar_tabela_de_clubes` | database_trim_club_table |
| 0x527f60 | `crescer_historico_de_competicoes` | database_grow_comp_history_table |
| 0x54d5b0 | `criar_arquivo_do_banco` | db_files_open_for_write |
| 0x827110 | `criar_jogo_pela_configuracao` | create_game_from_setup_file |
| 0x866f60 | `criterio_da_comissao` | staff_criterion |
| 0x4bff70 | `criterio_da_competicao` | competition_criterion |
| 0x6e8910 | `cumprir_regra_do_elenco` | apply_squad_rule |
| 0x68a160 | `data_da_rodada_da_liga` | league_round_set_date |
| 0x521eb0 | `data_do_jogo_da_rodada` | cup_set_round_match_date |
| 0x521e60 | `data_do_sorteio_da_rodada` | cup_set_round_draw_date |
| 0x6e9540 | `decidir_falta` | decide_foul |
| 0x7103e0 | `decidir_jogador` | decide_player |
| 0x713fc0 | `decidir_sem_bola` | decide_off_ball |
| 0x6e9820 | `decisao_da_falta` | foul_decision |
| 0x70d430 | `defender` | save_shot |
| 0x700420 | `defesa_do_goleiro` | goalkeeper_save |
| 0x6c87e0 | `desafio_adversario` | opposition_challenge |
| 0x933320 | `desempacotar_do_conteiner` | zextract_matching_files |
| 0x6eecb0 | `desempenho_na_partida` | match_performance |
| 0x402e70 | `desenhar_area` | Area::draw |
| 0x7260f0 | `desenhar_conferencia_da_escalacao` | match_screens_draw_team_selection |
| 0x70c040 | `desfecho_do_chute` | shot_outcome |
| 0x6f56c0 | `desgastar` | tire (fatigue) |
| 0x62c2f0 | `desgaste_diario` | daily_wear |
| 0x5b58e0 | `designado_da_posicao` | designated_for_position |
| 0x8c5430 | `desmarcar_contrato_acertado` | clear_agreed_contract_flag |
| 0x84aeb0 | `desmarcar_lances_pelo_jogador` | Shortlist_Manager::clear_bid_markers_for_player |
| 0x7293c0 | `desmontar_estatisticas_da_partida` | Match_Stats::~Match_Stats |
| 0x717470 | `destaque_do_jogador` | player_highlight |
| 0x6ebd60 | `destruir_jogador_da_partida` | destroy_match_player |
| 0x6b97a0 | `destruir_motor` | destroy_engine |
| 0x6cd770 | `desviar_bola` | deflect_ball |
| 0x521f10 | `dia_da_rodada_da_copa` | cup_set_round_match_day |
| 0x715f30 | `diferenca_do_jogador` | difference_of_player |
| 0x6c4130 | `diferenca_jogador` | player_difference |
| 0x6c4180 | `diferenca_lado` | side_difference |
| 0x6f4a30 | `direcao` | direction |
| 0x556240 | `disciplina_do_dia` | discipline_daily_update |
| 0x55cb20 | `disciplina_gravar_vaga` | discipline_set_slot |
| 0x556210 | `disciplina_temporada_nova` | discipline_new_season |
| 0x6c9cd0 | `disputa` | contest |
| 0x6fa1e0 | `distancia_ao_gol` | distance_to_goal |
| 0x6f09b0 | `distancia_ao_ponto` | distance_to_point |
| 0x6cc4e0 | `distancia_entre` | distance_between |
| 0x70ec80 | `dividida` | tackle |
| 0x70d690 | `dominio_do_lance` | control_of_move |
| 0x5c6710 | `e_amistoso` | is_friendly |
| 0x5c2990 | `e_competicao_do_amistoso` | Friendly::is_own_comp_record |
| 0x7103c0 | `e_o_guardado` | is_the_stored |
| 0x5409b0 | `e_segunda_selecao` | is_second_national_team |
| 0x53b900 | `e_selecao` | is_national_team |
| 0x70d630 | `em_perigo` | in_danger |
| 0x933610 | `empacotar_no_conteiner` | zadd_matching_files |
| 0x6b4190 | `encerrar_dia` | end_day |
| 0x8c16b0 | `enfileirar_transferencia_livre` | TRANSFER_MANAGER::queue_free_transfer |
| 0x7a7ed0 | `entrada_da_aposentadoria` | PLAYER_REGEN::find_retirement_entry |
| 0x849ce0 | `entrada_da_lista` | shortlist_entry |
| 0x729500 | `entrada_do_jogo_bate` | Match_Stats::fixture_entry_matches |
| 0x5d5ac0 | `entrada_do_treino` | training_manager_get_player_entry |
| 0x6cdd00 | `entrar_em_campo` | take_the_field |
| 0x5fed50 | `escalacao_guardada` | HUMAN_MANAGER::get_saved_lineup |
| 0x6e2000 | `escalar` | pick |
| 0x6dbc50 | `escalar_automatico` | select_automatically |
| 0x6db500 | `escalar_lado` | select_side |
| 0x6c4470 | `escanteio` | corner |
| 0x6bce80 | `escolher_casa` | choose_square |
| 0x6ceb90 | `escolher_cobrador` | choose_taker |
| 0x72b810 | `escolher_comentarista` | Media::pick_former_international_pundit |
| 0x6c6a30 | `escolher_companheiro` | choose_teammate |
| 0x548da0 | `escolher_companheiro_favorito` | staff_pick_favoured_team_mate |
| 0x6d0f80 | `escolher_destaque` | choose_standout |
| 0x6e0230 | `escolher_formacao` | choose_formation |
| 0x6b42a0 | `escolher_partida_do_dia` | pick_day_match |
| 0x6c2310 | `escolher_pela_condicao` | choose_by_condition |
| 0x6c2750 | `escolher_por_papel` | choose_by_role |
| 0x6c2880 | `escolher_por_papel_b` | choose_by_role_b |
| 0x6c2050 | `escolher_por_papel_c` | choose_by_role_c |
| 0x6c23f0 | `escolher_por_papel_e` | choose_by_role_e |
| 0x62d140 | `escolher_tipo_de_lesao` | Injury::choose_injury_type |
| 0x5408c0 | `escopo_da_reputacao` | database_get_reputation_scope |
| 0x547f90 | `esmaecer_relacoes` | staff_fade_staff_relationships |
| 0x6cbad0 | `espaco_a_frente` | space_ahead |
| 0x727fb0 | `esperar_os_outros_jogadores` | match_screens_handle_waiting_screen |
| 0x5afd00 | `esquecer_pessoa` | forget_person |
| 0x6f4a70 | `estado_do_adversario` | opponent_state |
| 0x6f4a40 | `estado_do_lado` | side_state |
| 0x7c47b0 | `estatistica_da_liga_do_jogador` | PlayerStats::get_club_league_stat |
| 0x6cf750 | `estatistica_de_posse` | possession_statistic |
| 0x7c2150 | `estatistica_do_jogador` | player_statistic |
| 0x6d5d00 | `evento_na_frente` | event_at_front |
| 0x6b74a0 | `eventos_do_pos_jogo` | post_match_events |
| 0x6d26b0 | `evoluir_jogador` | develop_player |
| 0x6f55c0 | `evoluir_momento` | evolve_moment |
| 0x6fad40 | `executar_passe` | execute_pass |
| 0x6cb960 | `faixa` | band |
| 0x549a30 | `faixa_de_forca_do_elenco` | club_squad_strength_band |
| 0x5af2f0 | `faixa_do_horario` | FIXTURE_MANAGER::get_time_slice |
| 0x6f5b20 | `faixa_jogador` | player_band |
| 0x6cfc60 | `falta_no_motor` | foul_in_engine |
| 0x5494d0 | `faltam_atacantes_pelo_alto` | club_lacks_aerial_strikers |
| 0x5495b0 | `faltam_zagueiros_pelo_alto` | club_lacks_aerial_defenders |
| 0x549690 | `faltam_zagueiros_rapidos` | club_lacks_fast_defenders |
| 0x540520 | `familia_da_cor` | database_get_colour_family |
| 0x547790 | `favorito_de` | favourite_of |
| 0x6d1cc0 | `fechamento` | match_wrap_up |
| 0x932b00 | `fechar_conteiner` | zclose |
| 0x71c170 | `fechar_detalhes_da_partida` | close_match_details |
| 0x6b5f00 | `fechar_partida_do_dia` | close_day_match |
| 0x8c6db0 | `fechar_propostas_da_oferta` | transfer_manager_close_offer_contract_offers |
| 0x71b120 | `fechar_tela_da_partida` | close_match_screen |
| 0x4cf6d0 | `ficha_de_contrato` | contract_record |
| 0x729e30 | `ficha_do_registro` | Match_Stats::find_record_fixture |
| 0x839290 | `filtro_do_inicio_rapido` | check_quick_start_file |
| 0x838e10 | `filtro_do_jogo_salvo` | check_saved_game_file |
| 0x6cb910 | `fora_do_meio` | off_centre |
| 0x6bbb60 | `forca_do_lado` | side_strength |
| 0x6c0c60 | `forca_do_time` | team_strength |
| 0x545a60 | `forma_do_adjetivo` | get_adjective_form_index |
| 0x53dba0 | `formatar_nome_da_pessoa` | database_format_staff_name |
| 0x527240 | `gerar_comissao_extra` | database_generate_extra_non_playing_staff |
| 0x68a030 | `girar_rodada` | rotate_round |
| 0x535920 | `global_9b5984` | global_9b5984 |
| 0x547810 | `gosta_de` | likes |
| 0x547d60 | `gosta_do_clube` | likes_club |
| 0x62ece0 | `grau_da_lesao` | injury_grade |
| 0x70a890 | `gravar_19c` | write_19c |
| 0x7160e0 | `gravar_2b` | write_2b |
| 0x700360 | `gravar_8ea7` | write_8ea7 |
| 0x700370 | `gravar_8ea8` | write_8ea8 |
| 0x700350 | `gravar_8ea9` | write_8ea9 |
| 0x700390 | `gravar_8eaa` | write_8eaa |
| 0x7003a0 | `gravar_8ed8` | write_8ed8 |
| 0x5506b0 | `gravar_arbitros` | db_files_write_officials |
| 0x524d30 | `gravar_banco_no_save` | save_database_for_game |
| 0x62c660 | `gravar_carga_de_treino` | Injury::set_training_load |
| 0x5b5290 | `gravar_casa` | set_square |
| 0x54d9a0 | `gravar_cidades` | db_files_write_cities |
| 0x54e5d0 | `gravar_clubes` | db_files_write_clubs |
| 0x5501d0 | `gravar_competicoes` | db_files_write_comps |
| 0x550470 | `gravar_competicoes_de_pessoal` | db_files_write_staff_comps |
| 0x54d6e0 | `gravar_continentes` | db_files_write_continents |
| 0x785790 | `gravar_dado_da_noticia` | set_news_field |
| 0x5fdd30 | `gravar_dado_do_humano` | set_human_field |
| 0x54daf0 | `gravar_estadios` | db_files_write_stadiums |
| 0x6d98b0 | `gravar_eventos` | record_events |
| 0x8967d0 | `gravar_formacao_tatica` | set_tactic_formation |
| 0x54fe50 | `gravar_historico_de_pessoas` | db_files_write_staff_history |
| 0x550320 | `gravar_historicos_de_competicao` | db_files_write_comp_history |
| 0x54fbb0 | `gravar_jogadores` | db_files_write_non_players |
| 0x82e430 | `gravar_jogo` | save_game_data |
| 0x4afeb0 | `gravar_jogo_na_noticia` | comp_util_write_fixture_to_news_slots |
| 0x8984d0 | `gravar_linha_tatica` | write_tactics_row |
| 0x786d30 | `gravar_marca_da_noticia` | News::set_item_slot |
| 0x54d840 | `gravar_nacoes` | db_files_write_nations |
| 0x54efd0 | `gravar_nao_jogadores` | db_files_write_players |
| 0x932e10 | `gravar_no_conteiner` | zwrite |
| 0x550950 | `gravar_nomes` | db_files_write_or_read_records_0x3c |
| 0x54e720 | `gravar_pessoas_110` | db_files_write_staff |
| 0x54fd00 | `gravar_preferencias` | db_files_write_staff_extra |
| 0x729820 | `gravar_registro_da_partida` | Match_Stats::write_match_record |
| 0x550800 | `gravar_registros_58` | db_files_write_records_0x3a |
| 0x8395b0 | `gravar_regras_fifa` | save_fifa_transfer_rules |
| 0x896a90 | `gravar_vaga_tatica` | set_tactic_slot |
| 0x62ec00 | `gravidade_da_lesao` | injury_severity |
| 0x6854a0 | `grupo_decidido` | group_decided |
| 0x6cd660 | `guardar_extremos` | store_extremes |
| 0x896fe0 | `guardar_instrucoes` | store_instructions |
| 0x6dbb10 | `guardar_tatica` | store_tactic |
| 0x6e86d0 | `habilidade_vista` | perceived_ability |
| 0x7357c0 | `historia_punivel_do_jogador` | Media::get_finable_story_kind |
| 0x5fdc30 | `hora_das_ferias` | HUMAN_MANAGER::get_holiday_stamp |
| 0x5fe420 | `humano_assiste_a_partida` | HUMAN_MANAGER::is_fixture_watched |
| 0x600740 | `humano_deu_apelido` | HUMAN_MANAGER::has_nickname |
| 0x600c90 | `humano_manda_no_clube` | human_runs_club |
| 0x9331c0 | `id_do_conteiner` | zget_archive_id |
| 0x53ab90 | `idade_da_pessoa` | database_get_staff_age |
| 0x76d430 | `idades_da_selecao` | national_team_ages |
| 0x5221f0 | `importancia_da_partida` | cup_match_importance |
| 0x68a3e0 | `importancia_do_jogo_da_liga` | League::get_match_importance |
| 0x53bd10 | `incluir_pessoa_no_clube` | database_add_staff_to_club |
| 0x402ac0 | `iniciar_area` | Area::init |
| 0x6b5a70 | `iniciar_proxima_partida` | start_next_match |
| 0x6e9470 | `iniciar_rigor` | init_strictness |
| 0x6ddab0 | `instrucoes` | instructions |
| 0x6cc540 | `interceptar` | intercept |
| 0x6b5040 | `interesse_do_dia` | match_day_interest |
| 0x70dfd0 | `jogada` | play |
| 0x6c2fe0 | `jogador_no_bloco` | player_in_block |
| 0x6e1cd0 | `jogador_pela_ficha` | player_by_record |
| 0x6cda20 | `jogador_saiu` | player_left |
| 0x6b6cc0 | `jogadores_do_pos_jogo` | post_match_players |
| 0x6cbc00 | `jogadores_na_linha` | players_in_line |
| 0x6cc070 | `jogadores_no_trajeto` | players_on_path |
| 0x6b4360 | `jogar_o_dia` | play_the_day |
| 0x544970 | `juntar_o_time_b_ao_a` | database_merge_b_team_staff |
| 0x549cf0 | `juntar_vinculo_de_opiniao` | staff_add_opinion_link |
| 0x6b98c0 | `laco_do_motor` | engine_loop |
| 0x714d30 | `lado_do_jogador` | player_side |
| 0x6bba50 | `lado_do_lance` | side_of_move |
| 0x6d42a0 | `lado_sob_controle` | side_under_control |
| 0x6fa490 | `lancar` | long_ball |
| 0x6bab90 | `lance_valendo` | move_counts |
| 0x6b6970 | `largar_partida_assistida` | match_day_drop_viewer_and_finish_match |
| 0x6c4290 | `lateral` | throw_in |
| 0x717400 | `ler_1bc` | read_1bc |
| 0x700320 | `ler_8ea7` | read_8ea7 |
| 0x700330 | `ler_8ea8` | read_8ea8 |
| 0x700340 | `ler_8ea9` | read_8ea9 |
| 0x700380 | `ler_8eaa` | read_8eaa |
| 0x700400 | `ler_8eb2` | read_8eb2 |
| 0x700410 | `ler_8f7a` | read_8f7a |
| 0x5505c0 | `ler_arbitros` | db_files_read_officials |
| 0x54d8a0 | `ler_cidades` | db_files_read_cities |
| 0x54db50 | `ler_clubes` | db_files_read_clubs |
| 0x54dc90 | `ler_clubes_v1` | db_files_read_clubs_v1 |
| 0x54feb0 | `ler_competicoes` | db_files_read_comps |
| 0x550380 | `ler_competicoes_de_pessoal` | db_files_read_staff_comps |
| 0x54ffd0 | `ler_competicoes_v1` | db_files_read_comps_v1 |
| 0x54d5e0 | `ler_continentes` | db_files_read_continents |
| 0x550710 | `ler_cores` | db_files_read_colours |
| 0x932c30 | `ler_do_conteiner` | zread |
| 0x54da00 | `ler_estadios` | db_files_read_stadiums |
| 0x6d9c40 | `ler_eventos` | read_events |
| 0x6d9e50 | `ler_eventos_cfg` | read_events_cfg |
| 0x5504d0 | `ler_historico_de_competicoes_de_pessoal` | db_files_read_staff_comp_history |
| 0x54fd60 | `ler_historico_de_pessoas` | db_files_read_staff_history |
| 0x550230 | `ler_historicos_de_competicao` | db_files_read_comp_history |
| 0x54f030 | `ler_jogadores` | db_files_read_non_players |
| 0x54f150 | `ler_jogadores_v1` | db_files_read_non_players_v1 |
| 0x8983f0 | `ler_linha_tatica` | read_tactics_row |
| 0x76d7b0 | `ler_marca_da_pessoa` | read_person_mark |
| 0x54d740 | `ler_nacoes` | db_files_read_nations |
| 0x54e880 | `ler_nao_jogadores` | db_files_read_players |
| 0x54e9a0 | `ler_nao_jogadores_v1` | db_files_read_players_v1 |
| 0x550860 | `ler_nomes` | db_files_read_names |
| 0x54e630 | `ler_pessoas_110` | db_files_read_staff |
| 0x54e780 | `ler_pessoas_157` | db_files_read_staff_v1 |
| 0x54fc10 | `ler_preferencias` | db_files_read_staff_extra |
| 0x729600 | `ler_registro_da_partida` | Match_Stats::read_match_record |
| 0x6cfe10 | `lesao_na_dividida` | tackle_injury |
| 0x6d0980 | `lesionar` | injure |
| 0x82ff40 | `liberar_dados_do_jogo` | free_game_data |
| 0x527ee0 | `liberar_historico_de_pessoas` | database_free_staff_history_table |
| 0x527f20 | `liberar_historico_de_premios` | database_free_staff_comp_history_table |
| 0x6d5b50 | `liberar_tabelas_de_eventos` | free_event_tables |
| 0x544f30 | `liberar_tabelas_em_ingles` | eng_lng_free_tables |
| 0x6847b0 | `liga_tem_rebaixamento` | League::has_relegation |
| 0x76c0f0 | `ligar_44_da_pessoa` | set_person_44 |
| 0x5b50d0 | `ligar_marca_da_formacao` | set_formation_flag |
| 0x6cb930 | `limite` | limit |
| 0x70df40 | `limite_jogador` | player_limit |
| 0x5b5960 | `limpar_designados` | clear_assigned |
| 0x6bba10 | `limpar_lance` | clear_move |
| 0x90cd50 | `limpar_pasta_temporaria` | utils_clear_temp_folder |
| 0x86fc20 | `limpar_registro_de_tecnico` | Staff_Records::clear_manager_record |
| 0x549c90 | `limpar_vinculos_de_opiniao` | staff_clear_opinion_links |
| 0x8982b0 | `linha_tatica` | tactics_row |
| 0x6c3aa0 | `linhas_defensivas` | defensive_lines |
| 0x5d5ab0 | `lista_de_observados` | transfer_manager_get_shortlist_manager |
| 0x6cb780 | `mais_perto_do_gol` | closest_to_goal |
| 0x6c30e0 | `mais_perto_no_lado` | closest_on_side |
| 0x6c3010 | `mais_proximo_do_ponto` | nearest_to_point |
| 0x6ce110 | `manter_posicao_um` | hold_position_one |
| 0x5b7160 | `mapa_de_casas_da_formacao` | Formation::build_position_lookup |
| 0x7163f0 | `marca_800000` | mark_800000 |
| 0x6bbf60 | `marcacao_individual` | individual_marking |
| 0x7160f0 | `marcado_designado` | designated_marked |
| 0x6d4330 | `marcador_direto` | direct_marker |
| 0x5af180 | `marcar_ano_no_calendario` | mark_calendar_year |
| 0x8c5410 | `marcar_contrato_acertado` | set_agreed_contract_flag |
| 0x6d10a0 | `marcar_craques` | mark_stars |
| 0x540980 | `marcar_elenco_da_pessoa` | staff_set_squad_flags |
| 0x5b5910 | `marcar_posicao` | mark_position |
| 0x8c1f80 | `marcar_termos_oferecidos` | TRANSFER_MANAGER::note_terms_offered_to_main_player |
| 0x897760 | `marcar_vaga` | mark_slot |
| 0x5b5270 | `marcas_da_posicao` | position_marks |
| 0x7163c0 | `marcas_do_jogador_rot` | player_marks_rot |
| 0x68a4d0 | `margem_do_ultimo_dia` | League::final_day_margin_needed |
| 0x5b59a0 | `mascara_da_formacao` | formation_mask |
| 0x5481a0 | `mascara_da_posicao_natural` | staff_get_natural_position_mask |
| 0x5b7d20 | `mascara_de_posicoes` | position_mask |
| 0x7c1950 | `media_das_notas` | ratings_average |
| 0x5d5a60 | `media_do_jogo` | game_average |
| 0x6e1a40 | `melhor_avaliado` | best_rated |
| 0x6e1940 | `melhor_da_lista` | best_in_list |
| 0x6c3950 | `menor_distancia` | smallest_distance |
| 0x6d4250 | `momento_marcado` | marked_moment |
| 0x71b630 | `montar_detalhes_da_partida` | build_match_details_screen |
| 0x729000 | `montar_estatisticas_da_partida` | Match_Stats::Match_Stats |
| 0x6844b0 | `montar_grade_de_turnos` | league_build_round_robin_grid |
| 0x539750 | `montar_indice_de_nomes` | database_build_nation_name_index |
| 0x717db0 | `montar_jogos_do_dia` | build_day_fixtures_screen |
| 0x6b3c10 | `montar_o_dia` | set_up_the_day |
| 0x591f80 | `montar_resumo_do_save` | build_save_game_summary |
| 0x718e90 | `montar_tela_da_partida` | build_match_screen |
| 0x71b300 | `montar_ultimos_placares` | build_latest_scores |
| 0x6cfd40 | `motor_cartao_pela_gravidade` | engine_card_by_severity |
| 0x6cfca0 | `motor_decidir_falta` | engine_decide_foul |
| 0x6cfd70 | `motor_procurar_impedimento` | engine_look_for_offside |
| 0x6cfd10 | `motor_punir_falta` | engine_punish_foul |
| 0x6cfcd0 | `motor_punir_lance` | engine_punish_move |
| 0x6f09f0 | `mover_jogador` | player_move |
| 0x6f46b0 | `mover_para` | move_to |
| 0x714d40 | `mover_sem_bola` | move_off_ball |
| 0x6f4650 | `na_area` | in_the_box |
| 0x714c60 | `na_lista` | in_list |
| 0x5446d0 | `nacao_britanica_ou_irlanda` | nation_is_uk_or_ireland |
| 0x544670 | `nacao_com_transferencia_livre` | nation_allows_free_transfers |
| 0x53d900 | `nacao_da_melanesia` | database_is_melanesian_nation |
| 0x53d940 | `nacao_da_polinesia` | database_is_polynesian_nation |
| 0x7e6f20 | `nacao_do_grupo` | group_nation |
| 0x549ea0 | `nacao_grande` | nation_is_major_nation |
| 0x776380 | `nao_gosta_da_selecao` | dislikes_national_side |
| 0x547840 | `nao_gosta_de` | dislikes |
| 0x547d30 | `nao_gosta_do_clube` | dislikes_club |
| 0x6be120 | `narrar_classificacao` | narrate_qualification |
| 0x6d7b00 | `narrar_no_painel` | commentate_on_panel |
| 0x8c5fc0 | `negociacoes_de_troca_do_jogador` | TRANSFER_MANAGER::get_exchange_negotiations_for_player |
| 0x5afe00 | `nevoa_apos_partida` | fog_after_match |
| 0x53ce90 | `nivel_do_contrato` | contract_level |
| 0x76d810 | `nivel_para_a_selecao` | national_team_level |
| 0x545770 | `nome_curto_ingles_do_clube` | club_get_english_short_name |
| 0x53ac40 | `nome_do_cargo_de_treino` | database_get_coaching_job_name |
| 0x6db2d0 | `nome_do_jogador_na_narracao` | commentary_player_name |
| 0x5455a0 | `nome_ingles_da_competicao` | comp_get_english_name |
| 0x5456a0 | `nome_ingles_da_competicao_de_pessoal` | staff_comp_get_english_name |
| 0x5453f0 | `nome_ingles_da_nacao` | nation_get_english_name |
| 0x545880 | `nome_ingles_do_clube` | club_get_english_name |
| 0x545990 | `nome_ingles_do_estadio` | stadium_get_english_name |
| 0x538bc0 | `normalizar_atributos_do_jogador` | staff_normalise_player_attributes |
| 0x539490 | `normalizar_atributos_do_nao_jogador` | staff_normalise_non_player_attributes |
| 0x538ec0 | `normalizar_atributos_fisicos_do_jogador` | staff_normalise_player_physical_attributes |
| 0x715f50 | `nota_ajustada` | adjusted_rating |
| 0x6d4a20 | `nota_do_arbitro` | referee_grade |
| 0x7ad220 | `nota_do_clube_para_o_jovem` | player_regen_score_club_for_player |
| 0x6ebd70 | `nota_do_jogador` | player_rating |
| 0x6e6340 | `nota_fora_da_partida` | off_match_score |
| 0x6e2f20 | `nota_para_a_vaga` | lineup_slot_score |
| 0x5442f0 | `nota_relativa_com_habilidade_reduzida` | player_attribute_rating_with_scaled_ability |
| 0x544380 | `nota_relativa_do_atributo` | player_attribute_rating |
| 0x735780 | `noticia_com_resposta_2` | story_with_response_two |
| 0x786fe0 | `noticias_nao_lidas` | News::count_unread |
| 0x6d3de0 | `nova_reputacao` | new_reputation |
| 0x6b6570 | `numero_da_competicao` | competition_number |
| 0x4cca30 | `oferta_acertada` | agreed_contract_offer |
| 0x4cc9e0 | `oferta_de_contrato` | contract_offer |
| 0x8d0d90 | `oferta_fechada` | transfer_offer_closed |
| 0x8c5cf0 | `ofertas_pelo_jogador` | TRANSFER_MANAGER::get_offers_for_player |
| 0x53cd60 | `olheiros_do_clube` | database_get_club_scouts |
| 0x683210 | `ordem_das_linhas_da_tabela` | league_compare_table_rows |
| 0x6b6510 | `ordem_das_partidas_do_dia` | day_match_order |
| 0x540270 | `ordem_das_pessoas_pelo_nome_completo` | database_compare_staff_by_full_name |
| 0x6bd840 | `ordem_de_acao` | action_order |
| 0x6834f0 | `ordem_do_confronto` | league_compare_h2h_rows |
| 0x5402f0 | `ordem_do_elenco_pela_habilidade` | qsort comparator (squad by player +0xb) |
| 0x549c30 | `ordem_do_elenco_pela_habilidade_atual` | compare_staff_by_player_ability_desc |
| 0x540780 | `ordem_do_historico_de_competicao` | database_compare_comp_history_rows |
| 0x53fd40 | `ordem_dos_clubes_pela_latitude` | database_compare_clubs_by_latitude |
| 0x53fe50 | `ordem_dos_clubes_pela_longitude` | database_compare_clubs_by_longitude |
| 0x540020 | `ordem_dos_clubes_pela_reputacao` | qsort comparator (club ids by reputation) |
| 0x540080 | `ordem_dos_clubes_pelo_nome_curto` | database_compare_clubs_by_short_name |
| 0x72b160 | `ordem_dos_eventos` | match_stats_compare_events |
| 0x6bd760 | `ordem_dos_jogadores` | player_order |
| 0x6be520 | `ordem_dos_penaltis` | penalty_order |
| 0x4b0780 | `ordem_por_5f` | order_by_5f |
| 0x4b07b0 | `ordem_por_80` | order_by_80 |
| 0x4b0810 | `ordem_por_80_cresc` | order_by_80_asc |
| 0x4b07d0 | `ordem_por_80_desc` | order_by_80_desc |
| 0x6e0fe0 | `ordem_por_condicao` | order_by_condition |
| 0x6e0df0 | `ordem_por_tabela` | order_by_table |
| 0x53db40 | `ordenar_clubes_pela_reputacao` | database_sort_clubs_by_reputation |
| 0x403350 | `organizar_area` | Area::layout |
| 0x71c270 | `painel_da_visao_geral_da_partida` | draw_match_overview_panel |
| 0x71e040 | `painel_das_estatisticas_da_partida` | draw_match_stats_panel |
| 0x7217a0 | `painel_das_estatisticas_do_time` | draw_team_stats_panel |
| 0x722ed0 | `painel_das_notas_dos_jogadores` | draw_player_ratings_panel |
| 0x71f500 | `painel_das_zonas_de_acao` | draw_action_zones_panel |
| 0x721000 | `painel_do_relatorio_da_partida` | draw_match_report_panel |
| 0x723d20 | `painel_dos_ultimos_placares` | draw_latest_scores_panel |
| 0x5c4680 | `pais_da_excursao` | Friendly::pick_tour_nation |
| 0x62c3d0 | `palavra_0c` | word_0c |
| 0x717c70 | `papel_880` | role_880 |
| 0x716310 | `papel_na_defesa` | defending_role |
| 0x716270 | `papel_no_ataque` | attacking_role |
| 0x6d5930 | `partida_abandonada` | match_abandoned |
| 0x5fe290 | `partida_assistida_pelo_humano` | HUMAN_MANAGER::is_watching_fixture |
| 0x4b0010 | `partida_das_vagas_da_noticia` | comp_util_read_fixture_from_news_slots |
| 0x6b61e0 | `partida_de_interesse` | match_of_interest |
| 0x6b66a0 | `partida_sem_humano` | match_without_human |
| 0x6fa250 | `passar` | pass_ball |
| 0x6b55c0 | `passo_em_rede` | network_step |
| 0x6f0630 | `pegar_bola` | take_ball |
| 0x6c50d0 | `penalti` | penalty |
| 0x6be710 | `penaltis` | penalties |
| 0x6bd930 | `periodo_encerrado` | period_completed |
| 0x70df70 | `perto_do_gol` | near_goal |
| 0x70def0 | `perto_do_proprio_gol` | near_own_goal |
| 0x7c6930 | `peso_da_competicao` | competition_weight |
| 0x53e990 | `pessoa_confere_com_os_nomes` | database_staff_matches_names |
| 0x897dc0 | `pessoa_da_linha` | row_person |
| 0x8968f0 | `pessoa_da_vaga_tatica` | tactic_slot_person |
| 0x53d740 | `pessoa_e_comunitaria` | database_is_staff_eu_national |
| 0x53d980 | `pessoa_e_estrangeira` | database_is_staff_foreign |
| 0x544600 | `pessoa_listada_a_pedido` | staff_is_listed_by_request |
| 0x544590 | `pessoa_listada_para_emprestimo` | staff_is_loan_listed |
| 0x544520 | `pessoa_listada_para_transferencia` | staff_is_transfer_listed_or_requested |
| 0x5444a0 | `pessoa_listada_pelo_clube` | staff_is_transfer_listed |
| 0x69fc60 | `pessoa_na_lista` | person_in_list |
| 0x8c1870 | `pessoa_na_lista_de_transferencias` | person_in_transfer_list |
| 0x548090 | `pessoas_favoritas` | staff_get_favoured_staff |
| 0x5480f0 | `pessoas_rejeitadas` | staff_get_disliked_staff |
| 0x6ce310 | `placar_agregado` | aggregate_score |
| 0x539ae0 | `podar_historico_de_competicoes` | database_prune_comp_history_table |
| 0x8ced40 | `pode_contratar_jogador` | TRANSFER_MANAGER::can_sign_player |
| 0x6bce20 | `pode_escolher` | can_choose |
| 0x6c51e0 | `pontape_inicial` | kick_off |
| 0x6cb6c0 | `por_diferenca` | by_difference |
| 0x933170 | `posicao_no_conteiner` | position_in_container |
| 0x6c2be0 | `posicionar` | position (place players) |
| 0x6d1770 | `posicoes_finas` | fine_positions |
| 0x6df460 | `postura` | stance |
| 0x5cb9b0 | `preencher_data_internacional` | friendly_international_date_set |
| 0x721cd0 | `preencher_estatisticas_do_time` | fill_team_stats_rows |
| 0x729ff0 | `preencher_registro_da_partida` | Match_Stats::fill_match_record |
| 0x682f70 | `premio_da_liga` | League::calc_prize_fund |
| 0x6b7f20 | `preparar_partida` | prepare_match |
| 0x6bcbd0 | `pressao` | pressure |
| 0x6bd6b0 | `pressao_na_area` | pressure_in_box |
| 0x6c3a00 | `pressao_no_ponto` | pressure_at_point |
| 0x539c60 | `primeiro_nome_da_nacao` | database_pick_first_name_for_nation |
| 0x539b90 | `primeiro_nome_pelo_peso` | database_pick_first_name_by_weight |
| 0x6ea760 | `procurar_impedimento` | look_for_offside |
| 0x6cd3d0 | `procurar_jogada` | look_for_play |
| 0x6cd330 | `procurar_no_lado` | search_on_side |
| 0x68d1b0 | `proximo_da_lista` | next_list_item |
| 0x6d78b0 | `proximo_evento` | next_event |
| 0x55cae0 | `punicao_da_tabela` | table_penalty |
| 0x55cb60 | `punicao_guardada` | Discipline::find_held_ban |
| 0x6d1210 | `punicoes` | punishments |
| 0x6e9630 | `punir_falta` | punish_foul |
| 0x6ea4e0 | `punir_lance` | punish_move |
| 0x5afde0 | `quadro_do_tecnico` | manager_table |
| 0x6d59b0 | `quantos_marcam` | how_many_mark |
| 0x6d5a10 | `quem_marca` | who_marks |
| 0x6bc030 | `quem_marcar` | who_to_mark |
| 0x6f5810 | `recuperar_folego` | recover_stamina |
| 0x62bd10 | `recuperar_lesao` | recover_injury |
| 0x6e1aa0 | `refazer_time` | redo_team |
| 0x5ca030 | `registrar_competicoes_do_amistoso` | Friendly::register_comp_objects |
| 0x68a1c0 | `registrar_jogo_da_rodada` | store_round_match |
| 0x8ce920 | `registrar_jogo_pela_selecao` | TRANSFER_MANAGER::record_international_appearance |
| 0x6bb9a0 | `registrar_lance` | record_move |
| 0x62be00 | `registrar_lesao` | register_injury |
| 0x79ce10 | `registrar_nota_do_arbitro` | record_referee_grade |
| 0x7293f0 | `registrar_partida` | Match_Stats::record_match |
| 0x7003b0 | `registrar_posse` | record_possession |
| 0x896570 | `registro_da_formacao` | formation_record |
| 0x6826d0 | `registro_da_liga` | league_record |
| 0x7f6cd0 | `registro_do_observador` | scout_record |
| 0x53a5b0 | `registro_livre` | free_record |
| 0x521d60 | `regras_da_rodada_no_jogo` | Cup::apply_round_rules_to_match |
| 0x6c54a0 | `reiniciar_lance` | restart_play |
| 0x452fe0 | `reiniciar_recordes` | reset_records |
| 0x5477d0 | `rejeitado_por` | rejected_by |
| 0x60ba80 | `remapear_chave_de_clube` | remap_club_key_id |
| 0x714ca0 | `remover_da_lista` | remove_from_list |
| 0x6c06b0 | `repartir_a` | split_a |
| 0x6c07a0 | `repartir_b` | split_b |
| 0x6c0a70 | `repartir_c` | split_c |
| 0x6c0890 | `repartir_d` | split_d |
| 0x6c0980 | `repartir_e` | split_e |
| 0x6c0b70 | `repartir_f` | split_f |
| 0x83bf40 | `reputacao_alvo_do_clube` | Shortlist_Manager::update_club_target_reputation |
| 0x545e90 | `reputacao_do_clube_para_pessoa` | club_reputation_for_staff |
| 0x6d3610 | `reputacao_do_jogador` | player_reputation |
| 0x5407e0 | `reputacao_vista_por` | database_get_staff_reputation_for_viewer |
| 0x5315a0 | `resolver_indices_do_banco` | resolve_database_indexes |
| 0x6f5c30 | `responder` | respond |
| 0x725ca0 | `responder_continuar_da_partida` | match_screens_handle_continue_command |
| 0x71bdc0 | `responder_detalhes_da_partida` | match_details_input |
| 0x727c30 | `responder_escalacao_da_partida` | match_screens_handle_team_selection_input |
| 0x722dc0 | `responder_estatisticas_do_time` | handle_team_stats_input |
| 0x718930 | `responder_jogos_do_dia` | handle_day_fixtures_input |
| 0x71a240 | `responder_tela_da_partida` | handle_match_screen_input |
| 0x71b3d0 | `responder_ultimos_placares` | latest_scores_input |
| 0x725860 | `responder_ultimos_placares_da_partida` | match_screens_handle_latest_scores_cycle |
| 0x4b0890 | `resultado_da_eliminatoria_na_noticia` | comp_util_classify_tie_result_for_news |
| 0x6ce460 | `resumo_da_partida` | match_summary |
| 0x541f90 | `reunir_elenco_do_clube` | database_collect_club_squad |
| 0x6bbe40 | `rolar_bola` | roll_ball |
| 0x725a70 | `rotulo_do_controle` | match_control_label |
| 0x72adf0 | `salvar_estatisticas_da_partida` | Match_Stats::save |
| 0x53a1b0 | `segundo_nome_da_nacao` | database_pick_second_name_for_nation |
| 0x53a0e0 | `segundo_nome_pelo_peso` | database_pick_second_name_by_weight |
| 0x53b4f0 | `selecao_b` | b_national_team |
| 0x775790 | `selecao_ou_especial` | national_or_special |
| 0x53b3d0 | `selecao_principal` | main_national_team |
| 0x6d4280 | `sem_partida` | no_match |
| 0x7febb0 | `sem_tela_seguinte` | ScreenManager::is_forward_unavailable |
| 0x5b73a0 | `separar_mascara` | split_mask |
| 0x844300 | `serve_para_a_vaga` | shortlist_slot_fits |
| 0x5454c0 | `sigla_inglesa_da_nacao` | nation_get_english_three_letter_name |
| 0x6beeb0 | `simulacao_rapida` | quick_simulation |
| 0x6f0970 | `soltar_bola` | release_ball |
| 0x7160d0 | `somar_2b` | add_2b |
| 0x6cf710 | `somar_disposicao` | add_disposition |
| 0x6d8c70 | `sons_terminados` | sounds_finished |
| 0x6f44a0 | `sortear_momento` | draw_moment |
| 0x6e1090 | `substituir` | substitute |
| 0x6c38c0 | `tabela_a` | table_a |
| 0x6c3920 | `tabela_a_1a` | table_a_1a |
| 0x717430 | `tabela_a_1a_do_jogador` | player_table_a_1a |
| 0x6c38f0 | `tabela_b` | table_b |
| 0x717450 | `tabela_b_do_jogador` | player_table_b |
| 0x7280e0 | `tabela_da_liga_ao_vivo` | match_screens_draw_live_league_table |
| 0x717410 | `tabela_do_jogador` | player_table |
| 0x933860 | `tamanho_dos_itens_do_padrao` | zget_matching_size |
| 0x5fd210 | `tecnico_9a01` | manager_9a01 |
| 0x5fd480 | `tecnico_9a05` | manager_9a05 |
| 0x5fd6b0 | `tecnico_9a06` | manager_9a06 |
| 0x5fe040 | `tecnico_9ace` | manager_9ace |
| 0x5fea50 | `tecnico_9b23` | manager_9b23 |
| 0x7ae570 | `tecnico_da_busca` | PLAYER_SEARCH::set_manager |
| 0x53ca90 | `tecnico_do_clube_ou_selecao` | database_get_club_manager |
| 0x600e90 | `tecnico_substituto` | human_manager_get_stand_in_manager |
| 0x6b59a0 | `tem_jogo` | has_match |
| 0x735840 | `tem_noticia` | has_story |
| 0x7164b0 | `teste_de_condicao` | condition_test |
| 0x84b230 | `teto_da_observacao` | shortlist_manager_max_fee_for_squad_status |
| 0x6d8d10 | `texto_do_evento` | event_text |
| 0x53b620 | `tipo_11_a_15` | type_11_to_15 |
| 0x7ae6e0 | `tipo_da_lista_da_busca` | PLAYER_SEARCH::set_list_type |
| 0x53b610 | `tipo_nao_11` | type_not_11 |
| 0x8cb630 | `tirar_bonus_pela_selecao` | TRANSFER_MANAGER::remove_international_team_appearance_money |
| 0x8cb590 | `tirar_bonus_por_jogo` | TRANSFER_MANAGER::remove_first_team_appearance_money |
| 0x5c64e0 | `tirar_clube_dos_amistosos` | Friendly::remove_club_from_ai_plans |
| 0x547f50 | `tirar_clube_favorito` | staff_remove_favoured_club |
| 0x4d0bd0 | `tirar_contrato_de_emprestimo` | CONTRACT_MANAGER::remove_loan_contract |
| 0x6cdd50 | `tirar_de_campo` | take_off_pitch |
| 0x7c6bf0 | `tirar_idioma_do_jogador` | PlayerStats::remove_language |
| 0x775ba0 | `tirar_jogador_da_selecao` | NationalTeams::remove_player |
| 0x7a8470 | `tirar_jogador_do_banco` | PLAYER_REGEN::remove_player_data_from_pool |
| 0x8cb4f0 | `tirar_percentual_da_proxima_venda` | TRANSFER_MANAGER::remove_next_sale_percentage |
| 0x53c190 | `tirar_pessoa_do_clube` | database_remove_staff_from_club |
| 0x547c10 | `tirar_pessoa_favorita` | staff_remove_favoured_staff |
| 0x547c50 | `tirar_pessoa_rejeitada` | staff_remove_disliked_staff |
| 0x7a82b0 | `tirar_reserva_de_tecnico` | PLAYER_REGEN::remove_spare_non_player_record |
| 0x549d60 | `tirar_vinculo_de_opiniao` | staff_remove_opinion_link |
| 0x6dbb80 | `titulares` | starters |
| 0x6ccb60 | `tracar_lance` | trace_move |
| 0x53cc30 | `treinadores_do_clube` | database_get_club_coaches |
| 0x8c5e40 | `trocas_oferecendo_o_jogador` | TRANSFER_MANAGER::get_exchange_offers_for_player |
| 0x7c1a60 | `ultima_nota_do_jogador` | PlayerStats::get_last_match_rating |
| 0x4b0850 | `unidades` | units_digit |
| 0x6d47e0 | `uniforme` | kit |
| 0x7ae640 | `vaga_da_busca` | PLAYER_SEARCH::set_slot_index |
| 0x5b2f10 | `vaga_da_casa` | slot_of_code |
| 0x785830 | `vaga_da_noticia` | NewsItem::get_slot |
| 0x8c61e0 | `vaga_da_oferta` | offer_slot |
| 0x8972c0 | `vaga_do_escalado` | Tactics::find_selection_slot |
| 0x727e20 | `vaga_do_mais_bem_avaliado` | match_screens_find_best_rated_starter_slot |
| 0x897ee0 | `vaga_extra_tatica` | extra_tactic_slot |
| 0x897590 | `vaga_marcada` | marked_slot |
| 0x6d5af0 | `vagas_do_lado_sem_conflito` | side_slots_without_conflict |
| 0x5b8960 | `vagas_sem_conflito` | slots_without_conflict |
| 0x700bd0 | `valor_do_chute` | shot_value |
| 0x4d0c20 | `valor_do_contrato` | contract_value |
| 0x7165b0 | `valor_pela_condicao` | value_by_condition |
| 0x71e750 | `valores_das_estatisticas_da_partida` | refresh_match_stats_values |
| 0x723350 | `valores_das_notas_dos_jogadores` | refresh_player_ratings_rows |
| 0x720320 | `valores_das_zonas_de_acao` | refresh_action_zones_values |
| 0x6d5d80 | `variante_do_evento` | event_variant |
| 0x6b7cf0 | `venceu_o_jogo` | won_the_match |
| 0x838eb0 | `versao_do_jogo_salvo_valida` | valid_saved_game_version |
| 0x53a610 | `vetor_de_atributos_do_jogador` | database_player_to_attribute_vector |
| 0x53a8b0 | `vetor_de_atributos_do_nao_jogador` | database_non_player_to_attribute_vector |
| 0x53aa10 | `vetor_para_o_jogador` | database_vector_to_player |
| 0x53a980 | `vetor_para_o_nao_jogador` | database_vector_to_non_player |
| 0x6fa140 | `virado_para` | facing |
| 0x70df20 | `x_fora_do_meio` | x_off_centre |
| 0x6d5b20 | `zagueiros_do_lado_sem_conflito` | side_centre_backs_without_conflict |
| 0x5b8dc0 | `zagueiros_sem_conflito` | centre_backs_without_conflict |
| 0x6ce0f0 | `zerar_guardado` | clear_stored |
| 0x76d7e0 | `zerar_marca_da_pessoa` | clear_person_mark |

## Random generator and dates

| Address | Português | English |
|---|---|---|
| 0x90cfc0 | `aleatorio(n)` | random(n) |
| 0x54a110 | `Data()` | Date() |
| 0x549ef0 | `Data(dia, ano)` | Date(day, year) |
| 0x549f70 | `Data(dia_do_mes, mes, ano, dia_da_semana)` | Date(day_of_month, month, year, weekday) |
| 0x54a3f0 | `dia_da_semana()` | weekday() |
| 0x54a2d0 | `ajustar_semana(alvo, dest)` | adjust_week(target, dest) |
| 0x54c770 | `mais_semanas(n)` | plus_weeks(n) |

## Helpers in `motor.rs`

| Português | English | What it is |
|---|---|---|
| `jogador_do_bloco` | block_player | player k of the side's 20-player block |
| `casa_da_grade` | grid_square | pitch grid square (x, y) |
| `sortear` | draw | calls the game's random draw (0x90cfc0) |
| `ftol` | ftol | float → integer truncation (_ftol, 0x945640) |
| `f32_em / f64_em` | f32_at / f64_at | float/double constant read from .rdata |
| `byte` | byte | signed byte at address + offset |
| `lado` | side | player's side (byte 0x27) |
| `degrau` | step | ladder of multipliers by attribute |
| `pow_do_jogo` | game_pow | power via the game's CRT (0x945ff0) |
| `comparar_*` | compare_* | runs original and Rust side by side and records divergences |
| `comentar / porta_6d6f70` | comment / gate_6d6f70 | commentary and the gate that records the calls |
| `PLACAR` | SCOREBOARD | counter of comparisons and divergences |

## Modules and tools

| File | English | What it is |
|---|---|---|
| `video.rs` | video | own DirectDraw |
| `apresentacao.rs` | presentation | frame → window, mouse |
| `ganchos.rs` | hooks (IAT) | import table hooks |
| `desvios.rs` | detours | detours in the exe, private stack |
| `simulacao.rs` | simulation | reproducible simulation |
| `alocador.rs` | allocator | fixed allocator |
| `aleatorio.rs` | random | the game's random generator |
| `data.rs` | date | dates (Date.cpp) |
| `motor.rs` | engine | match engine |
| `espiao.rs` | spy | diagnostics (CM_ESPIAO=1) |
| `comparacao.rs` | comparison | side-by-side comparison |
| `registro.rs` | log | logging to ddraw.log |
| `simular.py` | simulate | headless automatic career |
| `laboratorio.py` | laboratory | cross-session lock and memory watchdog |
| `desmonta.py` | disassemble | disassembly |
| `desvios.py` | detours | where a hook can go |
| `escuta_tela.py` | screen listener | listens to what the screen draws |
| `patch_portatil.py` | portable patch | generates the no-CD exe |
| `resolucao.py` | resolution | generates the 1920×1080 exe |
| `cobertura.py` | coverage | per-function call counter |
| `candidatos.py` | candidates | next routines |
| `mutacoes.py` | mutations | self-test with mutations |
| `prologos.py` | prologues | checks the hooked bytes |
| `textos.py / refs.py` | strings / refs | exe strings and who uses them |
