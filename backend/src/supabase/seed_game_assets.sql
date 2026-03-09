-- Seed: game_metadata, asset_groups, game_assets
-- Run on a DB where schema.sql has already been applied.
-- Safe to re-run: clears existing data first.

DELETE FROM public.game_metadata;

INSERT INTO public.game_metadata
    (game_code, display_name, chc_factor, cognitive_domain, description, scientific_basis, task_summary, metric_name, metric_direction, sort_order)
VALUES
    ('gt',
     'Reaction Time',
     'Gt',
     'Psychomotor Speed',
     'Measures how quickly you can respond to a visual stimulus. A fundamental indicator of neural processing speed.',
     'Simple reaction time paradigms (Donders, 1869; Jensen, 2006) quantify stimulus-detection and motor-response latency, loading on CHC factor Gt.',
     'Wait for a visual cue, then click/tap as fast as possible. Multiple trials are averaged.',
     'Median RT (ms)',
     'lower_is_better',
     1),

    ('gwm',
     'Sequence Memory',
     'Gwm',
     'Working Memory Span',
     'Tests how long a sequence you can hold in short-term memory and reproduce in order.',
     'Serial-order span tasks (Baddeley, 2003; Engle et al., 1999) index working memory capacity (CHC Gwm) and predict fluid reasoning.',
     'Watch a sequence of highlighted positions, then reproduce it in the same order. Length increases each round until failure.',
     'Max Sequence Length',
     'higher_is_better',
     2),

    ('gs',
     'Symbol Matching',
     'Gs',
     'Processing Speed',
     'Measures how rapidly and accurately you can compare visual symbols under time pressure.',
     'Derived from Digit Symbol / Symbol Search paradigms (Wechsler, 1997; Salthouse, 1996), loading on CHC Gs — perceptual speed and scanning.',
     'Determine whether a target symbol appears in a set of candidates. Complete as many trials as possible within the time limit.',
     'Correct Responses / Time',
     'higher_is_better',
     3),

    ('gf',
     'Pattern Logic',
     'Gf',
     'Fluid Reasoning',
     'Assesses your ability to identify abstract rules and complete visual patterns without prior knowledge.',
     'Matrix-reasoning tasks (Raven, 1938; Cattell, 1963) are the canonical measure of fluid intelligence (CHC Gf).',
     'Examine a visual pattern with a missing piece and select the correct completion from four options.',
     'Accuracy (%)',
     'higher_is_better',
     4),

    ('gv',
     'Mental Rotation',
     'Gv',
     'Spatial Visualization',
     'Tests your ability to mentally rotate objects and identify matching orientations.',
     'Mental rotation tasks (Shepard & Metzler, 1971) load on CHC Gv and strongly predict STEM aptitude (Wai et al., 2009).',
     'Compare a reference figure to four candidates and select the one that is a rotated match (not a mirror image).',
     'Accuracy (%)',
     'higher_is_better',
     5);

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
            (gid, 'option_0',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_0.svg',  TRUE),
            (gid, 'option_1',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_1.svg',  FALSE),
            (gid, 'option_2',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_2.svg',  FALSE),
            (gid, 'option_3',  base || '/' || gcode || '/group_' || LPAD(i::TEXT, 2, '0') || '/option_3.svg',  FALSE);
        END LOOP;
    END LOOP;

    -- Validate that each gv/gf asset group has exactly one correct asset
    PERFORM 1
    FROM public.asset_groups ag
    JOIN public.game_assets ga ON ga.group_id = ag.id
    WHERE ag.game_code IN ('gv', 'gf')
    GROUP BY ag.id
    HAVING SUM(CASE WHEN ga.is_correct THEN 1 ELSE 0 END) <> 1;

    IF FOUND THEN
        RAISE EXCEPTION
            'Seed validation failed: each asset group for game_code in (gv, gf) must have exactly one correct game_asset (is_correct = TRUE).';
    END IF;
END $$;
