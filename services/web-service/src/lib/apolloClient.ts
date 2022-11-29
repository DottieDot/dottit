import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client'
import { Page } from '../model'

function paginationMerger(existing: Page<unknown> | undefined, incoming: Page<unknown>) {
  return {
    next:  incoming.next,
    items: [
      ...(existing?.items ?? []),
      ...incoming.items
    ]
  }
}

const cache = new InMemoryCache({
  typePolicies: {
    Query: {
      fields: {
        getThreadsByBoard: {
          keyArgs: [ 'board' ],
          merge:   paginationMerger
        }
      }
    },
    Thread: {
      fields: {
        comments: {
          keyArgs: false,
          merge:   paginationMerger
        }
      }
    }
  }
})

const httpLink = createHttpLink({ uri: 'http://127.0.0.1:52810/graphql' })

const client = new ApolloClient({
  link:  httpLink,
  cache: cache
})

export default client
