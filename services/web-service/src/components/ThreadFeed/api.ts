import { Page, Thread } from '../../model'
import { gql } from '@apollo/client'

export type ResponseThread = Pick<Thread, 'id' | 'title' | 'text' | 'userId'>

export interface ResponseData {
  getThreadsByBoard: Page<ResponseThread, Date>
}

export interface RequestVariables {
  board: string,
  first: number,
  count: number
}

export const query = gql`
  query GetThreadsByBoard($board: UUID!, $first: DateTime!, $count: Int!) {
    getThreadsByBoard(boardId: $board, first: $first, count: $count) {
      items {
        id
        user
        title
        text
      }
      next
    }
  }
`

