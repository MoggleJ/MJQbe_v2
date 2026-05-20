from sqlalchemy import Column, Integer, ForeignKey, UniqueConstraint
from sqlalchemy.orm import relationship
from .base import Base


class Favorite(Base):
    __tablename__ = "favorites"
    __table_args__ = (UniqueConstraint("user_id", "app_id"),)

    id      = Column(Integer, primary_key=True)
    user_id = Column(Integer, ForeignKey("users.id", ondelete="CASCADE"), nullable=False)
    app_id  = Column(Integer, ForeignKey("apps.id",  ondelete="CASCADE"), nullable=False)

    user = relationship("User", back_populates="favorites")
    app  = relationship("App",  back_populates="favorites")
