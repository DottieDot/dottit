import { Pagination, Thread } from '../../model'
import { gql } from '@apollo/client'

export type ResponseThread = Pick<Thread, 'id' | 'score' | 'title' | 'text' | 'user'>

export interface ResponseData {
  getThreadsByBoard: Pagination<ResponseThread>
}

export interface RequestVariables {
  board: string,
  first: number,
  count: number
}

export const query = gql`
  query GetThreadsByBoard($board: String!, $first: Int!, $count: Int!) {
    getThreadsByBoard(board: $board, first: $first, count: $count) {
      items {
        id
        score
        title
        text
        user
      }
      next
    }
  }
`

