from sqlalchemy import Column, Integer, String, UniqueConstraint
from sqlalchemy.orm import relationship
from .base import Base


class Category(Base):
    __tablename__ = "categories"
    __table_args__ = (UniqueConstraint("name", "mode"),)

    id   = Column(Integer, primary_key=True)
    name = Column(String(64), nullable=False)
    mode = Column(String(16), nullable=False)

    apps = relationship("App", back_populates="category")
