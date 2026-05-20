from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
import os


def get_database_url() -> str:
    user     = os.getenv("POSTGRES_USER",     "mjqbe")
    password = os.getenv("POSTGRES_PASSWORD", "mjqbe")
    host     = os.getenv("POSTGRES_HOST",     "db")
    port     = os.getenv("POSTGRES_PORT",     "5432")
    db       = os.getenv("POSTGRES_DB",       "mjqbe")
    return f"postgresql://{user}:{password}@{host}:{port}/{db}"


engine       = create_engine(get_database_url(), pool_pre_ping=True)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
