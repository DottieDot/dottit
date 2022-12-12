import { Thread, Unauthenticated, ValidationError } from '../../model'
import { gql } from '@apollo/client'

export type ResponseThread = Pick<Thread, 'id'>

export type CreateThreadResult
  = ({ __typename: 'Thread' } & ResponseThread)
  | ({ __typename: 'ValidationError' } & ValidationError)
  | ({ __typename: 'Unauthenticated' } & Unauthenticated)

export interface CreateThreadResponse {
  createThread: CreateThreadResult
}

export const createThreadQuery = gql`
  mutation CreateThread($boardId: UUID!, $title: String!, $text: String!) {
    createThread(boardId: $boardId, title: $title, text: $text) {
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
      ... on Thread {
        id
      }
    }
  }
`
