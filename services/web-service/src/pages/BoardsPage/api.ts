import { gql } from '@apollo/client'
import { Board } from '../../model/Board'
import { Page, Unauthenticated, ValidationError } from '../../model'

export type GetBoardsResponseBoard = Pick<Board, 'id' | 'name'>

export type CreateBoardResponseBoard = Pick<Board, 'name'>

export interface GetBoardsResponse {
  boards: Page<GetBoardsResponseBoard, number>
}

export type CreateBoardsResult
  = ({ __typename: 'Board' } & CreateBoardResponseBoard)
  | ({ __typename: 'Unauthenticated' } & Unauthenticated)
  | ({ __typename: 'ValidationError' } & ValidationError)

export interface CreateBoardResponse {
  createBoard: CreateBoardsResult
}

export const getBoardsQuery = gql`
  query getBoards($first: Int!, $count: Int!) {
    boards(first: $first, count: $count) {
      items {
        id,
        name
      }
      next
      total
    }
  }
`

export const createBoardMutation = gql`
  mutation CreateBoard($name: String!) {
    createBoard(name: $name) {
      __typename
      ... on Unauthenticated {
        message
      }
      ... on ValidationError {
        errors {
          field
          errors
        }
        message
      }
      ... on Board {
        name
      }
    }
  }`

