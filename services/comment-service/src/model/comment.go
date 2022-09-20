package model

import (
	"database/sql"
	"time"

	"gorm.io/gorm"
)

type Comment struct {
	gorm.Model
	ID        uint64
	User      string
	Text      string
	ParentId  sql.NullInt64
	ThreadId  uint64
	Score     int `gorm:"default:0"`
	CreatedAt time.Time
}
