"""initial schema

Revision ID: 001
Revises:
Create Date: 2026-05-20
"""
from typing import Sequence, Union
from alembic import op
import sqlalchemy as sa
from sqlalchemy.dialects.postgresql import JSONB

revision: str = "001"
down_revision: Union[str, None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    op.create_table(
        "users",
        sa.Column("id",             sa.Integer(),     primary_key=True),
        sa.Column("username",       sa.String(64),    nullable=False),
        sa.Column("email",          sa.String(255)),
        sa.Column("password_hash",  sa.String(255)),
        sa.Column("oauth_provider", sa.String(32)),
        sa.Column("oauth_id",       sa.String(255)),
        sa.Column("role",           sa.String(16),    nullable=False, server_default="user"),
        sa.Column("created_at",     sa.DateTime(),    nullable=False, server_default=sa.text("NOW()")),
        sa.Column("last_login",     sa.DateTime()),
        sa.UniqueConstraint("username"),
        sa.UniqueConstraint("email"),
    )

    op.create_table(
        "categories",
        sa.Column("id",   sa.Integer(),    primary_key=True),
        sa.Column("name", sa.String(64),   nullable=False),
        sa.Column("mode", sa.String(16),   nullable=False),
        sa.UniqueConstraint("name", "mode"),
    )

    op.create_table(
        "apps",
        sa.Column("id",          sa.Integer(),    primary_key=True),
        sa.Column("name",        sa.String(128),  nullable=False),
        sa.Column("icon",        sa.String(255)),
        sa.Column("url",         sa.String(512)),
        sa.Column("category_id", sa.Integer(),    sa.ForeignKey("categories.id", ondelete="SET NULL")),
        sa.Column("mode",        sa.String(16),   nullable=False),
        sa.Column("is_web",      sa.Boolean(),    nullable=False, server_default="true"),
        sa.Column("is_active",   sa.Boolean(),    nullable=False, server_default="true"),
        sa.Column("created_at",  sa.DateTime(),   nullable=False, server_default=sa.text("NOW()")),
    )

    op.create_table(
        "settings",
        sa.Column("id",           sa.Integer(),   primary_key=True),
        sa.Column("user_id",      sa.Integer(),   sa.ForeignKey("users.id", ondelete="CASCADE"), unique=True),
        sa.Column("theme",        sa.String(32),  nullable=False, server_default="dark"),
        sa.Column("layout",       sa.String(16),  nullable=False, server_default="grid"),
        sa.Column("icon_size",    sa.String(8),   nullable=False, server_default="medium"),
        sa.Column("default_mode", sa.String(16),  nullable=False, server_default="tv"),
    )

    op.create_table(
        "favorites",
        sa.Column("id",      sa.Integer(), primary_key=True),
        sa.Column("user_id", sa.Integer(), sa.ForeignKey("users.id", ondelete="CASCADE"), nullable=False),
        sa.Column("app_id",  sa.Integer(), sa.ForeignKey("apps.id",  ondelete="CASCADE"), nullable=False),
        sa.UniqueConstraint("user_id", "app_id"),
    )

    op.create_table(
        "logs",
        sa.Column("id",         sa.Integer(),   primary_key=True),
        sa.Column("user_id",    sa.Integer(),   sa.ForeignKey("users.id", ondelete="SET NULL")),
        sa.Column("action",     sa.String(32),  nullable=False),
        sa.Column("metadata",   JSONB()),
        sa.Column("created_at", sa.DateTime(),  nullable=False, server_default=sa.text("NOW()")),
    )

    # Index recommandés (docs/data-model.md §3)
    op.create_index("idx_users_email",     "users",     ["email"])
    op.create_index("idx_users_oauth",     "users",     ["oauth_provider", "oauth_id"])
    op.create_index("idx_apps_mode",       "apps",      ["mode"])
    op.create_index("idx_apps_category",   "apps",      ["category_id"])
    op.create_index("idx_favorites_user",  "favorites", ["user_id"])
    op.create_index("idx_logs_user_time",  "logs",      ["user_id", sa.text("created_at DESC")])
    op.create_index("idx_logs_time",       "logs",      [sa.text("created_at DESC")])


def downgrade() -> None:
    op.drop_table("logs")
    op.drop_table("favorites")
    op.drop_table("settings")
    op.drop_table("apps")
    op.drop_table("categories")
    op.drop_table("users")
