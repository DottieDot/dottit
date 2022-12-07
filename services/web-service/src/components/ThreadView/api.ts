import { Page, Thread, Comment } from '../../model'
import { gql } from '@apollo/client'

export type ResponseComment = Pick<Comment, 'id' | 'score' | 'text' | 'user'>

export type ResponseThread = Pick<Thread, 'id' | 'title' | 'text'  | 'userId'> & {
  comments: Page<ResponseComment, number>
}

export interface ResponseData {
  getThreadById: ResponseThread
}

export const query = gql`
  query GetThreadById($threadId: ID!, $firstComment: Int!, $commentCount: Int!) {
    getThreadById(threadId: $threadId) {
      comments(first: $firstComment, count: $commentCount) {
        items {
          id
          score
          text
          user
        }
        next
      }
      id
      media
      score
      text
      title
      user
    }
  }
`
