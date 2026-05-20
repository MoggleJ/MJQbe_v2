from sqlalchemy import Column, Integer, String, DateTime, ForeignKey, func
from sqlalchemy.orm import relationship
from sqlalchemy.dialects.postgresql import JSONB
from .base import Base


class Log(Base):
    __tablename__ = "logs"

    id         = Column(Integer, primary_key=True)
    user_id    = Column(Integer, ForeignKey("users.id", ondelete="SET NULL"))
    action     = Column(String(32), nullable=False)
    meta       = Column("metadata", JSONB)
    created_at = Column(DateTime, nullable=False, server_default=func.now())

    user = relationship("User", back_populates="logs")
