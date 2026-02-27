-- Drop in dependency order
DROP TABLE IF EXISTS public.game_assets CASCADE;
DROP TABLE IF EXISTS public.asset_groups CASCADE;
DROP TABLE IF EXISTS public.anticheat_log CASCADE;
DROP TABLE IF EXISTS public.game_sessions CASCADE;
DROP TABLE IF EXISTS public.game_events CASCADE;
DROP TABLE IF EXISTS public.profiles CASCADE;
DROP FUNCTION IF EXISTS public.handle_new_user() CASCADE;
DROP VIEW IF EXISTS public.leaderboard_view;

-- PROFILES
CREATE TABLE public.profiles (
    id         UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    username   TEXT UNIQUE NOT NULL,
    is_banned  BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT now()
);

CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.profiles (id, username)
    VALUES (NEW.id, COALESCE(NEW.raw_user_meta_data->>'display_name', 'user_' || LEFT(NEW.id::text, 8)));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- BACKFILL EXISTING AUTH USERS INTO PROFILES
INSERT INTO public.profiles (id, username)
SELECT
    u.id,
    COALESCE(
        u.raw_user_meta_data->>'display_name',
        'user_' || LEFT(u.id::text, 8)
    )
FROM auth.users u
LEFT JOIN public.profiles p ON p.id = u.id
WHERE p.id IS NULL;

CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_user();

-- GAME SESSIONS
CREATE TABLE public.game_sessions (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID NOT NULL REFERENCES public.profiles(id) ON DELETE CASCADE,
    game_code    TEXT NOT NULL,
    status       TEXT NOT NULL DEFAULT 'in_progress',
    score        DOUBLE PRECISION,
    started_at   TIMESTAMPTZ DEFAULT now(),
    completed_at TIMESTAMPTZ
);

-- ANTI-CHEAT LOG
CREATE TABLE public.anticheat_log (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID NOT NULL REFERENCES public.profiles(id) ON DELETE CASCADE,
    session_id UUID REFERENCES public.game_sessions(id),
    action     TEXT NOT NULL,
    reason     TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

-- ASSET GROUPS
CREATE TABLE public.asset_groups (
    id         SERIAL PRIMARY KEY,
    game_code  TEXT NOT NULL,
    label      TEXT NOT NULL
);

-- GAME ASSETS
CREATE TABLE public.game_assets (
    id         SERIAL PRIMARY KEY,
    group_id   INT NOT NULL REFERENCES public.asset_groups(id) ON DELETE CASCADE,
    label      TEXT NOT NULL,
    image_url  TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL DEFAULT FALSE
);

-- INDEXES
CREATE INDEX idx_sessions_user  ON public.game_sessions(user_id);
CREATE INDEX idx_anticheat_user ON public.anticheat_log(user_id);
CREATE INDEX idx_groups_game    ON public.asset_groups(game_code);
CREATE INDEX idx_assets_group   ON public.game_assets(group_id);

-- CONSTRAINTS
-- Ensure at most one correct asset per group
CREATE UNIQUE INDEX idx_one_correct_per_group ON public.game_assets(group_id) WHERE is_correct = true;

CREATE VIEW public.leaderboard_view AS
SELECT
    gs.game_code,
    gs.score,
    gs.completed_at,
    p.id AS user_id,
    p.username
FROM public.game_sessions gs
JOIN public.profiles p ON p.id = gs.user_id
WHERE gs.status = 'completed' AND gs.score IS NOT NULL;

CREATE INDEX idx_sessions_leaderboard
    ON public.game_sessions(game_code, score DESC)
    WHERE status = 'completed' AND score IS NOT NULL;
