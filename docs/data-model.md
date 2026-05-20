# Modèle de données — MJQbe v2

## 1. Schéma SQL complet

```sql
-- Utilisateurs web
CREATE TABLE users (
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(64) UNIQUE NOT NULL,
    email       VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255),          -- NULL si OAuth uniquement
    oauth_provider VARCHAR(32),          -- 'google' | 'github' | NULL
    oauth_id    VARCHAR(255),            -- ID chez le provider OAuth
    role        VARCHAR(16) NOT NULL DEFAULT 'user',  -- 'user' | 'admin'
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login  TIMESTAMP
);

-- Catégories d'applications
CREATE TABLE categories (
    id      SERIAL PRIMARY KEY,
    name    VARCHAR(64) NOT NULL,
    mode    VARCHAR(16) NOT NULL,        -- 'tv' | 'desktop' | 'dev'
    UNIQUE (name, mode)
);

-- Applications
CREATE TABLE apps (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(128) NOT NULL,
    icon        VARCHAR(255),            -- URL ou nom d'icône
    url         VARCHAR(512),            -- URL externe
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    mode        VARCHAR(16) NOT NULL,    -- 'tv' | 'desktop' | 'dev'
    is_web      BOOLEAN NOT NULL DEFAULT true,  -- true = ouvre dans onglet navigateur
    is_active   BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Paramètres par utilisateur
CREATE TABLE settings (
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    theme       VARCHAR(32) NOT NULL DEFAULT 'dark',
    -- Valeurs valides : dark, dark-blue, dark-purple, amoled, dark-green,
    --                   light, light-warm, light-blue, light-purple, light-green
    layout      VARCHAR(16) NOT NULL DEFAULT 'grid',  -- 'grid' | 'list'
    icon_size   VARCHAR(8) NOT NULL DEFAULT 'medium', -- 'small' | 'medium' | 'large'
    default_mode VARCHAR(16) NOT NULL DEFAULT 'tv'    -- 'tv' | 'desktop'
);

-- Applications favorites par utilisateur
CREATE TABLE favorites (
    id      SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    app_id  INTEGER NOT NULL REFERENCES apps(id) ON DELETE CASCADE,
    UNIQUE (user_id, app_id)
);

-- Logs des actions web
CREATE TABLE logs (
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER REFERENCES users(id) ON DELETE SET NULL,
    action      VARCHAR(32) NOT NULL,   -- 'app_launch' | 'login' | 'logout' | 'settings_change'
    metadata    JSONB,                  -- données contextuelles (app_id, ip, etc.)
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);
```

---

## 2. ERD (Entity Relationship Diagram)

```
users ──────────── settings (1:1)
  │
  ├─────────────── favorites (1:N) ──── apps
  │
  └─────────────── logs (1:N)

apps ───────────── categories (N:1)
```

---

## 3. Index recommandés

```sql
-- Recherche par username / email (login)
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_oauth ON users(oauth_provider, oauth_id);

-- Filtrage des apps par mode et catégorie
CREATE INDEX idx_apps_mode ON apps(mode);
CREATE INDEX idx_apps_category ON apps(category_id);

-- Lookup des favoris par utilisateur
CREATE INDEX idx_favorites_user ON favorites(user_id);

-- Logs récents par utilisateur
CREATE INDEX idx_logs_user_time ON logs(user_id, created_at DESC);
CREATE INDEX idx_logs_time ON logs(created_at DESC);
```

---

## 4. Notes

- `settings` est en relation 1:1 avec `users` : créé automatiquement à l'inscription avec les valeurs par défaut.
- `password_hash` est NULL pour les utilisateurs OAuth — ils ne peuvent pas se connecter avec un mot de passe.
- `apps.is_web` détermine si l'app s'ouvre dans un onglet (`true`) ou est intégrée en iframe (`false`).
- `logs.metadata` est en JSONB pour stocker des données variables sans modifier le schéma (ex: `{"app_id": 3, "ip": "192.168.1.10"}`).
- Le mode `dev` n'a pas d'utilisateurs web — il est local uniquement. Les tables `categories` et `apps` avec `mode='dev'` existent mais ne sont pas exposées en web.
