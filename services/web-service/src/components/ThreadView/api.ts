import { Page, Thread, Comment, CommentUser, User, ThreadUser } from '../../model'
import { gql } from '@apollo/client'

export type ResponseComment = Pick<Comment, 'id' | 'text'> & CommentUser<Pick<User, 'username'>>

export type ResponseThread = Pick<Thread, 'id' | 'title' | 'text'> & {
  comments: Page<ResponseComment, number>
} & ThreadUser<Pick<User, 'username'>>

export interface ResponseData {
  getThreadById: ResponseThread
}

export const query = gql`
  query GetThreadById($threadId: UUID!, $firstComment: DateTime!, $commentCount: Int!) {
    getThreadById(threadId: $threadId) {
      comments(first: $firstComment, count: $commentCount) {
        items {
          id
          text
          user {
            username
          }
        }
        next
      }
      id
      text
      title
      user {
        username
      }
    }
  }
`
