import { Board } from '../../model'
import { gql } from '@apollo/client'

export type ResponseBoard = Pick<Board, 'id' | 'name'>

export interface GetBoardByNameResponse {
  getBoardByName: ResponseBoard | null
}

export const getBoardByNameQuery = gql`
  query GetBoardByName($name: String!) {
    getBoardByName(name: $name) {
      id
      name
    }
  }
`
