-- Seed: asset_groups + game_assets for gv and gf (10 groups each)
-- Run on a DB where schema.sql has already been applied.
-- Safe to re-run: clears existing data first.

DELETE FROM public.game_assets WHERE group_id IN (
    SELECT id FROM public.asset_groups WHERE game_code IN ('gv', 'gf')
);
DELETE FROM public.asset_groups WHERE game_code IN ('gv', 'gf');

DO $$
DECLARE
    base TEXT := 'https://<project>.supabase.co/storage/v1/object/public/game-assets';
    codes TEXT[] := ARRAY['gv', 'gf'];
    gcode TEXT;
    gid INT;
BEGIN
    FOREACH gcode IN ARRAY codes LOOP
        FOR i IN 0..9 LOOP
            INSERT INTO public.asset_groups (game_code, label)
            VALUES (gcode, 'group_' || LPAD(i::TEXT, 2, '0'))
            RETURNING id INTO gid;

            INSERT INTO public.game_assets (group_id, label, image_url, is_correct) VALUES
            (gid, 'reference', base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/reference.svg', FALSE),
            (gid, 'option_0',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_0.svg',  FALSE),
            (gid, 'option_1',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_1.svg',  FALSE),
            (gid, 'option_2',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_2.svg',  FALSE),
            (gid, 'option_3',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_3.svg',  FALSE);
        END LOOP;
    END LOOP;
END $$;
