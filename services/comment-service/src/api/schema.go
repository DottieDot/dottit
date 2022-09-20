package api

const Schema = `
schema {
	query: Query
	mutation: Mutation
}

type Query {
	comments(threadId: ID!, pagination: Pagination!): [Comment]!
	childComments(parentId: ID!, pagination: Pagination!): [Comment]!
}

type Mutation {
	createComment(threadId: ID!, parentId: ID, user: String!, text: String!): Comment!
	deleteComment(commentId: ID!): Boolean!
}

scalar Time

type Comment {
	Id: ID!
	User: String!
	Text: String!
	Parent: Comment
	Score: Int!
	CreatedAt: Time!
}

input Pagination {
	first: Int!
	count: Int!
}
`

type Pagination struct {
	First int32
	Count int32
}
