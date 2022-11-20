import { SettingsProvider } from './components'
import Theme from './Theme'
import { CssBaseline } from '@mui/material'
import Router from './Router'
import { ApolloProvider } from '@apollo/client'
import { apolloClient } from './lib'

function App() {
  return (
    <ApolloProvider client={apolloClient}>
      <SettingsProvider defaultSettings={{ theme: 'system' }}>
        <Theme>
          <CssBaseline />
          <Router />
        </Theme>
      </SettingsProvider>
    </ApolloProvider>
  )
}

export default App
