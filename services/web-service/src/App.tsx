import { SettingsProvider } from './components'
import Theme from './Theme'
import { CssBaseline } from '@mui/material'
import Router from './Router'
import { ApolloProvider } from '@apollo/client'
import { apolloClient } from './lib'
import UserProvider from './components/UserProvider'

function App() {
  return (
    <ApolloProvider client={apolloClient}>
      <UserProvider>
        <SettingsProvider defaultSettings={{ theme: 'system' }}>
          <Theme>
            <CssBaseline />
            <Router />
          </Theme>
        </SettingsProvider>
      </UserProvider>
    </ApolloProvider>
  )
}

export default App
