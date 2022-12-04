import { Unauthenticated, User } from '../../model'
import { gql } from '@apollo/client'

export type ResponseUser = Pick<User, 'id' | 'username'>

export type AuthenticatedUserResult
  = ({ __typename: 'User' } & ResponseUser)
  | ({ __typename: 'Unauthenticated' } & Unauthenticated)

export interface AuthenticatedUserResponse {
  authenticatedUser: AuthenticatedUserResult
}

export interface LoggedOut {
  message: string
}

export type LogoutUserResult
  = ({ __typename: 'LoggedOut' } & LoggedOut)
  | ({ __typename: 'Unauthenticated' } & Unauthenticated)

export interface LogoutUserResponse {
  logoutUser: LogoutUserResult
}

export const authenticatedUserQuery = gql`
  query AuthenticatedUser {
    authenticatedUser {
      __typename
      ... on User {
        id
        username
      }
      ... on Unauthenticated {
        message
      }
    }
  }
`

export const logoutUserQuery = gql`
  mutation LogoutUser {
    logoutUser {
      __typename
      ... on Unauthenticated {
        message
      }
      ... on LoggedOut {
        message
      }
    }
  }
`
