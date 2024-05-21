-- This file should undo anything in `up.sql`
drop TRIGGER if exists V_LAPTOP_TRIGGER on V_LAPTOP;

drop FUNCTION if exists V_LAPTOP_TRIGGER_FUNC;

drop VIEW if exists V_LAPTOP;
