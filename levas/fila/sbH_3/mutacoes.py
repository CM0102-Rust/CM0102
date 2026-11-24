# Mutacoes (sbH_3) pela carreira ENG ate 2002-06-01 e pelo comparador da rotina (o dia dos amistosos pergunta por cada
# amistoso planejado da IA, uma semana a frente):
# - oito dias no lugar de sete: os amistosos planejados para daqui a uma semana deixam de nascer (a carreira muda; o
#   comparador acusa);
# - a janela de um amistoso com dois dias depois dele: a resposta muda nos dias seguintes a um amistoso (o comparador
#   acusa quando um planejado cai ali);
# - o torneio ja montado responde 3 e o por montar 2: o comparador acusa (o dia dos amistosos so quer o 1).
MUTACOES = [
    ('sbH_3_oito_dias',
     '    compare_dates(G_CURRENT_DATE, at(&days_before(date, 7))) > 0',
     '    compare_dates(G_CURRENT_DATE, at(&days_before(date, 8))) > 0 // mutation',
     'sbH_3 date free'),
    ('sbH_3_janela_do_amistoso',
     '        let end = days_after(m + friendly_match::DATE, 1);',
     '        let end = days_after(m + friendly_match::DATE, 2); // mutation',
     'sbH_3 date near a friendly'),
    ('sbH_3_torneio_trocado',
     '            return Some(ClubDateStatus { tournament: slot, ..ClubDateStatus::answer(if created { DATE_TAKEN } else { DATE_TAKEN + 1 }) });',
     '            return Some(ClubDateStatus { tournament: slot, ..ClubDateStatus::answer(if created { DATE_TAKEN + 1 } else { DATE_TAKEN }) }); // mutation',
     'sbH_3 date in a tournament'),
]
