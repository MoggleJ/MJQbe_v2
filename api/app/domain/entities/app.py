from sqlalchemy import Column, Integer, String, Boolean, DateTime, ForeignKey, func
from sqlalchemy.orm import relationship
from .base import Base


class App(Base):
    __tablename__ = "apps"

    id          = Column(Integer, primary_key=True)
    name        = Column(String(128), nullable=False)
    icon        = Column(String(255))
    url         = Column(String(512))
    category_id = Column(Integer, ForeignKey("categories.id", ondelete="SET NULL"))
    mode        = Column(String(16), nullable=False)
    is_web      = Column(Boolean, nullable=False, server_default="true")
    is_active   = Column(Boolean, nullable=False, server_default="true")
    created_at  = Column(DateTime, nullable=False, server_default=func.now())

    category  = relationship("Category", back_populates="apps")
    favorites = relationship("Favorite", back_populates="app", cascade="all, delete-orphan")
