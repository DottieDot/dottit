import '../styles/index.css'
import type { AppProps } from 'next/app'
import { ApolloProvider } from '@apollo/client'
import { apolloClient } from '../lib'
import { createTheme, CssBaseline, ThemeProvider } from '@mui/material'
import DialogProvider from '../components/DialogProvider'

const theme = createTheme({
  palette: {
    mode:    'dark',
    primary: { main: '#ff6d00' }
  }
})

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ApolloProvider client={apolloClient}>
      <ThemeProvider theme={theme}>
        <DialogProvider>
          <CssBaseline />

          <Component {...pageProps} />
        </DialogProvider>
      </ThemeProvider>
    </ApolloProvider>
  )
}
