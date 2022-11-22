import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client'
import { Pagination } from '../model'

const cache = new InMemoryCache({
  typePolicies: {
    Query: {
      fields: {
        getThreadsByBoard: {
          keyArgs: [ 'board' ],

          merge(existing: Pagination<unknown> | undefined, incoming: Pagination<unknown>) {
            return {
              next:  incoming.next,
              items: [
                ...(existing?.items ?? []),
                ...incoming.items
              ]
            }
          }
        }
      }
    }
  }
})

const httpLink = createHttpLink({ uri: 'http://127.0.0.1:63377/graphql' })

const client = new ApolloClient({
  link:  httpLink,
  cache: cache
})

export default client
