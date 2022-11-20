import { gql, useLazyQuery } from '@apollo/client'
import { useEffect, useState } from 'react'
import ThreadCard from './ThreadCard'

const getThreadsQuery = gql`
  query GetThreadsByBoard($board: String!, $first: Int!, $count: Int!) {
    getThreadsByBoard(board: $board, first: $first, count: $count) {
      items {
        id
        score
        title
        text
        user
      }
      next
    }
  }
`

interface PostFeedProps {
  board: string
}

export default function ThreadFeed({ board }: PostFeedProps) {
  const [ threads, setThreads ] = useState<any[]>([])
  const [ next, setNext ] = useState<number | null>(0)
  const [ getThreadsApiCall, { loading, error, data }] = useLazyQuery(getThreadsQuery, {
    variables: {
      board,
      first: 0,
      count: 50
    }
  })

  useEffect(() => {
    getThreadsApiCall()
  }, [ getThreadsApiCall ])

  useEffect(() => {
    const queryResult = data?.getThreadsByBoard
    if (!queryResult) return

    setThreads(t => [
      ...t,
      ...queryResult.items
    ])
    setNext(queryResult.next)
  }, [ data ])

  return (
    <div>
      {threads.map(thread => (
        <ThreadCard key={thread.id} thread={thread} />
      ))}
    </div>
  )
}
