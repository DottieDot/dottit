import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client'
import { Page } from '../model'
import { setContext } from '@apollo/client/link/context'

function paginationMerger(existing: Page<unknown, unknown> | undefined, incoming: Page<unknown, unknown>) {
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

const httpLink = createHttpLink({ uri: `http://${process.env.REACT_APP_API_ADDRESS}/graphql` })

const client = new ApolloClient({
  link:           authLink.concat(httpLink),
  cache:          cache,
  defaultOptions: { query: { fetchPolicy: 'no-cache' }}
})

export default client
