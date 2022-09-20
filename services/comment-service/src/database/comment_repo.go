package database

import (
	"database/sql"
	"dottit/comment_service/model"

	"gorm.io/gorm"
)

type CommentRepoInterface interface {
	GetCommentById(commentId uint64) model.Comment
	GetCommentsByThreadId(threadId uint64, first int, count int) []model.Comment
	GetCommentsByParentId(parentId uint64, first int, count int) []model.Comment
	CreateComment(threadId uint64, parentId *uint64, user string, text string) model.Comment
	DeleteComment(id uint64)
}

type CommentRepo struct {
	db gorm.DB
}

func (repo CommentRepo) GetCommentById(commentId uint64) model.Comment {
	var comment model.Comment

	repo.db.First(&comment, commentId)

	return comment
}

func (repo CommentRepo) GetCommentsByThreadId(threadId uint64, first int, count int) []model.Comment {
	var comments []model.Comment

	repo.db.
		Order("score DESC").
		Where("parent_id IS NULL AND thread_id = '?'", threadId).
		Offset(first).
		Limit(count).
		Find(&comments)

	return comments
}

func (repo CommentRepo) GetCommentsByParentId(parentId uint64, first int, count int) []model.Comment {
	var comments []model.Comment

	repo.db.
		Order("score DESC").
		Where("parent_id = ?", parentId).
		Offset(first).
		Limit(count).
		Find(&comments)

	return comments
}

func (repo CommentRepo) CreateComment(threadId uint64, parentId *uint64, user string, text string) model.Comment {
	comment := model.Comment{
		User:     user,
		Text:     text,
		ThreadId: threadId,
		ParentId: sql.NullInt64 { Int64: int64(*parentId), Valid: parentId != nil },
	}

	repo.db.Create(&comment)

	return comment
}

func (repo CommentRepo) DeleteComment(id uint64) {
	repo.db.Delete(id)
}

func NewCommentRepo(db gorm.DB) CommentRepo {
	return CommentRepo{
		db: db,
	}
}
