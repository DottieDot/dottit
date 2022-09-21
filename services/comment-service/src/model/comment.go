package model

import (
	"time"

	"github.com/Thor-x86/nullable"
)

type Comment struct {
	ID              uint64
	User            string
	Text            string
	ParentCommentID nullable.Uint64
	ParentComment   *Comment `gorm:"constraint:OnUpdate:CASCADE,OnDelete:CASCADE;"`
	ThreadId        uint64
	Score           int `gorm:"default:0"`
	CreatedAt       time.Time
}
