import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client'
import { Pagination } from '../model'

const cache = new InMemoryCache({
  typePolicies: {
    Query: {
      fields: {
        getThreadsByBoard: {
          // Don't cache separate results based on
          // any of this field's arguments.
          keyArgs: false,

          // Concatenate the incoming list items with
          // the existing list items.
          merge(existing: Pagination<unknown> | undefined, incoming: Pagination<unknown>) {
            console.log(existing, incoming)
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

const httpLink = createHttpLink({ uri: 'http://127.0.0.1:53083/graphql' })

const client = new ApolloClient({
  link:  httpLink,
  cache: cache
})

export default client
