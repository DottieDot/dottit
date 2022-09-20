package model

import (
	"time"

	"gorm.io/gorm"
)

type Comment struct {
	gorm.Model
	ID        uint64
	User      string
	Text      string
	ParentId  *uint64
	ThreadId  uint64
	Score     int `gorm:"default:0"`
	CreatedAt time.Time
}
