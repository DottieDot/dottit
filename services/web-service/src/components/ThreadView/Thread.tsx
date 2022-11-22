import { useQuery } from '@apollo/client'
import { query, ResponseData } from './api'
import { Card, CardContent, Container, Typography } from '@mui/material'
import ThreadHeader from '../ThreadHeader'
import { Fragment, useCallback } from 'react'
import Comments from './Comments'

interface Props {
  threadId: string
}

export default function Thread({ threadId }: Props) {
  const { data, loading, fetchMore } = useQuery<ResponseData>(query, {
    variables: {
      threadId,
      firstComment: 0,
      commentCount: 50
    }
  })

  const next = data?.getThreadById.comments.next

  const handleLoadMore = useCallback(() => {
    if (!loading && next) {
      // noinspection JSIgnoredPromiseFromCall
      fetchMore({ variables: { firstComment: next }})
    }
  }, [ loading, next, fetchMore ])

  if (loading || !data?.getThreadById) {
    return null
  }

  const thread = data.getThreadById

  return (
    <Fragment>
      <ThreadHeader
        thread={thread.title}
      />

      <Container maxWidth="md">
        <Card sx={{ mt: 2 }}>
          <CardContent>
            <Typography
              sx={{ whiteSpace: 'pre-wrap' }}
              variant="body1"
            >
              {thread.text}
            </Typography>
          </CardContent>
        </Card>

        <Typography sx={{ mt: 2 }} variant="h5">
          Comments
        </Typography>

        <Comments comments={thread.comments.items} loadMore={handleLoadMore} sx={{ py: 2 }} />
      </Container>
    </Fragment>
  )
}
