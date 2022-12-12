import { useQuery } from '@apollo/client'
import { query, ResponseData } from './api'
import { Box, Card, CardContent, Container, Fab, Typography, useTheme } from '@mui/material'
import ThreadHeader from '../ThreadHeader'
import { Fragment, useCallback, useState } from 'react'
import Comments from './Comments'
import { Comment as NewCommentIcon } from '@mui/icons-material'
import { NewCommentDialog } from '../../dialogs'
import { useUser } from '../../hooks'

interface Props {
  threadId: string
}

export default function Thread({ threadId }: Props) {
  const theme = useTheme()
  const { data, loading, fetchMore } = useQuery<ResponseData>(query, {
    variables: {
      threadId,
      firstComment: new Date(),
      commentCount: 50
    }
  })
  const [ newComment, setNewComment ] = useState(false)
  const { state: user } = useUser()

  const next = data?.getThreadById.comments.next

  const handleLoadMore = useCallback(() => {
    if (!loading && next) {
      // noinspection JSIgnoredPromiseFromCall
      fetchMore({ variables: { firstComment: next }})
    }
  }, [ loading, next, fetchMore ])

  const handleNewCommentClick = useCallback(() => {
    setNewComment(true)
  }, [])

  if (loading || !data?.getThreadById) {
    return null
  }

  const thread = data.getThreadById

  return (
    <Fragment>
      <ThreadHeader
        thread={thread.title}
      />

      <Container
        maxWidth="md"
        sx={{
          display:       'flex',
          flexDirection: 'column',
          position:      'relative',
          flex:          1
        }}
      >
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


        <Box
          sx={{
            position: 'relative',
            flex:     1
          }}
        >
          {user.user && (
            <Fab
              color="secondary"
              onClick={handleNewCommentClick}
              sx={{
                position: 'absolute',
                bottom:   theme.spacing(2),
                right:    0
              }}
            >
              <NewCommentIcon />
            </Fab>
          )}
        </Box>
      </Container>

      <NewCommentDialog onClose={setNewComment} open={newComment} threadId={thread.id} />
    </Fragment>
  )
}
