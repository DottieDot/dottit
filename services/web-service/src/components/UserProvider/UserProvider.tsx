import { User } from '../../model'
import { createContext, ReactNode, useCallback, useEffect, useState } from 'react'
import { useApolloClient, useLazyQuery, useMutation } from '@apollo/client'
import { authenticatedUserQuery, AuthenticatedUserResponse, logoutUserQuery, LogoutUserResponse } from './api'

export interface UserProviderProps {
  children: ReactNode
}

export type UserContextState = {
  loggedIn: true,
  user: null | User
  apiToken: string
} | {
  loggedIn: false
  user: null
  apiToken: null
}

export interface UserContext {
  state: UserContextState
  setLoggedOut: () => void
  setLoggedIn: (apiKey: string, user: User) => void
}

export const userContext = createContext<UserContext|null>(null)

export default function UserProvider({ children }: UserProviderProps) {
  const client = useApolloClient()
  const [ logout ] = useMutation<LogoutUserResponse>(logoutUserQuery)
  const [ getAuthenticatedUser ] = useLazyQuery<AuthenticatedUserResponse>(authenticatedUserQuery)

  const [ state, setState ] = useState<UserContextState>({
    loggedIn: false,
    user:     null,
    apiToken: null
  })

  useEffect(() => {
    const token = localStorage.getItem('api_token')
    if (token) {
      setState(state => ({
        ...state,
        loggedIn: true,
        apiToken: token
      }))

      getAuthenticatedUser().then(response => {
        const data = response.data?.authenticatedUser
        if (!data) {
          setState({
            loggedIn: false,
            user:     null,
            apiToken: null
          })
          localStorage.removeItem('api_token')
        } else {
          switch (data.__typename) {
          case 'User':
            setState({
              loggedIn: true,
              user:     data,
              apiToken: token
            })
            break
          case 'Unauthenticated':
            setState({
              loggedIn: false,
              user:     null,
              apiToken: null
            })
            localStorage.removeItem('api_token')
            break
          }
        }

        client.resetStore().then()
      })
    }
  }, [ client, getAuthenticatedUser, setState ])

  const setLoggedOut = useCallback<UserContext['setLoggedOut']>(() => {
    setState({
      loggedIn: false,
      user:     null,
      apiToken: null
    })

    localStorage.removeItem('api_token')
    logout().then(() => {
      client.resetStore().then()
    })
  }, [ client, logout, setState ])

  const setLoggedIn = useCallback<UserContext['setLoggedIn']>((key, user) => {
    setState({
      loggedIn: true,
      apiToken: key,
      user
    })
    localStorage.setItem('api_token', key)
  }, [ setState ])

  return (
    <userContext.Provider
      value={{
        state,
        setLoggedOut,
        setLoggedIn
      }}
    >
      {children}
    </userContext.Provider>
  )
}
