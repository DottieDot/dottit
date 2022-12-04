import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client'
import { Page } from '../model'
import { setContext } from '@apollo/client/link/context'

function paginationMerger(existing: Page<unknown> | undefined, incoming: Page<unknown>) {
  return {
    next:  incoming.next,
    items: [
      ...(existing?.items ?? []),
      ...incoming.items
    ]
  }
}

const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem('api_token')
  if (token) {
    return {
      headers: {
        ...headers,
        authorization: token ? `Bearer ${token}` : ''
      }
    }
  } else {
    return headers
  }
})

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

const httpLink = createHttpLink({ uri: 'http://127.0.0.1:64344/graphql' })

const client = new ApolloClient({
  link:  authLink.concat(httpLink),
  cache: cache
})

export default client
