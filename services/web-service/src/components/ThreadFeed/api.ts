import { BoardThreads, Thread } from '../../model'
import { gql } from '@apollo/client'

export type ResponseThread = Pick<Thread, 'id' | 'title' | 'text' | 'userId'>

export type ResponseBoard = BoardThreads<ResponseThread>

export interface ResponseData {
  getBoardByName: ResponseBoard
}

export interface RequestVariables {
  board: string,
  first: Date,
  count: number
}

export const query = gql`
  query GetThreadsByBoard($board: String!, $first: DateTime!, $count: Int!) {
    getBoardByName(name: $board) {
      threads(first: $first, count: $count) {
        items {
          id
          user
          title
          text
        }
        next
      }
    }
  }
`

