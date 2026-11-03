# cqh_5 vazio (02/11/2026)

O item tinha a `League::set_club_season_expectations` (0x68aa80), a `League::relegate_club_to_division` (0x6831a0) e a `League::process_promotion_relegation_chain` (0x689c20). Nenhuma rodou na carreira automática até 2002-06-01 (as expectativas são do começo da temporada; o rebaixamento e a cadeia de promoções, do fim), e as três mutações escaparam.

- A 0x68aa80 e a 0x6831a0 foram como passageiras para a `cqh_4` (a tabela das rodadas da Liga dos Campeões, com mutações que a carreira pega).
- A 0x689c20 saiu (volta para a lista das candidatas).
- Este item fica sem rotinas; pode sair da ordem.
