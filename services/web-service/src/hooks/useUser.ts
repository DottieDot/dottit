import { useContext } from 'react'
import { userContext } from '../components/UserProvider'

export function useUser() {
  const context = useContext(userContext)

  if (context === null) {
    throw new Error('invalid user context')
  }

  return context
}
