import { Comment, Unauthenticated, Unauthorized, ValidationError } from '../../model'
import { gql } from '@apollo/client'

export type ResponseComment = Pick<Comment, 'id'>

export type CreateCommentResult
  = ({ __typename: 'Comment' } & ResponseComment)
  | ({ __typename: 'ValidationError' } & ValidationError)
  | ({ __typename: 'Unauthenticated' } & Unauthenticated)

export interface CreateCommentResponse {
  createComment: CreateCommentResult
}

export const createCommentQuery = gql`
  mutation CreateThread($threadId: UUID!, $text: String!) {
    createComment(threadId: $threadId, text: $text) {
      __typename,
      ... on Unauthenticated {
        message
      }
      ... on ValidationError {
        errors {
          errors
          field
        }
        message
      }
      ... on Comment {
        id
      }
    }
  }
`
