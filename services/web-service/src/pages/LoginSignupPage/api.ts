import { AlreadyLoggedIn, AuthenticatedUser, LoginFailed, User, ValidationError } from '../../model'
import { gql } from '@apollo/client'

export type ResponseUser = AuthenticatedUser<Pick<User, 'id' | 'username'>>

export type CreateUserResult
  = ({ __typename: 'AuthenticatedUser' } & ResponseUser)
  | ({ __typename: 'ValidationError' } & ValidationError)
  | ({ __typename: 'AlreadyLoggedIn' } & AlreadyLoggedIn)

export type LoginResult
  = ({ __typename: 'AuthenticatedUser' } & ResponseUser)
  | ({ __typename: 'LoginFailed' } & LoginFailed)
  | ({ __typename: 'AlreadyLoggedIn' } & AlreadyLoggedIn)

export interface CreateUserResponse {
  createUser: CreateUserResult
}

export interface LoginResponse {
  loginUser: LoginResult
}

export const createUserQuery = gql`
    mutation CreateUser($username: String!, $password: String!) {
      createUser(username: $username, password: $password) {
        __typename
        ... on AuthenticatedUser {
          apiToken
          user {
            id
            username
          }
        },
        ... on ValidationError {
          message
          errors {
            field
            errors
          }
        },
        ... on AlreadyLoggedIn {
          message
        }
      }
    }
`

export const loginQuery = gql`
  mutation LoginUser($username: String!, $password: String!) {
    loginUser(username: $username, password: $password) {
      __typename,
      ... on AuthenticatedUser {
        apiToken
        user {
          id
          username
        }
      },
      ... on LoginFailed {
        message
      },
      ... on AlreadyLoggedIn {
        message
      }
    }
  }
`
