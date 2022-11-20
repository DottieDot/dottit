import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client'

const httpLink = createHttpLink({ uri: 'http://127.0.0.1:53083/graphql' })

const client = new ApolloClient({
  link:  httpLink,
  cache: new InMemoryCache()
})

export default client
