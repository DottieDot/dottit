package api

import (
	"dottit/comment_service/database"
	"dottit/comment_service/model"
	"strconv"

	"github.com/graph-gophers/graphql-go"
)

type RootResolver struct {
	commentRepo database.CommentRepoInterface
}

func (self *RootResolver) Comments(args struct {
	ThreadId   graphql.ID
	Pagination Pagination
}) []*CommentResolver {
	threadId, _ := strconv.ParseUint(string(args.ThreadId), 10, 64)
	comments := self.commentRepo.GetCommentsByThreadId(threadId, int(args.Pagination.First), int(args.Pagination.Count))

	return self.createResolversForComment(comments)
}

func (self *RootResolver) ChildComments(args struct {
	ParentId   graphql.ID
	Pagination Pagination
}) []*CommentResolver {
	parentId, _ := strconv.ParseUint(string(args.ParentId), 10, 64)
	comments := self.commentRepo.GetCommentsByParentId(parentId, int(args.Pagination.First), int(args.Pagination.Count))

	return self.createResolversForComment(comments)
}

func (self *RootResolver) CreateComment(args struct {
	ThreadId graphql.ID
	ParentId *graphql.ID
	User     string
	Text     string
}) *CommentResolver {
	var parentId *uint64 = nil
	if args.ParentId != nil {
		tmp, _ := strconv.ParseUint(string(*args.ParentId), 10, 64)
		parentId = &tmp
	}
	threadId, _ := strconv.ParseUint(string(args.ThreadId), 10, 64)

	comment := self.commentRepo.CreateComment(threadId, parentId, args.User, args.Text)

	return &CommentResolver{commentRepo: self.commentRepo, comment: comment}
}

func (self *RootResolver) DeleteComment(args struct {
	CommentId string
}) bool {
	commentId, _ := strconv.ParseUint(args.CommentId, 10, 64)

	self.commentRepo.DeleteComment(commentId)

	return true
}

func NewRootResolver(commentRepo database.CommentRepoInterface) RootResolver {
	return RootResolver{
		commentRepo: commentRepo,
	}
}

func (self *RootResolver) createResolversForComment(models []model.Comment) []*CommentResolver {
	result := make([]*CommentResolver, len(models))
	for i, comment := range models {
		result[i] = &CommentResolver{
			commentRepo: self.commentRepo,
			comment:     comment,
		}
	}

	return result
}
