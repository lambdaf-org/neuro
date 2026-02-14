-- Drop in dependency order
DROP TABLE IF EXISTS public.game_assets CASCADE;
DROP TABLE IF EXISTS public.asset_groups CASCADE;
DROP TABLE IF EXISTS public.anticheat_log CASCADE;
DROP TABLE IF EXISTS public.scores CASCADE;
DROP TABLE IF EXISTS public.game_sessions CASCADE;
DROP TABLE IF EXISTS public.game_events CASCADE;
DROP TABLE IF EXISTS public.profiles CASCADE;
DROP FUNCTION IF EXISTS public.handle_new_user() CASCADE;

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

-- SCORES
CREATE TABLE public.scores (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES public.profiles(id) ON DELETE CASCADE,
    game_code   TEXT NOT NULL,
    session_id  UUID NOT NULL REFERENCES public.game_sessions(id),
    score       DOUBLE PRECISION NOT NULL,
    achieved_at TIMESTAMPTZ DEFAULT now(),
    UNIQUE(user_id, game_code)
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
CREATE INDEX idx_scores_user    ON public.scores(user_id);
CREATE INDEX idx_anticheat_user ON public.anticheat_log(user_id);
CREATE INDEX idx_groups_game    ON public.asset_groups(game_code);
CREATE INDEX idx_assets_group   ON public.game_assets(group_id);

