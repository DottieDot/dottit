package api

import (
	"dottit/comment_service/database"
	"dottit/comment_service/model"
	"fmt"

	"github.com/graph-gophers/graphql-go"
)

type CommentResolver struct {
	commentRepo database.CommentRepoInterface
	comment     model.Comment
}

func (resolver *CommentResolver) Id() graphql.ID {
	return graphql.ID(fmt.Sprint(resolver.comment.ID))
}

func (resolver *CommentResolver) User() string {
	return resolver.comment.User
}

func (resolver *CommentResolver) Text() string {
	return resolver.comment.Text
}

func (resolver *CommentResolver) Parent() *CommentResolver {
	comment := resolver.commentRepo.GetCommentById(resolver.comment.ID)

	return &CommentResolver{
		commentRepo: resolver.commentRepo,
		comment:     comment,
	}
}

func (resolver *CommentResolver) Score() int32 {
	return int32(resolver.comment.Score)
}

func (resolver *CommentResolver) CreatedAt() graphql.Time {
	return graphql.Time{Time: resolver.comment.CreatedAt}
}
