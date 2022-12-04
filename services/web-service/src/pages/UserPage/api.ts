import { gql } from '@apollo/client'
import { User } from '../../model'

export type ResponseUser = Pick<User, 'username'>

export interface GetUserByUsernameResponse {
  getUserByUsername: ResponseUser | null
}

export const getUserByUsernameQuery = gql`
  query GetUserByUsername($username: String!) {
    getUserByUsername(username: $username) {
      username
    }
  }
`
