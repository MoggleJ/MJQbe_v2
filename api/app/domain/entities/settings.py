from sqlalchemy import Column, Integer, String, ForeignKey
from sqlalchemy.orm import relationship
from .base import Base


class Settings(Base):
    __tablename__ = "settings"

    id           = Column(Integer, primary_key=True)
    user_id      = Column(Integer, ForeignKey("users.id", ondelete="CASCADE"), unique=True)
    theme        = Column(String(32), nullable=False, server_default="dark")
    layout       = Column(String(16), nullable=False, server_default="grid")
    icon_size    = Column(String(8),  nullable=False, server_default="medium")
    default_mode = Column(String(16), nullable=False, server_default="tv")

    user = relationship("User", back_populates="settings")
