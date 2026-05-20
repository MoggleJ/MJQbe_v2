from sqlalchemy import Column, Integer, String, DateTime, func
from sqlalchemy.orm import relationship
from .base import Base


class User(Base):
    __tablename__ = "users"

    id             = Column(Integer, primary_key=True)
    username       = Column(String(64), unique=True, nullable=False)
    email          = Column(String(255), unique=True)
    password_hash  = Column(String(255))
    oauth_provider = Column(String(32))
    oauth_id       = Column(String(255))
    role           = Column(String(16), nullable=False, server_default="user")
    created_at     = Column(DateTime, nullable=False, server_default=func.now())
    last_login     = Column(DateTime)

    settings  = relationship("Settings", back_populates="user", uselist=False, cascade="all, delete-orphan")
    favorites = relationship("Favorite", back_populates="user", cascade="all, delete-orphan")
    logs      = relationship("Log", back_populates="user")
