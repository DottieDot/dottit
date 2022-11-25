import { User } from '../model'
import { createContext, ReactNode, useCallback, useState } from 'react'

export interface UserProviderProps {
  children: ReactNode
}

export type UserContextState = {
  loggedIn: false
  loading: boolean
  user: null
} | {
  loggedIn: boolean
  loading: true
  user: null
} | {
  loggedIn: true
  loading: false
  user: User
}

export interface UserContext {
  state: UserContextState
  setLoggedOut: () => void
  setLoggedIn: (key: string, user: User) => void
}

export const userContext = createContext<UserContext|null>(null)

export default function UserProvider({ children }: UserProviderProps) {
  const [ state, setState ] = useState<UserContextState>({
    loggedIn: false,
    loading:  false,
    user:     null
  })

  const setLoggedOut = useCallback<UserContext['setLoggedOut']>(() => {
    setState({
      loggedIn: false,
      loading:  false,
      user:     null
    })
  }, [ setState ])

  const setLoggedIn = useCallback<UserContext['setLoggedIn']>((key, user) => {
    setState({
      loggedIn: true,
      loading:  false,
      user
    })
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
